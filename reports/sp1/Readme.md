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

- Input 1
```shell
2024-11-20T19:48:20.034231Z  INFO prove_core: summary: cycles=9107, gas=34869, e2e=9.818071s, khz=0.93, proofSize=2656912
2024-11-20T19:48:20.041980Z  INFO prove_core: close time.busy=9.95s time.idle=92.8ms
Successfully generated proof!
2024-11-20T19:48:20.213473Z  INFO verify: close time.busy=171ms time.idle=959ns
Successfully verified proof!
```

- Input 10

```shell
n: 10
2024-11-20T19:43:17.480786Z  INFO vk verification: true
n: 10
2024-11-20T19:43:18.605191Z  INFO prove_core:clone: close time.busy=22.4µs time.idle=1.17µs
2024-11-20T19:43:18.605979Z  INFO prove_core:execute: close time.busy=768µs time.idle=459ns
2024-11-20T19:43:18.606391Z  INFO prove_core:create memory checkpoint: close time.busy=406µs time.idle=250ns
2024-11-20T19:43:18.613297Z  INFO prove_core: generated 1 records    
2024-11-20T19:43:18.613316Z  INFO prove_core: waiting for turn 0    
2024-11-20T19:43:18.613383Z  INFO prove_core: deferred 1 records    
2024-11-20T19:43:18.613386Z  INFO prove_core: collecting checkpoints    
2024-11-20T19:43:18.613389Z  INFO prove_core: fixing shape
2024-11-20T19:43:18.613412Z  INFO prove_core: fixing shape
2024-11-20T19:43:19.406664Z  INFO prove_core: generated 1 records    
2024-11-20T19:43:19.406712Z  INFO prove_core: deferred 1 records    
2024-11-20T19:43:28.244662Z  INFO prove_core: execution report (totals): total_cycles=9161, total_syscall_cycles=20, touched_memory_addresses=15034
2024-11-20T19:43:28.244682Z  INFO prove_core: execution report (opcode counts):
2024-11-20T19:43:28.244689Z  INFO prove_core:   1869 add
2024-11-20T19:43:28.244690Z  INFO prove_core:   1410 xor
2024-11-20T19:43:28.244692Z  INFO prove_core:   1373 srl
2024-11-20T19:43:28.244693Z  INFO prove_core:   1291 or
2024-11-20T19:43:28.244695Z  INFO prove_core:   1278 sll
2024-11-20T19:43:28.244696Z  INFO prove_core:    594 lw
2024-11-20T19:43:28.244698Z  INFO prove_core:    499 sw
2024-11-20T19:43:28.244699Z  INFO prove_core:    445 and
2024-11-20T19:43:28.244701Z  INFO prove_core:    137 lbu
2024-11-20T19:43:28.244702Z  INFO prove_core:     54 sb
2024-11-20T19:43:28.245017Z  INFO prove_core:     47 bltu
2024-11-20T19:43:28.245022Z  INFO prove_core:     39 jalr
2024-11-20T19:43:28.245023Z  INFO prove_core:     25 bne
2024-11-20T19:43:28.245025Z  INFO prove_core:     24 auipc
2024-11-20T19:43:28.245027Z  INFO prove_core:     20 ecall
2024-11-20T19:43:28.245028Z  INFO prove_core:     19 sub
2024-11-20T19:43:28.245029Z  INFO prove_core:     19 beq
2024-11-20T19:43:28.245031Z  INFO prove_core:      8 sltu
2024-11-20T19:43:28.245032Z  INFO prove_core:      5 bgeu
2024-11-20T19:43:28.245034Z  INFO prove_core:      5 mul
2024-11-20T19:43:28.245035Z  INFO prove_core:      0 sra
2024-11-20T19:43:28.245037Z  INFO prove_core:      0 slt
2024-11-20T19:43:28.249746Z  INFO prove_core:      0 lb
2024-11-20T19:43:28.249762Z  INFO prove_core:      0 lh
2024-11-20T19:43:28.249764Z  INFO prove_core:      0 lhu
2024-11-20T19:43:28.249765Z  INFO prove_core:      0 sh
2024-11-20T19:43:28.249767Z  INFO prove_core:      0 blt
2024-11-20T19:43:28.249768Z  INFO prove_core:      0 bge
2024-11-20T19:43:28.249770Z  INFO prove_core:      0 jal
2024-11-20T19:43:28.249772Z  INFO prove_core:      0 ebreak
2024-11-20T19:43:28.249773Z  INFO prove_core:      0 mulh
2024-11-20T19:43:28.249775Z  INFO prove_core:      0 mulhu
2024-11-20T19:43:28.249776Z  INFO prove_core:      0 mulhsu
2024-11-20T19:43:28.249778Z  INFO prove_core:      0 div
2024-11-20T19:43:28.255558Z  INFO prove_core:      0 divu
2024-11-20T19:43:28.255572Z  INFO prove_core:      0 rem
2024-11-20T19:43:28.255574Z  INFO prove_core:      0 remu
2024-11-20T19:43:28.255576Z  INFO prove_core:      0 unimp
2024-11-20T19:43:28.255579Z  INFO prove_core: execution report (syscall counts):
2024-11-20T19:43:28.255590Z  INFO prove_core:   8 commit
2024-11-20T19:43:28.255613Z  INFO prove_core:   8 commit_deferred_proofs
2024-11-20T19:43:28.255615Z  INFO prove_core:   1 halt
2024-11-20T19:43:28.255616Z  INFO prove_core:   1 write
2024-11-20T19:43:28.255618Z  INFO prove_core:   1 hint_len
2024-11-20T19:43:28.255619Z  INFO prove_core:   1 hint_read
2024-11-20T19:43:28.255621Z  INFO prove_core:   0 enter_unconstrained
2024-11-20T19:43:28.257143Z  INFO prove_core:   0 exit_unconstrained
2024-11-20T19:43:28.257150Z  INFO prove_core:   0 verify_sp1_proof
2024-11-20T19:43:28.257152Z  INFO prove_core:   0 ed_decompress
2024-11-20T19:43:28.257154Z  INFO prove_core:   0 secp256k1_double
2024-11-20T19:43:28.257155Z  INFO prove_core:   0 secp256k1_decompress
2024-11-20T19:43:28.257157Z  INFO prove_core:   0 bn254_double
2024-11-20T19:43:28.257159Z  INFO prove_core:   0 bls12381_decompress
2024-11-20T19:43:28.257160Z  INFO prove_core:   0 bls12381_double
2024-11-20T19:43:28.257162Z  INFO prove_core:   0 sha_compress
2024-11-20T19:43:28.257163Z  INFO prove_core:   0 ed_add
2024-11-20T19:43:28.257165Z  INFO prove_core:   0 keccak_permute
2024-11-20T19:43:28.258814Z  INFO prove_core:   0 secp256k1_add
2024-11-20T19:43:28.258821Z  INFO prove_core:   0 bn254_add
2024-11-20T19:43:28.258822Z  INFO prove_core:   0 uint256_mul
2024-11-20T19:43:28.258824Z  INFO prove_core:   0 bls12381_add
2024-11-20T19:43:28.258826Z  INFO prove_core:   0 bls12381_fp_add
2024-11-20T19:43:28.258827Z  INFO prove_core:   0 bls12381_fp_sub
2024-11-20T19:43:28.258829Z  INFO prove_core:   0 bls12381_fp_mul
2024-11-20T19:43:28.258830Z  INFO prove_core:   0 bls12381_fp2_add
2024-11-20T19:43:28.258832Z  INFO prove_core:   0 bls12381_fp2_sub
2024-11-20T19:43:28.258833Z  INFO prove_core:   0 bls12381_fp2_mul
2024-11-20T19:43:28.258835Z  INFO prove_core:   0 bn254_fp_add
2024-11-20T19:43:28.260816Z  INFO prove_core:   0 bn254_fp_sub
2024-11-20T19:43:28.260823Z  INFO prove_core:   0 bn254_fp_mul
2024-11-20T19:43:28.260825Z  INFO prove_core:   0 bn254_fp2_add
2024-11-20T19:43:28.260827Z  INFO prove_core:   0 bn254_fp2_sub
2024-11-20T19:43:28.260828Z  INFO prove_core:   0 bn254_fp2_mul
2024-11-20T19:43:28.260830Z  INFO prove_core:   0 sha_extend
2024-11-20T19:43:28.418123Z  INFO prove_core: summary: cycles=9161, gas=34931, e2e=9.65890225s, khz=0.95, proofSize=2656912
2024-11-20T19:43:28.425728Z  INFO prove_core: close time.busy=9.78s time.idle=87.1ms
Successfully generated proof!
2024-11-20T19:43:28.596595Z  INFO verify: close time.busy=170ms time.idle=1.21µs
Successfully verified proof!
```

