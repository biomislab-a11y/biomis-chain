//! Biometric Authentication Library
//! 
//! This library provides secure biometric authentication capabilities including:
//! - Biometric data encryption
//! - Fuzzy cryptographic operations for biometric matching
//! - Zero-knowledge proof generation for privacy-preserving authentication

pub mod encryption;
pub mod fuzzy_crypto;
pub mod zkp;

// Re-export main functionality for easier access
pub use encryption::encrypt_data;
pub use fuzzy_crypto::derive_key;
pub use zkp::generate_proof;

/// Main biometric authentication structure
#[derive(Debug, Clone)]
pub struct BiometricAuth {
    pub user_id: String,
    pub biometric_hash: String,
}

impl BiometricAuth {
    /// Create a new biometric authentication instance
    pub fn new(user_id: String, biometric_data: &str) -> Self {
        let biometric_hash = derive_key(biometric_data);
        Self {
            user_id,
            biometric_hash,
        }
    }
    
    /// Verify biometric data against stored hash
    pub fn verify(&self, biometric_data: &str) -> bool {
        let computed_hash = derive_key(biometric_data);
        self.biometric_hash == computed_hash
    }
    
    /// Generate encrypted biometric token
    pub fn generate_token(&self) -> String {
        let token_data = format!("{}:{}", self.user_id, self.biometric_hash);
        encrypt_data(&token_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_biometric_auth_creation() {
        let auth = BiometricAuth::new("user123".to_string(), "fingerprint_data");
        assert_eq!(auth.user_id, "user123");
        assert!(!auth.biometric_hash.is_empty());
    }
    
    #[test]
    fn test_biometric_verification() {
        let auth = BiometricAuth::new("user123".to_string(), "fingerprint_data");
        assert!(auth.verify("fingerprint_data"));
        assert!(!auth.verify("wrong_data"));
    }
    
    #[test]
    fn test_token_generation() {
        let auth = BiometricAuth::new("user123".to_string(), "fingerprint_data");
        let token = auth.generate_token();
        assert!(!token.is_empty());
    }
}