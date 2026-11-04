//! Appended host-neutral cases for the newly merged public surface.
//!
//! Every body is an independent literal fixture over existing public APIs. The
//! module owns the exact finite crypto work performed by these appended cases so
//! the private interpreter allowance can account for it without a measured
//! minimum.

use alloc::{
    boxed::Box,
    format,
    rc::Rc,
    string::{String, ToString},
    sync::Arc,
    task::Wake,
    vec,
    vec::Vec,
};
use core::{
    cell::Cell,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

use serde_json::{Value, json};
use solana_kit_client::{
    Client, Error as ClientError, Limits,
    accounts::{assert_account_decoded, parse_json_rpc_account},
    methods::{Balance, accounts::AccountData},
    parsed::{
        Base64Encoding, BpfUpgradeableLoaderProgramAccount, ConfigProgramAccount,
        ParsedAccountData, StakeProgramAccount, SysvarAccount, TokenProgramAccount, UnknownParsed,
    },
};
use solana_kit_core::{
    AccountLookupMeta, AccountMeta, AccountRole, Address, Hash, Instruction, InstructionAccount,
    LookupRole,
    codecs::{self, Decoder, Encoder, Endian, Limits as CodecLimits, Paired, Size},
    crypto::{
        Entropy, GrindBudget, GrindError, GrindPattern, GrindPatternError, Keypair,
        constant_time_eq,
    },
    fixed_points::{DecimalFixedPoint, Shape, Signedness},
    plans::Plan,
    transaction::{
        DecompiledMessage, Error as TransactionError, HeapSize, Lifetime, LookupTable, Message,
        Transaction, V1Config, Version,
    },
};
use solana_kit_signers::{
    Error as SignerError, SignatureOutput, Signer, SignerCapability, TransactionPartialSigner,
    TransactionSignerConfig, local::Composition, local::Registration,
};
use solana_kit_test_support::{PendingExchange, ScriptStep, ScriptedTransport, canned};
use solana_kit_token::{
    CreateMintConfig, CreateMintInput, MintToAtaConfig, MintToAtaDerivedInput, MintToAtaInput,
    TransferToAtaConfig, TransferToAtaDerivedInput, TransferToAtaInput, create_mint_plan,
    mint_to_ata_derived_plan, mint_to_ata_plan, transfer_to_ata_derived_plan, transfer_to_ata_plan,
};

/// Distinct key derivations performed by the appended grinding assertions.
pub const GRIND_KEY_DERIVATIONS: u64 = 69;
/// Key derivations, signatures and verifications in the explicit-config case.
pub const SIGNER_CRYPTO_OPERATIONS: u64 = 5;
/// Associated-address searches performed by the derived Token plan assertions.
pub const DERIVED_ATA_SEARCHES: u64 = 6;

/// Exact finite crypto work owned by the appended parity-completion cases.
///
/// The private interpreter grants one existing five-million crypto unit per
/// entry. Counts are source-owned fixture facts, not measured minima.
pub const CRYPTO_WORK: &[(&str, u64)] = &[
    ("grind-key-derivations", GRIND_KEY_DERIVATIONS),
    ("signer-explicit-config", SIGNER_CRYPTO_OPERATIONS),
    ("token-derived-ata-searches", DERIVED_ATA_SEARCHES),
];

/// Number of named work categories above.
pub const CRYPTO_WORK_SITES: usize = 3;
/// Sum of [`CRYPTO_WORK`] counts.
pub const CRYPTO_WORK_UNITS: u64 = 80;

type Outcome = Result<(), &'static str>;

/// Appended groups, preserved in this order after every earlier case.
pub(crate) fn cases() -> [crate::CaseResult; 7] {
    [
        crate::case("grind-entropy-cancellation", grind()),
        crate::case("message-decompile-roundtrip", decompile()),
        crate::case("signer-explicit-config", signer_config()),
        crate::case("token-mint-ata-plans", token_plans()),
        crate::case("client-parsed-account-families", parsed_families()),
        crate::case("fixed-point-codec-pairing", fixed_point_pairing()),
        crate::case("test-support-scripted-client", scripted_client()),
    ]
}

// ---------------------------------------------------------------------------
// Grinding, constant-time equality, cooperative suspension and cancellation.
// ---------------------------------------------------------------------------

/// Deterministic seed source: candidate `n` receives 32 copies of byte `n`.
struct DeterministicEntropy {
    fills: Rc<Cell<u64>>,
}

impl DeterministicEntropy {
    fn new() -> Self {
        Self {
            fills: Rc::new(Cell::new(0)),
        }
    }

    fn handle(&self) -> Rc<Cell<u64>> {
        Rc::clone(&self.fills)
    }
}

struct Fill<'a> {
    output: &'a mut [u8],
    index: u64,
    fills: &'a Rc<Cell<u64>>,
}

impl Future for Fill<'_> {
    type Output = Result<(), &'static str>;

    fn poll(mut self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        let byte = u8::try_from(self.index & 0xff).expect("fixture index is one byte");

        for value in self.output.iter_mut() {
            *value = byte;
        }

        self.fills.set(self.fills.get() + 1);
        Poll::Ready(Ok(()))
    }
}

impl Entropy for DeterministicEntropy {
    type Error = &'static str;
    type Fill<'a> = Fill<'a>;

    fn fill<'a>(&'a mut self, output: &'a mut [u8]) -> Self::Fill<'a> {
        Fill {
            output,
            index: self.fills.get(),
            fills: &self.fills,
        }
    }
}

fn grind() -> Outcome {
    equality()?;
    invalid_patterns_before_entropy()?;
    zero_amount_uses_no_entropy()?;
    real_match()?;
    bounded_exhaustion()?;
    cooperative_pending()?;
    cancellation()
}

const MATCH_SEED: [u8; 32] = [0; 32];
const OVERLONG_LITERAL: usize = 45;
const GRIND_BATCH_ATTEMPTS: usize = 64;
const GRIND_BATCH_SIZE: u64 = 32;
const GRIND_BATCH_MATCH_CALL: u64 = 33;

fn equality() -> Outcome {
    crate::require(constant_time_eq(b"", b""), "empty equality")?;
    crate::require(
        constant_time_eq(b"solana-kit", b"solana-kit"),
        "equal content",
    )?;
    crate::require(
        !constant_time_eq(b"solana-kit", b"solana-kit!"),
        "unequal lengths",
    )?;
    crate::require(
        !constant_time_eq(b"solana-kit", b"solana-Jit"),
        "differing content",
    )
}

fn invalid_patterns_before_entropy() -> Outcome {
    let entropy = DeterministicEntropy::new();
    let fills = entropy.handle();

    crate::require(
        matches!(
            GrindPattern::prefix("0"),
            Err(GrindPatternError::InvalidCharacter { character: '0' })
        ),
        "invalid base58 prefix",
    )?;
    let overlong = "1".repeat(OVERLONG_LITERAL);
    crate::require(
        matches!(
            GrindPattern::suffix(&overlong),
            Err(GrindPatternError::TooLong {
                length: OVERLONG_LITERAL
            })
        ),
        "overlong base58 suffix",
    )?;
    crate::require(fills.get() == 0, "pattern rejection used entropy")
}

fn zero_amount_uses_no_entropy() -> Outcome {
    let mut entropy = DeterministicEntropy::new();
    let fills = entropy.handle();
    let pattern = GrindPattern::prefix("").map_err(|_| "empty pattern")?;
    let mut future = Box::pin(solana_kit_core::crypto::grind_keypairs(
        &mut entropy,
        pattern,
        GrindBudget::new(0, 0),
    ));
    let mut context = Context::from_waker(Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(Ok(keys)) if keys.is_empty() => {}
        Poll::Ready(Ok(_)) => return Err("zero amount returned keys"),
        Poll::Ready(Err(_)) => return Err("zero amount failed"),
        Poll::Pending => return Err("zero amount suspended"),
    }

    drop(future);
    crate::require(fills.get() == 0, "zero amount requested entropy")
}

fn real_match() -> Outcome {
    let expected = Keypair::from_seed(&MATCH_SEED).address();
    let literal = expected.to_string();
    let pattern = GrindPattern::prefix(&literal).map_err(|_| "matching literal")?;
    let mut entropy = DeterministicEntropy::new();
    let fills = entropy.handle();
    let mut future = Box::pin(solana_kit_core::crypto::grind_keypairs(
        &mut entropy,
        pattern,
        GrindBudget::new(1, 1),
    ));
    let mut context = Context::from_waker(Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(Ok(keys)) => {
            crate::require(keys.len() == 1, "exact match count")?;
            crate::require(
                keys[0].signer().address() == expected,
                "matched address differs",
            )?;
        }
        Poll::Ready(Err(_)) => return Err("real match failed"),
        Poll::Pending => return Err("real match suspended"),
    }

    drop(future);
    crate::require(fills.get() == 1, "real match candidate count")
}

fn bounded_exhaustion() -> Outcome {
    let mut entropy = DeterministicEntropy::new();
    let fills = entropy.handle();
    let mut future = Box::pin(solana_kit_core::crypto::grind_keypairs_with(
        &mut entropy,
        GrindBudget::new(1, 2),
        |_| false,
    ));
    let mut context = Context::from_waker(Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(Err(GrindError::Exhausted {
            attempts: 2,
            found: 0,
        })) => {}
        _ => return Err("bounded exhaustion contract"),
    }

    drop(future);
    crate::require(fills.get() == 2, "exhaustion candidate count")
}

