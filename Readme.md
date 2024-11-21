# ZK Benchmark Project

## Overview

This repository hosts the source code and resources for benchmarking various Zero-Knowledge (ZK) proofs. It builds upon the original repositoritories of Halo2, Nexus, SP1, Jolt, Zokrates, and Powdr with our implementations of proofs including SHA256, Fibonacci, and Merkle Merge functionalities.

Our aim is to benchmark the performance of these proofs, comparing them in terms of time, space, and other metrics. This comprehensive comparison will not only highlight the most popular ZK proofs and their implementations but also deepen our understanding of their inner workings and proving systems.

## Goals

- **Benchmark Operations:**
  - **Fibonacci:** Variants 1, 10, 100, 1000, 10000
  - **SHA256:** Variants 1k, 10k Bytes
  - **Poseidon:** 

- **Additional Benchmarks:**
  - Arithmetic Operations: Addition, Subtraction, Multiplication, Division
  - Loop: Variants 10, 100, 1000
  - Hash Functions: Pederson, Poseidon, RPO, Keccak (Variants 1k, 10k Bytes)
  - Merkle Tree: Generation, Membership/Inclusion, Merge
  - nth Prime: Variants 1, 10, 100, 1000, 10000
  - Cryptographic Verifications: ECDSA, Elliptic Curve, BLS Verification, BLS Aggregation

## Proving Systems

- Halo2
- Nexus zkVM
- Risc Zero
- Groth16 (Circom)
- SP1
- Jolt
- Powdr

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

## Benchmark Machine Specifications

- **Macbook M1 Pro:** Core 8, Memory 8 GB
- **AlmaLinux 8.10:** Core 16, Memory 32 GB, Disk 1 TB
- **Windows 11:** Core 16, Memory 32 GB, Disk 1 TB

## Project Structure

- [Halo2](./halo2/)
- [Nexus](./nexus/)
- [SP1](./sp1/)
- [Jolt](./jolt/)
- [Zokrates](./zokrates/)
- [Powdr](./powdr/)
- [Groth16](./groth16/)
- [Risc Zero](./risc-zero/)

## Reports
The reports can be found in the [reports](./reports/) directory.
- [Reports](./reports/)
  - [Halo2](./reports/halo2/)
  - [Nexus](./reports/nexus/)
  - [Jolt](./reports/jolt/)
  - [SP1](./reports/sp1/)
  - [Zokrates](./reports/zokrates/)
  - [Powdr](./reports/powdr/)

## Installation

Each project has its own README file with instructions on how to install the dependencies and run the benchmarks.

## Contributors

This project is part of the ZK and Scaling Bootcamp organized by Encode club.  
Tanmoy : https://x.com/mtanm0y
Rosemary :https://x.com/rosekoikara
Anubhav : https://x.com/anubhav_io
Yogesh : https://x.com/yogesh_eth

## Acknowledgments
Special thanks to the ZK and Scaling Bootcamp team for organizing this event and providing us with the opportunity to learn and contribute to the ZK space.
