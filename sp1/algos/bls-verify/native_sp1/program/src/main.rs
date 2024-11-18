//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasm::{bls_verify, hash, PublicKey, Serialize, Signature};

pub fn main() {
    let aggregate_signature = sp1_zkvm::io::read::<Vec<u8>>();
    let public_keys = sp1_zkvm::io::read::<Vec<Vec<u8>>>();

    println!("cycle-tracker-start: deserialization");
    let public_keys = public_keys
        .iter()
        .map(|pk| PublicKey::from_bytes(pk).expect("failed to decode public key"))
        .collect::<Vec<_>>();
    let aggregated_signature =
        Signature::from_bytes(&aggregate_signature).expect("failed to decode aggregated signature");
    let message = "message".as_bytes().to_vec();
    let hash = hash(&message);

    println!("cycle-tracker-end: deserialization");

    println!("cycle-tracker-start: execution");
    let res = bls_verify(aggregated_signature, public_keys, hash);
    println!("cycle-tracker-end: execution");

    sp1_zkvm::io::commit(&res);
}
