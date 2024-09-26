use ark_bn254::g1::G1Affine;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use serde::{Deserialize, Serialize};
use {
    anchor_groth_solana_demo::{instruction::Verify, ID},
    anchor_lang::Discriminator,
    groth16_solana::groth16::Groth16Verifyingkey,
    num_bigint::BigUint,
    solana_rpc_client::rpc_client,
    solana_sdk::{
        instruction::Instruction, pubkey::Pubkey, signature::Signer, signer::keypair, transaction,
    },
    std::{fs, ops::Neg, path::Path, str::FromStr},
    uint::construct_uint,
};

construct_uint! {
    pub struct U256(4);
}

// const RPC_ADDR: &str = "https://api.devnet.solana.com";
const RPC_ADDR: &str = "http://127.0.0.1:8899";

fn main() {
    // get_verifying_key();

    let program_id = Pubkey::from_str(ID.to_string().as_str()).unwrap();

    let path = Path::new("/Users/zhc/.config/solana/id.json");
    let owner_account = keypair::read_keypair_file(path).unwrap();
    let client = rpc_client::RpcClient::new(RPC_ADDR);

    // let ixs = vec![initialize_instruction(program_id)];
    let ixs = vec![verify_instruction(program_id)];
    // println!("ixs: {:?}", ixs);
    // 150 block都有效
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    println!("latest_blockhash: {}", latest_blockhash);

    let sig = client
        .send_and_confirm_transaction(&transaction::Transaction::new_signed_with_payer(
            &ixs,
            Some(&owner_account.pubkey()),
            &[&owner_account],
            latest_blockhash,
        ))
        .unwrap();

    println!("tx: {}", sig);
}

const PROOF_PATH: &str = "programs/anchor-groth-solana-demo/proof/proof.hex";

