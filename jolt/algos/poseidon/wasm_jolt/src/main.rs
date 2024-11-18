use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

pub fn main() {
    // Read the input JSON file
    let file = File::open("../../../inputs/poseidon.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let wasm = include_bytes!("../../wasm/target/wasm32-unknown-unknown/release/wasm.wasm").to_vec();
    // Extract the number from the JSON
    let input = json["num_iterations"].as_u64().expect("Failed to parse num_iterations from JSON") as u32;
    println!("Input num_iterations read from JSON: {}", input);
    // Call the poseidon_hash_wasm_wrapper function which is jolt::provable
    let start = std::time::Instant::now();
    let summary = guest::analyze_poseidon_hash_wasm_wrapper(input, &wasm);
    let duration = start.elapsed();
    println!("Trace length: {:?}", summary.trace_len());
    println!("Time elapsed: {:?}", duration);

    // let (prove_poseidon_hash_wasm_wrapper, verify_poseidon_hash_wasm_wrapper) = guest::build_poseidon_hash_wasm_wrapper();
    // let (output, proof) = prove_poseidon_hash_wasm_wrapper(input, &wasm);
    // let is_valid = verify_poseidon_hash_wasm_wrapper(proof);

    // println!("output: {:?}", output);
    // println!("valid: {}", is_valid);
}