fn cooperative_pending() -> Outcome {
    let mut entropy = DeterministicEntropy::new();
    let fills = entropy.handle();
    let calls = Cell::new(0_u64);
    let mut future = Box::pin(solana_kit_core::crypto::grind_keypairs_with(
        &mut entropy,
        GrindBudget::new(1, GRIND_BATCH_ATTEMPTS),
        |_| {
            calls.set(calls.get() + 1);
            calls.get() == GRIND_BATCH_MATCH_CALL
        },
    ));
    let mut context = Context::from_waker(Waker::noop());

    crate::require(
        matches!(future.as_mut().poll(&mut context), Poll::Pending),
        "batch boundary must suspend",
    )?;
    crate::require(fills.get() == GRIND_BATCH_SIZE, "first batch size")?;
    crate::require(
        calls.get() == GRIND_BATCH_SIZE,
        "first batch predicate calls",
    )?;

    match future.as_mut().poll(&mut context) {
        Poll::Ready(Ok(keys)) => crate::require(keys.len() == 1, "batch match count"),
        _ => Err("batch boundary did not resume"),
    }?;
    crate::require(
        fills.get() == GRIND_BATCH_MATCH_CALL,
        "resumed candidate count",
    )?;
    crate::require(
        calls.get() == GRIND_BATCH_MATCH_CALL,
        "resumed predicate calls",
    )
}

fn cancellation() -> Outcome {
    let mut entropy = DeterministicEntropy::new();
    let fills = entropy.handle();
    let mut future = Box::pin(solana_kit_core::crypto::grind_keypairs_with(
        &mut entropy,
        GrindBudget::new(1, GRIND_BATCH_ATTEMPTS),
        |_| false,
    ));
    let mut context = Context::from_waker(Waker::noop());

    crate::require(
        matches!(future.as_mut().poll(&mut context), Poll::Pending),
        "cancellation must start suspended",
    )?;
    crate::require(fills.get() == GRIND_BATCH_SIZE, "suspended candidate count")?;
    drop(future);

    crate::require(
        fills.get() == GRIND_BATCH_SIZE,
        "cancelled grind used entropy",
    )
}

// ---------------------------------------------------------------------------
// Message decompilation for legacy, v0 and v1 canonical roundtrips.
// ---------------------------------------------------------------------------

const PAYER: u8 = 11;
const PROGRAM: u8 = 12;
const STATIC_WRITABLE: u8 = 13;
const STATIC_READONLY: u8 = 14;
const EXTRA_SIGNER: u8 = 15;
const LOOKUP_TABLE: u8 = 16;
const LOOKUP_WRITABLE: u8 = 17;
const LOOKUP_READONLY: u8 = 18;
const BLOCKHASH: u8 = 19;
const LAST_VALID_BLOCK_HEIGHT: u64 = 12_345;

fn address(byte: u8) -> Address {
    Address::from_bytes([byte; 32])
}

fn lifetime() -> Lifetime {
    Lifetime::Blockhash {
        blockhash: Hash::from_bytes([BLOCKHASH; 32]),
        last_valid_block_height: LAST_VALID_BLOCK_HEIGHT,
    }
}

fn meta(byte: u8, role: AccountRole) -> AccountMeta {
    AccountMeta {
        address: address(byte),
        role,
    }
}

fn instruction(accounts: Vec<AccountMeta>, data: Vec<u8>) -> Instruction {
    Instruction {
        program_address: address(PROGRAM),
        accounts,
        data,
    }
}

fn decompile() -> Outcome {
    legacy_roundtrip()?;
    v0_roundtrip()?;
    v1_roundtrip()?;
    missing_lookup()
}

fn legacy_roundtrip() -> Outcome {
    let (message, decompiled) = legacy_fixture()?;

    legacy_shape(&decompiled)?;
    legacy_bytes(&message, &decompiled)
}

fn legacy_fixture() -> Result<(Message, DecompiledMessage), &'static str> {
    let instructions = vec![
        instruction(
            vec![
                meta(STATIC_WRITABLE, AccountRole::Writable),
                meta(STATIC_WRITABLE, AccountRole::Writable),
                meta(STATIC_READONLY, AccountRole::Readonly),
                meta(EXTRA_SIGNER, AccountRole::WritableSigner),
            ],
            vec![7, 8, 9],
        ),
        instruction(Vec::new(), Vec::new()),
    ];
    let message = Message::compile(
        Version::Legacy,
        address(PAYER),
        lifetime(),
        &instructions,
        &[],
    )
    .map_err(|_| "legacy compile")?;
    let decompiled = message.decompile(&[]).map_err(|_| "legacy decompile")?;

    Ok((message, decompiled))
}

fn legacy_shape(decompiled: &DecompiledMessage) -> Outcome {
    crate::require(decompiled.version() == Version::Legacy, "legacy version")?;
    crate::require(decompiled.payer() == address(PAYER), "legacy payer")?;
    crate::require(decompiled.lifetime() == lifetime(), "legacy lifetime")?;
    crate::require(decompiled.config().is_none(), "legacy config")?;
    let rebuilt = decompiled.instructions();

    crate::require(rebuilt.len() == 2, "legacy instruction count")?;
    crate::require(
        rebuilt[0].program_address == address(PROGRAM),
        "legacy program",
    )?;
    crate::require(
        rebuilt[0].accounts
            == [
                InstructionAccount::Account(meta(STATIC_WRITABLE, AccountRole::Writable)),
                InstructionAccount::Account(meta(STATIC_WRITABLE, AccountRole::Writable)),
                InstructionAccount::Account(meta(STATIC_READONLY, AccountRole::Readonly)),
                InstructionAccount::Account(meta(EXTRA_SIGNER, AccountRole::WritableSigner)),
            ],
        "legacy accounts",
    )?;
    crate::require(rebuilt[0].data == [7, 8, 9], "legacy data")
}

fn legacy_bytes(message: &Message, decompiled: &DecompiledMessage) -> Outcome {
    crate::require(
        decompiled
            .compile(&[])
            .map_err(|_| "legacy recompile")?
            .bytes()
            == message.bytes(),
        "legacy byte roundtrip",
    )
}

fn v0_roundtrip() -> Outcome {
    let (message, tables) = v0_fixture()?;
    let decompiled = message.decompile(&tables).map_err(|_| "v0 decompile")?;

    crate::require(decompiled.version() == Version::V0, "v0 version")?;
    crate::require(decompiled.lifetime() == lifetime(), "v0 lifetime metadata")?;
    v0_provenance(&decompiled)?;
    crate::require(
        decompiled
            .compile(&tables)
            .map_err(|_| "v0 recompile")?
            .bytes()
            == message.bytes(),
        "v0 byte roundtrip",
    )?;
    v0_received(&message, &tables)
}

fn v0_fixture() -> Result<(Message, Vec<LookupTable>), &'static str> {
    let tables = vec![LookupTable {
        address: address(LOOKUP_TABLE),
        addresses: vec![address(LOOKUP_WRITABLE), address(LOOKUP_READONLY)],
    }];
    let instructions = vec![instruction(
        vec![
            meta(LOOKUP_WRITABLE, AccountRole::Writable),
            meta(LOOKUP_READONLY, AccountRole::Readonly),
            meta(LOOKUP_WRITABLE, AccountRole::Writable),
            meta(STATIC_READONLY, AccountRole::Readonly),
        ],
        vec![3, 1, 4],
    )];
    let message = Message::compile(
        Version::V0,
        address(PAYER),
        lifetime(),
        &instructions,
        &tables,
    )
    .map_err(|_| "v0 compile")?;

    Ok((message, tables))
}

/// Checks each loaded account keeps its exact table, index and non-signer role.
fn v0_provenance(decompiled: &DecompiledMessage) -> Outcome {
    let mut writable = 0;
    let mut readonly = 0;

    for account in &decompiled.instructions()[0].accounts {
        let InstructionAccount::Lookup(AccountLookupMeta {
            address: loaded,
            lookup_table_address,
            address_index,
            role,
        }) = account
        else {
            continue;
        };

        crate::require(
            *lookup_table_address == address(LOOKUP_TABLE),
            "v0 table identity",
        )?;

        match (*loaded, *address_index) {
            (value, 0) if value == address(LOOKUP_WRITABLE) => {
                crate::require(*role == LookupRole::Writable, "v0 writable role")?;
                writable += 1;
            }
            (value, 1) if value == address(LOOKUP_READONLY) => {
                crate::require(*role == LookupRole::Readonly, "v0 readonly role")?;
                readonly += 1;
            }
            _ => return Err("v0 lookup address or index"),
        }
    }

    crate::require(
        writable == 2 && readonly == 1,
        "v0 duplicate provenance count",
    )
}

fn v0_received(message: &Message, tables: &[LookupTable]) -> Outcome {
    let decoded = Message::decode(message.bytes()).map_err(|_| "v0 wire decode")?;
    let received = decoded
        .decompile(tables)
        .map_err(|_| "v0 received decompile")?;

    crate::require(
        received.lifetime() == Lifetime::Unresolved(Hash::from_bytes([BLOCKHASH; 32])),
        "received lifetime must stay unresolved",
    )?;
    crate::require(
        received.compile(tables) == Err(TransactionError::UnresolvedLifetime),
        "unresolved lifetime must reject recompilation",
    )?;
    crate::require(
        received
            .with_lifetime(lifetime())
            .compile(tables)
            .map_err(|_| "v0 caller lifetime")?
            .bytes()
            == message.bytes(),
        "v0 caller-supplied lifetime roundtrip",
    )
}

const V1_COMPUTE_LIMIT: u32 = 1_400_000;
const V1_LOADED_DATA_LIMIT: u32 = 64;
const V1_HEAP_BYTES: u32 = 65_536;

fn v1_roundtrip() -> Outcome {
    let config = V1Config::default()
        .with_priority_fee(u64::MAX)
        .with_compute_unit_limit(V1_COMPUTE_LIMIT)
        .with_loaded_accounts_data_limit(V1_LOADED_DATA_LIMIT)
        .with_heap_size(HeapSize::new(V1_HEAP_BYTES).map_err(|_| "v1 heap")?);
    let instructions = vec![instruction(
        vec![
            meta(STATIC_WRITABLE, AccountRole::Writable),
            meta(STATIC_READONLY, AccountRole::ReadonlySigner),
        ],
        vec![4, 5],
    )];
    let message = Message::compile_v1(address(PAYER), lifetime(), &instructions, config)
        .map_err(|_| "v1 compile")?;
    let decompiled = message.decompile(&[]).map_err(|_| "v1 decompile")?;

    crate::require(decompiled.version() == Version::V1, "v1 version")?;
    crate::require(decompiled.config() == Some(config), "v1 inline config")?;
    crate::require(
        decompiled.compile(&[]).map_err(|_| "v1 recompile")?.bytes() == message.bytes(),
        "v1 byte roundtrip",
    )?;

    let decoded = Message::decode(message.bytes()).map_err(|_| "v1 wire decode")?;
    let received = decoded
        .decompile(&[])
        .map_err(|_| "v1 received decompile")?;

    crate::require(received.config() == Some(config), "v1 received config")?;
    crate::require(
        received.lifetime() == Lifetime::Unresolved(Hash::from_bytes([BLOCKHASH; 32])),
        "v1 received lifetime",
    )?;
    crate::require(
        received
            .with_lifetime(lifetime())
            .compile(&[])
            .map_err(|_| "v1 caller lifetime")?
            .bytes()
            == message.bytes(),
        "v1 caller-supplied lifetime roundtrip",
    )
}

