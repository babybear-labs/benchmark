#![no_main]

use wasmi::{Engine, Linker, Module, Store};

#[jolt::provable(stack_size = 100000, memory_size = 10000000, max_input_size = 10000000)]
pub fn bls_aggregate(num_signers: u32, wasm: &[u8]) -> u32 {
    // init Wasm
    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");

    let linker = <Linker<()>>::new(&engine);
    let mut store = Store::new(&engine, ());

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    // Call the merkelize function in the wasm
    let bls_aggregate = instance
        .get_typed_func::<u32, u32>(&mut store, "bls_aggregate")
        .expect("Failed to get typed_func");
    let res = bls_aggregate.call(&mut store, num_signers).expect("Failed to call");
    println!("bls aggregated: {}", res);

    res
}
