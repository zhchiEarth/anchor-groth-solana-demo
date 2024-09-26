mod inputs;
mod proof;
mod verifying_key;

use inputs::*;
use proof::*;
use verifying_key::*;

#[derive(Debug)]
pub struct VerifyParams {
    pub inputs: [[u8; 32]; 1],
    pub proof: [u8; 256],
    pub verifying_key: VerifyingKey,
}

const VERIFYING_KEY_PATH: &str = "cli/proof/block_vk.json";
const PROOF_PATH: &str = "cli/proof/proof.hex";
const INPUTS_PATH: &str = "cli/proof/input.json";

impl VerifyParams {
    pub fn load() -> Self {
        Self {
            inputs: get_input(INPUTS_PATH),
            proof: get_proof(PROOF_PATH),
            verifying_key: VerifyingKey::load(VERIFYING_KEY_PATH),
        }
    }
}
