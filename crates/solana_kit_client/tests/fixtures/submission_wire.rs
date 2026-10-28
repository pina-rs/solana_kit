//! Literal wire fixtures independent of the production transaction encoder.

/// Legacy/v0 wire starts with signatures; V1 places them after its message.
pub fn wire(version: u8, signature: u8) -> Vec<u8> {
    let mut bytes = Vec::new();

    if version == 1 {
        bytes.extend([0x81, 1, 0, 0]);
        bytes.extend([0; 4]);
        bytes.extend([2; 32]);
        bytes.extend([0, 1]);
        bytes.extend([1; 32]);
        bytes.extend([signature; 64]);

        return bytes;
    }

    bytes.push(1);
    bytes.extend([signature; 64]);

    if version == 0 {
        bytes.push(0x80);
    }

    bytes.extend([1, 0, 0, 1]);
    bytes.extend([1; 32]);
    bytes.extend([2; 32]);
    bytes.push(0);

    if version == 0 {
        bytes.push(0);
    }

    bytes
}

/// A complete 4096-byte V1 transaction must retain its larger packet allowance.
pub fn full_v1() -> Vec<u8> {
    let mut bytes = vec![0x81, 1, 0, 1];
    bytes.extend([0; 4]);
    bytes.extend([2; 32]);
    bytes.extend([1, 2]);
    bytes.extend([1; 32]);
    bytes.extend([3; 32]);
    bytes.extend([1, 0]);
    bytes.extend(3922u16.to_le_bytes());
    bytes.extend([0; 3922]);
    bytes.extend([7; 64]);
    assert_eq!(bytes.len(), 4096);

    bytes
}