- Input 100
```shell
2024-11-20T19:52:26.824100Z  INFO prove_core: summary: cycles=9701, gas=35556, e2e=10.189653541s, khz=0.95, proofSize=2656912
2024-11-20T19:52:26.831805Z  INFO prove_core: close time.busy=10.3s time.idle=178ms
Successfully generated proof!
2024-11-20T19:52:27.001519Z  INFO verify: close time.busy=169ms time.idle=1.71µs
Successfully verified proof!
```

- Input 1000
```shell
2024-11-20T19:54:02.757835Z  INFO prove_core: summary: cycles=15101, gas=41799, e2e=9.649789833s, khz=1.56, proofSize=2656912
2024-11-20T19:54:02.771249Z  INFO prove_core: close time.busy=9.83s time.idle=90.9ms
Successfully generated proof!
2024-11-20T19:54:02.950021Z  INFO verify: close time.busy=178ms time.idle=2.00µs
Successfully verified proof!
```

- Input 10000
```shell
2024-11-20T19:55:10.685594Z  INFO prove_core: summary: cycles=69101, gas=104237, e2e=18.871570208s, khz=3.66, proofSize=2656912
2024-11-20T19:55:10.695003Z  INFO prove_core: close time.busy=18.9s time.idle=259ms
Successfully generated proof!
2024-11-20T19:55:10.870054Z  INFO verify: close time.busy=174ms time.idle=1.58µs
Successfully verified proof!
```

