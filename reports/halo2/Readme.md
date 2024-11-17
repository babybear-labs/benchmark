# Halo2


## Bench KZG SHA-256

```shell
cd sha256test
cargo run --release <k> <sha-count>
```

### example:

```shell
cd sha256test
cargo run --release 17 2
```

### Output
- `sha256_params_k_17`: 8.4 MB
- `sha256_proof_k_17_count_2`: 3 KB

![](./halo2.png)


## Bench Proofs

```shell
cd halo2_proofs
cargo bench
```


### Output
```shell
double-and-add          time:   [1.9676 ms 1.9883 ms 2.0216 ms]                            
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

     Running benches/dev_lookup.rs (/Users/macbookpro2020/art/rust/zk-bench/benchmark/halo2/target/release/deps/dev_lookup-b6bcde20190d2734)
WARNING: HTML report generation will become a non-default optional feature in Criterion.rs 0.4.0.
This feature is being moved to cargo-criterion (https://github.com/bheisler/cargo-criterion) and will be optional in a future version of Criterion.rs. To silence this warning, either switch to cargo-criterion or enable the 'html_reports' feature in your Cargo.toml.

Gnuplot not found, using plotters backend
dev-lookup/14           time:   [2.8712 ms 3.0598 ms 3.2631 ms]                          
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe
dev-lookup/15           time:   [4.9315 ms 5.0824 ms 5.3301 ms]                          
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
dev-lookup/16           time:   [9.2736 ms 9.3612 ms 9.4482 ms]                         
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
dev-lookup/17           time:   [18.616 ms 19.146 ms 19.723 ms]                         
dev-lookup/18           time:   [36.601 ms 37.329 ms 38.675 ms]                         
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe

     Running benches/fft.rs (/Users/macbookpro2020/art/rust/zk-bench/benchmark/halo2/target/release/deps/fft-39a7ff2b9dd41a1d)
WARNING: HTML report generation will become a non-default optional feature in Criterion.rs 0.4.0.
This feature is being moved to cargo-criterion (https://github.com/bheisler/cargo-criterion) and will be optional in a future version of Criterion.rs. To silence this warning, either switch to cargo-criterion or enable the 'html_reports' feature in your Cargo.toml.

Gnuplot not found, using plotters backend
fft/k/3                 time:   [334.37 ns 336.11 ns 338.42 ns]                    
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe
fft/k/4                 time:   [12.679 µs 13.006 µs 13.501 µs]                     
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
fft/k/5                 time:   [18.356 µs 18.511 µs 18.656 µs]                     
fft/k/6                 time:   [25.632 µs 26.028 µs 26.668 µs]                     
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) low severe
  6 (6.00%) low mild
  2 (2.00%) high severe
fft/k/7                 time:   [33.250 µs 33.590 µs 33.899 µs]                     
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) low mild
  3 (3.00%) high severe
fft/k/8                 time:   [166.76 µs 214.53 µs 270.36 µs]                    
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
fft/k/9                 time:   [82.413 µs 84.523 µs 86.388 µs]                    
fft/k/10                time:   [180.62 µs 191.45 µs 208.60 µs]                     
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
fft/k/11                time:   [310.79 µs 335.24 µs 366.26 µs]                     
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe
fft/k/12                time:   [598.25 µs 652.03 µs 740.76 µs]                     
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
Benchmarking fft/k/13: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.5s, enable flat sampling, or reduce sample count to 60.
fft/k/13                time:   [1.0216 ms 1.0637 ms 1.1149 ms]                      
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
Benchmarking fft/k/14: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.8s, enable flat sampling, or reduce sample count to 50.
fft/k/14                time:   [2.0510 ms 2.2399 ms 2.4789 ms]                      
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
fft/k/15                time:   [3.5169 ms 3.9266 ms 4.4508 ms]                      
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) high mild
  12 (12.00%) high severe
fft/k/16                time:   [7.1348 ms 7.8506 ms 8.7236 ms]                     
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) high mild
  11 (11.00%) high severe
fft/k/17                time:   [13.938 ms 14.888 ms 16.041 ms]                     
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) high mild
  10 (10.00%) high severe
                                                                  fft/k/18                time:   [27.062 ms 28.994 ms 32.262 ms]
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe

     Running benches/hashtocurve.rs (/Users/macbookpro2020/art/rust/zk-bench/benchmark/halo2/target/release/deps/hashtocurve-16156593fc22e696)
WARNING: HTML report generation will become a non-default optional feature in Criterion.rs 0.4.0.
This feature is being moved to cargo-criterion (https://github.com/bheisler/cargo-criterion) and will be optional in a future version of Criterion.rs. To silence this warning, either switch to cargo-criterion or enable the 'html_reports' feature in your Cargo.toml.

Gnuplot not found, using plotters backend
Benchmarking hash-to-curve/Pallas: Collecting 100 samples in estim                                                                  hash-to-curve/Pallas    time:   [21.173 µs 21.531 µs 22.136 µs]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe
Benchmarking hash-to-curve/Vesta: Collecting 100 samples in estima                                                                  hash-to-curve/Vesta     time:   [21.278 µs 21.676 µs 22.147 µs]
Found 15 outliers among 100 measurements (15.00%)
  8 (8.00%) high mild
  7 (7.00%) high severe

     Running benches/plonk.rs (/Users/macbookpro2020/art/rust/zk-bench/benchmark/halo2/target/release/deps/plonk-6fdd61eb7a1686c3)
WARNING: HTML report generation will become a non-default optional feature in Criterion.rs 0.4.0.
This feature is being moved to cargo-criterion (https://github.com/bheisler/cargo-criterion) and will be optional in a future version of Criterion.rs. To silence this warning, either switch to cargo-criterion or enable the 'html_reports' feature in your Cargo.toml.

Gnuplot not found, using plotters backend
Benchmarking plonk-keygen/8: Collecting 10 samples in estimated 8.                                                                  plonk-keygen/8          time:   [66.524 ms 68.181 ms 70.529 ms]
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe
Benchmarking plonk-keygen/9: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 7.6s or enable flat sampling.
Benchmarking plonk-keygen/9: Collecting 10 samples in estimated 7.                                                                  plonk-keygen/9          time:   [138.61 ms 142.23 ms 147.79 ms]
Benchmarking plonk-keygen/10: Collecting 10 samples in estimated 5                                                                  plonk-keygen/10         time:   [283.27 ms 288.22 ms 293.96 ms]
Benchmarking plonk-keygen/11: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 6.1s.
Benchmarking plonk-keygen/11: Collecting 10 samples in estimated 6                                                                  plonk-keygen/11         time:   [589.32 ms 613.01 ms 637.95 ms]
Benchmarking plonk-keygen/12: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 12.5s.
Benchmarking plonk-keygen/12: Collecting 10 samples in estimated 1                                   plonk-keygen/12         time:   [1.2519 s 1.3312 s 1.4222 s]                             
Benchmarking plonk-keygen/13: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 25.5s.
plonk-keygen/13         time:   [2.5026 s 2.5549 s 2.6176 s]                             
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high mild
Benchmarking plonk-keygen/14: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 86.6s.
plonk-keygen/14         time:   [5.2942 s 5.6422 s 6.0925 s]                             
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking plonk-keygen/15: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 113.1s.
plonk-keygen/15         time:   [11.071 s 11.649 s 12.507 s]                             
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
Benchmarking plonk-keygen/16: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 280.0s.
plonk-keygen/16         time:   [23.709 s 28.724 s 35.578 s]                             
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe

plonk-prover/8          time:   [199.64 ms 226.75 ms 254.75 ms]                         
plonk-prover/9          time:   [256.31 ms 286.51 ms 317.47 ms]                         
plonk-prover/10         time:   [414.04 ms 442.36 ms 472.07 ms]                          
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) low mild
  1 (10.00%) high mild
Benchmarking plonk-prover/11: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 6.0s.
plonk-prover/11         time:   [630.90 ms 749.98 ms 881.73 ms]                          
Benchmarking plonk-prover/12: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 7.8s.
plonk-prover/12         time:   [571.51 ms 594.30 ms 616.60 ms]                          
Benchmarking plonk-prover/13: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 10.7s.
plonk-prover/13         time:   [1.2131 s 1.2800 s 1.3564 s]                             
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking plonk-prover/14: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 18.5s.
plonk-prover/14         time:   [1.8357 s 1.9378 s 2.0571 s]                             
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild
Benchmarking plonk-prover/15: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 30.1s.
plonk-prover/15         time:   [2.6834 s 2.8079 s 2.9508 s]                             
Benchmarking plonk-prover/16: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 64.3s.
plonk-prover/16         time:   [5.5590 s 5.9248 s 6.3145 s]                             
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

plonk-verifier/8        time:   [3.2657 ms 3.3974 ms 3.5928 ms]                              
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
plonk-verifier/9        time:   [4.8832 ms 5.1279 ms 5.4419 ms]                              
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
plonk-verifier/10       time:   [6.9639 ms 7.3133 ms 7.7064 ms]                              
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) high mild
  12 (12.00%) high severe
plonk-verifier/11       time:   [10.419 ms 10.625 ms 10.930 ms]                              
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
plonk-verifier/12       time:   [17.383 ms 18.394 ms 20.118 ms]                              
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe
plonk-verifier/13       time:   [29.396 ms 30.364 ms 31.643 ms]                              
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
Benchmarking plonk-verifier/14: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.2s, or reduce sample count to 90.
Benchmarking plonk-verifier/14: Collecting 100 samples in e                                                           plonk-verifier/14       time:   [51.192 ms 53.698 ms 56.478 ms]
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe
Benchmarking plonk-verifier/15: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 15.0s, or reduce sample count to 30.
Benchmarking plonk-verifier/15: Collecting 100 samples in e                                                           plonk-verifier/15       time:   [83.773 ms 88.444 ms 94.597 ms]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
Benchmarking plonk-verifier/16: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 18.6s, or reduce sample count to 20.
Benchmarking plonk-verifier/16: Collecting 100 samples in e                                                           plonk-verifier/16       time:   [160.52 ms 170.96 ms 183.82 ms]
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe
```


