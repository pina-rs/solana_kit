# Address-history pages and simulation

This off-chain application slice completes the two remaining history/submission
operations. It is not for on-chain programs; use Pina or official Solana program
libraries there. The historical Unit70 receipt remains unchanged.

`methods::address_history::TransactionsForAddress` supports omitted and explicit
configs, full or signatures pages, confirmed/finalized commitment, exact slot,
signature and signed time comparisons (including time.eq), execution/token account
filters, ascending/descending order, validated slot:position cursors, and the
100-full/1000-signature limits. Full requests select base58/base64/json/jsonParsed
and legacy/numbered 0/1 version negotiation. Mandatory transactionIndex and nullable
paginationToken cannot disappear. Full pages reuse the existing owned transaction
and metadata decoder; no raw standard response escapes to callers.

`methods::simulation::SimulateTransaction` accepts opaque bytes with explicit
base58/base64 transaction encoding and no inherited legacy packet-size assumption.
It serializes every pinned execution/account option, rejects conflicting
sigVerify/replaceRecentBlockhash before transport access, retains exact integers
and failures, and preserves requested account identities/cardinality. All account
representations use the existing rich WireAccount decoder, with base64 default,
jsonParsed fallback and explicit compressed bytes. No account allocation or
base58/base64 decoding algorithm is duplicated.

Required nullable fees, balances, token metadata, logs, loaded addresses, return
data, replacement hash and accounts reject absence. Versioned compute consumption
and loaded-account byte size distinguish absent/null/value because actual Agave
4.2 emits Option fields, wider than the static Kit alias. Simulation inner calls
reuse the existing compiled/parsed/fallback graph; only program-defined parsed
payloads retain bounded exact RawValue. Transaction metadata TokenAmount keeps its
exact Number annotations; account TokenAmount retains the separately audited
finite UI-number contract. Their distinct wire contracts are intentional.

The pinned Dart source is 0138bb956d2d81ccc0c56edc602401c0df2f2b46. Kit8.3.0
schema hashes and links remain in rpc-history-submission.md. Agave4.2.2
RpcSimulateTransactionResult at c9c6f3287e26f24e3476e13e751aebe710191e89 grounds
nullable compute/data-size fields. Account prerequisites are byte-identical to
reviewed74618819 except WireAccount and its owned conversion become pub(crate).
All146 scoped history IDs and100 account IDs remain explicit in their TSVs.
The history table marks130 implemented and16 pending:14 mechanical Dart
hashCode/toString contracts plus2 response equality contracts. Account counts
remain88 typed fixtures,4 partial Debug,4 pending equality and4 pending hash.

The shared Unit83 correction accepts historical missing metadata/version only for
getBlock/getTransaction. Address pages keep their stricter explicit-version
contract through VersionRequirement::RequiredWhenRequested. Simulation token
display numbers reuse the corrected scalar-only transaction metadata decoder;
actual objects resembling serde's private number representation reject.

Public literal fixtures execute both operations through the actual Client,
including field-removal/domain mutations, required nulls, maximum integers,
context/correlation/body bounds, escaped duplicate keys, allocation failures and
version/encoding refusal. Wasm target checks here are compilation only; no browser,
WASI, LeptosKit network, validator, installed app or mobile journey is claimed.
This branch does not include concurrent node/economics catalog modules or network
adapters. It does not establish whole Phase4 or registry release readiness.