- Input 100000
```shell
2024-11-20T19:57:38.141138Z  INFO prove_core: summary: cycles=609101, gas=728612, e2e=73.408582084s, khz=8.30, proofSize=8612096
2024-11-20T19:57:38.152487Z  INFO prove_core: close time.busy=73.4s time.idle=258ms
Successfully generated proof!
2024-11-20T19:57:38.771585Z  INFO verify: close time.busy=615ms time.idle=1.88µs
Successfully verified proof!
```

- Input 110000
```shell
2024-11-20T20:01:19.023467Z  INFO prove_core: summary: cycles=669101, gas=797987, e2e=80.641678917s, khz=8.30, proofSize=10100892
2024-11-20T20:01:19.033592Z  INFO prove_core: close time.busy=80.6s time.idle=280ms
Successfully generated proof!
2024-11-20T20:01:19.777422Z  INFO verify: close time.busy=740ms time.idle=1.71µs
Successfully verified proof!
```


## Sha256
```rust
pub fn sha2(input: &[u8]) -> u32 {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let res = Into::<[u8; 32]>::into(result);
    1
}
```

### Result
- 32 Bytes

```shell
2024-11-20T20:55:51.005655Z  INFO vk verification: true
n: 20
2024-11-20T20:55:53.110263Z  INFO prove_core:clone: close time.busy=81.6µs time.idle=2.33µs
2024-11-20T20:55:53.111273Z  INFO prove_core:execute: close time.busy=982µs time.idle=500ns
2024-11-20T20:55:53.111519Z  INFO prove_core:create memory checkpoint: close time.busy=236µs time.idle=250ns
2024-11-20T20:55:53.121383Z  INFO prove_core: generated 1 records    
2024-11-20T20:55:53.121443Z  INFO prove_core: waiting for turn 0    
2024-11-20T20:55:53.121603Z  INFO prove_core: deferred 1 records    
2024-11-20T20:55:53.121618Z  INFO prove_core: collecting checkpoints    
2024-11-20T20:55:53.121622Z  INFO prove_core: fixing shape
2024-11-20T20:55:53.121659Z  INFO prove_core: fixing shape
2024-11-20T20:55:54.086147Z  INFO prove_core: generated 1 records    
2024-11-20T20:55:54.086195Z  INFO prove_core: deferred 1 records    
2024-11-20T20:56:05.475271Z  INFO prove_core: execution report (totals): total_cycles=13479, total_syscall_cycles=20, touched_memory_addresses=15209
2024-11-20T20:56:05.475287Z  INFO prove_core: execution report (opcode counts):
2024-11-20T20:56:05.475294Z  INFO prove_core:   2670 add
2024-11-20T20:56:05.475295Z  INFO prove_core:   2116 xor
2024-11-20T20:56:05.475297Z  INFO prove_core:   2046 srl
2024-11-20T20:56:05.475298Z  INFO prove_core:   1926 or
2024-11-20T20:56:05.475300Z  INFO prove_core:   1912 sll
2024-11-20T20:56:05.475301Z  INFO prove_core:    891 lw
2024-11-20T20:56:05.475303Z  INFO prove_core:    744 sw
2024-11-20T20:56:05.475304Z  INFO prove_core:    657 and
2024-11-20T20:56:05.475306Z  INFO prove_core:    202 lbu
2024-11-20T20:56:05.475601Z  INFO prove_core:     65 bltu
2024-11-20T20:56:05.475604Z  INFO prove_core:     60 sb
2024-11-20T20:56:05.475605Z  INFO prove_core:     47 jalr
2024-11-20T20:56:05.475607Z  INFO prove_core:     28 auipc
2024-11-20T20:56:05.475608Z  INFO prove_core:     26 beq
2024-11-20T20:56:05.475610Z  INFO prove_core:     24 bne
2024-11-20T20:56:05.475611Z  INFO prove_core:     22 sub
2024-11-20T20:56:05.475612Z  INFO prove_core:     20 ecall
2024-11-20T20:56:05.475614Z  INFO prove_core:     12 sltu
2024-11-20T20:56:05.475615Z  INFO prove_core:      6 mul
2024-11-20T20:56:05.475617Z  INFO prove_core:      5 bgeu
2024-11-20T20:56:05.475618Z  INFO prove_core:      0 sra
2024-11-20T20:56:05.475619Z  INFO prove_core:      0 slt
2024-11-20T20:56:05.479847Z  INFO prove_core:      0 lb
2024-11-20T20:56:05.479854Z  INFO prove_core:      0 lh
2024-11-20T20:56:05.479855Z  INFO prove_core:      0 lhu
2024-11-20T20:56:05.479857Z  INFO prove_core:      0 sh
2024-11-20T20:56:05.479858Z  INFO prove_core:      0 blt
2024-11-20T20:56:05.479860Z  INFO prove_core:      0 bge
2024-11-20T20:56:05.479861Z  INFO prove_core:      0 jal
2024-11-20T20:56:05.479863Z  INFO prove_core:      0 ebreak
2024-11-20T20:56:05.479864Z  INFO prove_core:      0 mulh
2024-11-20T20:56:05.479866Z  INFO prove_core:      0 mulhu
2024-11-20T20:56:05.479868Z  INFO prove_core:      0 mulhsu
2024-11-20T20:56:05.479869Z  INFO prove_core:      0 div
2024-11-20T20:56:05.480850Z  INFO prove_core:      0 divu
2024-11-20T20:56:05.480852Z  INFO prove_core:      0 rem
2024-11-20T20:56:05.480853Z  INFO prove_core:      0 remu
2024-11-20T20:56:05.480855Z  INFO prove_core:      0 unimp
2024-11-20T20:56:05.480857Z  INFO prove_core: execution report (syscall counts):
2024-11-20T20:56:05.480867Z  INFO prove_core:   8 commit
2024-11-20T20:56:05.480882Z  INFO prove_core:   8 commit_deferred_proofs
2024-11-20T20:56:05.480883Z  INFO prove_core:   1 halt
2024-11-20T20:56:05.480885Z  INFO prove_core:   1 write
2024-11-20T20:56:05.480886Z  INFO prove_core:   1 hint_len
2024-11-20T20:56:05.480888Z  INFO prove_core:   1 hint_read
2024-11-20T20:56:05.480889Z  INFO prove_core:   0 enter_unconstrained
2024-11-20T20:56:05.481738Z  INFO prove_core:   0 exit_unconstrained
2024-11-20T20:56:05.481740Z  INFO prove_core:   0 verify_sp1_proof
2024-11-20T20:56:05.481742Z  INFO prove_core:   0 ed_decompress
2024-11-20T20:56:05.481743Z  INFO prove_core:   0 secp256k1_double
2024-11-20T20:56:05.481745Z  INFO prove_core:   0 secp256k1_decompress
2024-11-20T20:56:05.481746Z  INFO prove_core:   0 bn254_double
2024-11-20T20:56:05.481748Z  INFO prove_core:   0 bls12381_decompress
2024-11-20T20:56:05.481749Z  INFO prove_core:   0 bls12381_double
2024-11-20T20:56:05.481751Z  INFO prove_core:   0 sha_compress
2024-11-20T20:56:05.481752Z  INFO prove_core:   0 ed_add
2024-11-20T20:56:05.481754Z  INFO prove_core:   0 keccak_permute
2024-11-20T20:56:05.482580Z  INFO prove_core:   0 secp256k1_add
2024-11-20T20:56:05.482582Z  INFO prove_core:   0 bn254_add
2024-11-20T20:56:05.482584Z  INFO prove_core:   0 uint256_mul
2024-11-20T20:56:05.482585Z  INFO prove_core:   0 bls12381_add
2024-11-20T20:56:05.482587Z  INFO prove_core:   0 bls12381_fp_add
2024-11-20T20:56:05.482588Z  INFO prove_core:   0 bls12381_fp_sub
2024-11-20T20:56:05.482590Z  INFO prove_core:   0 bls12381_fp_mul
2024-11-20T20:56:05.482591Z  INFO prove_core:   0 bls12381_fp2_add
2024-11-20T20:56:05.482593Z  INFO prove_core:   0 bls12381_fp2_sub
2024-11-20T20:56:05.482594Z  INFO prove_core:   0 bls12381_fp2_mul
2024-11-20T20:56:05.482595Z  INFO prove_core:   0 bn254_fp_add
2024-11-20T20:56:05.484545Z  INFO prove_core:   0 bn254_fp_sub
2024-11-20T20:56:05.484551Z  INFO prove_core:   0 bn254_fp_mul
2024-11-20T20:56:05.484553Z  INFO prove_core:   0 bn254_fp2_add
2024-11-20T20:56:05.484555Z  INFO prove_core:   0 bn254_fp2_sub
2024-11-20T20:56:05.484556Z  INFO prove_core:   0 bn254_fp2_mul
2024-11-20T20:56:05.484558Z  INFO prove_core:   0 sha_extend
2024-11-20T20:56:05.646253Z  INFO prove_core: summary: cycles=13479, gas=40514, e2e=12.374450708s, khz=1.09, proofSize=2656912
2024-11-20T20:56:05.660716Z  INFO prove_core: close time.busy=12.5s time.idle=205ms
Successfully generated proof!
2024-11-20T20:56:05.871122Z  INFO verify: close time.busy=209ms time.idle=1.42µs
Successfully verified proof!
```

