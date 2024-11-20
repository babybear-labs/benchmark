use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use jolt::Serializable;

pub fn main() {
    let file = File::open("../../inputs/data.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let n = json["n"].as_u64().expect("Failed to parse n from JSON");
    println!("Input n read from JSON: {}", n);

    let start = std::time::Instant::now();
    // let prover_execution_trace = guest::analyze_fib(n);
    // prover_execution_trace.clone().write_to_file("trace.txt".into()).expect("should write");
    // let prover_execution_trace_duration = start.elapsed();
    // println!("Trace length: {:?}", prover_execution_trace.trace_len());
    // println!("Prover Execution Time {:?}", prover_execution_trace_duration);
    println!();

    let proving_time = std::time::Instant::now();
    let (prove_fib, verify_fib) = guest::build_fib();
    let (output, proof) = prove_fib(n);
    println!("Prover Time {:?}", proving_time.elapsed());

    proof.save_to_file("proof.bin").expect("Failed to save proof to file");
    println!("Proof Size {:?}", proof.size().unwrap());
    println!();

    let verify_time = std::time::Instant::now();
    let is_valid = verify_fib(proof);
    println!("Verify Time {:?}", verify_time.elapsed());
    println!();

    println!("result: {:?}", output);
    println!("valid: {}", is_valid);

    println!("Total Time elapsed: {:?}", start.elapsed());
}
