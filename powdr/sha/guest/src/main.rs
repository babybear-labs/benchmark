use powdr_riscv_runtime;
use powdr_riscv_runtime::io::read;
use sha2::{Sha256, Digest};

fn sha2(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}

fn main() {
    // Any serde-deserializable type can be read from a channel.
    // Read some data from channel 1.
    let data: Vec<u32> = read(1);
    // Read the claimed sum from channel 2.
    let sum: u32 = read(2);

    let input = &[5u8; 5000];
    let a = sha2(input);

    // Check that the claimed sum is correct.
    // assert_eq!(data.iter().sum::<u32>(), sum);
}