## Bench Gadgets

```shell
cd halo2_gadgets
cargo bench
```


### Output
```shell
Benchmarking WIDTH = 3, RATE = 2-prover: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.5s, or reduce sample count to 70.
Benchmarking WIDTH = 3, RATE = 2-prover: Collecting 100 samples in estimated 6.5176                                                                                   WIDTH = 3, RATE = 2-prover                        
                        time:   [60.642 ms 61.788 ms 63.281 ms]
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  6 (6.00%) high mild
  3 (3.00%) high severe

Benchmarking WIDTH = 3, RATE = 2-verifier: Collecting 100 samples in estimated 5.09                                                                                   WIDTH = 3, RATE = 2-verifier                        
                        time:   [3.4788 ms 3.7632 ms 4.1298 ms]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

Benchmarking WIDTH = 9, RATE = 8-prover: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 11.4s, or reduce sample count to 40.
Benchmarking WIDTH = 9, RATE = 8-prover: Collecting 100 samples in estimated 11.377                                                                                   WIDTH = 9, RATE = 8-prover                        
                        time:   [109.37 ms 114.36 ms 120.74 ms]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe

Benchmarking WIDTH = 9, RATE = 8-verifier: Collecting 100 samples in estimated 5.09                                                                                   WIDTH = 9, RATE = 8-verifier                        
                        time:   [3.9335 ms 4.0099 ms 4.1101 ms]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

Benchmarking WIDTH = 12, RATE = 11-prover: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 15.1s, or reduce sample count to 30.
Benchmarking WIDTH = 12, RATE = 11-prover: Collecting 100 samples in estimated 15.0                                                                                   WIDTH = 12, RATE = 11-prover                        
                        time:   [146.41 ms 152.76 ms 160.62 ms]
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe

Benchmarking WIDTH = 12, RATE = 11-verifier: Collecting 100 samples in estimated 5.                                                                                   WIDTH = 12, RATE = 11-verifier                        
                        time:   [4.1922 ms 4.2614 ms 4.3435 ms]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

     Running benches/primitives.rs (/Users/macbookpro2020/art/rust/zk-bench/benchmark/halo2/target/release/deps/primitives-82517677b770f37a)
WARNING: HTML report generation will become a non-default optional feature in Criterion.rs 0.4.0.
This feature is being moved to cargo-criterion (https://github.com/bheisler/cargo-criterion) and will be optional in a future version of Criterion.rs. To silence this warning, either switch to cargo-criterion or enable the 'html_reports' feature in your Cargo.toml.

Gnuplot not found, using plotters backend
Benchmarking Poseidon/2-to-1: Collecting 100 samples in estimated 5.0550 s (177k it                                                                                   Poseidon/2-to-1         time:   [25.823 µs 28.640 µs 32.107 µs]
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe

Benchmarking Sinsemilla/hash-to-point/510: Collecting 100 samples in estimated 5.33                                                                                   Sinsemilla/hash-to-point/510                        
                        time:   [85.994 µs 90.333 µs 96.271 µs]
Found 14 outliers among 100 measurements (14.00%)
  7 (7.00%) high mild
  7 (7.00%) high severe
Benchmarking Sinsemilla/hash/510: Collecting 100 samples in estimated 5.2002 s (56k                                                                                   Sinsemilla/hash/510     time:   [92.199 µs 92.675 µs 93.242 µs]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
Benchmarking Sinsemilla/commit/510: Collecting 100 samples in estimated 5.7055 s (3                                                                                   Sinsemilla/commit/510   time:   [156.86 µs 158.61 µs 160.87 µs]
Found 11 outliers among 100 measurements (11.00%)
  8 (8.00%) high mild
  3 (3.00%) high severe
Benchmarking Sinsemilla/short-commit/510: Collecting 100 samples in estimated 5.147                                                                                   Sinsemilla/short-commit/510                        
                        time:   [154.73 µs 163.56 µs 179.17 µs]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
Benchmarking Sinsemilla/hash-to-point/520: Collecting 100 samples in estimated 5.43                                                                                   Sinsemilla/hash-to-point/520                        
                        time:   [85.506 µs 86.149 µs 86.966 µs]
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe
Benchmarking Sinsemilla/hash/520: Collecting 100 samples in estimated 5.2491 s (56k                                                                                   Sinsemilla/hash/520     time:   [95.394 µs 100.86 µs 109.47 µs]
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
Benchmarking Sinsemilla/commit/520: Collecting 100 samples in estimated 5.7533 s (3                                                                                   Sinsemilla/commit/520   time:   [165.08 µs 174.22 µs 186.03 µs]
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe
Benchmarking Sinsemilla/short-commit/520: Collecting 100 samples in estimated 5.672                                                                                   Sinsemilla/short-commit/520                        
                        time:   [164.85 µs 169.74 µs 175.80 µs]
Found 15 outliers among 100 measurements (15.00%)
  8 (8.00%) high mild
  7 (7.00%) high severe
Benchmarking Sinsemilla/hash-to-point/1086: Collecting 100 samples in estimated 5.4                                                                                   Sinsemilla/hash-to-point/1086                        
                        time:   [181.35 µs 185.32 µs 191.08 µs]
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe
Benchmarking Sinsemilla/hash/1086: Collecting 100 samples in estimated 5.1192 s (25                                                                                   Sinsemilla/hash/1086    time:   [187.88 µs 189.31 µs 191.03 µs]
Found 15 outliers among 100 measurements (15.00%)
  5 (5.00%) high mild
  10 (10.00%) high severe
Benchmarking Sinsemilla/commit/1086: Collecting 100 samples in estimated 5.2875 s (                                                                                   Sinsemilla/commit/1086  time:   [255.04 µs 267.30 µs 290.09 µs]
Found 19 outliers among 100 measurements (19.00%)
  3 (3.00%) high mild
  16 (16.00%) high severe
Benchmarking Sinsemilla/short-commit/1086: Collecting 100 samples in estimated 5.11                                                                                   Sinsemilla/short-commit/1086                        
                        time:   [252.24 µs 268.22 µs 294.85 µs]
Found 14 outliers among 100 measurements (14.00%)
  11 (11.00%) high mild
  3 (3.00%) high severe

```