#![no_main]

#[no_mangle]
pub fn nth_prime(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut primes = vec![2];
    let mut candidate = 3;
    while primes.len() < n as usize {
        if primes.iter().all(|&p| candidate % p != 0) {
            primes.push(candidate);
        }
        candidate += 2;
    }
    primes[n as usize - 1]
}

#[jolt::provable(stack_size = 100_000, memory_size = 10_000_000, max_input_size = 10000000)]
fn nth_prime_wrapper(n: u64) -> u64 {
    let p = nth_prime(n);
    println!("The {}th prime is {}.", n, p);
    p
}