fn missing_lookup() -> Outcome {
    let tables = vec![LookupTable {
        address: address(LOOKUP_TABLE),
        addresses: vec![address(LOOKUP_WRITABLE)],
    }];
    let message = Message::compile(
        Version::V0,
        address(PAYER),
        lifetime(),
        &[instruction(
            vec![meta(LOOKUP_WRITABLE, AccountRole::Readonly)],
            vec![1],
        )],
        &tables,
    )
    .map_err(|_| "missing lookup compile")?;

    crate::require(
        message.decompile(&[]) == Err(TransactionError::InvalidLookup),
        "missing lookup must reject",
    )
}

// ---------------------------------------------------------------------------
// Per-call explicit signer configuration and local Rc composition.
// ---------------------------------------------------------------------------

const SIGNER_SEED: [u8; 32] = [42; 32];

struct ConfiguredBackend {
    output: SignatureOutput,
    effects: Rc<Cell<usize>>,
    last: Rc<Cell<Option<TransactionSignerConfig>>>,
}

impl Signer for ConfiguredBackend {
    fn address(&self) -> Address {
        self.output.address()
    }
}

struct ConfiguredSigning<'a> {
    backend: &'a ConfiguredBackend,
    config: TransactionSignerConfig,
    suspends: bool,
    local: Rc<Cell<usize>>,
}

impl Future for ConfiguredSigning<'_> {
    type Output = Result<SignatureOutput, SignerError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.suspends {
            self.suspends = false;
            context.waker().wake_by_ref();

            return Poll::Pending;
        }

        self.local.set(self.local.get() + 1);
        self.backend.effects.set(self.backend.effects.get() + 1);
        self.backend.last.set(Some(self.config));

        Poll::Ready(Ok(self.backend.output.clone()))
    }
}

impl TransactionPartialSigner for ConfiguredBackend {
    type Sign<'a> = ConfiguredSigning<'a>;

    fn sign_transaction<'a>(
        &'a self,
        _transaction: &'a solana_kit_core::transaction::Transaction,
        config: TransactionSignerConfig,
    ) -> Self::Sign<'a> {
        ConfiguredSigning {
            backend: self,
            config,
            suspends: true,
            local: Rc::new(Cell::new(0)),
        }
    }
}

const FORWARDED_CALLS: usize = 3;
const ABORTED_MIN_CONTEXT_SLOT: u64 = 7;

fn signer_config() -> Outcome {
    let (transaction, backend) = signer_fixture()?;
    let effects = Rc::clone(&backend.effects);
    let last = Rc::clone(&backend.last);
    let handle = SignerCapability::new(&backend);
    let composition =
        Composition::new(Registration::partial(&handle)).map_err(|_| "signer composition")?;

    forwarded_configs(&composition, &transaction, &effects, &last)?;
    aborted_config(&composition, &transaction, &effects, &last)
}

fn signer_fixture() -> Result<(Transaction, ConfiguredBackend), &'static str> {
    let key = Keypair::from_seed(&SIGNER_SEED);
    let transaction = Message::compile(Version::Legacy, key.address(), lifetime(), &[], &[])
        .map(Transaction::new)
        .map_err(|_| "signer fixture transaction")?;
    let output = SignatureOutput::new(
        key.address(),
        transaction.message().bytes(),
        key.sign(transaction.message().bytes()),
    );
    let backend = ConfiguredBackend {
        output,
        effects: Rc::new(Cell::new(0)),
        last: Rc::new(Cell::new(None)),
    };

    Ok((transaction, backend))
}

fn forwarded_configs(
    composition: &Composition<'_, SignerError>,
    transaction: &Transaction,
    effects: &Cell<usize>,
    last: &Cell<Option<TransactionSignerConfig>>,
) -> Outcome {
    for config in [
        TransactionSignerConfig::default(),
        TransactionSignerConfig {
            aborted: false,
            min_context_slot: Some(0),
        },
        TransactionSignerConfig {
            aborted: false,
            min_context_slot: Some(u64::MAX),
        },
    ] {
        let signed = poll_ready(Box::pin(composition.partially_sign(transaction, config)))
            .map_err(|_| "signer composition suspended")?
            .map_err(|_| "signer composition rejected")?;

        crate::require(last.get() == Some(config), "config forwarding mismatch")?;
        crate::require(
            signed.signatures()[0].is_some(),
            "signed slot was not installed",
        )?;
    }

    crate::require(
        effects.get() == FORWARDED_CALLS,
        "forwarded call effect count",
    )
}

fn aborted_config(
    composition: &Composition<'_, SignerError>,
    transaction: &Transaction,
    effects: &Cell<usize>,
    last: &Cell<Option<TransactionSignerConfig>>,
) -> Outcome {
    let aborted = TransactionSignerConfig {
        aborted: true,
        min_context_slot: Some(ABORTED_MIN_CONTEXT_SLOT),
    };
    let result = poll_ready(Box::pin(composition.partially_sign(transaction, aborted)))
        .map_err(|_| "aborted composition suspended")?;

    crate::require(
        result == Err(SignerError::Cancelled),
        "aborted must return cancellation",
    )?;
    crate::require(
        effects.get() == FORWARDED_CALLS,
        "aborted call reached the backend",
    )?;
    crate::require(
        last.get()
            == Some(TransactionSignerConfig {
                aborted: false,
                min_context_slot: Some(u64::MAX),
            }),
        "aborted call replaced the last config",
    )
}

fn poll_ready<F: Future>(mut future: Pin<Box<F>>) -> Result<F::Output, ()> {
    let mut context = Context::from_waker(Waker::noop());

    for _ in 0..4 {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return Ok(output);
        }
    }

    Err(())
}

// ---------------------------------------------------------------------------
// Token mint and associated-account instruction plans.
// ---------------------------------------------------------------------------

const PLAN_LIMITS: CodecLimits = CodecLimits::new(4096, 4096);
const TOKEN_DECIMALS: u8 = 6;
const MINT_AMOUNT: u64 = 7_654_321;
const TRANSFER_AMOUNT: u64 = 9_876_543_210;
const DEFAULT_RENT_LAMPORTS: u64 = 1_461_600;
const MINT_SPACE: u64 = 82;

const PLAN_PAYER: u8 = 1;
const PLAN_NEW_MINT: u8 = 2;
const PLAN_OWNER: u8 = 3;
const PLAN_MINT: u8 = 4;
const PLAN_AUTHORITY: u8 = 5;
const PLAN_SOURCE: u8 = 6;
const CUSTOM_TOKEN: u8 = 7;
const PLAN_DESTINATION: u8 = 8;
const CUSTOM_ATA: u8 = 9;
const CUSTOM_SYSTEM: u8 = 10;

const EXPECTED_OWNER_ATA: &str = "CxX9kqQpBX4PczGhFTxU4WmHjknEgpNtUAR9LbJCSw8u";
const EXPECTED_AUTHORITY_ATA: &str = "ExdJGUaW8C3EL2TDp4poEekZhYBUhmt8WJa7d8T1kD7d";
const EXPECTED_CUSTOM_OWNER_ATA: &str = "AWuSAwMmNtEny1hf4BitXHcJQBHBr2G8QsM15t2D37Sj";
const EXPECTED_CUSTOM_AUTHORITY_ATA: &str = "GGcnfFWVGymcLkXVb8BgWUPVRGPkmoAqhyCebaP6Z89s";

fn plan_children(plan: &Plan<Instruction>) -> Result<(&[Plan<Instruction>], bool), &'static str> {
    match plan {
        Plan::Sequential { plans, divisible } => Ok((plans, *divisible)),
        Plan::Single(_) | Plan::Parallel(_) => Err("expected a sequential token plan"),
    }
}

fn plan_single(plan: &Plan<Instruction>) -> Result<&Instruction, &'static str> {
    match plan {
        Plan::Single(instruction) => Ok(instruction),
        Plan::Sequential { .. } | Plan::Parallel(_) => Err("expected a single instruction"),
    }
}

fn create_mint_input() -> CreateMintInput {
    CreateMintInput {
        payer: address(PLAN_PAYER),
        new_mint: address(PLAN_NEW_MINT),
        decimals: TOKEN_DECIMALS,
        mint_authority: address(PLAN_OWNER),
        freeze_authority: None,
        mint_account_lamports: None,
    }
}

fn mint_to_ata_input() -> MintToAtaInput {
    MintToAtaInput {
        payer: address(PLAN_PAYER),
        ata: address(PLAN_SOURCE),
        owner: address(PLAN_OWNER),
        mint: address(PLAN_MINT),
        mint_authority: address(PLAN_AUTHORITY),
        amount: MINT_AMOUNT,
        decimals: TOKEN_DECIMALS,
    }
}

fn transfer_to_ata_input() -> TransferToAtaInput {
    TransferToAtaInput {
        payer: address(PLAN_PAYER),
        mint: address(PLAN_MINT),
        source: address(PLAN_SOURCE),
        authority: address(PLAN_AUTHORITY),
        destination: address(PLAN_DESTINATION),
        recipient: address(PLAN_OWNER),
        amount: TRANSFER_AMOUNT,
        decimals: TOKEN_DECIMALS,
    }
}

fn token_plans() -> Outcome {
    create_mint_contract()?;
    ata_contract()?;
    custom_contract()?;
    derived_contract()
}

