# Addresses and keys

Every account, program and signer on Solana is named by a 32-byte address. This page shows how to parse and format addresses with `solana_kit_core`, derive program addresses, and create or import Ed25519 keypairs.

The PDA and keypair parts of this page need `solana_kit_core` with the `crypto` feature. Generating a fresh key on a native host also needs `solana_kit_native` with `keys`:

```toml
[dependencies]
solana_kit_core = { git = "https://github.com/pina-rs/solana_kit", features = ["crypto"] }
solana_kit_native = { git = "https://github.com/pina-rs/solana_kit", features = ["keys"] }
```

## Addresses

`Address` is a `Copy` value holding exactly 32 bytes. It parses from base58 with `FromStr`, and its `Display` writes base58:

```rust
use solana_kit_core::Address;

let system: Address = "11111111111111111111111111111111".parse()?;
assert_eq!(system.to_string(), "11111111111111111111111111111111");

// Invalid characters and wrong lengths are rejected.
assert!("not-base58!".parse::<Address>().is_err());
```

Parsing returns `solana_kit_core::ParseError`. Its variants say whether the text had a character outside the base58 alphabet, decoded to the wrong number of bytes, or was too long to be an address at all. The length limit is checked before decoding, so untrusted input cannot force a large allocation.

Convert to and from bytes:

```rust
use solana_kit_core::Address;

let from_array = Address::from_bytes([0; 32]);
let from_slice = Address::try_from(&from_array.as_bytes()[..])?; // checks the length
let bytes: [u8; 32] = from_slice.to_bytes();
assert!(Address::try_from(&[0_u8; 31][..]).is_err());
```

`Hash` (blockhashes) and `Signature` (64 bytes) follow the same rules: `from_bytes`, `as_bytes`, `to_bytes`, `FromStr` and `Display`.

> [!NOTE]
> A parsed `Address` is only 32 bytes. Parsing does not check that it is a valid Ed25519 public key.

### Well-known addresses

`solana_kit_core::address_constants` has the standard program, sysvar and mint addresses as constants, so you do not have to parse them:

```rust
use solana_kit_core::address_constants::{SYSTEM_PROGRAM, TOKEN_PROGRAM};

assert_eq!(SYSTEM_PROGRAM.to_string(), "11111111111111111111111111111111");
println!("{TOKEN_PROGRAM}");
```

Sysvar addresses such as `CLOCK_ADDRESS` and `RENT_ADDRESS` live in `solana_kit_core::sysvars`.

## Program-derived addresses

With `crypto`, `find_program_address` finds the highest bump seed whose address lies off the Ed25519 curve:

```rust
use solana_kit_core::{
    Address,
    crypto::{find_program_address, is_on_curve},
};

let program: Address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".parse()?;
let owner: Address = "11111111111111111111111111111111".parse()?;

let pda = find_program_address(&[b"vault", owner.as_bytes()], &program)?;
println!("{} (bump {})", pda.address(), pda.bump());
assert!(!is_on_curve(&pda.address()));
```

The function enforces Solana's limits. You can pass at most 15 seeds, because the bump takes the 16th slot, and each seed can be at most 32 bytes. It tries every bump from 255 down to 0. Errors come back as `crypto::PdaError`.

Two related functions:

- `create_program_address(seeds, program)` derives an address from seeds that already include the bump. It fails with `PdaError::OnCurve` if the result is on the curve.
- `create_address_with_seed(base, seed, owner)` derives a "seeded" address, as used by `CreateAccountWithSeed`. Seeded addresses may lie on the curve.

```rust
use solana_kit_core::crypto::create_address_with_seed;

let seeded = create_address_with_seed(&owner, b"stake:0", &program)?;
```

Many program crates also have helpers for their own PDAs. `solana_kit_associated_token_account`, for example, can derive token account addresses.

## Keypairs

`solana_kit_core::crypto::Keypair` is a deterministic Ed25519 signer. It erases its secret key from memory when dropped. It has no `Debug` implementation and no method that returns the secret.

### From a seed or imported bytes

```rust
use solana_kit_core::crypto::{Keypair, verify};

// A fixed 32-byte seed. Use this for tests and fixtures, never for real funds.
let keypair = Keypair::from_seed(&[7; 32]);
println!("{}", keypair.address());

let signature = keypair.sign(b"hello");
verify(&keypair.address(), b"hello", &signature)?;
```

`Keypair::from_bytes` imports the 64-byte format that `solana-keygen` writes: a 32-byte seed followed by the matching public key. It returns `crypto::KeypairImportError` if the length is wrong or the public half does not match the seed. Your input buffer is left unchanged; you are responsible for erasing it.

### Generate a new keypair

Core does not pick a random-number source. You pass one in through the `crypto::Entropy` trait. On native hosts, `solana_kit_native::keys::NativeEntropy` provides one backed by the `ring` crypto provider:

```rust
use solana_kit_core::crypto::{ExportableKeypair, Keypair};
use solana_kit_native::keys::NativeEntropy;

let mut entropy = NativeEntropy::default();
let exportable = ExportableKeypair::generate(&mut entropy).await?;

// Optional: keep a backup before giving up the export capability.
let backup = exportable.backup();
let restored = Keypair::from_bytes(backup.expose_secret())?;

// A normal signer without export.
let signer: Keypair = exportable.into_signer();
assert_eq!(restored.address(), signer.address());
```

`ExportableKeypair` is the only key type that can hand out its secret, and only through an explicit `backup()`. `into_signer()` gives up that capability for good: you cannot turn a plain `Keypair` back into an exportable one. `KeypairBackup` erases its bytes on drop, and its debug output is redacted.

To save a backup as a `solana-keygen` compatible JSON file on Unix, use `solana_kit_native::keys::write_keypair_new(path, &exportable)`. It refuses to overwrite an existing file and writes the file with mode `0600`. The call blocks, so run it on a blocking executor.

<Callout kind="note" title="Other hosts">

In the browser, `solana_kit_browser` has a `BrowserEntropy` type behind its `keys` feature. Any other host can implement `crypto::Entropy` itself. Wallet-held keys never become a `Keypair`. They are signers instead; see [Signers](../core/signers.md) and [Wallets](../runtimes/wallets.md).

</Callout>

## Next steps

<Cards>
<Card title="Fetch an account" href="fetch-an-account.md">Read the account at an address.</Card>
<Card title="Build a transaction" href="build-a-transaction.md">Use a keypair as the fee payer.</Card>
</Cards>
