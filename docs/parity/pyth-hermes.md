# Hermes response models

The optional `solana_kit_pyth/hermes` feature supplies exact price and confidence
records, ordinary and EMA feed entries, publication metadata, binary envelopes,
feed-listing metadata, both encodings and all eleven asset categories from the
pinned Dart Pyth package. `decode_price_update` and `decode_price_feeds` apply the
existing Kit JSON admission layer before constructing these records.

The reference is Dart commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46`, specifically
`price_feed.dart`, `encoding.dart`, `hermes_config.dart` and `hermes_client.dart`.
An independent Dart execution uses an in-process HTTP peer with OS networking
denied. It captures response components, all three routes, query order, option
defaults, eleven asset names, two encoding names, header precedence and the
200/300/400/408/429/500/503 status classes. Source and package configuration remain
unchanged during that execution.

Prices and confidence use Kit's existing arbitrary-precision integer dependency.
The JSON feature is optional; the default normal graph remains Kit Core and bs58.
No Solana Rust client, executor, browser global or platform transport is added.
Integer strings and integer JSON tokens remain exact. Components are limited to
8192 text bytes before conversion; fractional, exponential, whitespace and
hexadecimal component text is rejected. Exponents use signed 32 bit integers,
Unix times use signed 64 bit integers and slots use unsigned 64 bit integers.

`Price::approximate_f64` is explicitly lossy and follows IEEE overflow and
underflow behavior, including NaN for zero multiplied by an infinite scale.
It preserves the recorded integer and performs no loop over the supplied
exponent. It provides no authenticity, freshness or confidence validation.

The bounded decoders also reject duplicate keys and wrong attribute value types.
The Dart reference silently substitutes an empty map for a non-object attributes
value; Rust returns a schema error. Metadata equality and hashing include every
attribute, correcting the reference equality operator's omission of that map.
Missing and null collection defaults follow the reference. Missing parsed feeds
remain distinct from an empty parsed array. Unknown response fields are ignored
for forward compatibility; unknown encoding variants remain errors.

Nine focused tests cover exact values above 128 bits, full timestamps and slots,
numeric-token inputs, canonical decimal-string serialization, optional collection
presence, every metadata field, map equality, all names, malformed data, JSON
budgets, duplicate keys, component limits and explicit approximation.

Four appended shared conformance groups exercise these public APIs without host
I/O. The suite advances to v41 with 366 groups and keeps every earlier named case
and the sole corrupt-hash control. Native, browser, WASI Preview 1, Preview 2 and
the production LeptosKit wasmi host consume the same registry. Actual execution
must pass on each host before its runtime cell is accepted.

## GET orchestration

The runtime-neutral `Client` uses the existing REST destination and clock
contracts. Its three operations preserve route segments, ordered repeated IDs,
both encoding names, optional listing filters, default options and exact signed
timestamps. Decoded query values remain verbatim. The shared encoder uses `%20`
for a space where Dart uses `+`; both encode the same query component value.
Request strings and pair counts are admitted before copying. The full encoded
relative target also has a byte ceiling before an exchange is constructed.

The default policy gives each attempt 5000 ms and permits three retries, delayed
by 100, 200 and 400 ms. Configured retries are limited to 32 and every exponential
delay is checked in advance. Clock ticks use checked addition. Actual timer
failures propagate separately from an elapsed deadline. Transient I/O, deadlines,
HTTP 408/429 and statuses at least 500 retry; policy, cancellation, budget,
framing and response-schema failures stop. Original bounded HTTP error bodies
remain accessible while ordinary diagnostics hide their contents.

Each exchange is destroyed before retry backoff starts. Cancelling the returned
future destroys its current exchange and timer without detached work or a retry.
Twelve native contracts exercise all three operations, ordered options, exact
backoff, last failures, transport categories, resource admission, timeout,
unrepresentable ticks, timer admission failure and physical future destruction.
The adapters and clocks deliberately contain Rc state, proving local futures do
not acquire a universal Send requirement.

Four appended shared groups exercise these GET futures in the same portable
registry, advancing it to v42 with 370 cases. Every earlier case and the sole
corrupt-hash control remain. Each runtime must execute that expanded registry
before its cell is accepted. Native test fixtures perform no host I/O.

Provider roots, headers and authentication use the existing approved transport
adapter policies. Actual native, browser, WASI and interpreted-host network
composition for this new client still requires its own evidence. Pyth instruction
builders and PDA helpers also remain open. This does not complete Pyth or the
full ecosystem.
