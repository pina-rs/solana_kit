# Codec size declarations and checked pairing

This source checkpoint adds value-free format declarations and checked independent
pairing for off-chain application clients. It has not been compiled or executed.
On-chain programs use Pina or official Solana program libraries.

`Encoder<T>::encode_size` and `Decoder<U>::decode_size` return `Result<Size, Error>`.
The default is `Unknown`, so existing custom implementations retain their ordinary
operations. `Fixed(0)`, `Variable { max: Some(0) }` and `Variable { max: None }`
are distinct declarations. These describe format policy. They do not replace
`encoded_len`, cursor behavior, operation limits or shared allocation claims.

`Size::checked_sum` and `checked_repeat` reject `usize` overflow. Empty sums,
empty choices and zero repetition produce `Fixed(0)`. A sum checks its known
subtotal even when another component has an unknown class or maximum. This is an
explicit order-independent Rust integer-domain policy. `choice` selects maxima
without arithmetic and preserves unknown maxima.

`Paired::try_new` checks the selected input and output implementations. It rejects
unknown declarations, mixed fixed and variable kinds, unequal fixed widths and
unequal complete optional maxima. It accepts two variable unknown maxima.
Private parts and the selected value-type parameters prevent unchecked assembly
and validation of one input type followed by use of another. `into_parts` consumes
that identity. No duplicate accepted metadata is stored.

The pair forwards all selected operations, including owned encode, full decode
and prefix decode overrides. This preserves independently chosen policies such
as cursor rewinding. Constructor agreement does not prove inverses or constrain
callback movement. Metadata stability for unchanged configuration is a logical
trait obligation. Private fields cannot prove purity of `Rc`, `Cell` or custom
clone behavior. Copy, Clone and Debug bounds depend only on stored codec parts.

`PairError` is separate from the ordinary byte-operation `Error`. Its three
source mismatch codes reuse 8078004, 8078005 and 8078006. Fixed widths and complete
optional maxima retain both source contexts. Unknown maxima use an explicit null
at the `ErrorFacts` boundary. Diagnostic normalization can remove that null.
Metadata failures retain their actual error through `source()` and delegate facts.
Unknown classification is a Rust extension without an invented source code.

## Source scope and remaining declarations

The written declarations cover Number, including signed, floating and wide
numbers, ShortU16, Bool, FixedBytes, Bytes, Unit, Mapped and reference queries.
Other wrappers retain the honest default `Unknown`. Their ordinary operations
are unchanged. This checkpoint does not establish complete metadata parity.

Reference implementations still forward only exact sizing, raw writing and raw
reading, in addition to the new queries. Their inherited owned-operation defaults
can lose custom overrides. This existing limitation is recorded separately from
the complete delegation provided by an owned `Paired`.

The later propagation unit must derive declarations beside each existing owner.
Offset forwards source declarations without invoking position callbacks. Skip
frames and sentinel declarations follow source format policy even when actual
movement differs. OptionalCount forwards its prefix declaration and retains
exact-end leniency. Fixed Resized transforms a real fixed declaration. Variable
Resized preserves the source declaration without claiming an allocation bound.
Its constructor timing, callback counts and error timing need a separate source
oracle before implementation. Dependent remains Unknown until it owns a genuine
value-free family declaration.

## Maintained pending evidence

Eight new bodies in `crates/solana_kit_core/tests/codec_size` are shared by native
tests, conformance and the private `verification/codec-size-consumer` package.
The registry appends them after the original 307 cases, giving a written v33
frontier of 315. This is source inventory, not an executed result.

Both existing `mappings_and_pairing_preserve_failures` bodies retain mapped
overflow, mapped decode, one-byte standalone encoding and standalone truncated
u32 decoding. Only their unchecked fixed1/fixed4 construction migrates to
constructor rejection. The added compatible asymmetric-value pair checks actual
bytes, a nonzero prefix offset and full-decode trailing-byte policy. All original
case names, registry order, golden rows, claim and failure-position assertions
remain in place.

The private consumer adds ordinary dependency use, legacy Unknown behavior,
selected multi-Encoder use, local callbacks, and conditional trait-bound proof.
Eight compile-failure examples reject the other input type, tuple construction,
each private part, struct-literal assembly, unchecked construction, tuple
conversion and Default. All fixtures, examples and controls remain pending.
