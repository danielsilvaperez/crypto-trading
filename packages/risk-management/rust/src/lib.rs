//! # Risk Management
//!
//! A modular risk management library for algorithmic trading.

pub mod circuit_breaker;
pub mod kill_switch;
pub mod position_sizing;
pub mod types;

pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};
pub use kill_switch::{KillSwitch, KillSwitchCondition, KillSwitchConfig};
pub use position_sizing::{
    PositionSizer, KellySizing, FixedFractionalSizing, 
    VolatilityBasedSizing, SizingStrategy
};
pub use types::{RiskCheck, RiskLevel, RiskReport};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{
        circuit_breaker::*,
        kill_switch::*,
        position_sizing::*,
        types::*,
    };
}
