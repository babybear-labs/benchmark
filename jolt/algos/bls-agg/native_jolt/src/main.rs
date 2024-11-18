use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
pub fn main() {
    // Read the JSON file
    let file = File::open("../../../inputs/bls_agg.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let num_signers = json["numSigners"]
        .as_u64()
        .expect("Failed to parse value from JSON") as u32;
    println!("Input value: {}", num_signers);
    
    let summary = guest::analyze_bls_aggregate_wrapper(num_signers);
    println!("Trace length: {:?}", summary.trace_len());

    // let (prove_merkelize_wrapper, verify_merkelize_wrapper) = guest::build_merkelize_wrapper();
    // let (output, proof) = prove_merkelize_wrapper(num_leaves);
    // let is_valid = verify_merkelize_wrapper(proof);

    // println!("output: {}", hex::encode(output));
    // println!("valid: {}", is_valid);
}
