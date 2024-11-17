# Nexus

- Run from nexus repo 

## Output

###  Fibonacci
```rust
#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[nexus_rt::main]
fn main() {
    let n = 7;
    let result = fib(n);
    assert_eq!(result, 13);
}
```

- Proving Time
```shell
  Setting up public parameters for IVC ... 35.2s
    Finished in 35.2s
  Loading public parameters ... 6.7s
 Finished in 6.7s
  Computing step 0 ... 3.4s
  Computing step 1 ... 3.9s
  Computing step 2 ... 4.1s
  Computing step 3 ... 3.2s
  Computing step 4 ... 3.1s
  Computing step 5 ... 3.2s
  Computing step 6 ... 3.1s
  Computing step 7 ... 3.2s
  Computing step 8 ... 3.3s
  Computing step 9 ... 3.1s
  Computing step 10 ... 3.0s
  Computing step 11 ... 3.0s
  Computing step 12 ... 3.1s
  Computing step 13 ... 3.0s
  Computing step 14 ... 2.8s
  Computing step 15 ... 2.8s
  Computing step 16 ... 2.8s
  Computing step 17 ... 2.7s
  Computing step 18 ... 2.7s
  Computing step 19 ... 2.8s
  Computing step 20 ... 3.1s
  Computing step 21 ... 3.5s
  Computing step 22 ... 4.0s
  Computing step 23 ... 3.1s
  Computing step 24 ... 3.1s
  Computing step 25 ... 2.9s
  Computing step 26 ... 2.7s
  Computing step 27 ... 3.4s
  Computing step 28 ... 3.3s
  Computing step 29 ... 3.3s
  Saving proof ... 75ms
Finished in 75ms
```

- Proof Size: `47864409 nexus-proof` 47.9 MB

- Verification Time
```shell
  Loading public parameters ... 6.8s
 Finished in 6.8s
  Verifying proof ... 3.1s
   Finished in 3.1s
```


### Factorial
```rust
#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::println;

#[nexus_rt::main]
fn main() {
    fn f(n: u32) -> u32 {
        if n <= 1 {
            1
        } else {
            // n * f(n - 1) would panic if the factorial overflows u32::MAX in debug build,
            // and wrap around in release. Therefore, use built-in checked methods to make
            // the output deterministic.
            n.saturating_mul(f(n - 1))
        }
    }
    let n = 3;
    assert_eq!(f(n), 6);
}
```

- Proving Time
```shell
 Loading public parameters ... 6.9s
 Finished in 6.9s
  Computing step 0 ... 2.5s
  Computing step 1 ... 2.9s
  Computing step 2 ... 2.9s
  Computing step 3 ... 3.0s
  Computing step 4 ... 3.0s
  Computing step 5 ... 3.3s
  Computing step 6 ... 2.9s
  Computing step 7 ... 2.7s
  Computing step 8 ... 3.6s
  Computing step 9 ... 3.5s
  Computing step 10 ... 3.3s
  Computing step 11 ... 3.2s
  Computing step 12 ... 3.4s
  Computing step 13 ... 3.2s
  Saving proof ... 159ms
Finished in 159ms
```
- Size: 47.9 MB
- Verification Time: 
```shell
  Loading public parameters ... 6.6s
 Finished in 6.6s
  Verifying proof ... 2.4s
   Finished in 2.4s
```


### Addition
```rust
#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::println;

#[nexus_rt::main]
fn main() {
    fn f(a: u32, b: u32) -> u32 {
        a + b
    }
    assert_eq!(f(10, 20), 30);
}
```

- Proving Time
```shell
  Loading public parameters ... 6.5s
 Finished in 6.5s
  Computing step 0 ... 2.3s
  Computing step 1 ... 2.5s
  Saving proof ... 150ms
Finished in 150ms
```
- Size: 47.9 MB
- Verification Time
```shell
 Loading public parameters ... 6.7s
 Finished in 6.7s
  Verifying proof ... 1.9s
   Finished in 1.9s
```

### Multiplication
```rust
#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::println;

#[nexus_rt::main]
fn main() {
    fn f(a: u32, b: u32) -> u32 {
        a * b
    }
    assert_eq!(f(10, 20), 200);
}
```

- Proving Time
```
Loading public parameters ... 6.5s
 Finished in 6.5s
  Computing step 0 ... 2.4s
  Computing step 1 ... 2.4s
  Saving proof ... 160ms
Finished in 160ms
```
- Size: 47.9 MB
- Verification Time
```shell
Loading public parameters ... 6.8s
 Finished in 6.8s
  Verifying proof ... 3.5s
   Finished in 3.5s
```