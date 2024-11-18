//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasm::keccak_n;

pub fn main() {
    let input = sp1_zkvm::io::read::<u32>();
    println!("cycle-tracker-start: execution");
    let res = keccak_n(input);
    println!("cycle-tracker-end: execution");
    println!("keccak_n result: {}", res);

    sp1_zkvm::io::commit(&res);
}
