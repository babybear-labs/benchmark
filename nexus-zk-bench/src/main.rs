#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::println;

#[nexus_rt::main]
fn main() {
    fn f(a: u32, b: u32) -> u32 {
        a * b
    }
    assert_eq!(f(10, 20), 200);
}