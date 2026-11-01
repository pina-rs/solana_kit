//! Explicit publication completion keeps every mutation behind caller intent.

use super::{Effect, Error, HydratedPublication, Snapshot, Stage, Storage, Workflow, text, value};
use crate::portal::{
    MarkReleaseCollectionAsVerified, MarkReleaseCollectionAsVerifiedResult, Operation,
    PortalService, PrepareReleaseNftTransaction, PrepareVerifyCollectionTransaction,
    PreparedReleaseTransaction, PreparedVerifyCollectionTransaction, SaveReleaseNftData,
    SaveReleaseNftDataResult, StoreAttestation, SubmitSignedTransaction,
    SubmitSignedTransactionResult, SubmitToStore, SubmitToStoreResult, Uploader,
};
use solana_kit_client::{rest::Transport, subscriptions::Clock};

/// Caller-approved inputs for one release-mint preparation.
pub struct MintPreparation<'a> {
    /// Display name approved for the release NFT.
    pub release_name: &'a str,
    /// App collection mint expected by the release.
    pub app_mint_address: &'a str,
    /// Publisher address expected to sign the prepared transaction.
    pub publisher_address: &'a str,
    /// Explicit fee payer for the prepared transaction.
    pub payer_address: &'a str,
}

/// Explicit purpose keeps mint and collection-verification submissions distinct.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionPurpose {
    /// Sign and submit the prepared release-mint transaction.
    ReleaseMint,
    /// Sign and submit the prepared collection-verification transaction.
    CollectionVerification,
}

/// Caller inputs for saving the acknowledged release-mint receipt.
pub struct MintSave<'a> {
    /// Release mint address reported by the prepared transaction.
    pub mint_address: &'a str,
    /// Submission signature, not proof of landing or confirmation.
    pub transaction_signature: &'a str,
    /// Publisher address recorded as the release owner.
    pub owner_address: &'a str,
    /// Display name approved for the release.
    pub release_name: &'a str,
    /// Release version name.
    pub release_version: &'a str,
    /// Android package identifier.
    pub android_package: &'a str,
    /// App collection mint.
    pub app_mint_address: &'a str,
}

/// Caller-approved store-review submission; the attestation is already signed data.
pub struct StoreSubmission<'a> {
    /// User-supplied release notes.
    pub whats_new: &'a str,
    /// Caller-selected critical-update flag.
    pub critical_update: bool,
    /// Optional review instructions.
    pub testing_instructions: Option<&'a str>,
    /// Optional resubmission flag; explicit false remains distinct.
    pub is_resubmission: Option<bool>,
    /// Signature-prefixed attestation payload and request identity.
    pub attestation: StoreAttestation,
}

