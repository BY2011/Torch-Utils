
use sha3::Digest;

/// ## Description
/// Checks provided merkle root validity
/// ## Params
/// * **merkle_root** is an object of type [`str`]
pub fn validate_merkle_root(merkle_root: &str) {
    let mut root_buf: [u8; 32] = [0; 32];
    assert!(
        hex::decode_to_slice(merkle_root, &mut root_buf).is_ok(),