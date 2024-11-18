//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasm::bls_aggregate;

pub fn main() {
    let num_signers = sp1_zkvm::io::read::<u32>();
    println!("cycle-tracker-start: execution");

    let res = bls_aggregate(num_signers);
    println!("cycle-tracker-end: execution");

    sp1_zkvm::io::commit(&res);
}
