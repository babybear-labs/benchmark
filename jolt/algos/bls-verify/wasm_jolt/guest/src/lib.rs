#![no_main]

use wasmi::{Engine, Linker, Module, Store};
use wasmi::core::Pages;
use serde_cbor;
use wasm::Inputs;

const PAGE_SIZE: u32 = 65536;

#[jolt::provable(stack_size = 1000000, memory_size = 10000000, max_input_size = 10000000)]
pub fn bls_verify(public_keys: Vec<Vec<u8>>, aggregate_signature: &[u8], wasm: &[u8]) -> u32 {
    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");

    let linker = <Linker<()>>::new(&engine);
    let mut store = Store::new(&engine, ());

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    // write list to memory
    let memory = instance
        .get_memory(&store, "memory")
        .expect("Failed to get memory");
    let ptr = u32::from(memory.current_pages(&mut store)) * PAGE_SIZE;
    // grow memory to fit the block
    let inputs = Inputs {
        aggregated_signature: aggregate_signature.to_vec(),
        public_keys: public_keys,
    };
    let serialized_inputs = serde_cbor::to_vec(&inputs).expect("Failed to serialize inputs");
    let inputs_len = serialized_inputs.len();
    let pages_to_grow = (inputs_len as u32 + PAGE_SIZE - 1) / PAGE_SIZE;
    memory
        .grow(
            &mut store,
            Pages::new(pages_to_grow).expect("Failed to grow memory"),
        )
        .expect("Failed to grow memory");
    memory
        .write(&mut store, ptr as usize, &serialized_inputs)
        .expect("Failed to write to memory");
    let bls_verify = instance
        .get_typed_func::<(i32, i32), u32>(&mut store, "bls_verify_wasm")
        .expect("Failed to get typed_func");
    let res = bls_verify
        .call(&mut store, (ptr as i32, inputs_len as i32))
        .expect("Failed to call");
    println!("bls_verify - {}", res);
    res
}
