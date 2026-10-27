# Stake clients

Deterministic Stake program data codecs, ordered instruction builders, metadata-preserving account decoding and pure instruction plans using Kit-owned values. The `no_std + alloc` production graph adds the existing small System client to core for account creation; it contains no host runtime or Solana SDK dependency.

`Delegation::activation_status_v2` calculates effective, activating and deactivating lamports from bounded, newest-first stake history. It uses integer V2 rate rules, including the rate-change epoch and missing-history behavior. Supply coherent history from the cluster whose program uses these rules. Activation amounts alone do not establish withdrawable funds: rent, lockups, authority and account balance require separate checks.

The private portable verifier's native tests compare the calculation with the exact published `solana-stake-interface` version 5.1.1. That reference package does not enter SDK crate, application or Wasm dependency graphs. Shared literal cases also run in the portable program contract suite.

`assess_withdrawal` checks proposed signer identities, lockup boundaries and the observed stake state. It returns separate reserve-preserving partial amounts and a full-close amount when inactive, because amounts in between may be invalid. Supply coherent account, clock and history observations, plus the rent reserve and rate epoch of the deployed program. The current program derives rent from the live Rent sysvar and actual allocation; historical programs may use different rules. Matching an identity does not prove a signature, and these limits do not validate a destination or guarantee later transaction execution.

This crate is exclusively for off-chain application clients. Use Pina or official Solana libraries for on-chain programs.

The exact pinned source, source discrepancies, and verification limits are recorded in the Stake/Loader receipt. Publication is disabled.
