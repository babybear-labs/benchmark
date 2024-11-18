#![no_main]

use wasmi::{Engine, Linker, Module, Store};

#[jolt::provable(stack_size = 100_000, memory_size = 10_000_000, max_input_size = 10_000_000)]
pub fn merkelize(num_leaves: i32, wasm: &[u8]) -> i32 {
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
    let merkelize = instance
        .get_typed_func::<i32, i32>(&mut store, "merkelize")
        .expect("Failed to get typed_func");
    let res = merkelize.call(&mut store, num_leaves).expect("Failed to call");
    println!("merkelized: {}", res);

    res
}
