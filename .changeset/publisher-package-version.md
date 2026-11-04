---
solana_kit_dapp_publisher: patch
solana_kit_dapp_publisher_cli: patch
---

# Report the publisher CLI's own package version

Make `--version` and the help heading use the executable's Cargo package version
instead of the publisher library's constant, and make the retained library
constant follow its own package metadata. The two packages can be released
independently, so the binary must not inherit a library version.
