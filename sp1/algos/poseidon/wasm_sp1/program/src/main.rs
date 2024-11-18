//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasmi::{Engine, Linker, Module, Store};

pub fn main() {
    let wasm = sp1_zkvm::io::read::<Vec<u8>>();
    let input = sp1_zkvm::io::read::<u32>();

    println!("cycle-tracker-start: instantiate wasm");
    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");
    println!("cycle-tracker-end: instantiate wasm");

    let linker = <Linker<u32>>::new(&engine);
    let mut store = Store::new(&engine, input.clone());

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    println!("cycle-tracker-start: call wasm");
    let poseidon_hash = instance
        .get_typed_func::<u32, u32>(&mut store, "poseidon_hash")
        .expect("Failed to get typed_func");
    let res = poseidon_hash.call(&mut store, input).expect("Failed to call");
    println!("cycle-tracker-end: call wasm");
    println!("poseidon_hash result: {}", res);

    sp1_zkvm::io::commit(&res);
}
