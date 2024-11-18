#![no_main]

use wasm::poseidon_hash;

#[jolt::provable(stack_size = 10_000_000, memory_size = 1_000_000_000)]
fn poseidon_hash_wrapper(n: u32) {
    poseidon_hash(n);
}
