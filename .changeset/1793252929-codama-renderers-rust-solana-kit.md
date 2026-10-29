---
codama_renderers_rust_solana_kit: minor
---

# Preserve numeric widths and byte orders

Add i32, u128, i128 and f32, retain scalar, prefix and u32 instruction discriminator byte order, and use existing Kit codecs for checked big-endian array, size and option prefixes. Float-bearing clients provide PartialEq and Display without impossible Eq or Hash implementations. Normal registry publishing remains disabled.
