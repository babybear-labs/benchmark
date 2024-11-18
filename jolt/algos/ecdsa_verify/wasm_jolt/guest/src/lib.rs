#![no_main]

use wasmi::{Engine, Linker, Module, Store};

#[jolt::provable(stack_size = 100000, memory_size = 10000000, max_input_size = 10000000)]
fn ecdsa_verify_n_wasm_wrapper(input: u32, wasm: &[u8]) -> u32 {
    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");

    let linker = <Linker<u32>>::new(&engine);
    let mut store = Store::new(&engine, input.clone());

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    let ecdsa_verify_n = instance
        .get_typed_func::<u32, u32>(&mut store, "ecdsa_verify_n")
        .expect("Failed to get typed_func");
    let res = ecdsa_verify_n.call(&mut store, input).expect("Failed to call");
    println!("ecdsa_verify_n result: {}", res);

    res
}
