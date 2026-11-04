---
solana_kit_native: patch
---

# Accept real MWA properties at sequence one

Real wallets (including the pinned Solana Mobile reference wallet) consume
sequence 1 for the encrypted HELLO_RSP session properties. The native local and
remote clients now start their receive counter at 1, so the properties frame
authenticates and the next wallet reply is sequence 2. Previously the native
peer expected 0, so a genuine wallet's first encrypted frame failed ordering and
the pending authorization closed before any request was served.

The native local and reflector peers now encode the same real sequence, and the
public encrypted-session handshake test uses the standard properties/reply
counters. No protocol framing, crypto, deadline or configuration contract
changes.
