# Portable Attestation, Squads and recurring Subscriptions contracts

This private fixture imports 83 original public-client behavior owners: 22 from
Attestation Service, 48 from Squads and 13 from recurring Subscriptions. The
remaining-contracts generator discovers the original functions and all 36 Squads
macro-emitted instruction cases. Assertions remain with their public owners.

The native executable and both WASI previews emit the complete named case stream.
The browser calls the same registry. The interpreted guest commits its report only
after every body returns through production LeptosKit WasmHost at revision
386e1cd81737fd6ddffdd763a1a5d2c96d8c0d6d.
Deliberate corruption and one-fuel exhaustion must reject before a report commit.
The compiled guest must pass the closed three-import SDK gate.

The intentional assertion has one shared marker. A private guest panic hook
forwards only that payload through the existing typed property import, where a
deliberately invalid boolean produces an observable host trap containing the
marker. The host requires that marked Trap and zero commits. An unrelated guest
panic must fail this confirmation rather than count as corruption evidence.

The fixture enables the three clients' PDA features and its matching local pda
configuration. The clients' default dependency graphs are checked independently.
All fixture packages remain private and excluded from placeholder publication.
This contract runner does not execute deployed program transitions or installed
desktop, Android or iOS application journeys; those remain separate required
parts of the full implementation plan.
