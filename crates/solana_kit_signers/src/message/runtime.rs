/// Bounded heterogeneous message composition with explicit local/native futures.
///
/// All modifiers receive the whole slice sequentially. All partials then borrow
/// one immutable final slice concurrently; successful results merge in registration
/// order. Dropping the operation cancels unfinished futures by dropping them.
pub struct Composition<'a> {
    registrations: Vec<Registration<'a>>,
}

impl<'a> Composition<'a> {
    /// Validates registrations before any backend call and deduplicates shared handles.
    ///
    /// # Errors
    /// Rejects more than 256 inputs and conflicting identities or interface declarations.
    pub fn new(registrations: Vec<Registration<'a>>) -> Result<Self, MessageError> {
        if registrations.len() > MAX_MESSAGES {
            return Err(MessageError::TooManySigners);
        }

        let mut unique: Vec<Registration<'a>> = Vec::with_capacity(registrations.len());

        for registration in registrations {
            let Some(previous) = unique
                .iter()
                .find(|r| r.address == registration.address || r.identity == registration.identity)
            else {
                unique.push(registration);
                continue;
            };

            if previous.address != registration.address
                || previous.identity != registration.identity
                || previous.partial.is_some() != registration.partial.is_some()
            {
                return Err(MessageError::ConflictingSigner);
            }
        }

        Ok(Self {
            registrations: unique,
        })
    }

    /// Signs at most 256 messages using actual whole-slice backend calls.
    ///
    /// No executor, retry, network call or implicit envelope is introduced.
    /// Existing signatures are checked before effects; without crypto this is
    /// structural validation only. Earlier backend effects cannot be rolled back.
    ///
    /// # Errors
    /// Rejects count, cardinality, ownership, stale bytes, missing/invalid signatures
    /// and backend failures. A failure prevents subsequent modifier calls.
    pub async fn sign(
        &self,
        messages: &[SignableMessage],
    ) -> Result<Vec<SignableMessage>, MessageError> {
        if messages.len() > MAX_MESSAGES {
            return Err(MessageError::TooManyMessages);
        }

        for message in messages {
            validation::signatures(message)?;
        }

        let current = self.modify(messages).await?;
        self.partial(current).await
    }

    async fn partial(
        &self,
        current: Vec<SignableMessage>,
    ) -> Result<Vec<SignableMessage>, MessageError> {
        let partials: Vec<_> = self
            .registrations
            .iter()
            .filter(|r| r.partial.is_some())
            .collect();
        let mut pending: Vec<_> = partials
            .iter()
            .map(|registration| {
                registration
                    .partial
                    .as_ref()
                    .map(|operation| operation.sign(&current))
            })
            .collect();
        let mut outputs = vec![None; partials.len()];

        poll_fn(|context| poll_operations(&mut pending, &mut outputs, context)).await?;
        drop(pending);
        merge_outputs(current, partials, outputs)
    }

    async fn modify(
        &self,
        messages: &[SignableMessage],
    ) -> Result<Vec<SignableMessage>, MessageError> {
        let mut current = messages.to_vec();

        for (registration, operation) in self
            .registrations
            .iter()
            .filter_map(|r| r.modify.as_ref().map(|operation| (r, operation)))
        {
            let next = operation.modify(&current).await?;

            validation::batch_modification(&current, &next, registration.address)?;
            current = next;
        }

        Ok(current)
    }
}

fn merge_outputs(
    mut result: Vec<SignableMessage>,
    partials: Vec<&Registration<'_>>,
    outputs: Vec<Option<Vec<MessageSignatureOutput>>>,
) -> Result<Vec<SignableMessage>, MessageError> {
    for (registration, output) in partials.into_iter().zip(outputs) {
        let output = output.ok_or(MessageError::Cardinality)?;

        if output.len() != result.len() {
            return Err(MessageError::Cardinality);
        }

        for (message, signatures) in result.iter_mut().zip(output) {
            validation::merge(message, signatures, registration.address)?;
        }
    }

    Ok(result)
}

fn poll_operations(
    pending: &mut [Option<Operation<'_, MessageSignatureOutput>>],
    outputs: &mut [Option<Vec<MessageSignatureOutput>>],
    context: &mut core::task::Context<'_>,
) -> Poll<Result<(), MessageError>> {
    for (operation, output) in pending.iter_mut().zip(outputs) {
        poll_one(operation, output, context)?;
    }

    if pending.iter().all(Option::is_none) {
        Poll::Ready(Ok(()))
    } else {
        Poll::Pending
    }
}

fn poll_one(
    operation: &mut Option<Operation<'_, MessageSignatureOutput>>,
    output: &mut Option<Vec<MessageSignatureOutput>>,
    context: &mut core::task::Context<'_>,
) -> Result<(), MessageError> {
    let Some(future) = operation else {
        return Ok(());
    };

    if let Poll::Ready(result) = future.as_mut().poll(context) {
        *output = Some(result?);
        *operation = None;
    }

    Ok(())
}
