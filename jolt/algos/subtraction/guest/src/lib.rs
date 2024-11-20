#![no_main]

#[jolt::provable]
fn sub(a: u64, b: u64) -> u64 {
    a - b
}