# ZK Benchmark Project

## Overview

This repository hosts the source code and resources for benchmarking various Zero-Knowledge (ZK) proofs. It builds upon the original repositories of Halo2, RISC0, Nexus, SP1, Jolt, Circom, and Powdr with our implementations of proofs including but not limited to SHA256, Fibonacci, and Poseidon operations.

Our aim is to benchmark the performance of these proofs, comparing them in terms of time, space, and other metrics. This comprehensive comparison will not only highlight the most popular ZK proofs and their implementations but also deepen our understanding of their inner workings, developer friendliness, and proving systems.

## Goals

- **Benchmark Operations:**
  - **Fibonacci:** Variants 1, 10, 100, 1000, 10000
  - **SHA256:** Variants 32b, 1k, 10k Bytes
  - **Poseidon:** Variants 32b, 100b

- **Additional Benchmarks:**
  - Arithmetic Operations: Addition, Subtraction, Multiplication, Division
  - Loop: Variants 10, 100, 1000
  - Hash Functions: Pederson, RPO, Keccak (Variants 1k, 10k Bytes)
  - Merkle Tree: Generation, Membership/Inclusion, Merge
  - nth Prime: Variants 1, 10, 100, 1000, 10000
  - Cryptographic Verifications: ECDSA, Elliptic Curve, BLS Verification, BLS Aggregation

## Proving Schemes

