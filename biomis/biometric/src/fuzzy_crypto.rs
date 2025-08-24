use sha2::{Sha256, Digest};

pub fn derive_key(biometric_data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(biometric_data.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}
