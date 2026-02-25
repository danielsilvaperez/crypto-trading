use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Supported blockchain networks
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Chain {
    #[serde(rename = "ethereum")]
    Ethereum,
    #[serde(rename = "polygon")]
    Polygon,
    #[serde(rename = "bsc")]
    Bsc,
    #[serde(rename = "arbitrum")]
    Arbitrum,
    #[serde(rename = "optimism")]
    Optimism,
    #[serde(rename = "base")]
    Base,
    #[serde(rename = "solana")]
    Solana,
}

impl Chain {
    /// Get chain ID
    pub fn chain_id(&self) -> u64 {
        match self {
            Chain::Ethereum => 1,
            Chain::Polygon => 137,
            Chain::Bsc => 56,
            Chain::Arbitrum => 42161,
            Chain::Optimism => 10,
            Chain::Base => 8453,
            Chain::Solana => 0, // Solana doesn't use EVM chain IDs
        }
    }
    
    /// Check if chain is EVM-compatible
    pub fn is_evm(&self) -> bool {
        !matches!(self, Chain::Solana)
    }
    
    /// Get native token symbol
    pub fn native_token(&self) -> &'static str {
        match self {
            Chain::Ethereum => "ETH",
            Chain::Polygon => "MATIC",
            Chain::Bsc => "BNB",
            Chain::Arbitrum | Chain::Optimism | Chain::Base => "ETH",
            Chain::Solana => "SOL",
        }
    }
    
    /// Get block explorer URL
    pub fn explorer_url(&self) -> &'static str {
        match self {
            Chain::Ethereum => "https://etherscan.io",
            Chain::Polygon => "https://polygonscan.com",
            Chain::Bsc => "https://bscscan.com",
            Chain::Arbitrum => "https://arbiscan.io",
            Chain::Optimism => "https://optimistic.etherscan.io",
            Chain::Base => "https://basescan.org",
            Chain::Solana => "https://solscan.io",
        }
    }
}

impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Wallet abstraction
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub chain: Chain,
    pub label: Option<String>,
}

impl Wallet {
    pub fn new(address: impl Into<String>, chain: Chain) -> Self {
        Self {
            address: address.into(),
            chain,
            label: None,
        }
    }
    
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    
    /// Get explorer URL for this wallet
    pub fn explorer_url(&self) -> String {
        format!("{}/address/{}", self.chain.explorer_url(), self.address)
    }
}

/// Token metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub chain: Chain,
    pub logo_url: Option<String>,
}

/// Order side
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

impl OrderSide {
    pub fn opposite(&self) -> Self {
        match self {
            OrderSide::Buy => OrderSide::Sell,
            OrderSide::Sell => OrderSide::Buy,
        }
    }
}

impl std::fmt::Display for OrderSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderSide::Buy => write!(f, "BUY"),
            OrderSide::Sell => write!(f, "SELL"),
        }
    }
}

/// Order type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "stop")]
    Stop,
}

/// Order representation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: Option<String>,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub token_id: String,
    pub size: f64,
    pub price: f64,
    pub wallet: Wallet,
    pub created_at: DateTime<Utc>,
    pub status: OrderStatus,
}

/// Order status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "partially_filled")]
    PartiallyFilled,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
}

/// Position tracking
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub token_id: String,
    pub token: Token,
    pub size: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub wallet: Wallet,
    pub opened_at: DateTime<Utc>,
}

impl Position {
    /// Calculate unrealized PnL
    pub fn unrealized_pnl(&self) -> f64 {
        (self.current_price - self.entry_price) * self.size
    }
    
    /// Calculate unrealized PnL percentage
    pub fn unrealized_pnl_pct(&self) -> f64 {
        if self.entry_price == 0.0 {
            return 0.0;
        }
        (self.current_price - self.entry_price) / self.entry_price * 100.0
    }
    
    /// Calculate position value
    pub fn value(&self) -> f64 {
        self.size * self.current_price
    }
}

/// Transaction receipt
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas_used: u64,
    pub gas_price: u64,
    pub status: bool,
    pub timestamp: DateTime<Utc>,
    pub chain: Chain,
}

impl Transaction {
    /// Get explorer URL for this transaction
    pub fn explorer_url(&self) -> String {
        format!("{}/tx/{}", self.chain.explorer_url(), self.hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chain_id() {
        assert_eq!(Chain::Ethereum.chain_id(), 1);
        assert_eq!(Chain::Polygon.chain_id(), 137);
    }
    
    #[test]
    fn test_wallet_explorer_url() {
        let wallet = Wallet::new("0x123...", Chain::Ethereum);
        assert!(wallet.explorer_url().contains("etherscan.io"));
    }
    
    #[test]
    fn test_position_pnl() {
        let pos = Position {
            token_id: "0x...".to_string(),
            token: Token {
                address: "0x...".to_string(),
                symbol: "TEST".to_string(),
                name: "Test Token".to_string(),
                decimals: 18,
                chain: Chain::Ethereum,
                logo_url: None,
            },
            size: 100.0,
            entry_price: 1.0,
            current_price: 1.5,
            wallet: Wallet::new("0x...", Chain::Ethereum),
            opened_at: Utc::now(),
        };
        
        assert_eq!(pos.unrealized_pnl(), 50.0);
        assert_eq!(pos.unrealized_pnl_pct(), 50.0);
    }
}
