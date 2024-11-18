//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasm::poseidon_hash;

pub fn main() {
    let input = sp1_zkvm::io::read::<u32>();
    println!("cycle-tracker-start: execution");
    let res = poseidon_hash(input);
    println!("cycle-tracker-end: execution");
    println!("poseidon_hash result: {}", res);

    sp1_zkvm::io::commit(&res);
}
