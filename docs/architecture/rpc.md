# Typed RPC design experiment

This is a private, executable off-chain client experiment. No public client API
is available from this work. Solana Kit must not be used for on-chain programs;
use Pina or the official Solana program libraries there.

## Grounding and worklist

- Ground: done. Read the pinned Dart catalog, request builders, convenience layer,
  response models, boundary experiments, and committed architecture.
- Sketch: compare concrete GAT exchange futures with a local erased future.
- Agree: use executable evidence; no human approval checkpoint is required.
- Implement: typed balance, blockhash and base64 account paths only.
- Scrap: discard the erased boundary if it prevents native Send propagation.

The future public caller shape is `client.call(Balance::new(address, config))`.
A single client owns protocol framing, response correlation and typed decoding.
Adapters own endpoints, authentication, streaming body bounds and cancellation.
A request carries its response type; callers never choose an arbitrary return
schema for a catalog method. Request IDs belong to one client owner, not adapters.
The pilot accepts an explicit numeric ID to isolate protocol experiments; that
is not the planned public request-ID ownership contract.

## Candidate contracts

A: `Transport::Exchange<'a>: Future<Output = Result<HttpReply, TransportError>>`;
`Client<T>::call<R: Method>(&self, id, request) -> R::Response` is an ordinary
async method. The native generic caller requires `T: Send + Sync`, each associated
exchange future Send, and the method and response Send. Local adapters retain Rc
through Pending without a universal Send bound or runtime dependency.

B: `ErasedTransport::exchange(...) -> Pin<Box<dyn Future<...> + '_>>` reduces
implementation syntax, but erases Send even for native adapters. A separate Send
trait or duplicate client path would be required. Execute this candidate locally
and require a compile-negative native spawn control before choosing A.

Neither candidate starts a runtime or detaches work. Both share one protocol
codec, so the comparison changes the async boundary rather than JSON semantics.
