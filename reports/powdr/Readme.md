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
```rust
use sha2::{Digest, Sha256};
use hex;

pub struct MerkleTree {
    leaves: Vec<[u8; 32]>,
    tree_levels: Vec<Vec<[u8; 32]>>,
}

impl MerkleTree {
    pub fn new(leaves: Vec<[u8; 32]>) -> Self {
        let mut tree = MerkleTree {
            leaves,
            tree_levels: Vec::new(),
        };
        tree.build_tree();
        tree
    }

    fn build_tree(&mut self) {
        if self.leaves.is_empty() {
            return;
        }

        let mut curr_level: Vec<[u8; 32]> = self.leaves.iter().map(|leaf| sha256_hash(leaf)).collect();
        self.tree_levels.push(curr_level.clone());

        while curr_level.len() > 1 {
            let mut next_level: Vec<[u8; 32]> = Vec::with_capacity((curr_level.len() + 1) / 2);
            for chunk in curr_level.chunks(2) {
                if chunk.len() == 2 {
                    let combined = [&chunk[0][..], &chunk[1][..]].concat();
                    next_level.push(sha256_hash(&combined));
                } else {
                    next_level.push(chunk[0]);
                }
            }
            self.tree_levels.push(next_level.clone());
            curr_level = next_level;
        }
    }

    pub fn get_root(&self) -> [u8; 32] {
        if self.tree_levels.is_empty() {
            return [0; 32];
        }
        self.tree_levels.last().unwrap()[0]
    }

    pub fn get_proof(&self, index: usize) -> Vec<[u8; 32]> {
        let mut proof = Vec::new();
        let mut idx = index;

        for level in &self.tree_levels {
            if level.len() <= 1 {
                break;
            }

            let sibling_index = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
            if sibling_index < level.len() {
                proof.push(level[sibling_index]);
            }

            idx /= 2;
        }

        proof
    }
}

fn sha256_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.into()
}

#[no_mangle]
pub fn generate_merkle_proof(leaves_base_2: u32) -> u32 {
    let total_leaves = 2_usize.pow(leaves_base_2);
    let leaves = vec![[0u8; 32]; total_leaves];
    let tree = MerkleTree::new(leaves);
    let root = hex::encode(tree.get_root());
    let leaf_idx: usize = 1;
    let proof = tree.get_proof(leaf_idx).iter().map(|hash| hex::encode(hash)).collect::<Vec<String>>();
    println!("Merkle tree with {} leaves has root {}", total_leaves, root);
    println!("Proof for leaf at index {} is {:?}", leaf_idx, proof);
    1
}
```
### Result

- Input 2
```shell
Running powdr-riscv executor in fast mode...
Merkle tree with 4 leaves has root 1223349a40d2ee10bd1bebb5889ef8018c8bc13359ed94b387810af96c6e4268
Proof for leaf at index 1 is ["66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925", "2eeb74a6177f588d80c0c752b99556902ddf9682d0b906f5aa2adbaf8466a4e9"]
Fast executor took: 820.184375ms
Trace length: 76906
Creating program ZK setup. This has to be done only once per program.
Merkle tree with 4 leaves has root 1223349a40d2ee10bd1bebb5889ef8018c8bc13359ed94b387810af96c6e4268
Proof for leaf at index 1 is ["66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925", "2eeb74a6177f588d80c0c752b99556902ddf9682d0b906f5aa2adbaf8466a4e9"]
Merkle tree with 4 leaves has root 1223349a40d2ee10bd1bebb5889ef8018c8bc13359ed94b387810af96c6e4268
Proof for leaf at index 1 is ["66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925", "2eeb74a6177f588d80c0c752b99556902ddf9682d0b906f5aa2adbaf8466a4e9"]
Running witness and proof generation for 1 chunks...
[00:00:08 (ETA: 00:00:14)] ████████░░░░░░░░░░░░ 41% - 23084 rows/s, 1785k identities/s, 89% progressMerkle tree with 4 leaves has root 1223349a40d2ee10bd1bebb5889ef8018c8bc13
[00:00:08 (ETA: 00:00:13)] ████████░░░░░░░░░░░░ 43% - 24509 rows/s, 1877k identities/s, 89% progressProof for leaf at index 1 is ["66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925", "2eeb74a6177f588d80c0c752b99556902ddf9682d0b906f5aa2adbaf8466a4
[00:00:08 (ETA: 00:00:00)] ████████████████████ 100% - 131216 rows/s, 9578k identities/s, 100% progressGenerating proof...
Proof generation took: 10.842923834s
```


## Poseidon
```rust
use starknet_crypto::poseidon_hash_single;
use starknet_types_core::felt::Felt;

/// Ref: https://github.com/xJonathanLEI/starknet-rs/blob/master/starknet-crypto/benches/poseidon_hash.rs
#[no_mangle]
pub fn poseidon_hash() {
    let input = &[5u8; 32];
    let elem = Felt::from_bytes_be(input);
    let hash = poseidon_hash_single(elem).to_fixed_hex_string();
}
```

### Results

- Input: 32 Bytes
```shell

1st Run

Running powdr-riscv executor in fast mode...
Fast executor took: 509.251917ms
Trace length: 286652
Loading program ZK setup.
Running witness and proof generation for 2 chunks...
[00:00:22 (ETA: 00:00:00)] ████████████████████ 100% - 20454 rows/s, 1637k identities/s, 89% progressGenerating proof...
Proof generation took: 13.594827208s
[00:00:08 (ETA: 00:00:00)] ████████████████████ 100% - 124533 rows/s, 9090k identities/s, 100% progressGenerating proof...
Proof generation took: 7.946486125s

2nd Run

Running powdr-riscv executor in fast mode...
Fast executor took: 512.843792ms
Trace length: 286652
Creating program ZK setup. This has to be done only once per program.
Running witness and proof generation for 2 chunks...
[00:00:28 (ETA: 00:00:00)] ████████████████████ 100% - 18141 rows/s, 1452k identities/s, 89% progressGenerating proof...
Proof generation took: 28.434556708s
[00:00:09 (ETA: 00:00:00)] ████████████████████ 100% - 111544 rows/s, 8142k identities/s, 100% progressGenerating proof...
Proof generation took: 9.714906s
```


- Input; 100 Bytes
```shell
Fast executor took: 1.063086s
Trace length: 836956
Creating program ZK setup. This has to be done only once per program.
Running witness and proof generation for 4 chunks...
[00:00:47 (ETA: 00:00:00)] ████████████████████ 100% - 11408 rows/s, 916k identities/s, 89% progress                     Generating proof...
Proof generation took: 30.177880459s
[00:00:37 (ETA: 00:00:00)] ████████████████████ 100% - 12310 rows/s, 989k identities/s, 89% progress                     Generating proof...
Proof generation took: 46.75899225s
[00:00:45 (ETA: 00:00:00)] ████████████████████ 100% - 11411 rows/s, 900k identities/s, 89% progress                     Generating proof...
Proof generation took: 45.871270584s
[00:00:22 (ETA: 00:00:00)] ████████████████████ 100% - 77261 rows/s, 5640k identities/s, 100% progress                   Generating proof...
Proof generation took: 48.474264875s
```

- Input; 1k Bytes
```shell

```