fn create_mint_contract() -> Outcome {
    let plan = create_mint_plan(
        create_mint_input(),
        CreateMintConfig::default(),
        PLAN_LIMITS,
    )
    .map_err(|_| "create mint plan")?;
    let (children, divisible) = plan_children(&plan)?;

    crate::require(!divisible, "create mint must be indivisible")?;
    crate::require(children.len() == 2, "create mint child count")?;

    let create = plan_single(&children[0])?;
    let initialize = plan_single(&children[1])?;

    crate::require(
        create.program_address == solana_kit_system::SYSTEM_PROGRAM,
        "create mint system identity",
    )?;
    let mut expected_create = vec![0, 0, 0, 0];
    expected_create.extend(DEFAULT_RENT_LAMPORTS.to_le_bytes());
    expected_create.extend(MINT_SPACE.to_le_bytes());
    expected_create.extend(solana_kit_token::PROGRAM.as_bytes());
    crate::require(create.data == expected_create, "create account payload")?;
    crate::require(
        create.accounts
            == [
                meta(PLAN_PAYER, AccountRole::WritableSigner),
                meta(PLAN_NEW_MINT, AccountRole::WritableSigner),
            ],
        "create account roles",
    )?;

    crate::require(
        initialize.program_address == solana_kit_token::PROGRAM,
        "initialize mint identity",
    )?;
    let mut expected_initialize = vec![20, TOKEN_DECIMALS];
    expected_initialize.extend(address(PLAN_OWNER).as_bytes());
    expected_initialize.push(0);
    crate::require(
        initialize.data == expected_initialize,
        "initialize mint payload",
    )?;
    crate::require(
        initialize.accounts == [meta(PLAN_NEW_MINT, AccountRole::Writable)],
        "initialize mint roles",
    )?;
    compiled_signers(&plan, &[address(PLAN_PAYER), address(PLAN_NEW_MINT)])
}

const IDEMPOTENT_ATA_TAG: u8 = 1;
const MINT_TO_CHECKED_TAG: u8 = 14;
const TRANSFER_CHECKED_TAG: u8 = 12;

fn ata_contract() -> Outcome {
    mint_to_contract()?;
    transfer_contract()
}

fn mint_to_contract() -> Outcome {
    let mint_plan = mint_to_ata_plan(mint_to_ata_input(), MintToAtaConfig::default(), PLAN_LIMITS)
        .map_err(|_| "mint to ATA plan")?;
    let (children, divisible) = plan_children(&mint_plan)?;

    crate::require(divisible, "mint to ATA must be divisible")?;
    crate::require(children.len() == 2, "mint to ATA child count")?;
    let create = plan_single(&children[0])?;
    let mint_to = plan_single(&children[1])?;

    crate::require(
        create.program_address == solana_kit_associated_token_account::PROGRAM,
        "ATA create identity",
    )?;
    crate::require(create.data == [IDEMPOTENT_ATA_TAG], "idempotent ATA tag")?;
    crate::require(
        create.accounts
            == [
                meta(PLAN_PAYER, AccountRole::WritableSigner),
                meta(PLAN_SOURCE, AccountRole::Writable),
                meta(PLAN_OWNER, AccountRole::Readonly),
                meta(PLAN_MINT, AccountRole::Readonly),
                AccountMeta {
                    address: solana_kit_system::SYSTEM_PROGRAM,
                    role: AccountRole::Readonly,
                },
                AccountMeta {
                    address: solana_kit_token::PROGRAM,
                    role: AccountRole::Readonly,
                },
            ],
        "ATA create metadata",
    )?;

    let mut expected_mint_to = vec![MINT_TO_CHECKED_TAG];
    expected_mint_to.extend(MINT_AMOUNT.to_le_bytes());
    expected_mint_to.push(TOKEN_DECIMALS);
    crate::require(mint_to.data == expected_mint_to, "mint checked payload")?;
    crate::require(
        mint_to.accounts
            == [
                meta(PLAN_MINT, AccountRole::Writable),
                meta(PLAN_SOURCE, AccountRole::Writable),
                meta(PLAN_AUTHORITY, AccountRole::ReadonlySigner),
            ],
        "mint checked roles",
    )?;
    crate::require(
        address(PLAN_AUTHORITY) != address(PLAN_PAYER),
        "authority must be distinct from payer",
    )?;

    compiled_signers(&mint_plan, &[address(PLAN_PAYER), address(PLAN_AUTHORITY)])
}

fn transfer_contract() -> Outcome {
    let transfer_plan = transfer_to_ata_plan(
        transfer_to_ata_input(),
        TransferToAtaConfig::default(),
        PLAN_LIMITS,
    )
    .map_err(|_| "transfer to ATA plan")?;
    let (children, divisible) = plan_children(&transfer_plan)?;

    crate::require(divisible, "transfer to ATA must be divisible")?;
    let create = plan_single(&children[0])?;
    let transfer = plan_single(&children[1])?;

    crate::require(
        create.accounts
            == [
                meta(PLAN_PAYER, AccountRole::WritableSigner),
                meta(PLAN_DESTINATION, AccountRole::Writable),
                meta(PLAN_OWNER, AccountRole::Readonly),
                meta(PLAN_MINT, AccountRole::Readonly),
                AccountMeta {
                    address: solana_kit_system::SYSTEM_PROGRAM,
                    role: AccountRole::Readonly,
                },
                AccountMeta {
                    address: solana_kit_token::PROGRAM,
                    role: AccountRole::Readonly,
                },
            ],
        "destination create metadata",
    )?;

    let mut expected_transfer = vec![TRANSFER_CHECKED_TAG];
    expected_transfer.extend(TRANSFER_AMOUNT.to_le_bytes());
    expected_transfer.push(TOKEN_DECIMALS);
    crate::require(
        transfer.data == expected_transfer,
        "transfer checked payload",
    )?;
    crate::require(
        transfer.accounts
            == [
                meta(PLAN_SOURCE, AccountRole::Writable),
                meta(PLAN_MINT, AccountRole::Readonly),
                meta(PLAN_DESTINATION, AccountRole::Writable),
                meta(PLAN_AUTHORITY, AccountRole::ReadonlySigner),
            ],
        "transfer checked roles",
    )?;

    compiled_signers(
        &transfer_plan,
        &[address(PLAN_PAYER), address(PLAN_AUTHORITY)],
    )
}

fn custom_contract() -> Outcome {
    custom_create_mint()?;
    custom_mint_to_ata()
}

fn custom_create_mint() -> Outcome {
    let custom_token = address(CUSTOM_TOKEN);
    let custom_system = address(CUSTOM_SYSTEM);
    let plan = create_mint_plan(
        create_mint_input(),
        CreateMintConfig {
            token_program: Some(custom_token),
            system_program: Some(custom_system),
        },
        PLAN_LIMITS,
    )
    .map_err(|_| "custom create mint")?;
    let (children, _) = plan_children(&plan)?;
    let create = plan_single(&children[0])?;
    let initialize = plan_single(&children[1])?;

    crate::require(
        create.program_address == custom_system,
        "custom system identity",
    )?;
    crate::require(
        &create.data[20..52] == custom_token.as_bytes(),
        "custom token owner",
    )?;
    crate::require(
        initialize.program_address == custom_token,
        "custom initialize identity",
    )
}

fn custom_mint_to_ata() -> Outcome {
    let custom_token = address(CUSTOM_TOKEN);
    let custom_ata = address(CUSTOM_ATA);
    let custom_system = address(CUSTOM_SYSTEM);
    let mint_plan = mint_to_ata_plan(
        mint_to_ata_input(),
        MintToAtaConfig {
            token_program: Some(custom_token),
            associated_token_program: Some(custom_ata),
            system_program: Some(custom_system),
        },
        PLAN_LIMITS,
    )
    .map_err(|_| "custom mint to ATA")?;
    let (children, _) = plan_children(&mint_plan)?;
    let create = plan_single(&children[0])?;

    crate::require(create.program_address == custom_ata, "custom ATA identity")?;
    crate::require(
        create
            .accounts
            .iter()
            .map(|account| account.address)
            .collect::<Vec<_>>()
            == [
                address(PLAN_PAYER),
                address(PLAN_SOURCE),
                address(PLAN_OWNER),
                address(PLAN_MINT),
                custom_system,
                custom_token,
            ],
        "custom ATA metadata",
    )?;
    crate::require(
        plan_single(&children[1])?.program_address == custom_token,
        "custom mint checked identity",
    )
}

fn derived_contract() -> Outcome {
    canonical_derived()?;
    custom_derived()
}

fn canonical_derived() -> Outcome {
    let derived = mint_to_ata_derived_plan(
        MintToAtaDerivedInput {
            payer: address(PLAN_PAYER),
            owner: address(PLAN_OWNER),
            mint: address(PLAN_MINT),
            mint_authority: address(PLAN_AUTHORITY),
            amount: MINT_AMOUNT,
            decimals: TOKEN_DECIMALS,
        },
        MintToAtaConfig::default(),
        PLAN_LIMITS,
    )
    .map_err(|_| "derived mint plan")?;
    let (children, _) = plan_children(&derived)?;
    let expected = pinned_address(EXPECTED_OWNER_ATA, "pinned owner ATA")?;

    crate::require(
        plan_single(&children[0])?.accounts[1].address == expected,
        "derived owner ATA",
    )?;
    compiled_signers(&derived, &[address(PLAN_PAYER), address(PLAN_AUTHORITY)])?;

    let derived = transfer_to_ata_derived_plan(
        TransferToAtaDerivedInput {
            payer: address(PLAN_PAYER),
            mint: address(PLAN_MINT),
            authority: address(PLAN_AUTHORITY),
            recipient: address(PLAN_OWNER),
            amount: TRANSFER_AMOUNT,
            decimals: TOKEN_DECIMALS,
        },
        TransferToAtaConfig::default(),
        PLAN_LIMITS,
    )
    .map_err(|_| "derived transfer plan")?;
    let (children, _) = plan_children(&derived)?;
    let transfer = plan_single(&children[1])?;
    let source = pinned_address(EXPECTED_AUTHORITY_ATA, "pinned authority ATA")?;
    let destination = pinned_address(EXPECTED_OWNER_ATA, "pinned owner ATA")?;

    crate::require(
        plan_single(&children[0])?.accounts[1].address == destination,
        "derived destination ATA",
    )?;
    crate::require(
        transfer.accounts[0].address == source && transfer.accounts[2].address == destination,
        "derived transfer ATAs",
    )
}

