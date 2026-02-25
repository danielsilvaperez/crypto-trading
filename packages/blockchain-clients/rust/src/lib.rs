//! # Blockchain Clients
//!
//! Reusable blockchain client libraries for interacting with various networks
//! and services. Provides unified interfaces for DeFi protocols, exchanges,
//! and data providers.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod types;

#[cfg(feature = "evm")]
#[cfg_attr(docsrs, doc(cfg(feature = "evm")))]
pub mod chains;

#[cfg(feature = "polymarket")]
#[cfg_attr(docsrs, doc(cfg(feature = "polymarket")))]
pub mod exchanges;

pub use error::{Error, Result};
pub use types::{Chain, Token, Wallet, Order, OrderSide, OrderType, Position};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{
        error::{Error, Result},
        types::*,
    };
    
    #[cfg(feature = "evm")]
    pub use crate::chains::evm::*;
    
    #[cfg(feature = "polymarket")]
    pub use crate::exchanges::polymarket::*;
}

/// Version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
