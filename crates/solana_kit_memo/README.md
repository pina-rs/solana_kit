# Solana Kit Memo

A handwritten, `no_std + alloc` off-chain Memo client using Kit-owned types.
Build raw UTF-8 invocations with ordered signers, parse complete invocations,
and extract memos across the v1, v3 and v4 program identities.
On-chain programs should use Pina or official Solana program libraries.

The implementation follows Dart source `0138bb956d2d81ccc0c56edc602401c0df2f2b46`
and Memo IDL `4a487432a7a57cc3739db43c6e7ac12fc96ff338`. It is not generated.
The finite generator cannot yet model untagged remainder layouts and remaining accounts.

UTF-8 is strict. Nulls remain present. Decoding removes a leading BOM, as the
actual Dart codec does. Valid UTF-8 suffixes are payload, rather than trailing
bytes. Rust strings exclude unpaired surrogates by construction. Builders accept
three supported identities, while the Dart low-level builder accepts arbitrary
program addresses. Full parsing additionally verifies program identity and signer
roles. Extraction ignores account roles, matching the source helper.

`Limits` bounds raw input bytes and decoded text allocation. Parsing shares its
item budget between text bytes and signer positions. Extraction bounds each
payload and the result count. No validator, transaction-size or compute limit is
invented by this client. Program acceptance and transaction signing remain separate.
