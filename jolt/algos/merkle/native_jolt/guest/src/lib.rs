// #![cfg_attr(feature = "guest")]
#![no_main]

use wasm::merkelize;

#[jolt::provable(stack_size = 10000000)]
fn merkelize_wrapper(leaves_count: i32) -> u32 {
    merkelize(leaves_count)
}

