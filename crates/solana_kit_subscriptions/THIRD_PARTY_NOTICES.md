# Pinned source and deliberate client contracts

The full unchanged primary Codama IDL is retained at
`tools/codama-renderer/fixtures/subscriptions-original.json`, SHA256
`461e1c41da256906644d3e20ab185e97de9d9c141661e1521bf5ef42e255ac40`, from
Subscriptions revision `5a347ffaa969036061d274d3c91e0277962e2b51`.
Its MIT distribution license is `UPSTREAM-LICENSE`. Pinned Dart revision
`0138bb956d2d81ccc0c56edc602401c0df2f2b46` contributes the MIT `DART-LICENSE`.
Original Rust implementation code is Unlicense.

The common typed generator consumes every original account field, instruction,
named type, explicit contiguous enum discriminant, event header/payload, error,
PDA recipe and role default. Events lower their complete literal header to the
existing typed affix codec. Account records and event payloads use the same
structure renderer as existing clients. The separately validated default graph
becomes real resolver code; metadata is not silently ignored.

Account fetch composition uses core-owned snapshots and retains owner, allocation,
address, lamports, executable flag and rent epoch. EventAuthority is an empty
codec/PDA identity in the exact IDL, not a sixth allocated discriminated account.
Its snapshot helper therefore takes an explicit expected owner instead of assuming
that a signing PDA has allocated state owned by the Subscriptions program.
Raw data codecs preserve source header/version values; typed account helpers
check the concrete first-byte discriminator. Strict full-account decoding rejects
trailing bytes. Explicit `*_for_revoke` helpers accept the frozen complete V1
prefix (187/211/155 bytes) and preserve future appended bytes. They return owned
values rather than unsafe references into potentially unaligned bytes.

The primary transfer-validation helper advances whole billing periods and caps
finite expiry at the last period beginning strictly before expiry. Equality at
expiry remains billable; later timestamps reject. The off-chain calculation
returns a proposed owned snapshot only on success, preserving the caller's state
on error. Signed/unsigned conversions and timestamp differences reject overflow
explicitly instead of saturating or casting. Plan-end arithmetic is also checked.
These helpers predict snapshot policy; they neither authenticate bank time/state
nor execute program transitions.

Fixed strings encode exactly 128 UTF-8 bytes, zero-padding short text and rejecting
oversize or malformed text. Decode preserves zero characters and uses the existing
Dart-compatible strict UTF-8/BOM policy. Explicit type discriminants are checked
against the source contiguous enum positions; unsupported noncontiguous values
reject before output replacement.

Every instruction supports explicit ordered roles/program identity. The public
safe builder requires the exact declared privileges; escalated privileges are an
explicitly narrower input policy than unrestricted manual account metadata.
Optional sponsor/receiver tails are omitted only under the original omitted policy.
Fixed defaults retain literal source identities; every derived PDA uses the chosen
program override. Supplied role addresses override defaults without re-derivation.

All source inventory IDs and statuses remain unchanged. No deployed validator,
installed application or whole-ecosystem parity claim follows from these clients.
They are exclusively for off-chain applications; on-chain programs use Pina or
ofﬁcial Solana program libraries.

Value types provide structural equality/hash and Rust diagnostic Display. Program
errors implement the standard Error contract with exact primary numeric codes and
messages. Rust hash numbers and diagnostic field spellings are explicit idiomatic
equivalents, not promises to reproduce Dart runtime identity or hash numbers.
