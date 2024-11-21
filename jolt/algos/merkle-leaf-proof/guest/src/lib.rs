#![no_main]
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
#[jolt::provable]
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