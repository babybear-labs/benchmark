#![no_main]

use wasm::nth_prime;

#[jolt::provable(stack_size = 100_000, memory_size = 10_000_000, max_input_size = 10000000)]
fn nth_prime_wrapper(n: u64) -> u64 {
    let p = nth_prime(n);
    println!("The {}th prime is {}.", n, p);
    p
}
