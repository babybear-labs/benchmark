#![no_main]

use wasm::bls_aggregate;

#[jolt::provable(stack_size = 1_000_000, memory_size = 1_000_000_0)]
fn bls_aggregate_wrapper(num_signers: u32) -> u32 {
    bls_aggregate(num_signers)
}
