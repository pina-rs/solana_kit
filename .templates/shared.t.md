<!-- {@offchainOnly:"subject"} -->
{{ subject }} is only for off-chain application code. Do not use it in Solana
on-chain programs; use Pina or official Solana program libraries instead.
`no_std + alloc` enables portable clients and does not imply on-chain support.
<!-- {/offchainOnly} -->

<!-- {@offchainCrateOnly} -->
This crate is exclusively for off-chain application clients. Use Pina or official Solana libraries for on-chain programs.
<!-- {/offchainCrateOnly} -->

<!-- {@stakeLoaderReceipt} -->
The exact pinned source, source discrepancies, and verification limits are recorded in the Stake/Loader receipt. Publication is disabled.
<!-- {/stakeLoaderReceipt} -->

<!-- {@offchainSdkNotice} -->
This SDK is exclusively for off-chain application clients. On-chain programs
should use Pina or official Solana program libraries.
<!-- {/offchainSdkNotice} -->

<!-- {@syntheticTlsFixturesDocs} -->
These local-only certificates and the deliberately public disposable test key
come from this repository's WASI socket proof. They identify only `localhost`
and `kit-local-test-ca`, have no external account or production use, and remain
under this project's Unlicense. The leaf has only a DNS localhost subjectAltName
so a connection by 127.0.0.1 proves destination-name rejection. Certificates are
valid from 2026-10-22 to 2036-10-19. No machine trust store or personal key is read.
<!-- {/syntheticTlsFixturesDocs} -->
