use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use jolt::Serializable;

pub fn main() {
    // Read the JSON file
    let file = File::open("../../../inputs/merkle.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let num_leaves = json["numLeavesBase2"]
        .as_i64()
        .expect("Failed to parse value from JSON") as i32;
    println!("Input value: {}", num_leaves);

    // let start = std::time::Instant::now();
    // let summary = guest::analyze_merkelize_wrapper(num_leaves);

    // let duration = start.elapsed();
    // println!("Trace length: {:?}", summary.trace_len());
    // println!("Time elapsed: {:?}", duration);
    
    let (prove_merkelize_wrapper, verify_merkelize_wrapper) = guest::build_merkelize_wrapper();
    let start = std::time::Instant::now();
    let (output, proof) = prove_merkelize_wrapper(num_leaves);
    let duration = start.elapsed();
    println!("Time taken to prove execution: {:.2} seconds", duration.as_secs_f64());
    proof
        .save_to_file("proof.bin")
        .expect("Failed to save proof to file");

    let is_valid = verify_merkelize_wrapper(proof);

    // println!("output: {}", hex::encode(output));
    // println!("valid: {}", is_valid);
}
