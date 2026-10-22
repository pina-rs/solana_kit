# Ecosystem delivery tasks

This is a delivery checklist. Release notes describe completed behavior only.

## Ground

- [x] Read the figure-it-out principles and establish an auditable workflow.
- [x] Confirm whole-ecosystem scope, underscore crate names, public repository, and WASI from the beginning.
- [x] Inspect pinned current-main snapshots of Dart Solana Kit, Wasm Solana, and LeptosKit without changing their working files.
- [x] Inventory all 71 top-level reference packages and distinguish source inventory from verified behavior.
- [x] Identify browser, native, WASI, and interpreted guest capability boundaries.

## Sketch

- [x] Compare two complete crate/API designs against host compatibility, interface depth, dependency isolation, parity, and release feasibility.
- [x] Review the candidates independently, including local versus Send futures and guest entropy/timers.
- [x] Record the selected shape, rejected alternatives, and concrete usage sketches.

## Agree and frame

- [x] Prepare the implementation plan and package disposition ledger.
- [x] Record required host changes and which repository owns them.
- [x] Make the concrete plan, reuse decisions, and verification limits reviewable.

## Implement in verified units

- [ ] Establish a Rust workspace, pinned tools, CI, monochange ownership, and reproducible reference fixtures.
- [ ] Prove local futures and native Send futures against the proposed transport contract.
- [ ] Run the same deterministic primitive/crypto vectors in native, browser, WASI, and a plain wasmi guest.
- [ ] Implement codecs, instructions, account models, legacy/v0/v1 wire formats, and signing.
- [ ] Implement typed RPC and native/browser/WASI transports.
- [ ] Integrate a cancellable LeptosKit guest host transport, timers, and entropy.
- [ ] Implement subscriptions, lifetime-aware confirmation, resource estimation, and instruction plans.
- [ ] Deliver reproducible program generation and all 17 reference program clients.
- [ ] Deliver wallet protocols, platform adapters, and LeptosKit wallet UI equivalents.
- [ ] Deliver Anchor, Helius, Jupiter, Pyth, SNS, Surfpool, publishing CLI, documentation, and test support.
- [ ] Execute generated desktop, Android, and iOS guest journeys before claiming support.
- [ ] Complete registry readiness and release previews through monochange.

## Scrap and verify

- [ ] Revisit the chosen shape if multiple unrelated features need the same workaround.
- [ ] Require evidence for every completed parity record and every supported runtime.
- [ ] Review the complete changes and decision trail before each PR merges.
