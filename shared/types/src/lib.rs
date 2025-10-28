//! Shared types for PayNode Aggregator
//! 
//! This crate contains all common data structures used across services.

pub mod enums;
pub mod error;
pub mod order;
pub mod provider;
pub mod proposal;
pub mod reputation;
pub mod payment;

// Re-export commonly used types
pub use enums::*;
pub use error::*;
pub use order::*;
pub use provider::*;
pub use proposal::*;
pub use reputation::*;
pub use payment::*;

// Helper functions
pub mod helpers {
    /// Convert hex string to bytes
    pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
        let hex = hex.trim_start_matches("0x");
        hex::decode(hex).unwrap_or_default()
    }
    
    /// Convert bytes to hex string with 0x prefix
    pub fn bytes_to_hex(bytes: &[u8]) -> String {
        format!("0x{}", hex::encode(bytes))
    }
    
    /// Validate Ethereum address format
    pub fn is_valid_address(addr: &str) -> bool {
        addr.starts_with("0x") && addr.len() == 42
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hex_helpers() {
        let bytes = vec![0x12, 0x34, 0x56];
        let hex = helpers::bytes_to_hex(&bytes);
        assert_eq!(hex, "0x123456");
        
        let decoded = helpers::hex_to_bytes(&hex);
        assert_eq!(decoded, bytes);
    }
    
    #[test]
    fn test_address_validation() {
        assert!(helpers::is_valid_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"));
        assert!(!helpers::is_valid_address("invalid"));
        assert!(!helpers::is_valid_address("0x123")); // Too short
    }
}