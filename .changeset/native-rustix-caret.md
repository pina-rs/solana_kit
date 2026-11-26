---
solana_kit_native: patch
---

# Relax `rustix` version constraint for downstream LeptosKit hosts

Relax the Linux/Android `rustix` dependency in `solana_kit_native` from exact `=1.1.5` to caret `1.1.5` so downstream workspaces that also pull `rustix ^1.1.6` (such as `xattr 1.7` in LeptosKit native hosts) resolve cleanly without patching `solana_kit_native`.
