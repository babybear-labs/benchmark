# ZK Benchmark

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
| [Polygon Miden](https://0xpolygonmiden.github.io/miden-vm/intro/main.html)   | TBA       | TBA      |
| [Aleo SnarkVM](https://github.com/ProvableHQ/snarkVM)                        | TBA       | TBA      |
| [Zokrates](https://zokrates.github.io)         | TBA                  | TBA                          |
| [Delphinus Lab ZKWASM](https://github.com/DelphinusLab/zkWasm)        | TBA              | TBA       |
| [Lita Valida](https://lita.gitbook.io)         | TBA                  | TBA                          |
| [Eigen ZKVM](https://github.com/0xEigenLabs/eigen-zkvm) | TBA         | TBA                          |
| [CairoVM](https://github.com/starkware-libs)   | TBA                  | TBA                          |
| [Noir](https://noir-lang.org/docs)             | TBA                  | TBA                          |
| [Ola VM](https://github.com/Sin7Y/olavm)       | TBA                  | TBA                          |
| [Triton VM](https://triton-vm.org/spec)        | TBA                  | TBA                          |
| [Lurk](https://docs.argument.xyz)              | TBA                  | TBA                          |
| [Ceno](https://github.com/scroll-tech/ceno)    | TBA                  | TBA                          |
| [Expander](https://expander.polyhedra.network) | TBA                  | TBA                          |
| [OpenVM](https://book.openvm.dev)              | TBA                  | TBA                          |
| [Brevis Pico](https://pico-docs.brevis.network)| TBA                  | TBA                          |

> TBA = To Be Added

#### General overview of features of proving schemes
<!---<img width="600" alt="Screenshot 2024-11-22 at 3 44 30 PM" src="https://github.com/user-attachments/assets/2768a9c5-7477-4e7b-86aa-f80a85818c2f">--->

| Proof System | Setup Complexity | Post-Quantum Resistance | Scalability | Parallel Execution | Features |
|--------------|-------------------|-----------|-------------------------|-------------|-------------------|
|     **Halo2**    |   Transparent  |      No (ECC based)    |     High    |     Moderate     |  Recursive SNARKs, Custom gates, Plonkish arithmetization    |
|   **Circom (Groth16)**   |  Trusted Setup     |     No (Pairing-based)  |   Moderate  |       High      | Efficient SNARK generation for circuits, good for small proof   | 
|   **Risc Zero**  |   Transparent   |         Yes       |     High    |       High      |  General-purpose zkVM, RISC-V architecture, supports arbitrary computations   |
|     **Jolt**     |   Both          |       Yes       |   Very High |     Very High   |  Sumcheck and lookup arguments, optimized for high-performance proof generation, upcoming [twist and shout](https://eprint.iacr.org/2025/105.pdf)   |
|  **Nexus zkVM**  |   Transparent   |       Partial     |   Moderate  |       High      | Privacy-focused zkVM, Ethereum compatibility, supports smart contract verification	    | 
|     **SP1**      |   Transparent   |         Yes       |   Very High |     Very High   | Optimized for rollups, efficient parallel proving, GPU-optimised, supports high-throughput applications, supports arbitrary computations, [whitepaper](https://www.provewith.us)   |
|    **Powdr**     |   Transparent   |         Yes       |     High    |       High      |  Dveloper-friendly, multiple proof systems, and zk-continuations for unbounded execution, minimal setup   |
| **Polygon Miden**   | Transparent            | Yes                     | High        | High                | STARK-based (Winterfell) zkVM, designed for program execution proofs, high scalability|
| **Aleo SnarkVM**    | Trusted Setup          | No (Pairing-based)      | Moderate    | High                | Privacy-preserving SNARK-based zkVM, supports private smart contracts    |
| **Zokrates**        | Trusted Setup          | No (Pairing-based)      | Moderate    | High                | High-level DSL for zk-SNARKs, extensive developer tooling, supports Groth16 and PLONK     |
| **Delphinus ZKWASM**| Transparent            | Yes                     | High        | High                | WebAssembly-based zkVM, supports proving WASM execution, ideal for cross-platform apps    |
| **Lita Valida**     | Transparent            | Yes                     | Moderate    | Moderate            | Modular chip, ensures the correctness of zk circuits, supports custom proofs  |
| **Eigen ZKVM**      | Transparent            | Yes                     | Very High   | High                | High-performance zkVM, designed for Ethereum scaling, supports recursive proofs           |
| **CairoVM**         | Transparent            | Yes                     | Very High   | High                | STARK-based VM, designed for StarkNet, supports general-purpose computation               |
| **Noir**            | Transparent            | Yes (STARK-based)       | High        | High                | High-level zkDSL, supports general-purpose zk circuits, STARK-based backend               |
| **Ola VM**          | Transparent            | Yes                     | High        | High                | STARK-based, Privacy-preserving zkVM, supports custom circuits, programmable scalable and private proof         |
| **Triton VM**       | Transparent            | Yes                     | High        | High                | Efficient recursive verification, Algebraic Execution Tables (AET) and Arithmetic Intermediate Representations (AIR) with a [STARK](https://neptune.cash/learn/stark-anatomy) proof system.   |
| **Lurk**            | Transparent            | Yes                     | Moderate    | Moderate            | zk-LISP interpreter, supports verifiable computations, ideal for symbolic execution       |
| **Ceno**            | Transparent            | Yes                     | High        | High                | Non-uniform prover based on GKR Protocol, [Segment and Parallel](https://eprint.iacr.org/2024/387.pdf) Zero-knowledge Virtual Machine                       |
| **Expander**        | Transparent            | Yes                     | Very High   | Very High           | GKR + [Libra](https://eprint.iacr.org/2019/317.pdf), Parallel computing    |
| **OpenVM**          | Transparent            | Yes                     | High        | High                | Modular architecture, supports general-purpose zkVM, adaptable to various use cases       |
| **Brevis**          | Transparent            | Yes                     | Very High   | High                | Focused on proof aggregation and recursion, [glue-and-coprocessor](https://vitalik.eth.limo/general/2024/09/02/gluecp.html) architecture, multiple proving backends, Coprocessor Integration               |

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
