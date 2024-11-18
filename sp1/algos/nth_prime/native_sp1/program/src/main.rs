//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasm::nth_prime;

pub fn main() {
    let input = sp1_zkvm::io::read::<u64>();
    println!("cycle-tracker-start: execution");
    let res = nth_prime(input);
    println!("cycle-tracker-end: execution");
    println!("The {}th prime is {}.", input, res);

    sp1_zkvm::io::commit(&res);
}