fn custom_derived() -> Outcome {
    let config = MintToAtaConfig {
        token_program: Some(address(CUSTOM_TOKEN)),
        associated_token_program: Some(address(CUSTOM_ATA)),
        system_program: Some(address(CUSTOM_SYSTEM)),
    };
    let derived = mint_to_ata_derived_plan(
        MintToAtaDerivedInput {
            payer: address(PLAN_PAYER),
            owner: address(PLAN_OWNER),
            mint: address(PLAN_MINT),
            mint_authority: address(PLAN_AUTHORITY),
            amount: MINT_AMOUNT,
            decimals: TOKEN_DECIMALS,
        },
        config,
        PLAN_LIMITS,
    )
    .map_err(|_| "custom derived mint plan")?;
    let (children, _) = plan_children(&derived)?;
    let custom_owner = pinned_address(EXPECTED_CUSTOM_OWNER_ATA, "pinned custom owner ATA")?;

    crate::require(
        plan_single(&children[0])?.accounts[1].address == custom_owner,
        "custom derived owner ATA",
    )?;

    let derived = transfer_to_ata_derived_plan(
        TransferToAtaDerivedInput {
            payer: address(PLAN_PAYER),
            mint: address(PLAN_MINT),
            authority: address(PLAN_AUTHORITY),
            recipient: address(PLAN_OWNER),
            amount: TRANSFER_AMOUNT,
            decimals: TOKEN_DECIMALS,
        },
        TransferToAtaConfig {
            token_program: Some(address(CUSTOM_TOKEN)),
            associated_token_program: Some(address(CUSTOM_ATA)),
            system_program: Some(address(CUSTOM_SYSTEM)),
        },
        PLAN_LIMITS,
    )
    .map_err(|_| "custom derived transfer plan")?;
    let (children, _) = plan_children(&derived)?;
    let transfer = plan_single(&children[1])?;
    let custom_authority =
        pinned_address(EXPECTED_CUSTOM_AUTHORITY_ATA, "pinned custom authority ATA")?;

    crate::require(
        transfer.accounts[0].address == custom_authority
            && transfer.accounts[2].address == custom_owner,
        "custom derived transfer ATAs",
    )
}

fn pinned_address(text: &str, explanation: &'static str) -> Result<Address, &'static str> {
    text.parse::<Address>().map_err(|_| explanation)
}

fn compiled_signers(plan: &Plan<Instruction>, expected: &[Address]) -> Outcome {
    let instructions: Vec<_> = plan.flatten().into_iter().cloned().collect();
    let message = Message::compile(
        Version::Legacy,
        address(PLAN_PAYER),
        lifetime(),
        &instructions,
        &[],
    )
    .map_err(|_| "token plan compile")?;

    crate::require(
        message.signers() == expected,
        "compiled signer requirements",
    )
}

// ---------------------------------------------------------------------------
// Bounded typed parsed-account families through the real account boundary.
// ---------------------------------------------------------------------------

const ACCOUNT_LIMITS: Limits = Limits {
    bytes: 8192,
    depth: 32,
};
const FIXTURE_ADDRESS: Address = Address::from_bytes([90; 32]);
const FIXTURE_OWNER: [u8; 32] = [0; 32];
const PARSED_SPACE: u64 = 165;

const TOKEN_ACCOUNT_MINT: Address = Address::from_bytes([61; 32]);
const TOKEN_ACCOUNT_OWNER: Address = Address::from_bytes([62; 32]);
const MULTISIG_SIGNER_ONE: Address = Address::from_bytes([63; 32]);
const MULTISIG_SIGNER_TWO: Address = Address::from_bytes([64; 32]);
const MULTISIG_SIGNER_THREE: Address = Address::from_bytes([65; 32]);
const TOKEN_AMOUNT_DECIMALS: u8 = 19;
const TOKEN_MINT_DECIMALS: u8 = 9;
const RENT_RESERVE_DECIMALS: u8 = 9;
const DELEGATED_AMOUNT_DECIMALS: u8 = 6;
const TOKEN_RENT_RESERVE: u64 = 2_039_280;
const MULTISIG_REQUIRED: u8 = 2;
const MULTISIG_VALID: u8 = 3;

const STAKE_STAKER: Address = Address::from_bytes([71; 32]);
const STAKE_WITHDRAWER: Address = Address::from_bytes([72; 32]);
const STAKE_CUSTODIAN: Address = Address::from_bytes([73; 32]);
const STAKE_VOTER: Address = Address::from_bytes([74; 32]);
const STAKE_INITIALIZED_STAKER: Address = Address::from_bytes([75; 32]);
const STAKE_INITIALIZED_WITHDRAWER: Address = Address::from_bytes([76; 32]);
const STAKE_INITIALIZED_CUSTODIAN: Address = Address::from_bytes([77; 32]);
const STAKE_LOCKUP_EPOCH: u64 = 7;
const STAKE_CREDITS: u64 = 169_965_713;
const STAKE_WARMUP_RATE: f64 = 0.25;
const STAKE_RENT_RESERVE: u64 = 2_282_880;

const VOTE_VOTER: Address = Address::from_bytes([81; 32]);
const VOTE_WITHDRAWER: Address = Address::from_bytes([82; 32]);
const VOTE_PRIOR_VOTER: Address = Address::from_bytes([83; 32]);
const VOTE_AUTHORIZED_EPOCH: u64 = 529;
const VOTE_COMMISSION: u8 = 50;
const VOTE_CREDIT_EPOCH: u64 = 466;
const VOTE_ROOT_SLOT: u64 = 228_884_499;
const VOTE_SLOT_ONE: u64 = 228_884_500;
const VOTE_SLOT_TWO: u64 = 228_884_501;
const VOTE_CONFIRMATIONS_ONE: u32 = 31;
const VOTE_CONFIRMATIONS_TWO: u32 = 30;
const VOTE_PENDING_REWARDS: u64 = 123_456_789;

const NONCE_AUTHORITY: Address = Address::from_bytes([84; 32]);
const LOOKUP_ADDRESS_ONE: Address = Address::from_bytes([85; 32]);
const LOOKUP_ADDRESS_TWO: Address = Address::from_bytes([86; 32]);
const LOOKUP_AUTHORITY: Address = Address::from_bytes([87; 32]);
const LOOKUP_LAST_EXTENDED_SLOT: u64 = 204_699_234;
const LOADER_PROGRAM_DATA: Address = Address::from_bytes([88; 32]);
const LOADER_SLOT: u64 = 259_942_942;

const CONFIG_SLASH_PENALTY: u8 = 12;
const CONFIG_WARMUP_RATE: f64 = 0.25;
const VALIDATOR_KEY_ONE: Address = Address::from_bytes([89; 32]);
const VALIDATOR_KEY_TWO: Address = Address::from_bytes([90; 32]);
const CLOCK_EPOCH: u64 = 123;
const CLOCK_SCHEDULE_EPOCH: u64 = 456;
const UNKNOWN_VALUE: u64 = 7;

fn parsed_account(
    program: &str,
    parsed: Value,
) -> Result<(String, solana_kit_core::accounts::Account<AccountData>), ClientError> {
    let parsed_text = parsed.to_string();
    let outer = json!({
        "executable": false,
        "lamports": 1,
        "owner": Address::from_bytes(FIXTURE_OWNER).to_string(),
        "rentEpoch": 0,
        "space": PARSED_SPACE,
        "data": {
            "parsed": parsed,
            "program": program,
            "space": PARSED_SPACE,
        },
    })
    .to_string();
    let account = parse_json_rpc_account(FIXTURE_ADDRESS, &outer)?
        .require()
        .map_err(|_| ClientError::AccountData)?;

    Ok((parsed_text, account))
}

fn typed_family(program: &str, parsed: Value) -> Result<ParsedAccountData, &'static str> {
    let (parsed_text, account) =
        parsed_account(program, parsed).map_err(|_| "parsed account boundary")?;
    let exact = exact_parsed(&account).ok_or("parsed account encoding")?;

    if exact != parsed_text {
        return Err("parsed account source bytes changed");
    }

    assert_account_decoded(&account)
        .map_err(|_| "parsed account projection")?
        .decode_typed(ACCOUNT_LIMITS)
        .map_err(|_| "typed parsed account decode")
}

fn exact_parsed(account: &solana_kit_core::accounts::Account<AccountData>) -> Option<&str> {
    match &account.data {
        AccountData::Parsed(parsed) => Some(parsed.parsed.get()),
        AccountData::Bytes { .. } | AccountData::Zstd { .. } => None,
    }
}

fn parsed_families() -> Outcome {
    token_family()?;
    stake_family()?;
    vote_family()?;
    nonce_family()?;
    lookup_family()?;
    loader_family()?;
    config_family()?;
    sysvar_family()?;
    unknown_families()?;
    admission_limits()?;
    binary_fallback()
}

fn token_family() -> Outcome {
    token_account_family()?;
    token_mint_family()?;
    token_multisig_family()
}

