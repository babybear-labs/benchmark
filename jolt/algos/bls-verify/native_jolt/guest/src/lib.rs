#![no_main]

use wasm::{bls_verify, PublicKey, Signature, hash, Serialize};

#[jolt::provable(stack_size = 1_000_000, memory_size = 1_000_000)]
fn bls_verify_wrapper(public_keys: Vec<Vec<u8>>, aggregate_signature: Vec<u8>) -> u32 {
    let public_keys = public_keys
        .iter()
        .map(|pk| PublicKey::from_bytes(pk).expect("failed to decode public key"))
        .collect::<Vec<_>>();
    let aggregated_signature =
        Signature::from_bytes(&aggregate_signature).expect("failed to decode aggregated signature");
    let message = "message".as_bytes().to_vec();
    let hash = hash(&message);

    let res = bls_verify(aggregated_signature, public_keys, hash);
    println!("cycle-tracker-end: execution");
    res
}
