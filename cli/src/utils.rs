use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

pub fn to_bytes_be(item: &str) -> [u8; 32] {
    let value = U256::from_dec_str(item).unwrap();
    let value: [u8; 32] = value.to_big_endian();
    value
}
