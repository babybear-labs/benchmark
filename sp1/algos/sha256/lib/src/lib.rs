use alloy_sol_types::sol;
use sha2::{Sha256, Digest};

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint32 n;
        uint32 a;
    }
}

pub fn sha2(input: &[u8]) -> u32 {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let res = Into::<[u8; 32]>::into(result);
    1
}