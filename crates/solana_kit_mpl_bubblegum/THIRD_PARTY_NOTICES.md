# Source notices

Original Rust code is Unlicense. Public models, helpers and fixtures derive from
the MIT Dart Solana Kit at commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. Its notice is in `DART-LICENSE`.

The Metaplex Bubblegum revision
`79e1a1954bddb7fe5dcd52a5570a66a9f7d465e4` is an independent private wire oracle.
Its repository uses the Metaplex NFT Open Source License. This package contains
no copied Metaplex implementation or IDL. Independently observed field ordering,
discriminators and account privileges correct explicit MIT Dart discrepancies.
The complete external IDL remains private and is never a public generator input.

The optional `sha3` dependency supplies Keccak-256 under its own MIT/Apache-2.0
terms. It supplies no host entropy or network capability.