- 1k bytes
```shell
2024-11-20T20:57:10.430560Z  INFO prove_core: summary: cycles=71249, gas=112903, e2e=17.558199042s, khz=4.06, proofSize=2656912
2024-11-20T20:57:10.438950Z  INFO prove_core: close time.busy=17.6s time.idle=170ms
Successfully generated proof!
2024-11-20T20:57:10.611882Z  INFO verify: close time.busy=172ms time.idle=750ns
Successfully verified proof!
```

- 10k bytes
```shell
2024-11-20T20:59:49.825781Z  INFO prove_core: summary: cycles=611980, gas=790779, e2e=73.776641458s, khz=8.30, proofSize=8612096
2024-11-20T20:59:49.834266Z  INFO prove_core: close time.busy=73.7s time.idle=278ms
Successfully generated proof!
2024-11-20T20:59:50.451971Z  INFO verify: close time.busy=614ms time.idle=1.33µs
Successfully verified proof!
```

- 20k bytes
```shell
2024-11-20T21:02:52.383073Z  INFO prove_core: summary: cycles=1210252, gas=1540801, e2e=142.716018s, khz=8.48, proofSize=16056076
2024-11-20T21:02:52.394993Z  INFO prove_core: close time.busy=143s time.idle=289ms
Successfully generated proof!
2024-11-20T21:02:53.576195Z  INFO verify: close time.busy=1.17s time.idle=2.62µs
Successfully verified proof!
```
