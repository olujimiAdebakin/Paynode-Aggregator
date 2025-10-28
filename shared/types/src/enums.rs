use serde::{Deserialize, Serialize};

/// Order classification tiers based on token amount ranges
/// These tiers determine order priority and matching strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "order_tier", rename_all = "UPPERCASE")]
pub enum OrderTier {
    /// Smallest orders: < 3,000 tokens
    Alpha,
    /// Small to medium orders: 3,000 - 10,000 tokens  
    Beta,
    /// Medium orders: 10,000 - 50,000 tokens
    Delta,
    /// Large orders: 50,000 - 100,000 tokens
    Omega,
    /// Largest orders: > 100,000 tokens
    Titan,
}

impl OrderTier {
    /// Determines the appropriate tier based on token amount and configured limits
    /// Used during order creation to classify orders for optimal provider matching
    /// 
    /// # Arguments
    /// * `amount` - Token amount as string to avoid precision loss with large numbers
    /// * `limits` - Tier limit configuration defining amount boundaries for each tier
    /// 
    /// # Returns
    /// * `OrderTier` - The classified tier for the given amount
    pub fn from_amount(amount: &str, limits: &TierLimits) -> Self {
        // Parse amount to u128, default to 0 if parsing fails to handle invalid input
        let amount: u128 = amount.parse().unwrap_or(0);
        
        // Determine tier based on amount ranges using configured limits
        // Orders are classified into tiers for optimized matching and risk management
        if amount <= limits.alpha {
            OrderTier::Alpha
        } else if amount <= limits.beta {
            OrderTier::Beta
        } else if amount <= limits.delta {
            OrderTier::Delta
        } else if amount <= limits.omega {
            OrderTier::Omega
        } else {
            OrderTier::Titan
        }
    }
    
    /// Returns the string representation for database storage
    /// Converts enum variant to uppercase string for PostgreSQL ENUM compatibility
    /// 
    /// # Returns
    /// * `&'static str` - Uppercase string representation of the tier
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderTier::Alpha => "ALPHA",
            OrderTier::Beta => "BETA",
            OrderTier::Delta => "DELTA",
            OrderTier::Omega => "OMEGA",
            OrderTier::Titan => "TITAN",
        }
    }
}

/// Configuration for tier amount boundaries
/// Defines the token amount thresholds that separate order tiers
/// These limits enable the system to classify orders for optimized matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierLimits {
    /// Maximum token amount for Alpha tier (smallest orders)
    pub alpha: u128,
    /// Maximum token amount for Beta tier (small to medium orders)
    pub beta: u128,
    /// Maximum token amount for Delta tier (medium orders)
    pub delta: u128,
    /// Maximum token amount for Omega tier (large orders)
    pub omega: u128,
    /// Minimum token amount for Titan tier (largest orders, no upper limit)
    pub titan: u128,
}

/// Order lifecycle status tracking order progression through the settlement pipeline
/// Each status represents a specific stage in the order fulfillment process
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    sqlx::Type
)]
#[sqlx(type_name = "order_status", rename_all = "UPPERCASE")]
pub enum OrderStatus {
    /// Order created but not yet accepted by any provider
    /// Orders remain in this state until a provider submits a settlement proposal
    Pending,
    /// Order accepted by a provider, awaiting settlement execution
    /// Transition to this state occurs when user accepts a provider's proposal
    Accepted,
    /// Order successfully completed and funds settled
    /// Final state indicating successful transaction completion
    Fulfilled,
    /// Order failed and funds were returned to user's refund address
    /// Can occur due to expiry, provider failure, or user cancellation
    Refunded,
    /// Order expired before being accepted or fulfilled
    /// Automatic transition based on order's expires_at timestamp
    Expired,
}

