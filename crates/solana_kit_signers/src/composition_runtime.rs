/// Validated automatic composition over heterogeneous registered backends.
///
/// Modifiers run in declaration order, followed by concurrent partial operations
/// over one immutable final snapshot. Results merge in declaration order. Dropping
/// this future drops unfinished operations; no task or runtime is started here.
pub struct Composition<'a, Failure = Error> {
    registrations: Vec<Registration<'a, Failure>>,
    duplicate_sender_only: bool,
}
impl<'a, Failure: From<Error>> Composition<'a, Failure> {
    /// Validates at most 256 registrations and deduplicates shared logical handles.
    ///
    /// # Errors
    /// Rejects distinct same-address handles and inconsistent declarations before effects.
    pub fn with_failure(
        registrations: impl Into<Vec<Registration<'a, Failure>>>,
    ) -> Result<Self, Error> {
        let registrations = registrations.into();

        if registrations.len() > MAX_ITEMS {
            return Err(Error::TooManySigners);
        }

        let metadata: Vec<_> = registrations.iter().map(|r| r.info).collect();
        let unique = validate_registrations(&metadata)?;
        let sender_only_count = metadata
            .iter()
            .filter(|d| d.interfaces.sending && !d.interfaces.partial && !d.interfaces.modifying)
            .count();
        let mut inputs = registrations.into_iter().map(Some).collect::<Vec<_>>();
        let registrations = unique
            .into_iter()
            .filter_map(|index| inputs[index].take())
            .collect();
        Ok(Self {
            registrations,
            duplicate_sender_only: sender_only_count > 1,
        })
    }

    /// Selected deduplicated roles, in declaration order, before backend effects.
    ///
    /// # Errors
    /// Sending requires a sender and at most one sender-only registration, including
    /// repeated references checked before deduplication as in the original Dart.
    pub fn roles(&self, sending: bool) -> Result<Vec<(Address, Role)>, Error> {
        if sending && self.duplicate_sender_only {
            return Err(Error::MultipleSenders);
        }

        select_roles(
            &self
                .registrations
                .iter()
                .map(|r| r.info)
                .collect::<Vec<_>>(),
            sending,
        )
    }

    /// Modifies sequentially and partially signs the final snapshot concurrently.
    ///
    /// # Errors
    /// Propagates backend failures and rejects tampered or foreign authorization.
    pub async fn partially_sign(&self, tx: &Transaction) -> Result<Transaction, Failure> {
        let roles = self.roles(false)?;
        self.apply(tx, &roles).await
    }
    async fn apply(
        &self,
        tx: &Transaction,
        roles: &[(Address, Role)],
    ) -> Result<Transaction, Failure> {
        let mut current = tx.clone();

        for (registration, (_, role)) in self.registrations.iter().zip(roles) {
            if *role == Role::Modifying {
                let operation = registration
                    .modify
                    .as_ref()
                    .ok_or(Error::ConflictingSigner)?;
                let next = operation.modify(&current).await?;
                crate::pipeline::validate_modification(&current, &next, registration.info.address)?;
                current = next;
            }
        }

        let partials: Vec<_> = self
            .registrations
            .iter()
            .zip(roles)
            .filter(|(_, (_, r))| *r == Role::Partial)
            .map(|(r, _)| r)
            .collect();
        let mut pending = Vec::with_capacity(partials.len());

        for registration in &partials {
            let operation = registration
                .partial
                .as_ref()
                .ok_or(Error::ConflictingSigner)?;
            pending.push(Some(operation.partial(&current)));
        }

        let mut outputs = vec![None; pending.len()];
        poll_fn(|cx| crate::pipeline::poll_all(&mut pending, &mut outputs, cx)).await?;
        drop(pending);
        let mut result = current.clone();

        for (registration, output) in partials.into_iter().zip(outputs.into_iter().flatten()) {
            result = output.apply(registration.info.address, &result)?;
        }

        Ok(result)
    }

    /// Requires complete signature coverage after automatic role selection.
    ///
    /// # Errors
    /// Also rejects missing signatures and, with crypto, invalid signatures.
    pub async fn sign(&self, tx: &Transaction) -> Result<Transaction, Failure> {
        let result = self.partially_sign(tx).await?;
        crate::pipeline::require_complete(&result)?;
        Ok(result)
    }

    /// Signs and submits once using the automatically resolved sending capability.
    ///
    /// Unlike Dart, missing non-sender signatures and zero submission identifiers
    /// reject. Existing signatures are validated. An identifier is not confirmation.
    ///
    /// # Errors
    /// Resolves sender cardinality before effects; propagates backend failures.
    pub async fn sign_and_send(&self, tx: &Transaction) -> Result<Signature, Failure> {
        let roles = self.roles(true)?;
        let signed = self.apply(tx, &roles).await?;
        let sender = self
            .registrations
            .iter()
            .zip(&roles)
            .find(|(_, (_, r))| *r == Role::Sending)
            .map(|(r, _)| r)
            .ok_or(Error::MissingSender)?;

        for (address, slot) in signed.message().signers().iter().zip(signed.signatures()) {
            if *address != sender.info.address && slot.is_none() {
                return Err(
                    Error::from(solana_kit_core::transaction::Error::MissingSignature).into(),
                );
            }

            if let Some(signature) = slot {
                SignatureOutput::new(*address, signed.message().bytes(), *signature)
                    .validate(*address, signed.message().bytes())?;
            }
        }

        let identifier = sender
            .send
            .as_ref()
            .ok_or(Error::MissingSender)?
            .send(&signed)
            .await?;

        if identifier.as_bytes() == &[0; 64] {
            return Err(Error::from(solana_kit_core::transaction::Error::InvalidSignature).into());
        }

        Ok(identifier)
    }

    /// Signs up to 256 transactions in input order, stopping on the first failure.
    ///
    /// Earlier backend effects cannot be rolled back. Size is checked before effects.
    ///
    /// # Errors
    /// Rejects excessive size, then propagates the first signing failure.
    pub async fn sign_batch(
        &self,
        transactions: &[Transaction],
    ) -> Result<Vec<Transaction>, Failure> {
        if transactions.len() > MAX_ITEMS {
            return Err(Error::TooManySigners.into());
        }

        let mut results = Vec::with_capacity(transactions.len());

        for tx in transactions {
            results.push(self.sign(tx).await?);
        }

        Ok(results)
    }
}

impl<'a> Composition<'a, Error> {
    /// Builds the default error-preserving composition.
    ///
    /// # Errors
    /// Rejects conflicting or excessive registrations before backend effects.
    pub fn new(registrations: impl Into<Vec<Registration<'a, Error>>>) -> Result<Self, Error> {
        Self::with_failure(registrations)
    }
}
