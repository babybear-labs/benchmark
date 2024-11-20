//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

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

pub fn main() {
    let input = sp1_zkvm::io::read::<u64>();
    println!("cycle-tracker-start: execution");
    let res = nth_prime(input);
    println!("cycle-tracker-end: execution");
    println!("The {}th prime is {}.", input, res);

    sp1_zkvm::io::commit(&res);
}
