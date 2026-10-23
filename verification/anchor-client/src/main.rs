//! A real linked portability probe; execution is recorded separately per host.

use solana_kit_anchor::{Coder, Idl, Limits, Values, instruction_discriminator};

const LIMITS: Limits = Limits::new(4096, 1024, 32);

fn main() {
    let idl = Idl::parse(
        r#"{"instructions":[{"name":"close","discriminator":[1,2,3]}]}"#,
        LIMITS,
    )
    .unwrap();
    let coder = Coder::new(&idl, LIMITS);
    let encoded = coder.encode_instruction("close", &Values::new()).unwrap();

    assert_eq!(encoded, [1, 2, 3]);
    assert_eq!(
        coder
            .decode_instruction("close", &encoded)
            .unwrap()
            .consumed,
        encoded.len()
    );
    assert_eq!(
        instruction_discriminator("initialize"),
        [175, 175, 109, 31, 13, 152, 155, 237]
    );
}
