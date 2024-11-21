use powdr_riscv_runtime;
use powdr_riscv_runtime::io::read;
use starknet_crypto::{PoseidonHasher};
use starknet_types_core::felt::Felt;

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
pub fn poseidon() {
    let input = &[5u8; 1000];
    // Convert input into `Felt` chunks
    let felt_chunks = bytes_to_felts(input);

    let mut hasher = PoseidonHasher::new();
    for chunk in &felt_chunks {
        hasher.update(*chunk);
    }
    let hash = hasher.finalize();
}

fn main() {
    // Any serde-deserializable type can be read from a channel.
    // Read some data from channel 1.
    let data: Vec<u32> = read(1);
    // Read the claimed sum from channel 2.
    let sum: u32 = read(2);

    poseidon();
    // Check that the claimed sum is correct.
    // assert_eq!(data.iter().sum::<u32>(), sum);
}
