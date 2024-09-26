use {
    ark_bn254::g1::G1Affine,
    ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate},
    num_bigint::BigUint,
    std::{fs, ops::Neg},
};

pub fn get_proof(path: &str) -> [u8; 256] {
    let contents = fs::read_to_string(path).unwrap();
    let hex_str = contents.as_str();
    let big_int = BigUint::parse_bytes(hex_str.as_bytes(), 16).unwrap(); // 将字符串转换为 BigUint
    let bytes = big_int.to_bytes_be();
    let mut bytes: [u8; 256] = bytes.as_slice().try_into().expect("Incorrect length");

    let proof_a: G1Affine = G1Affine::deserialize_with_mode(
        &*[&change_endianness(&bytes[0..64]), &[0u8][..]].concat(),
        Compress::No,
        Validate::Yes,
    )
    .unwrap();
    let mut proof_a_neg = [0u8; 65];
    proof_a
        .neg()
        .x
        .serialize_with_mode(&mut proof_a_neg[..32], Compress::No)
        .unwrap();
    proof_a
        .neg()
        .y
        .serialize_with_mode(&mut proof_a_neg[32..], Compress::No)
        .unwrap();
    let proof_a: [u8; 64] = change_endianness(&proof_a_neg[..64]).try_into().unwrap();
    bytes[..64].copy_from_slice(&proof_a);
    bytes
}

fn change_endianness(bytes: &[u8]) -> Vec<u8> {
    let mut vec = Vec::new();
    for b in bytes.chunks(32) {
        for byte in b.iter().rev() {
            vec.push(*byte);
        }
    }
    vec
}
