# Notices and provenance

The public reuse boundary is the complete MIT Dart package
`solana_kit_attestation_service` at revision
`0138bb956d2d81ccc0c56edc602401c0df2f2b46` from the pinned Dart reference.
`DART-LICENSE` retains its complete MIT copyright and permission notice.
Original Rust source is Unlicense. Both licenses are declared in Cargo metadata.

The primary Solana Attestation Service revision
`5b64cf09843d62ca800f7d73f8438ad3505de70e` remains an independent private schema
and wire oracle. Its exact 16,475-byte IDL SHA256 is
`924728c7b8327b5770ef63c34c804430d0a1953adcf96873bcee1f4f4ccb4a75`.
Its archive SHA256 is
`46270a1f71127bfceb547acd262ca36a7513ce66547d4b80fbab765672e7e3f4`.
The reference ledger records an unresolved distribution license and the checkout
has no root LICENSE, LICENSE.md or COPYING. Manifest license strings do not grant
permission to distribute its implementation or IDL. Neither is copied into
shipping files or required for public regeneration from the MIT-derived model.

The Rust API makes account/instruction discriminators immutable codec prefixes.
It also validates instruction identity and exact privileges, whereas the Dart
parser inspects data only. Account decoders reject trailing bytes and invalid
headers. Structural Rust equality/hash/display replace Dart identity-sensitive
byte-list operations and presentation; exact Dart hashes or formatting are not
promised. Typed schema maps reject duplicate names rather than accepting their
ambiguous map projection. These differences stay explicit in the source parity
receipt. Error display uses exact enum labels because the Dart enum defines no
message strings; this is not a claim of upstream error-message equivalence.

The event payload codec retains its explicit discriminator field. The source's
internal CPI envelope has an additional eight-byte event prefix and is a distinct
representation. The public `EmitEvent` operation preserves the Dart first-byte
228 contract, which the pinned primary entrypoint also dispatches by first byte.
