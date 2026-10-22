# Solana Kit Compute Budget

Portable, exclusively off-chain client code for application transaction construction.
Do not use this crate in on-chain programs. Use Pina or official Solana program
libraries for those programs. `no_std + alloc` supports portable applications.

All five pinned instruction layouts have strict bounded codecs and owned Kit
instruction builders/parsers. `RequestUnits` is a deprecated historical layout.
Modern validators reject that instruction. Building its bytes does not imply node
acceptance. Heap alignment and network limits remain validator policy.

The native private generator consumes the immutable Compute Budget IDL recorded in
`docs/parity/program-inputs.tsv`. Source revision and hash are recorded in the
verification receipt. Authored code is Unlicense. The upstream Apache-2.0 notice
is included as COMPUTE-BUDGET-LICENSE.
