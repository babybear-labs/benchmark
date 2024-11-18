use std::fs::File;
use std::io::BufReader;
use serde_json::Value;

pub fn main() {
    // Read the input JSON file
    let file = File::open("../../../inputs/keccak.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    // Extract the number from the JSON
    let input = json["num_iterations"].as_u64().expect("Failed to parse num_iterations from JSON") as u32;
    println!("Input num_iterations read from JSON: {}", input);
    // Call the keccak_n_wrapper function which is jolt::provable
    let start = std::time::Instant::now();
    let summary = guest::analyze_keccak_n_wrapper(input);
    let duration = start.elapsed();
    println!("Trace length: {:?}", summary.trace_len());
    println!("Time elapsed: {:?}", duration);

    // let (prove_keccak_n_wrapper, verify_keccak_n_wrapper) = guest::build_keccak_n_wrapper();
    // let (output, proof) = prove_keccak_n_wrapper(input);
    // let is_valid = verify_keccak_n_wrapper(proof);

    // println!("output: {:?}", output);
    // println!("valid: {}", is_valid);
}
