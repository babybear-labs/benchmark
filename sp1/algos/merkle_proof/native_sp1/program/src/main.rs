//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasm::generate_merkle_proof;

pub fn main() {
    let input = sp1_zkvm::io::read::<u32>();
    println!("cycle-tracker-start: execution");
    let res = generate_merkle_proof(input);
    println!("cycle-tracker-end: execution");
    println!("merkle_proof result: {}", res);

    sp1_zkvm::io::commit(&res);
}
