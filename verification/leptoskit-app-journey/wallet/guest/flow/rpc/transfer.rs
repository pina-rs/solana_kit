//! Approval binds the visible transfer purpose to the exact immutable message.
use super::super::approval::{self, Decision};
use super::{
    AppError, Error, LocalWalletCommands, LocalWalletEvents, Rpc, balance, bounded, cancelled,
    read_config, retain, show, submission,
};
use solana_kit_client::methods::LatestBlockhash;
use solana_kit_core::{
    Address, codecs,
    transaction::{Lifetime, Message, Transaction, Version},
};
use solana_kit_leptoskit::{GuestLocalWallet, wallet_ui::local_wallet_panel};
use solana_kit_system::{TransferSol, TransferSolAccounts};

const AMOUNT: u64 = 1_000_000;
const PACKET_BYTES: usize = 1232;
const COLLECTION_ITEMS: usize = 256;
// This is a public test recipient on the fixture's explicit local validator.
// It grants no signing authority and is never derived from a guest private key.
const RECIPIENT: Address = Address::from_bytes([98; 32]);

struct Prepared {
    transaction: Transaction,
    recipient_before: u64,
}

/// Preparation performs only safe reads. Rejection returns the same wallet;
/// only an approval for these exact bytes can enter signing and submission.
pub(super) async fn run(
    client: &Rpc,
    wallet: GuestLocalWallet,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
    generation: u64,
) -> Result<(GuestLocalWallet, String), AppError> {
    let prepared = match bounded(prepare(client, wallet.address()), actions).await {
        Ok(prepared) => prepared,
        Err(error) => return retain(wallet, error),
    };

    let payer = wallet.address();
    let purpose =
        format!("Transfer {AMOUNT} lamports from {payer} to {RECIPIENT}. Network fees also apply.");
    let message = prepared.transaction.message().bytes();

    match approval::request(&purpose, message, generation, actions, commands).await {
        Decision::Reject => return Ok((wallet, "Transfer rejected; nothing submitted".into())),
        Decision::Cancel => return Err(Error::Cancelled.into()),
        Decision::Approve => {}
    }

    approved(client, wallet, prepared, actions, commands).await
}

/// Preserve the primary signature before any submission poll. A failed or
/// cancelled attempt is never automatically re-signed or sent a second time.
async fn approved(
    client: &Rpc,
    wallet: GuestLocalWallet,
    prepared: Prepared,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
) -> Result<(GuestLocalWallet, String), AppError> {
    let before = prepared.recipient_before;
    let (wallet, transaction) = sign(wallet, prepared.transaction, actions, commands).await?;
    let signature = transaction
        .signatures()
        .first()
        .copied()
        .flatten()
        .ok_or(Error::MissingSignature)?;
    let address = wallet.address().to_string();
    show(
        local_wallet_panel(Some(&address), true, false, commands),
        &format!("Confirming transfer {signature}"),
    );
    let result = bounded(
        submission::send(client, &transaction, RECIPIENT, before, AMOUNT),
        actions,
    )
    .await;
    finish(wallet, signature, result)
}

fn finish(
    wallet: GuestLocalWallet,
    signature: solana_kit_core::Signature,
    result: Result<u64, AppError>,
) -> Result<(GuestLocalWallet, String), AppError> {
    let after = match result {
        Ok(after) => after,
        Err(source) if cancelled(&source) => {
            return Err(Error::Submitted { signature, source }.into());
        }
        Err(source) => return retain(wallet, Error::Submitted { signature, source }.into()),
    };

    Ok((
        wallet,
        format!("Transfer confirmed: {signature}. Recipient balance: {after} lamports"),
    ))
}

async fn prepare(client: &Rpc, payer: Address) -> Result<Prepared, AppError> {
    let before = balance(client, RECIPIENT).await?;
    before.checked_add(AMOUNT).ok_or(Error::AmountOverflow)?;
    let latest = client.call(LatestBlockhash(read_config())).await?;
    let instruction = TransferSol { amount: AMOUNT }.instruction(
        TransferSolAccounts {
            source: payer,
            destination: RECIPIENT,
        },
        codecs::Limits::new(PACKET_BYTES, COLLECTION_ITEMS),
    )?;
    let lifetime = Lifetime::Blockhash {
        blockhash: latest.value.blockhash,
        last_valid_block_height: latest.value.last_valid_block_height,
    };
    let message = Message::compile(Version::Legacy, payer, lifetime, &[instruction], &[])?;
    Ok(Prepared {
        transaction: Transaction::new(message),
        recipient_before: before,
    })
}

async fn sign(
    wallet: GuestLocalWallet,
    transaction: Transaction,
    actions: &mut LocalWalletEvents,
    commands: &LocalWalletCommands,
) -> Result<(GuestLocalWallet, Transaction), AppError> {
    show(
        local_wallet_panel(Some(&wallet.address().to_string()), true, false, commands),
        "Signing the approved transfer",
    );
    let bytes = transaction.message().bytes().to_vec();
    let operation = wallet.sign_with_approval(&bytes, std::future::ready(true));
    let (wallet, signature) = bounded(operation, actions).await?;
    let signature = signature.ok_or(Error::MissingSignature)?;
    let transaction = transaction.with_signature(wallet.address(), &bytes, signature)?;
    transaction.verify()?;
    Ok((wallet, transaction))
}
