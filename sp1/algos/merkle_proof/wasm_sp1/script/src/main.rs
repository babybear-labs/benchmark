//! A simple script to generate and verify the proof of a given program.

use clap::Parser;
use serde_json::Value;
use sp1_sdk::{ProverClient, SP1Stdin};
use std::fs::File;
use std::io::BufReader;

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

    // Read in wasm file from disk
    let wasm =
        include_bytes!("../../../wasm/target/wasm32-unknown-unknown/release/wasm.wasm").to_vec();

    // Read the JSON file
    let file =
        File::open("../../../../inputs/merkle_proof.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    // Extract the number from the JSON
    let input = json["numLeavesBase2"]
        .as_u64()
        .expect("Failed to parse numLeavesBase2 from JSON") as u32;
    println!("Input numLeavesBase2 read from JSON: {}", input);

    let args = Args::parse();
    // Setup the prover client.
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    stdin.write(&wasm);
    stdin.write(&input);

    if args.execute {
        // Execute the program
        let (mut output, report) = client.execute(ELF, stdin).run().unwrap();
        println!("Program executed successfully.");
        let res = output.read::<u32>();
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

        println!("Successfully generated proof!");

        let proof_bytes = proof.bytes();
        println!("Proof Size: {}", proof_bytes.len());

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
