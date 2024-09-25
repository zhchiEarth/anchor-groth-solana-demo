import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorGrothSolanaDemo } from "../target/types/anchor_groth_solana_demo";

type NumberArray256 = [number, ...number[]] & { length: 256 };

describe("anchor-groth16-demo", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env()
  const program = anchor.workspace.AnchorGrothSolanaDemo as Program<AnchorGrothSolanaDemo>;

  it("test initialized!", async () => {
    const computeBudgetInstruction = anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
      units: 1500000,
    });

    const initializeInstruction = await program.methods.initialize().instruction();
    const transaction = new anchor.web3.Transaction().add(computeBudgetInstruction).add(initializeInstruction);

    const signature = await provider.sendAndConfirm(transaction);
    console.log("signature:", signature);
  });

  // it("test verify!", async () => {
  //   const uint8Array = getProof();
  //   type RustArrayEquivalent = number[][];
  //   const rustArrayEquivalent: RustArrayEquivalent = [
  //     [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
  //   ];
  //   // Add your test here.
  //   const tx = await program.methods.verify(uint8Array, rustArrayEquivalent).rpc();
  //   console.log("Your transaction signature", tx);
  // });

});


function getProof(): NumberArray256 {
  const hexString = "0f6b24e58e29b977d0fd5db874fcaba1fe07a2128d9bc038cca0a64b100e7139138d64f68467f24759e01c9699b3994b52c9ff15d44a24dd45d644e404d8ab4504467dd4387f352289599da8df816edb1a0a70a7ee06a70c4c6b9c5c4284a9980dfaf44f445e0924410d9bddcde6b662d4884c8a0cc4ff17ac21d1d0163a64e010f4d96cbd6c94869ae9c3199bff3f7be482b75d257a93867371341f822ab2ed0e3f3972f28854185e1651a2305b8d92aaa14f41a3670e3ccb0e85fa752b7bf61be962c8531abf97146f271a877189f1005417803043c7a40285a946e5d0d60b2844d5ba04615c0b4296b43426d474a7fd17a751a8118743b845a2c0d974ded7";

  if (!hexString || hexString.length % 2 !== 0 || !/^[0-9a-fA-F]+$/.test(hexString)) {
    throw new Error('Invalid hex string');
  }

  let u8Array: NumberArray256 = Array.from({ length: 256 }, () => 0) as NumberArray256;
  let index = 0;
  for (let i = 0; i < hexString.length; i += 2) {
    const byte = parseInt(hexString.substr(i, 2), 16);
    u8Array.push(byte);
    u8Array[index] = byte;
    index++;
  }
  return u8Array;
}