use powdr_riscv_runtime;
use powdr_riscv_runtime::io::read;

fn fib(n: u32) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }
    b
}

fn main() {
    // Any serde-deserializable type can be read from a channel.
    // Read some data from channel 1.
    let input: u32 = read(1);
    // Read the claimed sum from channel 2.
    let result: u128 = read(2);

    // Check that the claimed sum is correct.
    let res = fib(input);
    // assert_eq!(, result);
}
