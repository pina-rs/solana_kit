---
"solana_kit_leptoskit": minor
---

# Borrow an application-owned interpreter for HTTP

Add HttpDriver and opaque HttpCompletion so installed applications can keep their
WasmHost while native HTTP waits independently. HttpHost delegates to the same
driver. Bind completions to the actual interpreter and Bridge, preserve capacity
until join, and provide explicit admission-closing abort-and-join shutdown.
