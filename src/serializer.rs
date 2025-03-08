use bincode::{self, Encode, config::standard};
use serde::Serialize;
use sha3::{Digest, Sha3_256};

pub fn serialize<T>(value: &T) -> Vec<u8>
where
    T: ?Sized + Serialize + Encode,
{
    let config = standard();
    bincode::encode_to_vec(value, config).unwrap()
}

pub fn hash_str(value: &[u8]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(value);
    let hash_result = hasher.finalize();
    let hash_string = hash_result
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();
    hash_string
}
