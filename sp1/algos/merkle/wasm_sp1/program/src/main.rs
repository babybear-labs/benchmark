//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasmi::{Engine, Linker, Module, Store};
use bincode;

pub fn main() {
    println!("cycle-tracker-start: read inputs");
    let wasm = sp1_zkvm::io::read::<Vec<u8>>();
    let leaves_count = sp1_zkvm::io::read::<i32>();
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
    let merklize = instance
        .get_typed_func::<i32, i32>(&mut store, "merkelize")
        .expect("Failed to get typed_func");
    let res = merklize.call(&mut store, leaves_count).expect("Failed to call");
    println!("cycle-tracker-end: call wasm");

    sp1_zkvm::io::commit(&res);
}
