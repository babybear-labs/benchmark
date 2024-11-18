use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use jolt::Serializable;

pub fn main() {
    // Read in wasm file from disk
    let wasm =
        include_bytes!("../../wasm/target/wasm32-unknown-unknown/release/wasm.wasm").to_vec();
    // Read the input JSON file
    let file = File::open("../../../inputs/merkle.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let input_value = json["numLeavesBase2"]
        .as_i64()
        .expect("Failed to parse value from JSON") as i32;
    println!("Input value: {}", input_value);

    // let summary = guest::analyze_merkelize(input_value, &wasm);
    // println!("Trace length: {:?}", summary.trace_len());

    let (prove_merkelize, verify_merkelize) = guest::build_merkelize();
    let start = std::time::Instant::now();
    let (output, proof) = prove_merkelize(input_value, &wasm);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    proof
        .save_to_file("proof.bin")
        .expect("Failed to save proof to file");
    let is_valid = verify_merkelize(proof);

    println!("output: {:?}", output);
    println!("valid: {}", is_valid);
}
