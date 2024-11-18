//! A simple script to generate and verify the proof of a given program.

use clap::Parser;
use hex;
use serde_json::Value;
use sp1_sdk::{ProverClient, SP1Stdin};
use std::fs::File;
use std::io::BufReader;
use wasm::{PrivateKey, PublicKey, Serialize, Signature};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    let args = Args::parse();
    // Read in wasm file from disk
    let wasm =
        include_bytes!("../../../wasm/target/wasm32-unknown-unknown/release/wasm.wasm").to_vec();
    // Read the JSON file
    let file = File::open("../../../../inputs/bls_verify.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let input_value = json["numSigners"]
        .as_u64()
        .expect("Failed to parse value from JSON") as u32;
    println!("Input value: {}", input_value);
    let aggregated_signature = json["aggregateSignature"]
        .as_str()
        .expect("Failed to parse value from JSON");
    println!("Input value: {}", aggregated_signature);

    let private_keys: Vec<PrivateKey> = (0..input_value)
        .map(|i| PrivateKey::new(&[i as u8; 32]))
        .collect();

    let public_keys = private_keys
        .iter()
        .map(|pk| pk.public_key().as_bytes().to_vec())
        .collect::<Vec<_>>();
    // Setup the prover client.
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    stdin.write(&wasm);
    stdin.write(
        &hex::decode(aggregated_signature).expect("Failed to decode aggregated signature from hex"),
    );
    stdin.write(&public_keys);

    if args.execute {
        // Execute the program
        let (mut output, report) = client.execute(ELF, stdin).run().unwrap();
        println!("Program executed successfully.");
        let res = output.read::<i32>();
        println!("res: {}", res);
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, stdin)
            .groth16()
            .run()
            .expect("failed to generate proof");

        let proof_bytes = proof.bytes();
        println!("Proof Size: {}", proof_bytes.len());

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
