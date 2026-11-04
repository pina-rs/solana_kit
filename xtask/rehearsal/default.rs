//! Independent external behavior tests for the exact local-registry SDK archives.
//!
//! Cargo resolves every `solana_kit*` dependency through the generated local
//! registry, never through a path dependency or this checkout. Default mode keeps
//! the portable feature floor. `--features selected` additionally enables crypto,
//! parsed/reactive client reads, MWA, Surfpool and Token plans. The only transport
//! endpoints are a scripted in-process reply and a caller-owned loopback listener.
#![forbid(unsafe_code)]

use solana_kit_client::{
    Client, Limits, Reply, Request, Transport, TransportError,
    methods::Balance,
};
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Hash, Instruction,
    codecs::Limits as CodecLimits,
    transaction::{Lifetime, Message, Version},
};
use std::future::{Ready, ready};
use std::time::Duration;

const LIMITS: CodecLimits = CodecLimits::new(64, 8);
const ATA_LIMITS: CodecLimits = CodecLimits::new(1, 8);
const TRANSFER_AMOUNT: u64 = 7;
const READ_TIMEOUT: Duration = Duration::from_secs(5);
const REQUEST_BYTES: usize = 4096;

/// One literal JSON-RPC reply with a request-identity check.
struct Script;

impl Transport for Script {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let text = std::str::from_utf8(&request.body).expect("UTF-8 request");
        assert!(text.contains("\"method\":\"getBalance\""), "{text}");
        assert!(text.contains("\"id\":1"), "{text}");
        ready(Ok(Reply {
            status: 200,
            body: br#"{"jsonrpc":"2.0","id":1,"result":{"context":{"slot":9},"value":42}}"#
                .to_vec(),
        }))
    }
}

/// The registry package preserves an exact external address and legacy wire bytes.
#[test]
fn core_address_and_legacy_message_bytes() {
    let bytes = core::array::from_fn(|index| u8::try_from(index + 1).expect("32 bytes"));
    let address = Address::from(bytes);

    assert_eq!(
        address.to_string(),
        "4wBqpZM9xaSheZzJSMawUKKwhdpChKbZ5eu5ky4Vigw"
    );
    assert_eq!(address.to_string().parse::<Address>(), Ok(address));
    assert!("0".parse::<Address>().is_err());

    let hash = Hash::from([7; 32]);
    assert_eq!(hash.to_string().parse::<Hash>(), Ok(hash));
    assert!(Hash::try_from(&[7; 31][..]).is_err());

    let program = Address::from([2; 32]);
    let blockhash = Hash::from([3; 32]);
    let instruction = Instruction {
        program_address: program,
        accounts: vec![],
        data: vec![],
    };
    let message = Message::compile(
        Version::Legacy,
        address,
        Lifetime::Blockhash {
            blockhash,
            last_valid_block_height: 42,
        },
        &[instruction],
        &[],
    )
    .expect("valid legacy message");
    let mut expected = vec![1, 0, 1, 2];
    expected.extend(address.as_bytes());
    expected.extend(program.as_bytes());
    expected.extend(blockhash.as_bytes());
    expected.extend([1, 1, 0, 0]);

    assert_eq!(message.bytes(), expected);
}

/// System keeps its four-byte little-endian tag and exact account roles.
#[test]
fn system_transfer_bytes_are_exact() {
    use solana_kit_system::{SYSTEM_PROGRAM, TransferSol, TransferSolAccounts};

    let source = Address::from([4; 32]);
    let destination = Address::from([5; 32]);
    let instruction = TransferSol {
        amount: TRANSFER_AMOUNT,
    }
    .instruction(
        TransferSolAccounts {
            source,
            destination,
        },
        LIMITS,
    )
    .expect("valid transfer");

    assert_eq!(instruction.program_address, SYSTEM_PROGRAM);
    assert_eq!(
        instruction.data,
        [2, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: source,
                role: AccountRole::WritableSigner,
            },
            AccountMeta {
                address: destination,
                role: AccountRole::Writable,
            },
        ]
    );
}

/// Memo stores the caller's UTF-8 bytes and exposes ordered signer positions.
#[test]
fn memo_bytes_are_exact() {
    use solana_kit_memo::add_memo;

    let signer = Address::from([4; 32]);
    let memo = add_memo("registry", &[signer], LIMITS).expect("valid memo");

    assert_eq!(memo.data, b"registry");
    assert_eq!(memo.accounts[0].role, AccountRole::ReadonlySigner);
}

