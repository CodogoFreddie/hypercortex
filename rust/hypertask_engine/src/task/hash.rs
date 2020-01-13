use super::Task;
use blake2::{Blake2b, Digest};
use sha1::{Digest, Sha1};

impl Task {
    pub fn calculate_hash(&self) -> u64 {
        let mut hasher = Blake2b::new();
        let str_rep = serde_json::to_string(&self).expect("could not serialise task");
        hasher.input(str_rep);
        hasher.result()
    }
}