impl<T, U, S, C> Workflow<'_, T, U, S, C>
where
    T: Transport<PortalService>,
    U: Uploader,
    S: Storage,
    C: Clock,
{
    /// Prepares an unsigned release-mint transaction after bundle identity checks.
    ///
    /// The caller owns signing and submission. No checkpoint mutation is recorded
    /// because the portal proposal itself is not a durable effect.
    pub async fn prepare_release_mint(
        &self,
        hydrated: &HydratedPublication,
        input: MintPreparation<'_>,
    ) -> Result<PreparedReleaseTransaction, Error> {
        prepare_stage(hydrated, Stage::PreparedForMint)?;
        require_text(input.release_name, "missing release name")?;
        require_text(input.publisher_address, "missing publisher address")?;
        require_text(input.payer_address, "missing payer address")?;
        let authority = authority(hydrated)?;
        let expected_app_mint =
            text(&authority.app_mint_address).ok_or(Error::Invalid("bundle lacks an app mint"))?;
        require_text(input.app_mint_address, "missing app mint")?;

        if input.app_mint_address != expected_app_mint {
            return Err(Error::Invalid(
                "app mint differs from the publication bundle",
            ));
        }

        if let Some(expected) =
            text(&authority.required_signer).or_else(|| text(&authority.dapp_wallet_address))
            && input.publisher_address != expected
        {
            return Err(Error::Invalid(
                "publisher differs from the publication bundle",
            ));
        }

        if let Some(expected) = text(&authority.fee_payer)
            && input.payer_address != expected
        {
            return Err(Error::Invalid("payer differs from the publication bundle"));
        }

        self.call(&PrepareReleaseNftTransaction {
            release_id: hydrated.release_id().into(),
            release_name: input.release_name.into(),
            release_metadata_uri: hydrated.metadata_uri().into(),
            app_mint_address: input.app_mint_address.into(),
            publisher_address: input.publisher_address.into(),
            payer_address: input.payer_address.into(),
        })
        .await
    }

    /// Prepares collection verification from the acknowledged session and bundle.
    ///
    /// The release mint comes from the session, then the release record. Missing
    /// authority fields fail before the portal is called.
    pub async fn prepare_collection_verification(
        &self,
        hydrated: &HydratedPublication,
        payer_address: &str,
    ) -> Result<PreparedVerifyCollectionTransaction, Error> {
        prepare_stage(hydrated, Stage::MintSaved)?;
        require_text(payer_address, "missing payer address")?;
        let bundle = hydrated.backend_bundle();
        let dapp = value(&bundle.dapp).ok_or(Error::Invalid("bundle lacks a dApp"))?;
        let release = value(&bundle.release).ok_or(Error::Invalid("bundle lacks a release"))?;
        let authority = authority(hydrated)?;
        let dapp_id = text(&dapp.id).ok_or(Error::Invalid("bundle lacks a dApp identity"))?;
        let mint = text(&hydrated.backend_session().expected_mint_address)
            .or_else(|| text(&release.nft_mint_address))
            .ok_or(Error::Invalid("publication lacks a release mint"))?;
        let collection =
            text(&authority.app_mint_address).ok_or(Error::Invalid("bundle lacks an app mint"))?;
        let collection_authority = text(&authority.collection_authority)
            .ok_or(Error::Invalid("bundle lacks a collection authority"))?;

        self.call(&PrepareVerifyCollectionTransaction {
            dapp_id: dapp_id.into(),
            nft_mint_address: mint.into(),
            collection_mint_address: collection.into(),
            collection_authority: collection_authority.into(),
            payer_address: payer_address.into(),
        })
        .await
    }

    /// Records durable intent, submits one fully signed transaction and clears it.
    ///
    /// A failed or cancelled call leaves the effect pending; callers must reconcile
    /// the signature with their signing authority before another submission.
    pub async fn submit_signed_transaction(
        &mut self,
        hydrated: &HydratedPublication,
        purpose: TransactionPurpose,
        signed_transaction: &str,
    ) -> Result<SubmitSignedTransactionResult, Error> {
        let required = match purpose {
            TransactionPurpose::ReleaseMint => Stage::PreparedForMint,
            TransactionPurpose::CollectionVerification => Stage::MintSaved,
        };
        prepare_stage(hydrated, required)?;
        require_text(signed_transaction, "missing signed transaction")?;

        let request = SubmitSignedTransaction {
            signed_transaction: signed_transaction.into(),
            publication_session_id: Some(hydrated.publication_session_id().into()),
        };
        self.mutate(hydrated, Effect::SubmitTransaction, &request)
            .await
    }

    /// Saves the acknowledged release-mint receipt after a submitted transaction.
    pub async fn save_release_nft_data(
        &mut self,
        hydrated: &HydratedPublication,
        input: MintSave<'_>,
    ) -> Result<SaveReleaseNftDataResult, Error> {
        prepare_stage(hydrated, Stage::MintSubmitted)?;
        for (field, reason) in [
            (input.mint_address, "missing release mint"),
            (input.transaction_signature, "missing transaction signature"),
            (input.owner_address, "missing release owner"),
            (input.release_name, "missing release name"),
            (input.release_version, "missing release version"),
            (input.android_package, "missing android package"),
            (input.app_mint_address, "missing app mint"),
        ] {
            require_text(field, reason)?;
        }

        let authority = authority(hydrated)?;
        let expected_app_mint =
            text(&authority.app_mint_address).ok_or(Error::Invalid("bundle lacks an app mint"))?;

        if input.app_mint_address != expected_app_mint {
            return Err(Error::Invalid(
                "app mint differs from the publication bundle",
            ));
        }

        if let Some(expected) = text(&hydrated.backend_session().expected_mint_address)
            && expected != input.mint_address
        {
            return Err(Error::Invalid(
                "release mint differs from the publication session",
            ));
        }

        let request = SaveReleaseNftData {
            release_id: hydrated.release_id().into(),
            mint_address: input.mint_address.into(),
            transaction_signature: input.transaction_signature.into(),
            metadata_uri: hydrated.metadata_uri().into(),
            owner_address: input.owner_address.into(),
            release_name: input.release_name.into(),
            release_version: input.release_version.into(),
            android_package: input.android_package.into(),
            app_mint_address: input.app_mint_address.into(),
        };
        let result = self
            .mutate(hydrated, Effect::SaveReleaseNftData, &request)
            .await?;

        if !result.success {
            return Err(Error::Invalid("release data save was not acknowledged"));
        }

        Ok(result)
    }

    /// Marks the release collection verified after its submission was acknowledged.
    pub async fn mark_collection_verified(
        &mut self,
        hydrated: &HydratedPublication,
    ) -> Result<MarkReleaseCollectionAsVerifiedResult, Error> {
        prepare_stage(hydrated, Stage::VerificationSubmitted)?;
        let request = MarkReleaseCollectionAsVerified {
            release_id: hydrated.release_id().into(),
        };
        let result = self
            .mutate(hydrated, Effect::MarkCollectionVerified, &request)
            .await?;

        if !result.success {
            return Err(Error::Invalid(
                "collection verification was not acknowledged",
            ));
        }

        if result.release_id != hydrated.release_id() {
            return Err(Error::Invalid(
                "collection verification returned another release",
            ));
        }

        Ok(result)
    }

    /// Submits the caller-signed attestation to store review.
    ///
    /// The attestation is never created or signed here. A failed call leaves the
    /// submission pending; the caller must reconcile the provider state before retry.
    pub async fn submit_to_store(
        &mut self,
        hydrated: &HydratedPublication,
        input: StoreSubmission<'_>,
    ) -> Result<SubmitToStoreResult, Error> {
        if hydrated.stage() != Stage::Verified && hydrated.stage() != Stage::Attested {
            return Err(Error::Invalid(
                "publication stage does not allow store submission",
            ));
        }

        require_text(input.whats_new, "missing release notes")?;
        require_text(&input.attestation.payload, "missing attestation payload")?;
        require_text(
            &input.attestation.request_unique_id,
            "missing attestation request identity",
        )?;

        let request = SubmitToStore {
            release_id: hydrated.release_id().into(),
            whats_new: input.whats_new.into(),
            critical_update: input.critical_update,
            testing_instructions: input.testing_instructions.map(str::to_owned),
            is_resubmission: input.is_resubmission,
            attestation: input.attestation,
        };
        self.mutate(hydrated, Effect::SubmitStore, &request).await
    }

    async fn mutate<O: Operation>(
        &mut self,
        hydrated: &HydratedPublication,
        effect: Effect,
        request: &O,
    ) -> Result<O::Response, Error> {
        let mut snapshot = self.checkpoint(hydrated).await?;
        self.intent(&mut snapshot, effect).await?;
        let response = self.call(request).await?;
        snapshot.pending = None;
        self.save(&snapshot).await?;
        Ok(response)
    }

    async fn checkpoint(&self, hydrated: &HydratedPublication) -> Result<Snapshot, Error> {
        let snapshot = self
            .load()
            .await?
            .ok_or(Error::Invalid("missing publication checkpoint"))?;
        snapshot.admit()?;

        if snapshot.release_id.as_deref() != Some(hydrated.release_id())
            || snapshot.publication_id.as_deref() != Some(hydrated.publication_session_id())
        {
            return Err(Error::Invalid(
                "hydrated publication differs from the checkpoint",
            ));
        }

        Ok(snapshot)
    }
}

fn prepare_stage(hydrated: &HydratedPublication, stage: Stage) -> Result<(), Error> {
    if hydrated.stage() == stage {
        Ok(())
    } else {
        Err(Error::Invalid(
            "publication stage does not allow this operation",
        ))
    }
}

fn authority(
    hydrated: &HydratedPublication,
) -> Result<&crate::portal::BackendSignerAuthority, Error> {
    value(&hydrated.backend_bundle().signer_authority)
        .ok_or(Error::Invalid("bundle lacks a signer authority"))
}

fn require_text(value: &str, reason: &'static str) -> Result<(), Error> {
    if value.trim().is_empty() {
        Err(Error::Invalid(reason))
    } else {
        Ok(())
    }
}
