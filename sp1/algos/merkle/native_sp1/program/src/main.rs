//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasm::merkelize;

pub fn main() {
    let leaves_count = sp1_zkvm::io::read::<i32>();
    println!("cycle-tracker-start: execution");

    let res = merkelize(leaves_count);
    println!("cycle-tracker-end: execution");

    sp1_zkvm::io::commit(&res);
}
