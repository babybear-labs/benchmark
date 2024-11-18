//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use wasmi::{Engine, Linker, Module, Store};
use wasm::Inputs;
use serde_cbor;
const PAGE_SIZE: u32 = 65536;

pub fn main() {
    println!("cycle-tracker-start: read inputs");
    let wasm = sp1_zkvm::io::read::<Vec<u8>>();
    let aggregated_signature = sp1_zkvm::io::read::<Vec<u8>>();
    let public_keys = sp1_zkvm::io::read::<Vec<Vec<u8>>>();
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

    // write list to memory
    let memory = instance.get_memory(&store,"memory").expect("Failed to get memory");
    let ptr = memory.data_size(&mut store) as i32;

    // grow memory to fit the block
    let inputs = Inputs {
        aggregated_signature: aggregated_signature,
        public_keys: public_keys,
    };
    let serialized_inputs = serde_cbor::to_vec(&inputs).expect("Failed to serialize inputs");
    let inputs_len = serialized_inputs.len();
    let memory_size = (inputs_len as u32 + PAGE_SIZE - 1) / PAGE_SIZE; // round up to the nearest 64KiB page range
    memory.grow(&mut store, memory_size).expect("Failed to grow memory");
    memory.write(&mut store, ptr as usize, &serialized_inputs).expect("Failed to write to memory");
    println!("cycle-tracker-end: instantiate wasm");

    println!("cycle-tracker-start: call wasm");
    let bls_verify = instance
        .get_typed_func::<(i32, i32), u32>(&mut store, "bls_verify_wasm")
        .expect("Failed to get typed_func");
    let res = bls_verify.call(&mut store, (ptr, inputs_len as i32)).expect("Failed to call");
    println!("cycle-tracker-end: call wasm");
    println!("bls_verify - {}", res);

    sp1_zkvm::io::commit(&res);
}
