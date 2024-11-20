#![no_main]

#[jolt::provable]
fn for_loop(n: u64) {
    for i in 0..n { }
}