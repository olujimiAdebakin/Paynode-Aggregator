
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Payment proof submitted by provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProof {
    /// Proposal ID this payment is for
    pub proposal_id: String,
    
    /// Provider who made the payment
    pub provider: String,
    
    /// External payment transaction reference
    pub transaction_reference: String,
    
    /// When payment was made
    pub timestamp: DateTime<Utc>,
    
    /// Amount paid (in fiat currency)
    pub amount: String,
    
    /// Currency of payment
    pub currency: String,
    
    /// Provider's signature of the proof
    pub signature: String,
    
    /// Additional metadata (bank details, screenshots, etc.)
    pub metadata: Value,
}

impl PaymentProof {
    /// Verify the proof is for the correct proposal
    pub fn is_for_proposal(&self, proposal_id: &str) -> bool {
        self.proposal_id == proposal_id
    }
    
    /// Check if proof is recent (within last hour)
    pub fn is_recent(&self) -> bool {
        let now = Utc::now();
        let diff = now - self.timestamp;
        diff.num_hours() < 1
    }
}

/// Payment request sent to provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub proposal_id: String,
    pub order_id: String,
    pub provider: String,
    pub amount: String,
    pub currency: String,
    pub recipient_details: RecipientDetails,
    pub deadline: DateTime<Utc>,
}

/// Recipient details for off-chain payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientDetails {
    pub account_name: String,
    pub account_number: String,
    pub bank_name: Option<String>,
    pub bank_code: Option<String>,
    pub phone_number: Option<String>,
    pub additional_info: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_payment_proof() {
        let proof = PaymentProof {
            proposal_id: "0xproposal...".to_string(),
            provider: "0xprovider...".to_string(),
            transaction_reference: "TXN123456".to_string(),
            timestamp: Utc::now(),
            amount: "500000".to_string(),
            currency: "NGN".to_string(),
            signature: "0xsig...".to_string(),
            metadata: serde_json::json!({"bank": "GTBank"}),
        };
        
        assert!(proof.is_for_proposal("0xproposal..."));
        assert!(proof.is_recent());
    }
}