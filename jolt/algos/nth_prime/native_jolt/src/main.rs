use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use jolt::Serializable;

pub fn main() {
    // Read the input JSON file
    let file = File::open("../../../inputs/nth_prime.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    // Extract the number from the JSON
    let input = json["n"].as_u64().expect("Failed to parse n from JSON");
    println!("Input n read from JSON: {}", input);
    // Call the nth_prime_wrapper function which is jolt::provable
    let start = std::time::Instant::now();
    let summary = guest::analyze_nth_prime_wrapper(input);
    // summary.clone().write_to_file("100th_prime.txt".into()).expect("should write");
    let duration = start.elapsed();
    println!("Trace length: {:?}", summary.trace_len());
    println!("Time elapsed: {:?}", duration);

    // let (prove_nth_prime_wrapper, verify_nth_prime_wrapper) = guest::build_nth_prime_wrapper();
    // let (output, proof) = prove_nth_prime_wrapper(input);
    // proof
    //     .save_to_file("proof.bin")
    //     .expect("Failed to save proof to file");

    // let is_valid = verify_nth_prime_wrapper(proof);

    // println!("output: {:?}", output);
    // println!("valid: {}", is_valid);
}