fn token_account_family() -> Outcome {
    let mint = TOKEN_ACCOUNT_MINT;
    let owner = TOKEN_ACCOUNT_OWNER;
    let extensions = json!([{"extension": "transferFeeConfig", "state": {"withheldAmount": 42}}]);
    let extensions_text = extensions.to_string();
    let projected = typed_family(
        "spl-token",
        json!({
            "type": "account",
            "info": {
                "isNative": true,
                "mint": mint.to_string(),
                "owner": owner.to_string(),
                "state": "frozen",
                "tokenAmount": {
                    "amount": u64::MAX.to_string(),
                    "decimals": TOKEN_AMOUNT_DECIMALS,
                    "uiAmount": 1.8446744073709552e19,
                    "uiAmountString": "18.446744073709551615",
                },
                "rentExemptReserve": {
                    "amount": TOKEN_RENT_RESERVE.to_string(),
                    "decimals": RENT_RESERVE_DECIMALS,
                    "uiAmountString": "0.00203928",
                },
                "delegate": null,
                "delegatedAmount": {
                    "amount": "18446744073709551614",
                    "decimals": DELEGATED_AMOUNT_DECIMALS,
                    "uiAmountString": "18446744073709.551614",
                },
                "closeAuthority": owner.to_string(),
                "extensions": extensions,
            },
        }),
    )?;

    match projected {
        ParsedAccountData::Token(TokenProgramAccount::Account(info)) => {
            crate::require(info.is_native, "token native flag")?;
            crate::require(info.mint == mint && info.owner == owner, "token addresses")?;
            crate::require(
                info.state == solana_kit_client::parsed::TokenAccountState::Frozen,
                "token state",
            )?;
            crate::require(
                info.token_amount.amount == u64::MAX,
                "token exact u64 amount",
            )?;
            crate::require(
                info.token_amount.decimals == TOKEN_AMOUNT_DECIMALS,
                "token decimals",
            )?;
            crate::require(
                info.token_amount.ui_amount_string == "18.446744073709551615",
                "token display spelling",
            )?;
            crate::require(
                info.rent_exempt_reserve
                    .as_ref()
                    .is_some_and(|amount| amount.amount == TOKEN_RENT_RESERVE),
                "token rent reserve",
            )?;
            crate::require(
                info.delegated_amount
                    .as_ref()
                    .is_some_and(|amount| amount.amount == u64::MAX - 1),
                "token delegated amount",
            )?;
            crate::require(info.close_authority == Some(owner), "token close authority")?;
            crate::require(
                info.extensions.as_ref().map(|value| value.get()) == Some(&extensions_text),
                "token extension bytes",
            )
        }
        _ => Err("token account family"),
    }
}

fn token_mint_family() -> Outcome {
    let projected = typed_family(
        "spl-token-2022",
        json!({
            "type": "mint",
            "info": {
                "decimals": TOKEN_MINT_DECIMALS,
                "isInitialized": true,
                "supply": u64::MAX.to_string(),
                "mintAuthority": TOKEN_ACCOUNT_MINT.to_string(),
                "freezeAuthority": null,
            },
        }),
    )?;

    match projected {
        ParsedAccountData::Token(TokenProgramAccount::Mint(info)) => {
            crate::require(info.supply == u64::MAX, "mint exact u64 supply")?;
            crate::require(info.decimals == TOKEN_MINT_DECIMALS, "mint decimals")?;
            crate::require(
                info.mint_authority == Some(TOKEN_ACCOUNT_MINT),
                "mint authority address",
            )?;
            crate::require(info.freeze_authority.is_none(), "mint freeze absence")?;
            crate::require(info.is_initialized, "mint initialized")
        }
        _ => Err("mint family"),
    }
}

fn token_multisig_family() -> Outcome {
    let projected = typed_family(
        "spl-token",
        json!({
            "type": "multisig",
            "info": {
                "isInitialized": true,
                "numRequiredSigners": MULTISIG_REQUIRED,
                "numValidSigners": MULTISIG_VALID,
                "signers": [
                    MULTISIG_SIGNER_ONE.to_string(),
                    MULTISIG_SIGNER_TWO.to_string(),
                    MULTISIG_SIGNER_THREE.to_string(),
                ],
            },
        }),
    )?;

    match projected {
        ParsedAccountData::Token(TokenProgramAccount::Multisig(info)) => {
            crate::require(
                info.num_required_signers == MULTISIG_REQUIRED,
                "multisig threshold",
            )?;
            crate::require(
                info.num_valid_signers == MULTISIG_VALID,
                "multisig capacity",
            )?;
            crate::require(
                info.signers.len() == usize::from(MULTISIG_VALID),
                "multisig signer count",
            )?;
            crate::require(
                info.signers[2] == MULTISIG_SIGNER_THREE,
                "multisig signer order",
            )
        }
        _ => Err("multisig family"),
    }
}

fn stake_family() -> Outcome {
    delegated_stake_family()?;
    initialized_stake_family()
}

fn delegated_stake_family() -> Outcome {
    let projected = typed_family(
        "stake",
        json!({
            "type": "delegated",
            "info": {
                "meta": {
                    "authorized": {
                        "staker": STAKE_STAKER.to_string(),
                        "withdrawer": STAKE_WITHDRAWER.to_string(),
                    },
                    "lockup": {
                        "custodian": STAKE_CUSTODIAN.to_string(),
                        "epoch": STAKE_LOCKUP_EPOCH,
                        "unixTimestamp": i64::MIN,
                    },
                    "rentExemptReserve": STAKE_RENT_RESERVE.to_string(),
                },
                "stake": {
                    "creditsObserved": STAKE_CREDITS,
                    "delegation": {
                        "activationEpoch": "386",
                        "deactivationEpoch": u64::MAX.to_string(),
                        "stake": u64::MAX.to_string(),
                        "voter": STAKE_VOTER.to_string(),
                        "warmupCooldownRate": STAKE_WARMUP_RATE,
                    },
                },
            },
        }),
    )?;

    match projected {
        ParsedAccountData::Stake(StakeProgramAccount::Delegated(info)) => {
            crate::require(
                info.meta.rent_exempt_reserve == STAKE_RENT_RESERVE,
                "stake rent reserve",
            )?;
            crate::require(
                info.meta.lockup.unix_timestamp == i64::MIN,
                "stake lockup timestamp",
            )?;
            let stake = info.stake.ok_or("delegated stake data")?;
            crate::require(
                stake.credits_observed == STAKE_CREDITS,
                "stake credits observed",
            )?;
            crate::require(
                stake.delegation.stake == u64::MAX,
                "stake exact delegated amount",
            )?;
            crate::require(
                stake.delegation.deactivation_epoch == u64::MAX,
                "stake exact deactivation epoch",
            )?;
            crate::require(stake.delegation.voter == STAKE_VOTER, "stake voter address")?;
            crate::require(
                stake.delegation.warmup_cooldown_rate == Some(STAKE_WARMUP_RATE),
                "stake warmup rate",
            )
        }
        _ => Err("delegated stake family"),
    }
}

fn initialized_stake_family() -> Outcome {
    let projected = typed_family(
        "stake",
        json!({
            "type": "initialized",
            "info": {
                "meta": {
                    "authorized": {
                        "staker": STAKE_INITIALIZED_STAKER.to_string(),
                        "withdrawer": STAKE_INITIALIZED_WITHDRAWER.to_string(),
                    },
                    "lockup": {
                        "custodian": STAKE_INITIALIZED_CUSTODIAN.to_string(),
                        "epoch": 0,
                        "unixTimestamp": 0,
                    },
                    "rentExemptReserve": "0",
                },
                "stake": null,
            },
        }),
    )?;

    match projected {
        ParsedAccountData::Stake(StakeProgramAccount::Initialized(info)) => {
            crate::require(info.stake.is_none(), "initialized stake absence")?;
            crate::require(
                info.meta.authorized.staker == STAKE_INITIALIZED_STAKER,
                "initialized stake staker",
            )
        }
        _ => Err("initialized stake family"),
    }
}

fn vote_family() -> Outcome {
    let projected = typed_family("vote", vote_fixture())?;

    match projected {
        ParsedAccountData::Vote(info) => {
            crate::require(
                info.authorized_voters.len() == 1,
                "vote authorized voter count",
            )?;
            crate::require(
                info.authorized_voters[0].authorized_voter == VOTE_VOTER,
                "vote authorized voter address",
            )?;
            crate::require(info.votes.len() == 2, "vote count")?;
            crate::require(info.votes[1].slot == VOTE_SLOT_TWO, "vote slot order")?;
            crate::require(
                info.epoch_credits[0].credits == u64::MAX,
                "vote exact epoch credits",
            )?;
            crate::require(
                info.pending_delegator_rewards == Some(VOTE_PENDING_REWARDS),
                "vote exact pending rewards",
            )?;
            crate::require(info.root_slot == Some(VOTE_ROOT_SLOT), "vote root slot")?;
            crate::require(
                info.last_timestamp.timestamp == i64::MIN,
                "vote last timestamp",
            )?;
            crate::require(info.prior_voters.len() == 1, "vote prior voter count")
        }
        _ => Err("vote family"),
    }
}

fn vote_fixture() -> Value {
    json!({
        "type": "vote",
        "info": {
            "authorizedVoters": [
                {"authorizedVoter": VOTE_VOTER.to_string(), "epoch": VOTE_AUTHORIZED_EPOCH},
            ],
            "authorizedWithdrawer": VOTE_WITHDRAWER.to_string(),
            "commission": VOTE_COMMISSION,
            "epochCredits": [
                {
                    "credits": u64::MAX.to_string(),
                    "epoch": VOTE_CREDIT_EPOCH,
                    "previousCredits": "68325825",
                },
            ],
            "lastTimestamp": {"slot": u64::MAX, "timestamp": i64::MIN},
            "nodePubkey": VOTE_VOTER.to_string(),
            "pendingDelegatorRewards": VOTE_PENDING_REWARDS.to_string(),
            "priorVoters": [
                {
                    "authorizedPubkey": VOTE_PRIOR_VOTER.to_string(),
                    "epochOfLastAuthorizedSwitch": 1,
                    "targetEpoch": 2,
                },
            ],
            "rootSlot": VOTE_ROOT_SLOT,
            "votes": [
                {"confirmationCount": VOTE_CONFIRMATIONS_ONE, "latency": 2, "slot": VOTE_SLOT_ONE},
                {"confirmationCount": VOTE_CONFIRMATIONS_TWO, "slot": VOTE_SLOT_TWO},
            ],
        },
    })
}

