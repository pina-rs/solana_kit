# Anchor

`solana_kit_anchor` talks to any Anchor program from its IDL, with no code generation. You parse an Anchor 0.30-format IDL at runtime, and a `Coder` then encodes and decodes instructions, accounts, and events as dynamic Borsh values. It resolves both standard Anchor errors and the program's own. The crate is `no_std + alloc`. It starts no executor and makes no network calls.

## Parse the IDL and encode an instruction

`Idl::parse(text, limits)` reads the JSON. `Idl::from_value` does the same for an already parsed `serde_json::Value`. The coder borrows the IDL. The output is plain instruction data: you add the accounts yourself, as `AccountMeta` values, to build a normal Kit `Instruction`.

```rust
use solana_kit_anchor::{Coder, Idl, Limits, Value, Values};
use solana_kit_core::{AccountMeta, AccountRole, Address, Instruction};

const IDL: &str = r#"{
  "address": "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS",
  "instructions": [{
    "name": "deposit",
    "discriminator": [242, 35, 198, 137, 82, 225, 242, 182],
    "accounts": [],
    "args": [{"name": "amount", "type": "u64"}]
  }]
}"#;

fn deposit(user: Address) -> Result<Instruction, Box<dyn std::error::Error>> {
    let limits = Limits::new(64 * 1024, 4096, 32);
    let idl = Idl::parse(IDL, limits)?;
    let coder = Coder::new(&idl, limits);

    let mut args = Values::new();
    args.insert("amount".into(), Value::Unsigned(1_000));
    let data = coder.encode_instruction("deposit", &args)?;

    Ok(Instruction {
        program_address: idl.address.parse()?,
        accounts: vec![AccountMeta { address: user, role: AccountRole::WritableSigner }],
        data,
    })
}
```

The IDL's `accounts` entries are not used to build account metas. The pinned source has no account resolver, PDA resolver, provider, or program builder, and this crate does not add them. For signing and sending, use the regular APIs in [Signers](../core/signers.md) and [Sign and send](../getting-started/sign-and-send.md).

Anchor's `Limits::new(max_bytes, max_items, max_depth)` has a third budget for nesting depth. That makes it a different type from the two-field core `Limits`. Nesting is also hard-capped at 128 levels to protect the stack.

## Values

`Value` represents any Anchor value without losing precision. `Values` is a `BTreeMap<String, Value>` of named fields. Encoding always follows the IDL's field order, not the map order.

| Variant                      | Anchor types                                                         |
| ---------------------------- | -------------------------------------------------------------------- |
| `Unsigned(u128)`             | `u8` to `u128`, checked against the declared width                   |
| `Signed(i128)`               | `i8` to `i128`, checked against the declared width                   |
| `F32`, `F64`                 | `f32`, `f64`, with exact bit patterns                                |
| `Bool`                       | `bool`                                                               |
| `String`, `Bytes`            | `string`, `bytes` (u32 length prefix)                                |
| `Address`                    | `pubkey`                                                             |
| `Sequence(Vec<Value>)`       | `vec` (u32 count) and fixed arrays (no prefix)                       |
| `Option(Option<Box<Value>>)` | `option` (one-byte tag). Nested options keep every tag.              |
| `Struct(Values)`             | defined structs                                                      |
| `Enum { variant, fields }`   | defined enums. Tuple fields are named `field0`, `field1`, and so on. |

A wrong type, a missing field, or an out-of-range integer is an error. Encoding errors say which field or account failed.

## Decode instructions and accounts

`decode_instruction(name, bytes)` and `decode_account(name, bytes)` check the discriminator, decode the fields, and report how many bytes they consumed. Account data often has allocation padding after the value, so trailing bytes are allowed and `consumed` tells you where the value ended.

```rust
use solana_kit_anchor::{Coder, Error, Value};

fn amount_of(coder: &Coder<'_>, data: &[u8]) -> Result<Option<u128>, Error> {
    let decoded = coder.decode_instruction("deposit", data)?;
    Ok(match decoded.value.get("amount") {
        Some(Value::Unsigned(amount)) => Some(*amount),
        _ => None,
    })
}
```

`DecodedAccount` holds the matched `discriminator`, the `data` value, and `consumed`. If you need a strict full read of one type, use `coder.codec_for(&ty)` to get a `ValueCodec`. Its `decode` rejects trailing bytes, and `decode_prefix` does not.

The coder uses the IDL's discriminators exactly as given, including empty, zero, or non-eight-byte ones. The helpers `instruction_discriminator`, `account_discriminator`, `event_discriminator`, and `anchor_sighash` compute the standard eight-byte SHA-256 prefixes separately. `has_discriminator` checks a prefix.

## Events

`decode_event_logs(&logs)` reads complete transaction logs and returns a `DecodedEvent { name, data }` for each event. It only accepts `Program data:` lines emitted while the IDL's program is the active invocation. Payloads from other programs called by CPI, and fake markers inside `Program log:` messages, are ignored. Unknown event discriminators are skipped. Malformed base64 or a malformed payload for a known event is an error.

```rust
use solana_kit_anchor::{Coder, Error};

fn events(coder: &Coder<'_>, logs: &[String]) -> Result<(), Error> {
    for event in coder.decode_event_logs(logs)? {
        println!("{}: {:?}", event.name, event.data);
    }
    Ok(())
}
```

<Callout kind="important" title="Check the transaction succeeded first">

Decoding events does not tell you whether the transaction succeeded. Check the transaction status before you act on an event. Failed nested invocations restore the caller's context, as in the pinned source.

</Callout>

## Errors

`program_error(code, idl)` resolves any signed 64-bit error code. It checks the IDL's `errors` first, then the 77-entry `STANDARD_ERRORS` table, and otherwise returns a `ProgramError` named `Unknown` with the code kept. An IDL error with an empty message keeps the empty message. One with no message gets the default `Custom program error: <code>`.

```rust
use solana_kit_anchor::program_error;

let error = program_error(2000, None);
assert_eq!(error.name, "ConstraintMut");
println!("{error}"); // AnchorProgramError(2000, ConstraintMut): A mut constraint was violated
```

The crate's own failures are `solana_kit_anchor::Error`. It implements `Display` and the standard error source chain.

## Limits and status

Unsupported field types are parsed lazily. An IDL that uses generic instantiations, aliases, or wide option prefixes still parses and can be inspected, but using one of those fields returns an error.

The Anchor contract bodies (43 portable bodies, 169 assertion sites) run on native Rust, Chrome, both WASI previews, and the LeptosKit interpreter. All 62 original Dart test bodies also run against the pinned Dart source. Not claimed:

- Full source or ecosystem parity.
- Account resolution.
- Validator execution.

The exact differences from the Dart source, such as rejecting fractional array lengths, are in [`docs/parity/anchor.md`](https://github.com/pina-rs/solana_kit/tree/main/docs/parity/anchor.md) and [`docs/verification/anchor-runtime.md`](https://github.com/pina-rs/solana_kit/tree/main/docs/verification/anchor-runtime.md).
