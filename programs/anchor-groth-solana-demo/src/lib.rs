use anchor_lang::prelude::*;
use groth16_solana::groth16::{Groth16Verifier, Groth16Verifyingkey};

declare_id!("3dg1wKkhN4WexPV1z9Q9hJnSoqB5j3pCboBGm36UC129");

#[program]
pub mod anchor_groth_solana_demo {

    use super::*;

    pub fn verify(
        ctx: Context<Initialize>,
        proof: [u8; 256],
        public_inputs: [[u8; 32]; 1],
        vk_alpha_g1: [u8; 64],
        vk_beta_g2: [u8; 128],
        vk_gamme_g2: [u8; 128],
        vk_delta_g2: [u8; 128],
        vk_ic: [[u8; 64]; 2],
    ) -> Result<bool> {
        // pub vk_alpha_g1: [u8; 64],
        // pub vk_beta_g2: [u8; 128],
        // pub vk_gamme_g2: [u8; 128],
        // pub vk_delta_g2: [u8; 128],
        // pub vk_ic: &'a [[u8; 64]],
        // let proof_a: G1Affine = G1Affine::deserialize_with_mode(
        //     &*[&change_endianness(&PROOF[0..64]), &[0u8][..]].concat(),
        //     Compress::No,
        //     Validate::Yes,
        // )
        // .unwrap();
        // let mut proof_a_neg = [0u8; 65];
        // proof_a
        //     .neg()
        //     .x
        //     .serialize_with_mode(&mut proof_a_neg[..32], Compress::No)
        //     .unwrap();
        // proof_a
        //     .neg()
        //     .y
        //     .serialize_with_mode(&mut proof_a_neg[32..], Compress::No)
        //     .unwrap();
        // let proof_a = change_endianness(&proof_a_neg[..64]).try_into().unwrap();

        let verifying: Groth16Verifyingkey = Groth16Verifyingkey {
            nr_pubinputs: 10,
            vk_alpha_g1,
            vk_beta_g2,
            vk_gamme_g2,
            vk_delta_g2,
            vk_ic: &vk_ic,
        };
        let proof_a = proof[..64].try_into().unwrap();
        let proof_b = proof[64..192].try_into().unwrap();
        let proof_c = proof[192..256].try_into().unwrap();
        let mut verifier =
            Groth16Verifier::new(&proof_a, &proof_b, &proof_c, &public_inputs, &verifying).unwrap();

        let res = verifier.verify();

        msg!("verify status: {:?}", res);

        Ok(true)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
