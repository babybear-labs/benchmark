//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasm::ecdsa_verify_n;

pub fn main() {
    let input = sp1_zkvm::io::read::<u32>();
    println!("cycle-tracker-start: execution");
    let res = ecdsa_verify_n(input);
    println!("cycle-tracker-end: execution");
    println!("ecdsa_verify_n result: {}", res);

    sp1_zkvm::io::commit(&res);
}
