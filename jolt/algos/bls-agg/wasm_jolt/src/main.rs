use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

pub fn main() {
    // Read in wasm file from disk
    let wasm = include_bytes!("../../wasm/target/wasm32-unknown-unknown/release/wasm.wasm").to_vec();
    // Read the input JSON file
    let file = File::open("../../../inputs/bls_agg.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let input_value = json["numSigners"].as_u64().expect("Failed to parse value from JSON") as u32;
    println!("Input value: {}", input_value);

    let summary = guest::analyze_bls_aggregate(input_value, &wasm);
    println!("Trace length: {:?}", summary.trace_len());
}
