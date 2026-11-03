# Fetch an account

Account reads keep three steps apart: the transport fetches bytes, the result tells you whether the account exists, and a codec you pick decodes the data. This page fetches raw account bytes with `Client::fetch_encoded_account`, handles accounts that do not exist, and decodes a sysvar and a System nonce account.

The examples assume the `client` from the [Quick start](quick-start.md). Account data can be larger than the default 8192-byte response budget, so build the client with a larger `Limits`:

```rust
use solana_kit_client::{Client, Limits};
use solana_kit_native::{HttpConfig, NativeHttp};

let transport = NativeHttp::new("https://api.devnet.solana.com", HttpConfig::default())?;
let client = Client::new(transport, Limits { bytes: 64 * 1024, depth: 32 });
```

## Fetch one account

```rust
use solana_kit_client::methods::{Contextual, ReadConfig};
use solana_kit_core::{Address, accounts::MaybeAccount};

let address: Address = "SysvarRent111111111111111111111111111111111".parse()?;

let Contextual { context, value } = client
    .fetch_encoded_account(address, ReadConfig::default())
    .await?;

match value {
    MaybeAccount::Present(account) => {
        println!(
            "slot {}: owner {}, {} lamports, {} bytes",
            context.slot,
            account.owner,
            account.lamports,
            account.data.len()
        );
    }
    MaybeAccount::Missing(address) => println!("no account at {address}"),
}
```

`fetch_encoded_account` asks for the whole account in base64 and returns `Contextual<MaybeEncodedAccount>`. `MaybeEncodedAccount` is short for `MaybeAccount<Vec<u8>>`:

- `MaybeAccount::Present(Account<Vec<u8>>)` holds `address`, `owner`, `lamports`, `executable`, `space`, `rent_epoch` and the raw `data`.
- `MaybeAccount::Missing(Address)` means the node reported no account. A missing account is a normal result, not an error.

To treat a missing account as an error, call `.require()`. It returns `Result<Account<T>, AccountError>`, and the error still holds the missing address.

`ReadConfig` sets `commitment` and `min_context_slot`. `ReadConfig::default()` sends neither, so the node uses its defaults.

## Fetch several accounts

`fetch_encoded_accounts` returns results in the order you asked, keeping duplicates and missing entries:

```rust
use solana_kit_client::methods::{Commitment, ReadConfig};

let config = ReadConfig {
    commitment: Some(Commitment::Confirmed),
    ..ReadConfig::default()
};
let accounts = client
    .fetch_encoded_accounts(vec![address, Address::from_bytes([9; 32])], config)
    .await?;

for account in &accounts.value {
    println!("{} exists: {}", account.address(), account.exists());
}
```

If the node returns a different number of entries than you requested, the call fails.

## Decode account data

An encoded account decodes with any `solana_kit_core::codecs::Decoder`. `Account::decode(decoder, expected_owner, limits)` checks three things before it decodes: the owner matches, the data length matches the reported `space`, and no bytes are left over. The result is an `Account<T>` that keeps all the metadata.

The `limits` argument is `solana_kit_core::codecs::Limits`, not the client's `Limits`. It caps the bytes and decoded elements the decoder may consume. The examples import it as `DecodeLimits` to keep the two apart.

### A sysvar

```rust
use solana_kit_client::methods::ReadConfig;
use solana_kit_core::{
    codecs::Limits as DecodeLimits,
    sysvars::{CLOCK_ADDRESS, Clock, OWNER_ADDRESS, SysvarCodec},
};

let clock = client
    .fetch_encoded_account(CLOCK_ADDRESS, ReadConfig::default())
    .await?
    .value
    .require()?
    .decode(&SysvarCodec::<Clock>::NEW, OWNER_ADDRESS, DecodeLimits::new(40, 0))?;

println!("epoch {} at slot {}", clock.data.epoch, clock.data.slot);
```

`solana_kit_core::sysvars` has fixed layouts and addresses for the common sysvars. Variable-length sysvars such as `SlotHashes` need an element budget in their limits.

### A System nonce account

Program crates provide the decoders for their accounts. `solana_kit_system::decode_nonce` checks that the System Program owns the account and that it is exactly 80 bytes, then decodes it:

```rust
use solana_kit_core::{accounts::MaybeAccount, codecs::Limits as DecodeLimits};
use solana_kit_system::decode_nonce;

let maybe = client
    .fetch_encoded_account(nonce_address, ReadConfig::default())
    .await?
    .value;

if let MaybeAccount::Present(account) = maybe {
    let nonce = decode_nonce(account, DecodeLimits::new(80, 0))?;
    if let Some(data) = nonce.data.durable_data() {
        println!("authority {}, nonce {}", data.authority, data.blockhash);
    }
}
```

`durable_data()` returns the fields only for an initialized nonce of the `Current` version. Legacy and uninitialized nonce accounts stay distinct, so you never mistake their bytes for a usable nonce.

## Parsed JSON reads

`fetch_json_parsed_account` and `fetch_json_parsed_accounts` ask the node for `jsonParsed` encoding. They return `MaybeAccount<AccountData>`. `AccountData` keeps the program's parsed JSON exactly as received, or the binary fallback if the node could not parse the account. Use encoded reads with a codec when you need byte-exact decoding.

For data slices, other encodings, program account scans and token reads, see `solana_kit_client::methods::accounts` and `methods::tokens`, described in [Accounts and sysvars](../core/accounts.md) and [RPC](../core/rpc.md).

## Next steps

<Cards>
<Card title="Build a transaction" href="build-a-transaction.md">Turn instructions into a signed-ready message.</Card>
<Card title="Codecs" href="../core/codecs.md">Write decoders for your own account layouts.</Card>
</Cards>
