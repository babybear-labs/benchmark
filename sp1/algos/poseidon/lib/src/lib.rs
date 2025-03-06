use alloy_sol_types::sol;
use starknet_crypto::{PoseidonHasher};
use starknet_types_core::felt::Felt;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint32 n;
        uint32 a;
        uint32 b;
    }
}

/// Converts an arbitrary byte slice into `Felt` elements.
fn bytes_to_felts(input: &[u8]) -> Vec<Felt> {
    const FELT_BYTE_SIZE: usize = 31; // Maximum bytes for a Felt element in BN254

    input
        .chunks(FELT_BYTE_SIZE)
        .map(|chunk| {
            let mut buffer = [0u8; 32]; // BN254 requires 32 bytes, pad with zeroes
            buffer[32 - chunk.len()..].copy_from_slice(chunk); // Right-align the chunk
            Felt::from_bytes_be(&buffer)
        })
        .collect()
}

/// Ref: https://github.com/xJonathanLEI/starknet-rs/blob/master/starknet-crypto/benches/poseidon_hash.rs
#[no_mangle]
pub fn pos() {
    let input = &[5u8; 32];
    // Convert input into `Felt` chunks
    let felt_chunks = bytes_to_felts(input);

    let mut hasher = PoseidonHasher::new();
    for chunk in &felt_chunks {
        hasher.update(*chunk);
    }
    let _hash = hasher.finalize();
}