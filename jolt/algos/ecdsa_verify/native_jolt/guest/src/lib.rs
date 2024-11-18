#![no_main]

use wasm::ecdsa_verify_n;

#[jolt::provable(stack_size = 1000_000, memory_size = 10000000, max_input_size = 10000000)]
fn ecdsa_verify_n_wrapper(n: u32) {
    ecdsa_verify_n(n);
}
