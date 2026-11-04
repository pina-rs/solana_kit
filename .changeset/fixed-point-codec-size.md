---
"solana_kit_core": patch
---

# Declare fixed-point codec widths for composition

Report the validated fixed byte width through both codec traits so fixed-point
encoder and decoder halves compose directly through `Paired::try_new`. Mismatched
widths remain rejected before reading or writing.