pub fn get_proof() -> [u8; 256] {
    let contents = fs::read_to_string(PROOF_PATH).unwrap();
    let hex_str = contents.as_str();
    // let hex_str = "0f6b24e58e29b977d0fd5db874fcaba1fe07a2128d9bc038cca0a64b100e7139138d64f68467f24759e01c9699b3994b52c9ff15d44a24dd45d644e404d8ab4504467dd4387f352289599da8df816edb1a0a70a7ee06a70c4c6b9c5c4284a9980dfaf44f445e0924410d9bddcde6b662d4884c8a0cc4ff17ac21d1d0163a64e010f4d96cbd6c94869ae9c3199bff3f7be482b75d257a93867371341f822ab2ed0e3f3972f28854185e1651a2305b8d92aaa14f41a3670e3ccb0e85fa752b7bf61be962c8531abf97146f271a877189f1005417803043c7a40285a946e5d0d60b2844d5ba04615c0b4296b43426d474a7fd17a751a8118743b845a2c0d974ded7";

    let big_int = BigUint::parse_bytes(hex_str.as_bytes(), 16).unwrap(); // 将字符串转换为 BigUint
                                                                         // println!("big_int:{}", big_int);
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

const INPUT_PATH: &str = "programs/anchor-groth-solana-demo/proof/input.json";

#[derive(Serialize, Deserialize, Debug)]
struct PublicInput([String; 1]);

pub fn get_input() -> [[u8; 32]; 1] {
    let contents = fs::read_to_string(INPUT_PATH).unwrap();
    let data: PublicInput = serde_json::from_str(&contents).unwrap();

    let value = U256::from_dec_str(&data.0[0]).unwrap();
    let value: [u8; 32] = value.to_big_endian();
    // println!("value:{:?}", value.to_big_endian());
    //     // let hex_str = data.0;
    //     let big_int = BigUint::parse_bytes(data.0[0].as_bytes(), 16).unwrap();
    //     // 将字符串转换为 BigUint
    //     let bytes = big_int.to_bytes_be();
    //     println!("big_int: {:?}", big_int);
    //     let mut bytes: [u8; 32] = bytes.as_slice().try_into().expect("Incorrect length");
    //     println!("bytes:{:?}", bytes);
    [value]
}

pub fn verify_instruction(program_id: Pubkey) -> Instruction {
    let instruction_data = Verify {
        proof: get_proof(),
        public_inputs: get_input(),
    };
    let mut data = anchor_lang::prelude::borsh::to_vec(&instruction_data).unwrap();

    data.splice(0..0, Verify::DISCRIMINATOR.iter().cloned());

    Instruction {
        program_id,
        accounts: vec![],
        data,
    }
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

#[derive(Serialize, Deserialize, Debug)]
struct G1 {
    Alpha: Point,
    Beta: Point,
    Delta: Point,
    K: Vec<Point>,
}

#[derive(Serialize, Deserialize, Debug)]
struct G2 {
    Beta: Point2,
    Delta: Point2,
    Gamma: Point2,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    X: String,
    Y: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point2 {
    X: Complex,
    Y: Complex,
}

#[derive(Serialize, Deserialize, Debug)]
struct Complex {
    A0: String,
    A1: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct VerifyingKey {
    #[serde(rename = "G1")]
    g1: G1,
    G2: G2,
}

const VERIFYING_KEY_PATH: &str = "programs/anchor-groth-solana-demo/proof/block_vk.json";

pub fn get_verifying_key() {
    let contents = fs::read_to_string(VERIFYING_KEY_PATH).unwrap();
    let vk: VerifyingKey = serde_json::from_str(&contents).unwrap();

    // println!("Verifying_Key: {:?}", data);

    let vk_alpha_g1: [u8; 64] = [to_bytes_be(&vk.g1.Alpha.X), to_bytes_be(&vk.g1.Alpha.Y)]
        .concat()
        .try_into()
        .expect("Incorrect length for vk_alpha_g1");

    let vk_beta_g2: [u8; 128] = [
        to_bytes_be(&vk.G2.Beta.X.A1),
        to_bytes_be(&vk.G2.Beta.X.A0),
        to_bytes_be(&vk.G2.Beta.Y.A1),
        to_bytes_be(&vk.G2.Beta.Y.A0),
    ]
    .concat()
    .try_into()
    .expect("Incorrect length for vk_beta_g2");

    let vk_gamme_g2: [u8; 128] = [
        to_bytes_be(&vk.G2.Gamma.X.A1),
        to_bytes_be(&vk.G2.Gamma.X.A0),
        to_bytes_be(&vk.G2.Gamma.Y.A1),
        to_bytes_be(&vk.G2.Gamma.Y.A0),
    ]
    .concat()
    .try_into()
    .expect("Incorrect length for vk_gamme_g2");

    let vk_delta_g2: [u8; 128] = [
        to_bytes_be(&vk.G2.Delta.X.A1),
        to_bytes_be(&vk.G2.Delta.X.A0),
        to_bytes_be(&vk.G2.Delta.Y.A1),
        to_bytes_be(&vk.G2.Delta.Y.A0),
    ]
    .concat()
    .try_into()
    .expect("Incorrect length for vk_delta_g2");

    let vk_ic: [[u8; 64]; 2] = [
        [to_bytes_be(&vk.g1.K[0].X), to_bytes_be(&vk.g1.K[0].Y)]
            .concat()
            .try_into()
            .expect("Incorrect length for vk_ic[0]"),
        [to_bytes_be(&vk.g1.K[1].X), to_bytes_be(&vk.g1.K[1].Y)]
            .concat()
            .try_into()
            .expect("Incorrect length for vk_ic[1]"),
    ];

    let verifying_key: Groth16Verifyingkey = Groth16Verifyingkey {
        nr_pubinputs: 10,
        vk_alpha_g1,
        vk_beta_g2,
        vk_gamme_g2,
        vk_delta_g2,
        vk_ic: &vk_ic,
    };
    println!("verifying_key: {:?}", verifying_key);
}

pub fn to_bytes_be(item: &str) -> [u8; 32] {
    let big_int = BigUint::parse_bytes(item.as_bytes(), 10).unwrap(); // 将字符串转换为 BigUint

    let mut bytes = big_int.to_bytes_be();
    // 如果字节向量小于32字节，前面填充零
    if bytes.len() < 32 {
        let mut padded_bytes = vec![0u8; 32 - bytes.len()];
        padded_bytes.extend_from_slice(&bytes);
        bytes = padded_bytes;
    }

    // 大于32位
    if bytes.len() > 32 {
        panic!("Number is too large to fit into 128 bytes");
    }
    bytes.as_slice().try_into().expect("Incorrect length")
}
