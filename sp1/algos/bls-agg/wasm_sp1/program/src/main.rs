//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasmi::{Engine, Linker, Module, Store};
use bincode;

pub fn main() {
    println!("cycle-tracker-start: read inputs");
    let wasm = sp1_zkvm::io::read::<Vec<u8>>();
    let signers_count = sp1_zkvm::io::read::<u32>();
    println!("cycle-tracker-end: read inputs");

    println!("cycle-tracker-start: instantiate wasm");
    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");

    let mut linker = <Linker<()>>::new(&engine);
    let mut store = Store::new(&engine, ());

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();
    println!("cycle-tracker-end: instantiate wasm");

    println!("cycle-tracker-start: call wasm");
    let bls_aggregate = instance
        .get_typed_func::<u32, u32>(&mut store, "bls_aggregate")
        .expect("Failed to get typed_func");
    let res = bls_aggregate.call(&mut store, signers_count).expect("Failed to call");
    println!("cycle-tracker-end: call wasm");

    sp1_zkvm::io::commit(&res);
}
