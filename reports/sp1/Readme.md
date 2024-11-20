# SP1

## Fibonacci

```rust
pub fn fibonacci(n: u32) -> (u32, u32) {
    let mut a = 0u32;
    let mut b = 1u32;
    for _ in 0..n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    (a, b)
}
```

### Result

- Input 20

```shell
n: 20
2024-11-20T15:47:18.254508Z  INFO prove_core:clone: close time.busy=131µs time.idle=1.21µs
2024-11-20T15:47:18.256564Z  INFO prove_core:execute: close time.busy=1.70ms time.idle=1.33µs
2024-11-20T15:47:18.257044Z  INFO prove_core:create memory checkpoint: close time.busy=462µs time.idle=875ns
2024-11-20T15:47:18.270350Z  INFO prove_core: generated 1 records    
2024-11-20T15:47:18.270372Z  INFO prove_core: waiting for turn 0    
2024-11-20T15:47:18.270448Z  INFO prove_core: deferred 1 records    
2024-11-20T15:47:18.270452Z  INFO prove_core: collecting checkpoints    
2024-11-20T15:47:18.270457Z  INFO prove_core: fixing shape
2024-11-20T15:47:18.270495Z  INFO prove_core: fixing shape
2024-11-20T15:47:19.604640Z  INFO prove_core: generated 1 records    
2024-11-20T15:47:19.604733Z  INFO prove_core: deferred 1 records    
2024-11-20T15:47:37.268429Z  INFO prove_core: execution report (totals): total_cycles=9221, total_syscall_cycles=20, touched_memory_addresses=15034
2024-11-20T15:47:37.268466Z  INFO prove_core: execution report (opcode counts):
2024-11-20T15:47:37.268479Z  INFO prove_core:   1919 add
2024-11-20T15:47:37.268482Z  INFO prove_core:   1410 xor
2024-11-20T15:47:37.268484Z  INFO prove_core:   1373 srl
2024-11-20T15:47:37.268486Z  INFO prove_core:   1291 or
2024-11-20T15:47:37.268489Z  INFO prove_core:   1278 sll
2024-11-20T15:47:37.268491Z  INFO prove_core:    594 lw
2024-11-20T15:47:37.268493Z  INFO prove_core:    499 sw
2024-11-20T15:47:37.268496Z  INFO prove_core:    445 and
2024-11-20T15:47:37.268498Z  INFO prove_core:    137 lbu
2024-11-20T15:47:37.268500Z  INFO prove_core:     54 sb
2024-11-20T15:47:37.268539Z  INFO prove_core:     47 bltu
2024-11-20T15:47:37.268543Z  INFO prove_core:     39 jalr
2024-11-20T15:47:37.268545Z  INFO prove_core:     35 bne
2024-11-20T15:47:37.268548Z  INFO prove_core:     24 auipc
2024-11-20T15:47:37.268550Z  INFO prove_core:     20 ecall
2024-11-20T15:47:37.268552Z  INFO prove_core:     19 sub
2024-11-20T15:47:37.268555Z  INFO prove_core:     19 beq
2024-11-20T15:47:37.268557Z  INFO prove_core:      8 sltu
2024-11-20T15:47:37.268559Z  INFO prove_core:      5 bgeu
2024-11-20T15:47:37.268562Z  INFO prove_core:      5 mul
2024-11-20T15:47:37.268564Z  INFO prove_core:      0 sra
2024-11-20T15:47:37.268566Z  INFO prove_core:      0 slt
2024-11-20T15:47:37.268628Z  INFO prove_core:      0 lb
2024-11-20T15:47:37.268631Z  INFO prove_core:      0 lh
2024-11-20T15:47:37.268634Z  INFO prove_core:      0 lhu
2024-11-20T15:47:37.268636Z  INFO prove_core:      0 sh
2024-11-20T15:47:37.268638Z  INFO prove_core:      0 blt
2024-11-20T15:47:37.268640Z  INFO prove_core:      0 bge
2024-11-20T15:47:37.268643Z  INFO prove_core:      0 jal
2024-11-20T15:47:37.268645Z  INFO prove_core:      0 ebreak
2024-11-20T15:47:37.268648Z  INFO prove_core:      0 mulh
2024-11-20T15:47:37.268650Z  INFO prove_core:      0 mulhu
2024-11-20T15:47:37.268653Z  INFO prove_core:      0 mulhsu
2024-11-20T15:47:37.268656Z  INFO prove_core:      0 div
2024-11-20T15:47:37.268658Z  INFO prove_core:      0 divu
2024-11-20T15:47:37.268661Z  INFO prove_core:      0 rem
2024-11-20T15:47:37.268663Z  INFO prove_core:      0 remu
2024-11-20T15:47:37.268665Z  INFO prove_core:      0 unimp
2024-11-20T15:47:37.268668Z  INFO prove_core: execution report (syscall counts):
2024-11-20T15:47:37.268673Z  INFO prove_core:   8 commit
2024-11-20T15:47:37.268700Z  INFO prove_core:   8 commit_deferred_proofs
2024-11-20T15:47:37.268703Z  INFO prove_core:   1 halt
2024-11-20T15:47:37.268705Z  INFO prove_core:   1 write
2024-11-20T15:47:37.268708Z  INFO prove_core:   1 hint_len
2024-11-20T15:47:37.268710Z  INFO prove_core:   1 hint_read
2024-11-20T15:47:37.268713Z  INFO prove_core:   0 enter_unconstrained
2024-11-20T15:47:37.268715Z  INFO prove_core:   0 exit_unconstrained
2024-11-20T15:47:37.268718Z  INFO prove_core:   0 verify_sp1_proof
2024-11-20T15:47:37.268721Z  INFO prove_core:   0 ed_decompress
2024-11-20T15:47:37.268723Z  INFO prove_core:   0 secp256k1_double
2024-11-20T15:47:37.268726Z  INFO prove_core:   0 secp256k1_decompress
2024-11-20T15:47:37.268729Z  INFO prove_core:   0 bn254_double
2024-11-20T15:47:37.268731Z  INFO prove_core:   0 bls12381_decompress
2024-11-20T15:47:37.268734Z  INFO prove_core:   0 bls12381_double
2024-11-20T15:47:37.268737Z  INFO prove_core:   0 sha_compress
2024-11-20T15:47:37.268740Z  INFO prove_core:   0 ed_add
2024-11-20T15:47:37.268742Z  INFO prove_core:   0 keccak_permute
2024-11-20T15:47:37.268745Z  INFO prove_core:   0 secp256k1_add
2024-11-20T15:47:37.268748Z  INFO prove_core:   0 bn254_add
2024-11-20T15:47:37.268751Z  INFO prove_core:   0 uint256_mul
2024-11-20T15:47:37.268753Z  INFO prove_core:   0 bls12381_add
2024-11-20T15:47:37.268756Z  INFO prove_core:   0 bls12381_fp_add
2024-11-20T15:47:37.268758Z  INFO prove_core:   0 bls12381_fp_sub
2024-11-20T15:47:37.268761Z  INFO prove_core:   0 bls12381_fp_mul
2024-11-20T15:47:37.268763Z  INFO prove_core:   0 bls12381_fp2_add
2024-11-20T15:47:37.268766Z  INFO prove_core:   0 bls12381_fp2_sub
2024-11-20T15:47:37.268768Z  INFO prove_core:   0 bls12381_fp2_mul
2024-11-20T15:47:37.268771Z  INFO prove_core:   0 bn254_fp_add
2024-11-20T15:47:37.268773Z  INFO prove_core:   0 bn254_fp_sub
2024-11-20T15:47:37.268776Z  INFO prove_core:   0 bn254_fp_mul
2024-11-20T15:47:37.268819Z  INFO prove_core:   0 bn254_fp2_add
2024-11-20T15:47:37.268830Z  INFO prove_core:   0 bn254_fp2_sub
2024-11-20T15:47:37.268833Z  INFO prove_core:   0 bn254_fp2_mul
2024-11-20T15:47:37.268836Z  INFO prove_core:   0 sha_extend
2024-11-20T15:47:37.806125Z  INFO prove_core: summary: cycles=9221, gas=35001, e2e=19.015414541s, khz=0.48, proofSize=2656912
2024-11-20T15:47:37.819252Z  INFO prove_core: close time.busy=19.5s time.idle=115ms
Successfully generated proof!
2024-11-20T15:47:38.102612Z  INFO verify: close time.busy=282ms time.idle=3.46µs
```