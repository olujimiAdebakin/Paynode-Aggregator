
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::ProposalStatus;

/// Settlement proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    /// Proposal ID (bytes32 as hex)
    pub proposal_id: String,
    
    /// Order this proposal is for
    pub order_id: String,
    
    /// Provider who will fulfill this
    pub provider: String,
    
    /// Proposed fee in basis points
    pub proposed_fee_bps: u64,
    
    /// Current status
    pub status: ProposalStatus,
    
    /// When proposal was created
    pub created_at: DateTime<Utc>,
    
    /// When proposal expires if not accepted
    pub deadline: DateTime<Utc>,
    
    /// When provider accepted (if accepted)
    pub accepted_at: Option<DateTime<Utc>>,
    
    /// When settlement was executed (if executed)
    pub executed_at: Option<DateTime<Utc>>,
    
    /// Transaction hash of execution
    pub tx_hash: Option<String>,
}

impl Proposal {
    /// Check if proposal has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.deadline && self.status == ProposalStatus::Pending
    }
    
    /// Accept the proposal
    pub fn accept(&mut self) {
        self.status = ProposalStatus::Accepted;
        self.accepted_at = Some(Utc::now());
    }
    
    /// Reject the proposal
    pub fn reject(&mut self) {
        self.status = ProposalStatus::Rejected;
    }
    
    /// Mark as executed
    pub fn execute(&mut self, tx_hash: String) {
        self.status = ProposalStatus::Executed;
        self.executed_at = Some(Utc::now());
        self.tx_hash = Some(tx_hash);
    }
}

/// Proposal created event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalCreatedEvent {
    pub proposal_id: String,
    pub order_id: String,
    pub provider: String,
    pub proposed_fee_bps: u64,
    pub deadline: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
}

/// Proposal accepted event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalAcceptedEvent {
    pub proposal_id: String,
    pub provider: String,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    
    #[test]
    fn test_proposal_lifecycle() {
        let mut proposal = Proposal {
            proposal_id: "0xproposal...".to_string(),
            order_id: "0xorder...".to_string(),
            provider: "0xprovider...".to_string(),
            proposed_fee_bps: 300,
            status: ProposalStatus::Pending,
            created_at: Utc::now(),
            deadline: Utc::now() + Duration::minutes(5),
            accepted_at: None,
            executed_at: None,
            tx_hash: None,
        };
        
        assert_eq!(proposal.status, ProposalStatus::Pending);
        
        proposal.accept();
        assert_eq!(proposal.status, ProposalStatus::Accepted);
        assert!(proposal.accepted_at.is_some());
        
        proposal.execute("0xtx123...".to_string());
        assert_eq!(proposal.status, ProposalStatus::Executed);
        assert!(proposal.executed_at.is_some());
    }
}