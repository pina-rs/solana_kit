# Stake activation sources

Integer activation and cooldown behavior is adapted from [Solana Stake Interface](https://github.com/solana-program/stake/tree/b37e4f76d4fbd08ad28c291f77620541489089c7/interface), licensed under Apache-2.0. The license is retained in `UPSTREAM-LICENSE`.

The implementation changes the upstream types to Kit-owned values, validates history ordering and size, and walks borrowed history without allocation. It retains the integer saturation, minimum progress, rate transition and history fallback rules. Native differential tests use the separately published interface version 5.1.1.
