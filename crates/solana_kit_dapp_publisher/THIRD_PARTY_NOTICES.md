# Source provenance

Attestation field ordering, publication signer-set checks, canonical outer
program policy, collection checks and prepared-transaction contracts derive
from the MIT-licensed Dart `solana_kit_dapp_publisher_cli` package at immutable
commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46` in
<https://github.com/openbudgetfun/solana_kit>.

The original MIT notice is retained in `MIT-DART.txt`. New original Rust code
uses the Unlicense. No upstream networking, secret, CLI, handwritten cryptography
or platform implementation is bundled. Test fixtures independently express the
documented JSON/Borsh/Solana wire formats; no Dart test bodies are copied.

Prepared-message parsing, Ed25519, Base64 and Token Metadata/PDA operations reuse
existing Kit crates and their retained third-party notices. Their upstream
dependencies do not grant any portal or publishing authorization.
