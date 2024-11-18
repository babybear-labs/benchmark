#![no_main]

use wasm::keccak_n;

#[jolt::provable(stack_size = 100000, memory_size = 10000000, max_input_size = 10000000)]
fn keccak_n_wrapper(n: u32) {
    keccak_n(n);
}
