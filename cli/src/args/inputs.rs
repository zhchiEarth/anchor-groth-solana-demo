use crate::utils::to_bytes_be;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct PublicInput([String; 1]);

pub fn get_input(path: &str) -> [[u8; 32]; 1] {
    let contents = fs::read_to_string(path).unwrap();

    let data: PublicInput = serde_json::from_str(&contents).unwrap();
    [to_bytes_be(&data.0[0])]
}
