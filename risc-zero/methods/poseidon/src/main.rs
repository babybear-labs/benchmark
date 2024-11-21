#![no_main]
#![no_std]

extern crate alloc;

use core::hint::black_box;

use alloc::str::FromStr;
use risc0_zkvm::guest::env;
use starknet_crypto::poseidon_hash_single;
use starknet_types_core::felt::Felt;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let n: u32 = env::read();
    black_box(bench_poseidon_n(n));
}

fn bench_poseidon_n(n_thousands: u32) -> Felt {
    let felt = Felt::from_str("1").unwrap();
    let mut last_hash = felt;
    
    for _ in 0..(n_thousands * 1) {
        last_hash = poseidon_hash_single(felt);
    }
    
    last_hash
}