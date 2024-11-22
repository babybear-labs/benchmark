# Jolt

Hardware
- Macbook Pro M2 Pro - Core 10 - Memory 16 GB

## Addition
```rust
fn add(a: u64, b: u64) -> u64 {
    a + b
}
```
#### Result

```
Trace length: 141
Prover Time 31.297901583s
Proof Size 266277

Verify Time 45.633542ms

result: 30
valid: true
Total Time elapsed: 31.505612833s
```


## Subtraction
```rust
fn mul(a: u64, b: u64) -> u64 {
    a - b
}
```

#### Result
```shell
Trace length: 141
Prover Time 30.57905825s
Proof Size 266277

Verify Time 38.961667ms

result: 90
valid: true
Total Time elapsed: 30.735463166s
```

## Multiplication
```rust
fn mul(a: u64, b: u64) -> u64 {
    a * b
}
```

#### Result

```shell
Trace length: 157
Prover Time 36.085309041s
Proof Size 266278

Verify Time 42.617333ms

result: 200
valid: true
Total Time elapsed: 36.249379542s

```

## Division

```rust
fn div(a: u64, b: u64) -> u64 {
    a / b
}
```

#### Result
```shell
Trace length: 191
Prover Time 23.293768375s
Proof Size 267469

Verify Time 43.647ms

result: 4
valid: true
Total Time elapsed: 23.472635375s
```

## Fibonacci
> variants: 1, 10, 100, 1000, 10000, 100000

```rust
fn fib(n: u64) -> u128 {
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
```

#### Result

- Input: 1
```shell
Input n read from JSON: 1

Trace length: 102
Prover Time 22.956921625s
Proof Size 251908

Verify Time 36.636334ms

result: 1
valid: true
Total Time elapsed: 23.139271375s
```


- Input: 10
```shell
Input n read from JSON: 10

Trace length: 369
Prover Time 21.068039375s
Proof Size 281060

Verify Time 40.597584ms

result: 55
valid: true
Total Time elapsed: 21.249532792s
```

- Input: 100
```shell
Input n read from JSON: 100

Trace length: 3021
Prover Time 21.102921209s
Proof Size 327917

Verify Time 43.000709ms

result: 354224848179261915075
valid: true
Total Time elapsed: 21.297147917s
```

- Input: 1000
```shell
Input n read from JSON: 1000

Trace length: 28280
Prover Time 21.951744708s
Proof Size 378519

Verify Time 50.479167ms

result: 101760851154547862183199185335023067211
valid: true
Total Time elapsed: 22.184175s
```

- Input: 10000
```shell
Input n read from JSON: 10000

Trace length: 280287
Prover Time 36.798820375s
Proof Size 452398

Verify Time 59.920792ms

result: 45370814669650777014160265265247951323
valid: true
Total Time elapsed: 37.046944542s
```


- Input: 100000
```shell
Input n read from JSON: 100000

Trace length: 2800285
zsh: killed     cargo run --release
```

## SHA256
> variants: 32b, 1k bytes, 2k bytes, 3k bytes, 4k bytes, 5k bytes, 10k bytes
```rust
fn sha2(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}
```

#### Result
- Input Size: 32 Bytes
```shell
Trace length: 4662
Prover Time 34.90987475s
Proof Size 348307

Verify Time 51.032167ms

result: [248, 73, 214, 115, 37, 250, 207, 4, 23, 123, 198, 99, 178, 220, 84, 64, 81, 131, 28, 88, 158, 245, 129, 212, 18, 242, 235, 164, 72, 52, 231, 124]
valid: true
Total Time elapsed: 35.1319985s

```

- Input Size: 1k Bytes
```shell
Trace length: 62231
Prover Time 26.391225s
Proof Size 401116

Verify Time 54.102875ms

result: [16, 148, 78, 213, 22, 241, 187, 1, 208, 96, 241, 147, 59, 128, 182, 120, 151, 152, 55, 88, 164, 63, 92, 40, 20, 4, 193, 51, 240, 220, 127, 117]
valid: true
Total Time elapsed: 26.644473791s
```

- Input Size: 2k bytes
```shell
Trace length: 123315
Prover Time 26.650922291s
Proof Size 420428

Verify Time 53.676917ms

result: [13, 169, 211, 116, 216, 227, 44, 229, 184, 96, 192, 190, 38, 207, 117, 159, 67, 205, 78, 230, 6, 68, 110, 253, 54, 113, 138, 126, 199, 187, 253, 238]
valid: true
Total Time elapsed: 26.912961083s
```

- Input Size: 3k bytes
```shell
Trace length: 184823
Prover Time 32.895957125s
Proof Size 440156

Verify Time 61.172083ms

result: [39, 33, 95, 58, 253, 215, 82, 90, 235, 249, 141, 230, 154, 134, 99, 66, 87, 243, 128, 247, 93, 211, 31, 74, 167, 67, 237, 143, 255, 92, 3, 228]
valid: true
Total Time elapsed: 33.181559083s
```

- Input Size: 4k bytes
```shell
Trace length: 241759
Prover Time 35.158499833s
Proof Size 441156

Verify Time 68.891166ms

result: [110, 240, 21, 30, 168, 157, 239, 3, 73, 170, 177, 3, 12, 39, 222, 163, 59, 229, 65, 43, 197, 52, 153, 177, 162, 166, 232, 35, 180, 22, 95, 181]
valid: true
Total Time elapsed: 35.458870084s
```

- Input Size: 5k Bytes
```shell
Trace length: 175
thread 'main' panicked at /Users/muhtasim/.cargo/git/checkouts/jolt-6b856340b98daf0c/2e9002d/jolt-core/src/jolt/vm/read_write_memory.rs:251:9:
assertion failed: program_io.inputs.len() <= program_io.memory_layout.max_input_size as usize
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

- Input Size: 10k Bytes
```shell
Trace length: 175
thread 'main' panicked at /Users/muhtasim/.cargo/git/checkouts/jolt-6b856340b98daf0c/2e9002d/jolt-core/src/jolt/vm/read_write_memory.rs:251:9:
assertion failed: program_io.inputs.len() <= program_io.memory_layout.max_input_size as usize
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Poseidon
```rust
use starknet_crypto::{PoseidonHasher};
use starknet_types_core::felt::Felt;

/// Converts an arbitrary byte slice into `Felt` elements.
fn bytes_to_felts(input: &[u8]) -> Vec<Felt> {
    const FELT_BYTE_SIZE: usize = 31; // Maximum bytes for a Felt element in BN254

    input
        .chunks(FELT_BYTE_SIZE)
        .map(|chunk| {
            let mut buffer = [0u8; 32]; // BN254 requires 32 bytes, pad with zeroes
            buffer[32 - chunk.len()..].copy_from_slice(chunk); // Right-align the chunk
            Felt::from_bytes_be(&buffer)
        })
        .collect()
}

/// Ref: https://github.com/xJonathanLEI/starknet-rs/blob/master/starknet-crypto/benches/poseidon_hash.rs
#[no_mangle]
#[jolt::provable]
pub fn pos() {
    let input = &[5u8; 32];
    // Convert input into `Felt` chunks
    let felt_chunks = bytes_to_felts(input);

    let mut hasher = PoseidonHasher::new();
    for chunk in &felt_chunks {
        hasher.update(*chunk);
    }
    let hash = hasher.finalize();
}
``` 

#### Result
- Input Size: 32 bytes
```shell
Trace length: 554595
Prover Time 434.08379375s
Proof Size 477746

Verify Time 238.556041ms

result: ()
valid: true
Total Time elapsed: 434.536444333s
```
