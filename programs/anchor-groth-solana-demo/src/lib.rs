use anchor_lang::prelude::*;
use ark_bn254::g1::G1Affine;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use groth16_solana::groth16::{Groth16Verifier, Groth16Verifyingkey};
use std::ops::Neg;

declare_id!("3dg1wKkhN4WexPV1z9Q9hJnSoqB5j3pCboBGm36UC129");

pub const VERIFYING_KEY: Groth16Verifyingkey = Groth16Verifyingkey {
    nr_pubinputs: 10,

    vk_alpha_g1: [
        36, 136, 15, 114, 154, 117, 84, 129, 37, 239, 118, 68, 223, 197, 188, 97, 33, 222, 248, 31,
        67, 88, 140, 32, 185, 254, 58, 203, 157, 42, 230, 180, 32, 130, 159, 2, 222, 30, 68, 210,
        168, 188, 27, 148, 190, 74, 126, 40, 80, 194, 104, 85, 238, 64, 177, 231, 251, 198, 234,
        99, 131, 254, 35, 65,
    ],

    vk_beta_g2: [
        46, 21, 1, 156, 194, 61, 133, 129, 52, 38, 212, 172, 248, 209, 30, 190, 187, 215, 86, 204,
        53, 137, 152, 177, 72, 139, 189, 62, 62, 245, 238, 108, 5, 166, 51, 2, 211, 142, 42, 250,
        3, 10, 1, 199, 42, 189, 202, 54, 190, 209, 116, 55, 53, 40, 209, 233, 183, 136, 129, 66,
        79, 60, 100, 148, 40, 19, 233, 133, 147, 173, 94, 99, 184, 60, 152, 19, 226, 110, 199, 132,
        12, 252, 186, 250, 242, 131, 185, 117, 248, 162, 93, 86, 231, 115, 103, 179, 30, 241, 136,
        10, 234, 241, 154, 38, 152, 107, 231, 205, 238, 155, 124, 35, 57, 75, 130, 151, 89, 126,
        76, 196, 96, 192, 110, 82, 93, 43, 206, 12,
    ],

    vk_gamme_g2: [
        44, 153, 59, 129, 44, 132, 7, 67, 55, 79, 174, 98, 228, 57, 29, 8, 57, 131, 80, 176, 142,
        188, 173, 104, 149, 233, 43, 193, 155, 66, 165, 34, 18, 241, 141, 221, 90, 181, 162, 176,
        104, 152, 64, 105, 135, 118, 59, 155, 183, 232, 107, 169, 136, 178, 27, 108, 61, 182, 248,
        195, 239, 11, 84, 150, 2, 215, 217, 103, 243, 45, 151, 167, 83, 82, 200, 149, 175, 57, 91,
        4, 59, 133, 154, 237, 63, 190, 162, 207, 99, 98, 222, 7, 175, 42, 27, 63, 17, 156, 236,
        170, 63, 60, 62, 72, 126, 7, 160, 71, 207, 131, 226, 172, 37, 247, 202, 233, 156, 36, 208,
        101, 166, 204, 34, 34, 169, 84, 172, 35,
    ],

    vk_delta_g2: [
        1, 167, 131, 165, 42, 239, 227, 204, 10, 23, 227, 77, 147, 130, 76, 202, 117, 47, 233, 165,
        65, 217, 140, 191, 247, 3, 215, 8, 52, 148, 33, 56, 39, 127, 207, 23, 110, 25, 62, 140,
        233, 74, 46, 250, 115, 166, 249, 200, 195, 144, 93, 201, 62, 202, 254, 66, 243, 155, 31,
        129, 210, 64, 138, 243, 30, 82, 80, 108, 143, 70, 31, 89, 196, 95, 134, 112, 25, 247, 141,
        73, 117, 46, 120, 250, 7, 127, 37, 179, 180, 141, 227, 108, 242, 181, 239, 58, 45, 74, 2,
        54, 116, 141, 228, 161, 166, 86, 24, 225, 57, 96, 154, 207, 160, 109, 244, 52, 171, 105,
        83, 93, 160, 76, 145, 37, 192, 172, 171, 184,
    ],

    vk_ic: &[
        [
            17, 24, 116, 127, 0, 209, 6, 119, 14, 60, 48, 253, 146, 233, 146, 46, 215, 8, 218, 30,
            130, 104, 22, 172, 147, 142, 79, 209, 244, 38, 4, 133, 42, 136, 105, 54, 233, 174, 139,
            143, 7, 92, 72, 44, 152, 34, 39, 57, 227, 61, 175, 224, 153, 227, 50, 86, 250, 132,
            160, 193, 104, 109, 32, 133,
        ],
        [
            32, 61, 217, 196, 247, 218, 225, 13, 124, 150, 127, 237, 71, 42, 93, 62, 118, 121, 1,
            113, 50, 120, 44, 188, 82, 242, 88, 169, 143, 124, 118, 110, 17, 182, 190, 19, 198,
            137, 63, 161, 131, 133, 248, 94, 24, 56, 4, 142, 113, 76, 90, 244, 237, 39, 74, 149,
            16, 46, 139, 243, 92, 127, 173, 4,
        ],
    ],
};