/// The associated-token discriminator is one byte; six positions stay ordered.
#[test]
fn associated_token_create_is_exact() {
    use solana_kit_associated_token_account::{
        AssociatedTokenInstruction, CreateAccounts,
    };
    use solana_kit_system::SYSTEM_PROGRAM;

    let source = Address::from([4; 32]);
    let destination = Address::from([5; 32]);
    let create = AssociatedTokenInstruction::Create(CreateAccounts {
        payer: source,
        associated_token: destination,
        owner: source,
        mint: destination,
        system_program: SYSTEM_PROGRAM,
        token_program: Address::from([6; 32]),
    })
    .instruction(ATA_LIMITS)
    .expect("valid associated-token create");

    assert_eq!(create.data, [0]);
    assert_eq!(create.accounts.len(), 6);
}

/// Anchor's public discriminator remains the independent historical literal.
#[test]
fn anchor_discriminator_is_literal() {
    assert_eq!(
        solana_kit_anchor::instruction_discriminator("initialize"),
        [175, 175, 109, 31, 13, 152, 155, 237]
    );
}

/// Wallet metadata has no network or platform discovery side effect.
#[test]
fn wallet_metadata_is_local_and_validated() {
    use solana_kit_wallet::{WalletAccount, wallet_logo_fallback};

    // The bundled logo is metadata; this call does not discover or contact a wallet.
    let icon = wallet_logo_fallback("Phantom")
        .expect("bundled logo lookup")
        .expect("Phantom logo");
    assert!(icon.data_uri().starts_with("data:image/svg+xml"));
    let account = WalletAccount::new(
        "display-address".into(),
        [7; 32],
        vec!["solana:mainnet".into()],
        vec!["standard:connect".into()],
        None,
        Some(icon),
    )
    .expect("valid account");
    assert_eq!(account.public_key(), &[7; 32]);
    assert!(
        WalletAccount::new(
            String::new(),
            [0; 32],
            vec![],
            vec![],
            None,
            None,
        )
        .is_err()
    );
}

/// Immutable message snapshots reject duplicate signature positions.
#[test]
fn signer_messages_retain_exact_bare_content() {
    use solana_kit_core::Signature;
    use solana_kit_signers::message::SignableMessage;

    // Bare content is copied exactly; duplicate signer positions never overwrite.
    let message = SignableMessage::from_text("registry message");
    assert_eq!(message.content(), b"registry message");
    assert!(message.signatures().is_empty());
    let address = Address::from([1; 32]);
    let signature = Signature::from_bytes([2; 64]);
    assert!(
        SignableMessage::with_signatures(
            b"duplicate",
            [(address, signature), (address, signature)],
        )
        .is_err()
    );
}

/// The typed client executes the registry package against a literal reply.
#[tokio::test]
async fn client_executes_literal_balance() {
    let client = Client::new(Script, Limits::default());
    let value = client
        .call(Balance(Address::from([9; 32]), None))
        .await
        .expect("literal balance");
    assert_eq!(value.context.slot, 9);
    assert_eq!(value.value, 42);
}

/// Native HTTP keeps caller-owned loopback behavior without production network.
#[tokio::test]
async fn native_http_executes_owned_loopback() {
    use solana_kit_native::{HttpConfig, NativeHttp};
    use std::{
        io::{Read, Write},
        net::TcpListener,
    };

    // One caller-owned listener proves the real native adapter without a public host.
    let listener = TcpListener::bind("127.0.0.1:0").expect("loopback listener");
    let endpoint = listener.local_addr().expect("loopback address");

    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("loopback connection");
        stream
            .set_read_timeout(Some(READ_TIMEOUT))
            .expect("read timeout");
        let mut request = [0; REQUEST_BYTES];
        let read = stream.read(&mut request).expect("request bytes");
        let text = String::from_utf8_lossy(&request[..read]);
        assert!(text.contains("getBalance"), "{text}");
        let body = r#"{"jsonrpc":"2.0","id":1,"result":{"context":{"slot":9},"value":42}}"#;
        write!(
            stream,
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
            body.len()
        )
        .expect("response bytes");
        stream.flush().expect("response flush");
    });
    let transport =
        NativeHttp::new(&format!("http://{endpoint}"), HttpConfig::default()).expect("transport");
    let client = Client::new(transport, Limits::default());
    let value = client
        .call(Balance(Address::from([9; 32]), None))
        .await
        .expect("loopback balance");
    assert_eq!(value.value, 42);
    server.join().expect("loopback server");
}
