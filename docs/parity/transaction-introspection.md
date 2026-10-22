# Transaction introspection

The client `introspection` module resolves ordered accounts and instructions from
existing Kit-owned RPC transaction results. It also inspects validated core
messages directly. Legacy, v0 and v1 share core's normalized instruction model.
The module adds no JSON DTO, wire parser, dependencies or host requirements.

`decode_rpc_transaction` validates header partitions, signature counts, lookup
counts and account references before exposing a trace. A binary response passes
through `solana_kit_core::transaction::Transaction::decode` and retains its exact message
and signature bytes. Structured JSON retains its original typed message and has
no reconstructed binary transaction. Every received lifetime stays unresolved.
A hash alone never establishes expiry or durable nonce state.

Top-level raw JSON invoked programs must use a non-payer static account index,
matching core wire validation for every supported version. Inner CPI program
indices resolve against the complete account table and may use loaded accounts.

`decode_rpc_wire` decodes binary RPC transactions independently of execution and
lookup metadata. A v0 transaction without loaded addresses still retains its
exact signed message and signature slots. `decode_rpc_transaction` requires
complete lookup resolutions before exposing its account table or instructions.
Its `MissingLoadedAddresses` error does not invalidate the wire-only result.

`get_account_metas` and `get_instructions` inspect a core message. The decoded RPC
view provides the same operations, `get_inner_instructions`, `walk_instructions`,
signers, signature slots and original execution metadata. Lookup writable accounts
precede lookup readonly accounts. Duplicate positional accounts retain their order
and roles. Missing or null lookup resolutions fail when the message needs them.
Inner groups may arrive out of order. Walking places each outer instruction before
its ordered inner calls and rejects duplicate or orphan groups, including empty
groups. Stack heights retain missing, null and numeric values.

Server-parsed instructions remain borrowed opaque JSON. Partial parsed instructions
become ordinary Kit instructions with exact bytes and account roles. Address-based
instructions reject unknown accounts and ambiguous privileges. Parsed payloads
must be JSON objects. Their program-specific fields remain uninterpreted, including
full-width numeric spellings. The original typed result retains lookup source,
configuration and every metadata presence annotation.

Instruction traces describe attempted execution. `metadata()` returns `None` when
execution metadata is unavailable. A caller checks `meta.err` before treating a
trace as successful execution. An empty returned inner list does not establish
that recording was enabled. The original `meta.inner_instructions` distinguishes
missing, null and an explicitly empty recording. Structural wire validation does
not authenticate signatures or establish validator acceptance.

The [declaration receipt](transaction-introspection.tsv) contains all 42 canonical
IDs, with concrete APIs, tests and limits. All remain partial. The [portable runtime receipt](../verification/introspection-runtime.md)
executes all 30 public contracts through three groups on native Rust, Chrome,
both WASI previews and the actual pinned LeptosKit host. Rust enum
construction, structural equality, hashing and `Debug` replace Dart constructors,
equality, hashes and display methods. Exact Dart hash values and display strings
are outside the Rust contract.

The [source inventory](transaction-introspection-sources.tsv) records the complete
package from Dart commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46`.
The original package tests execute from an immutable archive with an explicit
package configuration. `tests/introspection/oracle.dart` independently produces
wire bytes and trace literals for all three versions. Rust compares those bytes
against independent literal envelopes. The retained MIT notice is
`crates/solana_kit_client/tests/introspection/DART-LICENSE`.

The API serves off-chain applications. Solana on-chain programs should use Pina
or official Solana program libraries. Portable fixture execution does not establish network, validator or installed
device behavior.
