# Codec framing structure

The existing Encoder and Decoder traits use one Reader or Writer. Reader frames
share allocation claims. Existing SizePrefix and Sentinel demand exact payload
consumption. Padded zeroes padding. These contracts remain intact.

Concrete wrappers extend this model. ByteSizePrefix selects a typed u64-valued
prefix and an explicit frame consumption policy. FixedSize bounds scratch encoding
and selects truncation plus an optional declared fixed decoder width. Reverse
reverses one declared width. Resized changes allocation size while preserving the
underlying cursor result. Offset supplies borrowed pre/post byte contexts and
checked repositioning, including backwards movement. Existing Sentinel gains an
explicit bounded Dart-compatible constructor. Padded retains its deliberate
zero-fill difference.

A second dynamic codec hierarchy with fixed/variable subclasses was rejected.
It duplicates existing traits and makes callers carry runtime type information.
The selected wrappers require explicit widths where the Rust Decoder trait has
no metadata. Width declarations are caller contracts, not inferred parity.

The work is exclusively for off-chain application clients.

## Local workflow

1. Ground the existing subsystem and original Dart implementations.
2. Sketch the wrapper structure and compare it with metadata subclasses.
3. Agree. No human checkpoint is needed for this authorized local work.
4. Implement and verify each wrapper through public operations.
5. Scrap the design if repeated workarounds reveal an ownership mismatch.

The feature workflow's blocking first step is the source and receipt audit.
Independent workstreams are not applicable because cursor and wrapper contracts
share one source owner. Shared mutable state is isolated in this sibling checkout
and its fresh target directory. One worker is the smallest safe decomposition.
Nested delegation and PR creation are excluded by the unit brief.