pub const PROOF: [u8; 256] = [
    15, 107, 36, 229, 142, 41, 185, 119, 208, 253, 93, 184, 116, 252, 171, 161, 254, 7, 162, 18,
    141, 155, 192, 56, 204, 160, 166, 75, 16, 14, 113, 57, 19, 141, 100, 246, 132, 103, 242, 71,
    89, 224, 28, 150, 153, 179, 153, 75, 82, 201, 255, 21, 212, 74, 36, 221, 69, 214, 68, 228, 4,
    216, 171, 69, 4, 70, 125, 212, 56, 127, 53, 34, 137, 89, 157, 168, 223, 129, 110, 219, 26, 10,
    112, 167, 238, 6, 167, 12, 76, 107, 156, 92, 66, 132, 169, 152, 13, 250, 244, 79, 68, 94, 9,
    36, 65, 13, 155, 221, 205, 230, 182, 98, 212, 136, 76, 138, 12, 196, 255, 23, 172, 33, 209,
    208, 22, 58, 100, 224, 16, 244, 217, 108, 189, 108, 148, 134, 154, 233, 195, 25, 155, 255, 63,
    123, 228, 130, 183, 93, 37, 122, 147, 134, 115, 113, 52, 31, 130, 42, 178, 237, 14, 63, 57,
    114, 242, 136, 84, 24, 94, 22, 81, 162, 48, 91, 141, 146, 170, 161, 79, 65, 163, 103, 14, 60,
    203, 14, 133, 250, 117, 43, 123, 246, 27, 233, 98, 200, 83, 26, 191, 151, 20, 111, 39, 26, 135,
    113, 137, 241, 0, 84, 23, 128, 48, 67, 199, 164, 2, 133, 169, 70, 229, 208, 214, 11, 40, 68,
    213, 186, 4, 97, 92, 11, 66, 150, 180, 52, 38, 212, 116, 167, 253, 23, 167, 81, 168, 17, 135,
    67, 184, 69, 162, 192, 217, 116, 222, 215,
];

pub const PUBLIC_INPUTS: [[u8; 32]; 1] = [[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
]];

#[program]
pub mod anchor_groth_solana_demo {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("initialize Greetings from: {:?}", ctx.program_id);
        let proof_a: G1Affine = G1Affine::deserialize_with_mode(
            &*[&change_endianness(&PROOF[0..64]), &[0u8][..]].concat(),
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

        let proof_a = change_endianness(&proof_a_neg[..64]).try_into().unwrap();
        let proof_b = PROOF[64..192].try_into().unwrap();
        let proof_c = PROOF[192..256].try_into().unwrap();

        let mut verifier =
            Groth16Verifier::new(&proof_a, &proof_b, &proof_c, &PUBLIC_INPUTS, &VERIFYING_KEY)
                .unwrap();

        let flag = verifier.verify().unwrap();

        msg!("initialize flag: {:?}", flag);

        Ok(())
    }

    pub fn verify(
        ctx: Context<Initialize>,
        proof: [u8; 256],
        public_inputs: [[u8; 32]; 1],
    ) -> Result<bool> {
        msg!("verify Greetings from: {:?}", ctx.program_id);
        msg!("proof: {:?}", proof);
        msg!("public_inputs: {:?}", public_inputs);
        let proof_a: G1Affine = G1Affine::deserialize_with_mode(
            &*[&change_endianness(&proof[0..64]), &[0u8][..]].concat(),
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

        let proof_a = change_endianness(&proof_a_neg[..64]).try_into().unwrap();
        let proof_b = proof[64..192].try_into().unwrap();
        let proof_c = proof[192..256].try_into().unwrap();

        let mut verifier =
            Groth16Verifier::new(&proof_a, &proof_b, &proof_c, &public_inputs, &VERIFYING_KEY)
                .unwrap();

        let flag = verifier.verify().unwrap();

        msg!("verify flag: {:?}", flag);

        Ok(true)
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

#[derive(Accounts)]
pub struct Initialize {}