| Scheme | Benchmark Code | Reports |
|--------------|-------------------|-----------|
| [Halo2](https://zcash.github.io/halo2) | [code](./halo2/) | [report](./reports/halo2/)   |
| [Circom](https://docs.circom.io)       | [code](./circom/)|  [report](./reports/circom/) |
| [Nexus](https://docs.nexus.xyz)        | [code](./nexus/) | [report](./reports/nexus/)   |
| [Risc Zero](https://dev.risczero.com/api)| [code](./risc-zero/) | [report](./reports/risc-zero/) |
| [SP1](https://docs.succinct.xyz/docs/sp1/introduction) | [code](./sp1/)     | [report](./reports/sp1/)     |
| [Jolt](https://jolt.a16zcrypto.com)    | [code](./jolt/)    | [report](./reports/jolt/)    |
| [Powdr](https://docs.powdr.org/)       | [code](./powdr/)   | [report](./reports/powdr/)   |
| [Polygon Miden](./polygon-miden/)              | TBA                  | TBA                          |
| [Aleo SnarkVM](./aleo-snarkvm/)                | TBA                  | TBA                          |
| [Zokrates](./zokrates/)                        | TBA                  | TBA                          |
| [Delphinus Lab ZKWASM](./delphinus-zkwasm/)    | TBA                  | TBA                          |
| [Lita Valida](./lita-valida/)                  | TBA                  | TBA                          |
| [Eigen ZKVM](./eigen-zkvm/)                    | TBA                  | TBA                          |
| [CairoVM](./cairovm/)                          | TBA                  | TBA                          |
| [Noir](./noir/)                                | TBA                  | TBA                          |
| [Ola VM](./ola-vm/)                            | TBA                  | TBA                          |
| [Triton VM](./triton-vm/)                      | TBA                  | TBA                          |
| [Lurk](./lurk/)                                | TBA                  | TBA                          |
| [Ceno](./ceno/)                                | TBA                  | TBA                          |
| [Expander](./expander/)                        | TBA                  | TBA                          |
| [OpenVM](./openvm/)                            | TBA                  | TBA                          |
| [Brevis](./brevis/)                            | TBA                  | TBA                          |

> TBA = To Be Added

#### General overview of features of proving schemes
<!---<img width="600" alt="Screenshot 2024-11-22 at 3 44 30 PM" src="https://github.com/user-attachments/assets/2768a9c5-7477-4e7b-86aa-f80a85818c2f">--->

| Proof System | Setup Complexity | Features | Post-Quantum Resistance | Scalability | Parallel Execution |
|--------------|-------------------|-----------|-------------------------|-------------|-------------------|
|     Halo2    |   Transparent generally  |   Recursive proofs    |    No (ECC based)    |     High    |     Limited     |
|   Circom (Groth16)   |      Trusted Setup     |   Efficient proofs   |   No (Pairing-based)  |   Moderate  |       High      |
|   Risc Zero  |   Transparent   |   General purpose   |       Yes       |     High    |       High      |
|     Jolt     |  Can support both|   Efficient proofs   |       Yes       |   Very High |     Very High   |
|  Nexus zkVM  |   Transparent   |  Privacy focused    |      Partial     |   Moderate  |       High      |
|     SP1      |   Transparent   |  rollup optimized   |       Yes       |   Very High |     Very High   |
|    Powdr     |   Transparent   |     Extensible      |       Yes       |     High    |       High      |

The key differences between these proof systems include their setup complexity, proof features, post-quantum resistance, scalability, and parallel execution capabilities. Some use recursive proofs, while others focus on efficiency or general-purpose functionality. The systems also vary in their post-quantum resistance and ability to scale and parallelize.

## Metrics

- Prover Time
- Verifier Time
- Prover Space
- Verifier Space
- Proof Size
- Parallel Execution
- CPU Usage
- Memory Usage
- Verifier Gas Consumption
- Cycles Count
- Developer Friendliness

## Reports

The detailed reports are in the [reports](./reports/) directory.
- [Reports](./reports/)
  - [Jolt](./reports/jolt/)
  - [SP1](./reports/sp1/)
  - [Powdr](./reports/powdr/)
  - [Halo2](./reports/halo2/)
  - [Nexus](./reports/nexus/)
  - [Circom](./reports/circom/)
 
**Summary:** Only selected operations are shown. The detailed [reports](./reports/) have metrics for other variants and additional operations.

### Fibonacci 
> 10000 th number
<!---
<img width="600" alt="fib" src="https://github.com/user-attachments/assets/e99bd993-ecb3-43dd-8e39-c90a4a8d17f6">
-->

| Proof System           | Prover Time (s) | Cycles  | Verifier Time (s) | Prover Memory (KB) | Proof Size         |
|------------------------|-----------------|---------|-------------------|---------------------|--------------------|
| Halo2                  | 0.196           | -       | 0.004             | 9.8                 | 1664               |
| Circom                 | 1.75            | 9999    | 0.81              | 466280              | 805                |
| Risc Zero              | 6.37            | 65536   | -                 | -                   | 206182             |
| Jolt                   | 21.73           | 280287  | 0.01              | -                   | 452398             |
| Nexus (max 100)        | 35.2            | -       | 2.4               | -                   | 47.9 MB            |
| SP1                    | 18.87           | 69101   | 0.174             | -                   | 2656912            |
| Powdr                  | 8.64            | 2990    | -                 | -                   | -                  |


### SHA256 
> 1 KB Input
<!---
<img width="600" alt="SHA256" src="https://github.com/user-attachments/assets/bd8a369b-1b4a-44cf-b541-0bdfdd5a5367">
-->

| Proof System           | Prover Time (s) | Cycles  | Verifier Time (s) | Prover Memory (KB) | Proof Size (B)     |
|------------------------|-----------------|---------|-------------------|---------------------|--------------------|
| Halo2                  | 14.78           | -       | 0.13              | 1134                | 4064               |
| Circom                 | 46.07           | 540736  | 1.14              | 3920848             | 805                |
| Risc Zero              | 2.5             | 65536   | -                 | -                   | 210157             |
| Jolt                   | 2.199           | 62231   | 0.052             | -                   | 401116             |
| Nexus                  | 30+ mins        | -       | -                 | -                   | -                  |
| SP1                    | 17.6            | 71249   | 0.172             | -                   | 265691             |
| Powdr                  | 9.07            | 73731   | -                 | -                   | -                  |


### Poseidon 
> 32 Byte input
<!---
<img width="600" alt="Poseidon" src="https://github.com/user-attachments/assets/c468f0fc-163d-44f8-9b0e-b726dd7a8653">
-->

| Proof System | Prover Time (s)|     Cycles        | Verifier Time (s) | Prover Memory (KB) | Proof Size (B) |
|-------------|-----------------|-------------------|-------------------|----------------|---------------|
| Halo2       |       8.74      |      -            |      0.086        |       25       |      2144     | 
| Circom      |       1.19      |     4184          |      0.72         |     373560     |       804     | 
| Risc Zero   |       5.47      |     524288        |        -          |       -        |      256742   |
| Jolt        |       91.38     |     554595        |       0.19        |       -        |     477746    |
| SP1         |       112.5     |      39479        |      0.509        |       -        |     2876912   |
| Powdr       |       21.54     |     286652        |       -           |       -        |       -       |

## Benchmark Machine Specifications

- **Macbook M1 Pro:** Core 8, Memory 8 GB
- **AlmaLinux 8.10:** Core 16, Memory 32 GB, Disk 1 TB
- **Windows 11:** Core 16, Memory 32 GB, Disk 1 TB
- **Intel(R):** Core 4 - Memory 8GB
- **Macbook M2 Pro:** Core 16, Memory 16 GB

## Installation

Each project has its own README file with instructions on installing the dependencies and running the benchmarks.

## Roadmap
Please refer to the [issues](https://github.com/babybear-labs/benchmark/issues).

## Contributors

This project is part of the ZK and Scaling Bootcamp organized by the Encode club.  
- Tanmoy : https://x.com/mtanm0y
- Rosemary :https://x.com/rosekoikara
- Yogesh : https://x.com/yogesh_eth
- Anubha

## Other benchmarks
- [Benchmarking ZKP Development Frameworks: the Pantheon of ZKP](https://ethresear.ch/t/benchmarking-zkp-development-frameworks-the-pantheon-of-zkp/14943)
- [Lurk 0.5 Benchmarks](https://argument.xyz/blog/perf-2024)
- [Lita Benchmark](https://lita.gitbook.io/lita-documentation/architecture/benchmarks)
- [ZK Benchmark](https://github.com/polybase/zk-benchmarks)
- [a16z Benchmark](https://github.com/a16z/zkvm-benchmarks)
- [A zkVM Survey for the Nomos Coordination Layer](https://blog.nomos.tech/a-zkvm-survey-for-the-nomos-coordination-layer)
- [Your definitive guide to zkVMs](https://mirror.xyz/stackrlabs.eth/jEBSBZtKEiMiTrRIGMCxN7n6r7al-vi25lmrnD610W4)
- [Risc Zero Benchmark](https://reports.risczero.com/benchmarks/Linux-cpu)

## Acknowledgments
Special thanks to the ZK and Scaling Bootcamp team for organizing this event and providing us with the opportunity to learn and contribute to the ZK space.