fn nonce_family() -> Outcome {
    let blockhash = Hash::from_bytes([9; 32]);
    let projected = typed_family(
        "nonce",
        json!({
            "type": "initialized",
            "info": {
                "authority": NONCE_AUTHORITY.to_string(),
                "blockhash": blockhash.to_string(),
                "feeCalculator": {"lamportsPerSignature": u64::MAX.to_string()},
            },
        }),
    )?;

    match projected {
        ParsedAccountData::Nonce(info) => {
            crate::require(info.blockhash == blockhash, "nonce blockhash")?;
            crate::require(
                info.fee_calculator.lamports_per_signature == u64::MAX,
                "nonce exact fee",
            )
        }
        _ => Err("nonce family"),
    }
}

fn lookup_family() -> Outcome {
    let projected = typed_family(
        "address-lookup-table",
        json!({
            "type": "lookupTable",
            "info": {
                "addresses": [
                    LOOKUP_ADDRESS_ONE.to_string(),
                    LOOKUP_ADDRESS_TWO.to_string(),
                ],
                "authority": LOOKUP_AUTHORITY.to_string(),
                "deactivationSlot": u64::MAX.to_string(),
                "lastExtendedSlot": LOOKUP_LAST_EXTENDED_SLOT.to_string(),
                "lastExtendedSlotStartIndex": 255,
            },
        }),
    )?;

    match projected {
        ParsedAccountData::AddressLookupTable(info) => {
            crate::require(info.addresses.len() == 2, "lookup address count")?;
            crate::require(
                info.deactivation_slot == u64::MAX,
                "lookup exact deactivation slot",
            )?;
            crate::require(
                info.last_extended_slot == LOOKUP_LAST_EXTENDED_SLOT,
                "lookup last extended slot",
            )?;
            crate::require(
                info.last_extended_slot_start_index == 255,
                "lookup start index",
            )?;
            crate::require(
                info.authority == Some(LOOKUP_AUTHORITY),
                "lookup authority address",
            )
        }
        _ => Err("address lookup table family"),
    }
}

fn loader_family() -> Outcome {
    let projected = typed_family(
        "bpf-upgradeable-loader",
        json!({
            "type": "program",
            "info": {"programData": LOADER_PROGRAM_DATA.to_string()},
        }),
    )?;

    match projected {
        ParsedAccountData::BpfUpgradeableLoader(BpfUpgradeableLoaderProgramAccount::Program(
            info,
        )) => crate::require(
            info.program_data == LOADER_PROGRAM_DATA,
            "loader program data address",
        ),
        _ => Err("loader program family"),
    }?;

    let projected = typed_family(
        "bpf-upgradeable-loader",
        json!({
            "type": "programData",
            "info": {
                "authority": null,
                "data": ["AQID", "base64"],
                "slot": LOADER_SLOT,
            },
        }),
    )?;

    match projected {
        ParsedAccountData::BpfUpgradeableLoader(
            BpfUpgradeableLoaderProgramAccount::ProgramData(info),
        ) => {
            crate::require(info.authority.is_none(), "loader authority absence")?;
            crate::require(info.data.data == "AQID", "loader encoded bytes")?;
            crate::require(
                info.data.encoding == Base64Encoding::Base64,
                "loader encoding",
            )?;
            crate::require(info.slot == LOADER_SLOT, "loader slot")
        }
        _ => Err("loader program data family"),
    }
}

fn config_family() -> Outcome {
    let projected = typed_family(
        "config",
        json!({
            "type": "stakeConfig",
            "info": {
                "slashPenalty": CONFIG_SLASH_PENALTY,
                "warmupCooldownRate": CONFIG_WARMUP_RATE,
            },
        }),
    )?;

    match projected {
        ParsedAccountData::Config(ConfigProgramAccount::StakeConfig(info)) => {
            crate::require(
                info.slash_penalty == CONFIG_SLASH_PENALTY,
                "config slash penalty",
            )?;
            crate::require(
                info.warmup_cooldown_rate == CONFIG_WARMUP_RATE,
                "config warmup rate",
            )
        }
        _ => Err("stake config family"),
    }?;

    let config_data = json!({"name": "HoldTheNode"});
    let config_text = config_data.to_string();
    let projected = typed_family(
        "config",
        json!({
            "type": "validatorInfo",
            "info": {
                "configData": config_data,
                "keys": [
                    {"pubkey": VALIDATOR_KEY_ONE.to_string(), "signer": false},
                    {"pubkey": VALIDATOR_KEY_TWO.to_string(), "signer": true},
                ],
            },
        }),
    )?;

    match projected {
        ParsedAccountData::Config(ConfigProgramAccount::ValidatorInfo(info)) => {
            crate::require(info.keys.len() == 2, "validator key count")?;
            crate::require(info.keys[1].signer, "validator signer flag")?;
            crate::require(
                info.config_data.get() == config_text.as_str(),
                "validator opaque config bytes",
            )
        }
        _ => Err("validator config family"),
    }
}

fn sysvar_family() -> Outcome {
    let projected = typed_family(
        "sysvar",
        json!({
            "type": "clock",
            "info": {
                "epoch": CLOCK_EPOCH,
                "epochStartTimestamp": i64::MIN,
                "leaderScheduleEpoch": CLOCK_SCHEDULE_EPOCH,
                "slot": u64::MAX,
                "unixTimestamp": -1,
            },
        }),
    )?;

    match projected {
        ParsedAccountData::Sysvar(SysvarAccount::Clock(info)) => {
            crate::require(info.epoch == CLOCK_EPOCH, "clock epoch")?;
            crate::require(info.epoch_start_timestamp == i64::MIN, "clock start")?;
            crate::require(
                info.leader_schedule_epoch == CLOCK_SCHEDULE_EPOCH,
                "clock schedule epoch",
            )?;
            crate::require(info.slot == u64::MAX, "clock exact u64 slot")?;
            crate::require(info.unix_timestamp == -1, "clock timestamp")
        }
        _ => Err("clock sysvar family"),
    }
}

fn unknown_families() -> Outcome {
    let unknown = json!({"type": "customState", "info": {"value": UNKNOWN_VALUE}});
    let text = unknown.to_string();
    let projected = typed_family("future-program", unknown)?;

    match projected {
        ParsedAccountData::UnknownProgram(UnknownParsed {
            program,
            type_name,
            value,
        }) => {
            crate::require(program == "future-program", "unknown parser name")?;
            crate::require(
                type_name.as_deref() == Some("customState"),
                "unknown parser type",
            )?;
            crate::require(value.get() == text, "unknown program raw preservation")
        }
        _ => Err("unknown program family"),
    }?;

    let unknown = json!({"type": "futureState"});
    let text = unknown.to_string();
    let projected = typed_family("spl-token", unknown)?;

    match projected {
        ParsedAccountData::UnknownType(UnknownParsed {
            program,
            type_name,
            value,
        }) => {
            crate::require(program == "spl-token", "unknown type parser")?;
            crate::require(
                type_name.as_deref() == Some("futureState"),
                "unknown type discriminator",
            )?;
            crate::require(value.get() == text, "unknown discriminator preserves raw")
        }
        _ => Err("unknown discriminator family"),
    }
}

fn admission_limits() -> Outcome {
    let (_, account) = parsed_account(
        "spl-token",
        json!({
            "type": "mint",
            "info": {
                "decimals": 0,
                "isInitialized": true,
                "supply": "0",
            },
        }),
    )
    .map_err(|_| "admission fixture")?;
    let parsed = assert_account_decoded(&account).map_err(|_| "admission projection")?;

    crate::require(
        matches!(
            parsed.decode_typed(Limits {
                bytes: 16,
                depth: 32
            }),
            Err(ClientError::BodyLimit)
        ),
        "parsed byte admission",
    )?;
    crate::require(
        matches!(
            parsed.decode_typed(Limits {
                bytes: 8192,
                depth: 1
            }),
            Err(ClientError::DepthLimit)
        ),
        "parsed depth admission",
    )?;

    let duplicate = r#"{"executable":false,"lamports":1,"owner":"11111111111111111111111111111111","rentEpoch":0,"space":165,"data":{"parsed":{"type":"mint","type":"mint","info":{"decimals":0,"isInitialized":true,"supply":"0"}},"program":"spl-token","space":165}}"#;
    let account = parse_json_rpc_account(FIXTURE_ADDRESS, duplicate)
        .map_err(|_| "duplicate fixture")?
        .require()
        .map_err(|_| "duplicate presence")?;
    let parsed = assert_account_decoded(&account).map_err(|_| "duplicate projection")?;

    crate::require(
        matches!(
            parsed.decode_typed(ACCOUNT_LIMITS),
            Err(ClientError::Schema(_))
        ),
        "duplicate keys must reject",
    )
}

fn binary_fallback() -> Outcome {
    let raw = r#"{"executable":false,"lamports":1,"owner":"11111111111111111111111111111111","rentEpoch":0,"space":3,"data":["AQID","base64"]}"#;
    let account = parse_json_rpc_account(FIXTURE_ADDRESS, raw)
        .map_err(|_| "binary fallback fixture")?
        .require()
        .map_err(|_| "binary fallback presence")?;

    match &account.data {
        AccountData::Bytes { bytes, slice } => {
            crate::require(bytes == &[1, 2, 3], "binary fallback bytes were re-encoded")?;
            crate::require(slice.is_none(), "binary fallback slice")
        }
        AccountData::Parsed(_) | AccountData::Zstd { .. } => {
            Err("binary fallback changed representation")
        }
    }?;

    crate::require(
        assert_account_decoded(&account).is_err(),
        "binary fallback must not become typed",
    )
}

// ---------------------------------------------------------------------------
// Fixed-point codec widths permit direct Paired composition.
// ---------------------------------------------------------------------------

const PAIR_RAW: i32 = -125;
const BINARY_PAIR_RAW: u8 = 255;

fn fixed_point_pairing() -> Outcome {
    signed_pair()?;
    binary_pair()
}

