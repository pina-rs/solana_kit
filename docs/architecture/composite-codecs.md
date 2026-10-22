# Typed codec composition

Application clients compose ordinary Rust values with existing Encoder, Decoder,
Reader and Writer contracts. This supports portable off-chain code. On-chain
programs should use Pina or official Solana program libraries.

Two sketches were compared locally using the existing public contracts.

| Shape                                                  | Ownership and allocation                                                    | Decision |
| ------------------------------------------------------ | --------------------------------------------------------------------------- | -------- |
| Generic wrappers over concrete codecs and callbacks    | Reader owns the shared claim budget; structs and enums retain field types   | Selected |
| Dynamic codec registry producing generic record values | Registry and records allocate; caller recovers types through dynamic checks | Rejected |

The selected public shape pairs independent encoder/decoder implementations,
provides fallible value mappings, and uses typed Either and dependent pairs for
heterogeneous data. Sequence count prefixes reuse Sequence with a fixed count.
No runtime dependencies or existing trait changes are needed. Public read/write
failures retain existing partial cursor semantics.

The scope is a verified capability slice. Arbitrary rewind/relative offsets,
metadata introspection, dynamic records, general map/set policies, permissive
resize and complete reference callback contexts remain separate work.

The blocking step is grounding cursor and original Dart behavior. One writer
owns new composition modules and tests. Shared inventory, runtime registration,
numeric/text modules and dependency gates are excluded. Verification runs against
actual local Dart and a clean external Rust consumer before integration.
