#![no_main]

use wasm::generate_merkle_proof;

#[jolt::provable(stack_size = 1000000, memory_size = 10000000, max_input_size = 10000000)]
fn generate_merkle_proof_wrapper(n: u32) {
    generate_merkle_proof(n);
}
