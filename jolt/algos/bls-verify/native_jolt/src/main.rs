use hex;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use wasm::{bls_verify, PrivateKey, PublicKey, Signature, Serialize};

pub fn main() {
    // Read the JSON file
    let file = File::open("../../../inputs/bls_verify.json").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let num_signers = json["numSigners"]
        .as_u64()
        .expect("Failed to parse value from JSON") as u32;
    println!("Input value: {}", num_signers);

    let aggregate_signature = json["aggregateSignature"]
        .as_str()
        .expect("Failed to parse value from JSON");

    let private_keys: Vec<PrivateKey> = (0..num_signers)
        .map(|i| PrivateKey::new(&[i as u8; 32]))
        .collect();

    let public_keys = private_keys
        .iter()
        .map(|pk| pk.public_key().as_bytes().to_vec())
        .collect::<Vec<_>>();

    let summary = guest::analyze_bls_verify_wrapper(
        public_keys,
        hex::decode(aggregate_signature).expect("Failed to decode hex string"),
    );
    println!("Trace length: {:?}", summary.trace_len());
}
