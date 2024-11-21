# Powdr

## Fibonacci
```rust
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
```

### Result
- Input 1
```shell
Running powdr-riscv executor in fast mode...
Fast executor took: 322.353291ms
Trace length: 2861
Loading program ZK setup.
Running witness and proof generation for 1 chunks...
[00:00:04 (ETA: 00:00:00)] ████████████████████ 100% - 128667 rows/s, 9392k identities/s, 100% progressGenerating proof...
Proof generation took: 4.351545417s
```

- Input 10
```shell
Running powdr-riscv executor in fast mode...
Fast executor took: 314.574625ms
Trace length: 2861
Loading program ZK setup.
Running witness and proof generation for 1 chunks...
[00:00:05 (ETA: 00:00:00)] ████████████████████ 100% - 129299 rows/s, 9438k identities/s, 100% progressGenerating proof...
Proof generation took: 4.53269075s
```

- Input 100
```shell
Running powdr-riscv executor in fast mode...
Fast executor took: 317.027167ms
Trace length: 2903
Creating program ZK setup. This has to be done only once per program.
Running witness and proof generation for 1 chunks...
[00:00:06 (ETA: 00:00:00)] ████████████████████ 100% - 126871 rows/s, 9261k identities/s, 100% progressGenerating proof...
Proof generation took: 4.686578958s
```

- Input 1000
```shell
Running powdr-riscv executor in fast mode...
Fast executor took: 373.51425ms
Trace length: 2990
Loading program ZK setup.
Running witness and proof generation for 1 chunks...
[00:00:05 (ETA: 00:00:00)] ████████████████████ 100% - 125172 rows/s, 9137k identities/s, 100% progressGenerating proof...
Proof generation took: 4.856331542s
```


- Input 10000
```shell
Running powdr-riscv executor in fast mode...
Fast executor took: 320.073541ms
Trace length: 2990
Loading program ZK setup.
Running witness and proof generation for 1 chunks...
[00:00:04 (ETA: 00:00:00)] ████████████████████ 100% - 127210 rows/s, 9286k identities/s, 100% progressGenerating proof...
Proof generation took: 8.6471005s
```

- Input 100000
```shell
Running powdr-riscv executor in fast mode...
Fast executor took: 347.174625ms
Trace length: 3040
Loading program ZK setup.
Running witness and proof generation for 1 chunks...
[00:00:04 (ETA: 00:00:00)] ████████████████████ 100% - 121862 rows/s, 8895k identities/s, 100% progressGenerating proof...
Proof generation took: 9.044389709s
```

## SHA256
```rust
fn sha2(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Into::<[u8; 32]>::into(result)
}
```

- Input 32 Bytes
```shell
Running powdr-riscv executor in fast mode...
Fast executor took: 361.698625ms
Trace length: 8255
Creating program ZK setup. This has to be done only once per program.
Running witness and proof generation for 1 chunks...
[00:00:04 (ETA: 00:00:00)] ████████████████████ 100% - 131337 rows/s, 9587k identities/s, 100% progressGenerating proof...
Proof generation took: 12.130208666s=
```

- Input 1k Bytes
```shell
Running powdr-riscv executor in fast mode...
Fast executor took: 389.0595ms
Trace length: 73731
Creating program ZK setup. This has to be done only once per program.
Running witness and proof generation for 1 chunks...
[00:00:08 (ETA: 00:00:00)] ████████████████████ 100% - 125203 rows/s, 9139k identities/s, 100% progressGenerating proof...
Proof generation took: 9.071026708s
```

- Input 5k Bytes
```shell
Running powdr-riscv executor in fast mode...
Fast executor took: 484.632584ms
Trace length: 347165
Creating program ZK setup. This has to be done only once per program.
Running witness and proof generation for 2 chunks...
[00:00:31 (ETA: 00:00:00)] ████████████████████ 100% - 9131 rows/s, 773k identities/s, 92% progressGenerating proof...
Proof generation took: 36.286408167s
[00:00:13 (ETA: 00:00:00)] ████████████████████ 100% - 120656 rows/s, 8807k identities/s, 100% progressGenerating proof...
Proof generation took: 14.808813125s
```

- Input 10k Bytes
```shell
Running powdr-riscv executor in fast mode...
Fast executor took: 604.942042ms
Trace length: 685802
Creating program ZK setup. This has to be done only once per program.
Running witness and proof generation for 3 chunks...
[00:00:24 (ETA: 00:00:00)] ████████████████████ 100% - 12718 rows/s, 1082k identities/s, 92% progressGenerating proof...
Proof generation took: 14.953730666s
[00:00:17 (ETA: 00:00:00)] ████████████████████ 100% - 5848 rows/s, 485k identities/s, 91% progressGenerating proof...
Proof generation took: 15.406207208s
[00:00:16 (ETA: 00:00:00)] ████████████████████ 100% - 127210 rows/s, 9286k identities/s, 100% progressGenerating proof...
Proof generation took: 15.174332083s
```

## Merkle Membership Proof
