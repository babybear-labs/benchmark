#![no_main]

use wasmi::{Engine, Linker, Module, Store};

#[jolt::provable(stack_size = 100000, memory_size = 10000000, max_input_size = 10000000)]
fn nth_prime_wasm_wrapper(input: u64, wasm: &[u8]) -> u64 {
    let engine = Engine::default();
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");

    let linker = <Linker<u64>>::new(&engine);
    let mut store = Store::new(&engine, input.clone());

    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();

    let nth_prime = instance
        .get_typed_func::<u64, u64>(&mut store, "nth_prime")
        .expect("Failed to get typed_func");
    let res = nth_prime.call(&mut store, input).expect("Failed to call");
    println!("The {}th prime is {}.", input, res);

    res
}