impl OrderStatus {
    /// Returns the string representation for database storage
    /// Converts enum variant to PostgreSQL ENUM compatible string
    /// 
    /// # Returns
    /// * `&'static str` - Uppercase string representation of the status
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::Pending => "PENDING",
            OrderStatus::Accepted => "ACCEPTED", 
            OrderStatus::Fulfilled => "FULFILLED",
            OrderStatus::Refunded => "REFUNDED",
            OrderStatus::Expired => "EXPIRED",
        }
    }
    
    /// Parses string from database into OrderStatus enum
    /// Used when converting database records to domain models
    /// 
    /// # Arguments
    /// * `s` - String representation from database
    /// 
    /// # Returns
    /// * `Option<Self>` - Some(OrderStatus) if valid, None if unrecognized
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "PENDING" => Some(OrderStatus::Pending),
            "ACCEPTED" => Some(OrderStatus::Accepted),
            "FULFILLED" => Some(OrderStatus::Fulfilled),
            "REFUNDED" => Some(OrderStatus::Refunded),
            "EXPIRED" => Some(OrderStatus::Expired),
            _ => None,
        }
    }
}

/// Proposal lifecycle status tracking provider responses to orders
/// Represents the state of settlement proposals between providers and users
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "proposal_status", rename_all = "UPPERCASE")]
pub enum ProposalStatus {
    /// Proposal created but not yet accepted/rejected by user
    /// Providers can submit proposals for pending orders
    Pending,
    /// Proposal accepted by user, awaiting on-chain execution
    /// User has selected this provider for order settlement
    Accepted,
    /// Proposal rejected by user
    /// User has declined this provider's settlement offer
    Rejected,
    /// Proposal expired before being accepted by user
    /// Automatic transition based on proposal deadline
    TimedOut,
    /// Proposal successfully executed and order fulfilled on-chain
    /// Final state indicating successful settlement execution
    Executed,
}

impl ProposalStatus {
    /// Returns the string representation for database storage
    /// 
    /// # Returns
    /// * `&'static str` - Uppercase string representation of the proposal status
    pub fn as_str(&self) -> &'static str {
        match self {
            ProposalStatus::Pending => "PENDING",
            ProposalStatus::Accepted => "ACCEPTED",
            ProposalStatus::Rejected => "REJECTED", 
            ProposalStatus::TimedOut => "TIMED_OUT",
            ProposalStatus::Executed => "EXECUTED",
        }
    }
}

/// Supported fiat currencies for off-ramping operations
/// Defines the target currencies users can receive for their crypto assets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Currency {
    /// Nigerian Naira - Primary currency for Nigerian users
    NGN,
    /// Kenyan Shilling - Primary currency for Kenyan users
    KES,
    /// Ghanaian Cedi - Primary currency for Ghanaian users
    GHS,
    /// South African Rand - Primary currency for South African users
    ZAR,
    /// US Dollar - International reserve currency
    USD,
    /// Euro - European Union currency
    EUR,
    /// Custom currency for future expansion and regional support
    /// Allows dynamic addition of new currencies without code changes
    Custom(String),
}

impl Currency {
    /// Returns the currency code as string for display and storage
    /// 
    /// # Returns
    /// * `String` - Currency code string (e.g., "NGN", "USD")
    pub fn as_str(&self) -> String {
        match self {
            Currency::NGN => "NGN".to_string(),
            Currency::KES => "KES".to_string(),
            Currency::GHS => "GHS".to_string(),
            Currency::ZAR => "ZAR".to_string(),
            Currency::USD => "USD".to_string(),
            Currency::EUR => "EUR".to_string(),
            Currency::Custom(s) => s.clone(),
        }
    }
    
    /// Parses currency code string into Currency enum
    /// Used when converting database strings to domain types
    /// Unknown currencies are stored as Custom variant for flexibility
    /// 
    /// # Arguments
    /// * `s` - Currency code string from database or user input
    /// 
    /// # Returns
    /// * `Currency` - Corresponding enum variant, Custom if unrecognized
    pub fn from_str(s: &str) -> Self {
        match s {
            "NGN" => Currency::NGN,
            "KES" => Currency::KES,
            "GHS" => Currency::GHS,
            "ZAR" => Currency::ZAR,
            "USD" => Currency::USD,
            "EUR" => Currency::EUR,
            _ => Currency::Custom(s.to_string()),
        }
    }
}