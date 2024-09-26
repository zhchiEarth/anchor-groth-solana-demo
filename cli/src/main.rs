use {
    anchor_groth_solana_demo::{instruction::Verify, ID},
    anchor_lang::{prelude::borsh, Discriminator},
    dotenv::dotenv,
    solana_rpc_client::rpc_client,
    solana_sdk::{
        instruction::Instruction,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction,
    },
    std::{env, str::FromStr},
};

mod args;
mod utils;

use args::VerifyParams;

const RPC_ADDR: &str = "https://api.devnet.solana.com";
// const RPC_ADDR: &str = "http://127.0.0.1:8899";

fn main() {
    dotenv().ok();
    let proof = VerifyParams::load();
    let program_id = Pubkey::from_str(ID.to_string().as_str()).unwrap();

    let private_key_str = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not found in .env file");
    let private_key_bytes: Vec<u8> = private_key_str
        .trim_matches(|c: char| !c.is_digit(10) && c != ',')
        .split(',') // 以逗号分割字符串
        .filter_map(|s| s.parse::<u8>().ok())
        .collect();

    let signer = Keypair::from_bytes(&private_key_bytes).unwrap();
    let client = rpc_client::RpcClient::new(RPC_ADDR);

    let ixs = vec![verify_instruction(program_id, &proof)];
    let latest_blockhash = client.get_latest_blockhash().unwrap();

    println!("latest_blockhash: {}", latest_blockhash);

    let sig = client
        .send_and_confirm_transaction(&transaction::Transaction::new_signed_with_payer(
            &ixs,
            Some(&signer.pubkey()),
            &[&signer],
            latest_blockhash,
        ))
        .unwrap();

    println!("tx: {}", sig);
}

pub fn verify_instruction(program_id: Pubkey, params: &VerifyParams) -> Instruction {
    let instruction_data = Verify {
        proof: params.proof,
        public_inputs: params.inputs,
        vk_alpha_g1: params.verifying_key.vk_alpha_g1(),
        vk_beta_g2: params.verifying_key.vk_beta_g2(),
        vk_gamme_g2: params.verifying_key.vk_gamme_g2(),
        vk_delta_g2: params.verifying_key.vk_delta_g2(),
        vk_ic: params.verifying_key.vk_ic(),
    };
    let mut data = borsh::to_vec(&instruction_data).unwrap();

    data.splice(0..0, Verify::DISCRIMINATOR.iter().cloned());

    Instruction {
        program_id,
        accounts: vec![],
        data,
    }
}