fn signed_pair() -> Outcome {
    let shape = Shape::<10>::new(Signedness::Signed, 16, 2).map_err(|_| "decimal shape")?;
    let codec = shape.codec(Endian::Little).map_err(|_| "decimal codec")?;

    crate::require(
        codec.encode_size() == Ok(Size::Fixed(2)),
        "codec encode size",
    )?;
    crate::require(
        codec.decode_size() == Ok(Size::Fixed(2)),
        "codec decode size",
    )?;
    crate::require(codec.fixed_size() == 2, "codec fixed size")?;

    let paired = Paired::<_, _, DecimalFixedPoint, DecimalFixedPoint>::try_new(codec, codec)
        .map_err(|_| "built-in halves must pair directly")?;
    let value = shape.raw(PAIR_RAW).map_err(|_| "decimal raw")?;

    crate::require(
        paired.encode_size() == Ok(Size::Fixed(2)),
        "paired encode size",
    )?;
    crate::require(
        paired.encode(&value, 2).map_err(|_| "paired encode")? == [0x83, 0xff],
        "paired encode bytes",
    )?;
    crate::require(
        paired
            .decode(&[0x83, 0xff], CodecLimits::new(2, 2))
            .map_err(|_| "paired decode")?
            == value,
        "paired decode value",
    )?;

    let wide = Shape::<10>::new(Signedness::Signed, 32, 2)
        .map_err(|_| "wide shape")?
        .codec(Endian::Little)
        .map_err(|_| "wide codec")?;

    crate::require(
        Paired::<_, _, DecimalFixedPoint, DecimalFixedPoint>::try_new(codec, wide).err()
            == Some(codecs::PairError::FixedSizeMismatch {
                encoder: 2,
                decoder: 4,
            }),
        "mismatched built-in widths",
    )
}

fn binary_pair() -> Outcome {
    let unsigned = Shape::<2>::new(Signedness::Unsigned, 8, 4).map_err(|_| "binary shape")?;
    let binary = unsigned.codec(Endian::Big).map_err(|_| "binary codec")?;
    let paired =
        Paired::<_, _, solana_kit_core::fixed_points::FixedPoint<2>, _>::try_new(binary, binary)
            .map_err(|_| "binary halves must pair directly")?;
    let value = unsigned.raw(BINARY_PAIR_RAW).map_err(|_| "binary raw")?;

    crate::require(
        paired.encode(&value, 1).map_err(|_| "binary encode")? == [0xff],
        "binary paired encode",
    )?;
    crate::require(
        paired
            .decode(&[0xff], CodecLimits::new(1, 1))
            .map_err(|_| "binary decode")?
            == value,
        "binary paired decode",
    )
}

// ---------------------------------------------------------------------------
// Real scripted client exchange, pending completion/drop and reentrant wakers.
// ---------------------------------------------------------------------------

fn scripted_client() -> Outcome {
    exact_client_exchange()?;
    pending_completion()?;
    pending_drop()
}

const CLIENT_ADDRESS: Address = Address::from_bytes([91; 32]);
const BALANCE_SLOT: u64 = 42;
const COMPLETION_SLOT: u64 = 7;
const COMPLETION_VALUE: u64 = 5;

fn exact_client_exchange() -> Outcome {
    let transport =
        ScriptedTransport::new([ScriptStep::success(canned::balance(BALANCE_SLOT, u64::MAX))]);
    let client = Client::new(transport.clone(), Limits::default());
    let mut call = Box::pin(client.call(Balance(CLIENT_ADDRESS, None)));
    let mut context = Context::from_waker(Waker::noop());

    let response = match call.as_mut().poll(&mut context) {
        Poll::Ready(Ok(response)) => response,
        Poll::Ready(Err(_)) => return Err("scripted success failed"),
        Poll::Pending => return Err("scripted success stayed pending"),
    };

    crate::require(
        response.context.slot == BALANCE_SLOT,
        "scripted context slot",
    )?;
    crate::require(response.value == u64::MAX, "scripted exact u64 value")?;
    crate::require(transport.remaining() == 0, "scripted step consumption")?;
    captured_requests(&transport)
}

fn captured_requests(transport: &ScriptedTransport) -> Outcome {
    let requests = transport.requests();

    crate::require(requests.len() == 1, "captured request count")?;
    crate::require(requests[0].id == 1, "captured request id")?;
    crate::require(requests[0].method == "getBalance", "captured method")?;
    crate::require(
        requests[0].body
            == format!(
                r#"{{"jsonrpc":"2.0","id":1,"method":"getBalance","params":["{}"]}}"#,
                CLIENT_ADDRESS
            )
            .into_bytes(),
        "captured exact request body",
    )?;
    crate::require(
        requests[0].max_reply_bytes == Limits::default().bytes,
        "captured receive budget",
    )
}

struct ReentrantWaker {
    wakes: Arc<core::sync::atomic::AtomicUsize>,
    drops: Arc<core::sync::atomic::AtomicUsize>,
    reenters: Arc<core::sync::atomic::AtomicUsize>,
}

std::thread_local! {
    // The exchange is not `Send` on wasm32-unknown-unknown, while `Waker` must be.
    // A per-thread probe keeps the callback deterministic without unsafe code.
    static PENDING_PROBE: core::cell::RefCell<Option<PendingExchange>> =
        const { core::cell::RefCell::new(None) };
}

fn reenter_pending(counter: &core::sync::atomic::AtomicUsize) {
    PENDING_PROBE.with(|slot| {
        if let Some(exchange) = slot.borrow().as_ref() {
            let _ = exchange.status();
            counter.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        }
    });
}

impl Wake for ReentrantWaker {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        reenter_pending(&self.reenters);
        self.wakes
            .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    }
}

impl Drop for ReentrantWaker {
    fn drop(&mut self) {
        reenter_pending(&self.reenters);
        self.drops
            .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    }
}

fn pending_completion() -> Outcome {
    let (step, exchange) = ScriptStep::pending();
    let transport = ScriptedTransport::new([step]);
    let client = Client::new(transport, Limits::default());
    let mut call = Box::pin(client.call(Balance(CLIENT_ADDRESS, None)));
    let wakes = Arc::new(core::sync::atomic::AtomicUsize::new(0));
    let drops = Arc::new(core::sync::atomic::AtomicUsize::new(0));
    let reenters = Arc::new(core::sync::atomic::AtomicUsize::new(0));
    PENDING_PROBE.with(|slot| *slot.borrow_mut() = Some(exchange.clone()));

    poll_pending_once(&mut call, &exchange, &wakes, &drops, &reenters)?;
    pending_registration(&exchange)?;
    crate::require(
        exchange.complete_success(canned::balance(COMPLETION_SLOT, COMPLETION_VALUE)),
        "pending completion acceptance",
    )?;
    completion_callbacks(&exchange, &wakes, &drops, &reenters)?;

    let mut context = Context::from_waker(Waker::noop());

    let response = match call.as_mut().poll(&mut context) {
        Poll::Ready(Ok(response)) => response,
        Poll::Ready(Err(_)) => return Err("completed pending call failed"),
        Poll::Pending => return Err("completed pending call stayed pending"),
    };

    crate::require(
        response.context.slot == COMPLETION_SLOT,
        "pending context slot",
    )?;
    crate::require(response.value == COMPLETION_VALUE, "pending exact value")?;
    pending_delivery(&exchange)?;
    drop(call);
    PENDING_PROBE.with(|slot| *slot.borrow_mut() = None);

    crate::require(
        !exchange.status().cancelled,
        "delivered call was marked cancelled",
    )
}

fn poll_pending_once<F: Future + Unpin>(
    call: &mut F,
    exchange: &PendingExchange,
    wakes: &Arc<core::sync::atomic::AtomicUsize>,
    drops: &Arc<core::sync::atomic::AtomicUsize>,
    reenters: &Arc<core::sync::atomic::AtomicUsize>,
) -> Outcome {
    {
        let waker = Waker::from(Arc::new(ReentrantWaker {
            wakes: Arc::clone(wakes),
            drops: Arc::clone(drops),
            reenters: Arc::clone(reenters),
        }));

        crate::require(
            matches!(
                Pin::new(call).poll(&mut Context::from_waker(&waker)),
                Poll::Pending
            ),
            "pending call must suspend",
        )?;
    }

    crate::require(exchange.status().polls == 1, "pending poll count")
}

fn pending_registration(exchange: &PendingExchange) -> Outcome {
    let status = exchange.status();

    crate::require(status.request_id == Some(1), "pending request id")?;
    crate::require(status.has_waker, "pending waker registration")?;
    crate::require(
        !status.completed && !status.delivered,
        "pending initial state",
    )
}

fn completion_callbacks(
    exchange: &PendingExchange,
    wakes: &Arc<core::sync::atomic::AtomicUsize>,
    drops: &Arc<core::sync::atomic::AtomicUsize>,
    reenters: &Arc<core::sync::atomic::AtomicUsize>,
) -> Outcome {
    crate::require(
        wakes.load(core::sync::atomic::Ordering::SeqCst) == 1,
        "reentrant wake delivery",
    )?;
    crate::require(exchange.status().wakes == 1, "pending recorded wake count")?;
    crate::require(
        drops.load(core::sync::atomic::Ordering::SeqCst) == 1,
        "completed waker drop callback",
    )?;
    crate::require(
        reenters.load(core::sync::atomic::Ordering::SeqCst) == 2,
        "reentrant pending status callbacks",
    )
}

fn pending_delivery(exchange: &PendingExchange) -> Outcome {
    let status = exchange.status();

    crate::require(
        status.completed && status.delivered,
        "pending delivered state",
    )?;
    crate::require(status.polls == 2, "pending second poll count")
}

fn pending_drop() -> Outcome {
    let (step, exchange) = ScriptStep::pending();
    let transport = ScriptedTransport::new([step]);
    let client = Client::new(transport, Limits::default());
    let mut call = Box::pin(client.call(Balance(CLIENT_ADDRESS, None)));

    crate::require(
        matches!(
            call.as_mut().poll(&mut Context::from_waker(Waker::noop())),
            Poll::Pending
        ),
        "dropped call must suspend",
    )?;
    drop(call);

    let status = exchange.status();

    crate::require(status.cancelled, "dropped call cancellation state")?;
    crate::require(!status.has_waker, "dropped call released waker")?;
    crate::require(!status.delivered, "dropped call was delivered")?;
    crate::require(
        !exchange.complete_success(canned::balance(1, 1)),
        "completion after drop must reject",
    )?;
    crate::require(!exchange.wake(), "wake after drop must reject")
}
