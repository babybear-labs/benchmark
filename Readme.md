# ZK Benchmark

### Goals

- Benchmark different ZK proofs for following operations.
    - Addition, Subtraction, Multiplication, Division
    - Loop
        - Variant 10, 100, 1000
    - Fibonacci
        - Variant 1, 10, 100,  1000, 10000
    - SHA256
        - Variant 1k, 10k Bytes
    - Pederson Hash
        - Variant 1k, 10k Bytes
    - Poseidon
        - Variant 1k, 10k Bytes
    - RPO Hash
        - Variant 1k, 10k Bytes
    - Keccak
        - Variant 1k, 10k Bytes
    - Merkle Tree Generation
    - Merkle Tree Membership / Inclusion
    - Merkle Tree Merge
    - nth Prime
        - Variant 1, 10, 100,  1000, 10000
    - ECDSA Verification
    - Elliptic Curve
    - BLS Verification
    - BLS Aggregation
- Metrics to measure
     - Prover time
     - Verifier time
     - Parallel execution
     - Proof size
     - CPU usage
     - Memory usage
     - Verifier gas consumtion
     - Cycles Count 
     - Developer friendliness
- Facilitate program execution and visualize findings from the web interface.

### Projects
- Scroll, Halo2 + KGZ
- Nexus zkVM
- Risc Zero
- Stakeware - Cairo
- SP1, Plonky3
- Jolt
- powdr
- Miden VM
- Noir
- Aleo
- ZoKrates
- ZKSync Era
- Arkworks
- Circom
- Mina
- Gnark

### Benchmark Machine Specification
- Macbook M1 Pro - Core 8 - Memory 8 GB
- AlmaLinux 8.10 - Core 16 - Memory 32 GB - Disk 1 TB

### Projects
- [Halo2](./halo2/)
- [Nexus](./nexus-zk-bench/)
- [SP1](./sp1/)
- [Jolt](./jolt/)
- [Zokrates](./zokrates/)

### Reports
- [Reports](./reports/)