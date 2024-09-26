use crate::utils::to_bytes_be;
use groth16_solana::groth16::Groth16Verifyingkey;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct G1 {
    #[serde(rename = "Alpha")]
    alpha: Point,
    #[serde(rename = "Beta")]
    beta: Point,
    #[serde(rename = "Delta")]
    delta: Point,
    #[serde(rename = "K")]
    k: Vec<Point>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct G2 {
    #[serde(rename = "Beta")]
    beta: Point2,
    #[serde(rename = "Delta")]
    delta: Point2,
    #[serde(rename = "Gamma")]
    gamma: Point2,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct Point {
    #[serde(rename = "X")]
    x: String,
    #[serde(rename = "Y")]
    y: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct Point2 {
    #[serde(rename = "X")]
    x: Complex,
    #[serde(rename = "Y")]
    y: Complex,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Complex {
    #[serde(rename = "A0")]
    a0: String,
    #[serde(rename = "A1")]
    a1: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VerifyingKey {
    #[serde(rename = "G1")]
    g1: G1,
    #[serde(rename = "G2")]
    g2: G2,
}

impl VerifyingKey {
    pub fn load(path: &str) -> Self {
        let contents = fs::read_to_string(path).unwrap();
        serde_json::from_str(&contents).unwrap()
    }

    pub fn vk_alpha_g1(&self) -> [u8; 64] {
        [to_bytes_be(&self.g1.alpha.x), to_bytes_be(&self.g1.alpha.y)]
            .concat()
            .try_into()
            .expect("Incorrect length for vk_alpha_g1")
    }

    pub fn vk_beta_g2(&self) -> [u8; 128] {
        [
            to_bytes_be(&self.g2.beta.x.a1),
            to_bytes_be(&self.g2.beta.x.a0),
            to_bytes_be(&self.g2.beta.y.a1),
            to_bytes_be(&self.g2.beta.y.a0),
        ]
        .concat()
        .try_into()
        .expect("Incorrect length for vk_beta_g2")
    }

    pub fn vk_gamme_g2(&self) -> [u8; 128] {
        [
            to_bytes_be(&self.g2.gamma.x.a1),
            to_bytes_be(&self.g2.gamma.x.a0),
            to_bytes_be(&self.g2.gamma.y.a1),
            to_bytes_be(&self.g2.gamma.y.a0),
        ]
        .concat()
        .try_into()
        .expect("Incorrect length for vk_gamme_g2")
    }

    pub fn vk_delta_g2(&self) -> [u8; 128] {
        [
            to_bytes_be(&self.g2.delta.x.a1),
            to_bytes_be(&self.g2.delta.x.a0),
            to_bytes_be(&self.g2.delta.y.a1),
            to_bytes_be(&self.g2.delta.y.a0),
        ]
        .concat()
        .try_into()
        .expect("Incorrect length for vk_delta_g2")
    }

    pub fn vk_ic(&self) -> [[u8; 64]; 2] {
        [
            [to_bytes_be(&self.g1.k[0].x), to_bytes_be(&self.g1.k[0].y)]
                .concat()
                .try_into()
                .expect("Incorrect length for vk_ic[0]"),
            [to_bytes_be(&self.g1.k[1].x), to_bytes_be(&self.g1.k[1].y)]
                .concat()
                .try_into()
                .expect("Incorrect length for vk_ic[1]"),
        ]
    }

    pub fn print_verifying_key(&self) {
        let verifying_key: Groth16Verifyingkey = Groth16Verifyingkey {
            nr_pubinputs: 10,
            vk_alpha_g1: self.vk_alpha_g1(),
            vk_beta_g2: self.vk_beta_g2(),
            vk_gamme_g2: self.vk_gamme_g2(),
            vk_delta_g2: self.vk_delta_g2(),
            vk_ic: &self.vk_ic(),
        };
        println!("verifying_key: {:?}", verifying_key);
    }
}
