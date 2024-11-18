//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasmi::{Engine, Linker, Module, Store};

pub fn main() {
    let wasm = sp1_zkvm::io::read::<Vec<u8>>();
    let input = sp1_zkvm::io::read::<u64>();

    let engine = Engine::default();
    println!("cycle-tracker-start: instantiate wasm");
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");
    println!("cycle-tracker-end: instantiate wasm");

    let linker = <Linker<u64>>::new(&engine);
    let mut store = Store::new(&engine, input.clone());

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    println!("cycle-tracker-start: call wasm");
    let nth_prime = instance
        .get_typed_func::<u64, u64>(&mut store, "nth_prime")
        .expect("Failed to get typed_func");
    let res = nth_prime.call(&mut store, input).expect("Failed to call");
    println!("cycle-tracker-end: call wasm");
    println!("The {}th prime is {}.", input, res);

    sp1_zkvm::io::commit(&res);
}
