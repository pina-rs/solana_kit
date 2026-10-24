# Core implementation package verification

This local receipt covers the implementation archive and offline registry
consumers. It does not publish an implementation release. Normal publishing
remains disabled.

The reviewed `solana_kit_core 0.1.0` archive has SHA256
`307f2b70f2d35b23c36094faed5fab6530bb3e675b5758701873fd0436dac10d`.
Cargo packages the actual dirty implementation source with locked, offline,
all-feature verification. The archive contains 85 regular GNU tar members.
Every authored shipping member matches its reviewed source bytes. Cargo's
normalized manifest, generated lockfile and VCS record remain explicit inputs.

The [maintained archive gate](../../xtask/PACKAGE_REVIEW.md) checks actual raw GNU
and USTAR headers, checksums, membership, paths, types and source bytes. All 80
supported members meet both 90-point floors with width 100, generated ignoring
disabled and no ignore pruning. Five unsupported metadata/license members are
enumerated without invented scores. Aggregate scores are 98.397 readability and
99.431 complexity. Existing licenses and third-party notices are retained; the
normalized package has no personal author metadata.

An omitted actual changelog score rejects the retained-report checker. A real
three-line changelog scores 86.554 readability and rejects fresh archive review.
Raw-header tests cover GNU/USTAR, duplicate records, invalid checksums, truncated
end markers, links, extensions and escaped paths. Fresh review always invokes the
analyzer; retained-report checking cannot replace fresh source analysis.

The exact archive is installed into an ordinary Cargo local registry alongside
31 checksum-verified dependencies selected by its packaged lockfile. Cached
archives and exact sparse-index records supply the fixture offline. No Git
registry metadata, external registry write or credentials are involved.

Consumers depend only on the exact package name and version. Both actual Rust
1.99.1 and Rust 1.89.0 compile and execute eight public behavior tests with zero
failed, ignored or filtered cases. The bodies cover address/hash bytes, signing
rejection, codec size and pairing, key import, exact decimal arithmetic and signed
transaction wire bytes. Original checkout reads are denied to the actual Cargo
and compiler children. The resolved registry PackageID, concrete compiler argv,
selected artifacts and all 85 unpacked member digests are retained per producer.

The borrowed-codec regression first fails with `Err(InvalidValue)` instead of
`Ok([7])`. The reference implementations now forward `encode`, `decode` and
`decode_prefix` to their underlying codec. The same maintained callback and error
policy fixture passes natively and as the eighth packaged-consumer body. Each
native producer also passes 171 core tests, 18 conformance tests, six doctests and
the complete canonical/corrupt 321/320 reports. The earlier seven-test archive
proof remains historical evidence for its original bytes.

Separate fresh local registry namespaces reject a missing core archive and a
corrupted archive checksum for their intended causes. The checksum control
changes a gzip timestamp byte, preserving decompression. Negative compiler output
uses a target separate from the positive consumers. Selected core libraries and
executables are preserved before another producer reuses the shared target.

The maintained tooling passes 31 executed tests and Clippy. One pre-existing
style-CI test remains ignored because it requires the separately pinned public
Monostyle 0.3.26 command. Actual package quality uses a verified private
Monostyle successor; this is not a new official analyzer release or public CI pin.

Complete repository inventory and aggregate scores are recorded separately. Its
22 pre-existing per-file failures require the independent baseline quality layer
before public delivery; this receipt does not turn them into a passing gate.
Earlier unchanged core runtime evidence carries for its exact source bytes.
MSRV Wasm, portable mutants, default standalone Wasmi and installed application
journeys remain separate pending boundaries.

Solana Kit supports off-chain application clients. On-chain programs should use
Pina or official Solana program libraries.
