# Solana Kit SNS

Offline Solana Name Service client primitives for deriving domain, record and
reverse addresses and parsing registry and record data. These deterministic
operations work without RPC, a runtime, or host entropy. Applications provide
account data and select their own network transport and validation policy.

This is a client library. For on-chain programs use Pina or Solana program
libraries. A record's validation identifiers are bytes, not proof of ownership
or association.
