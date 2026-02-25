use chrono::{DateTime, Duration, Utc};
use std::collections::VecDeque;

use crate::types::{RiskCheck, RiskLevel};

/// Circuit breaker configuration
#[derive(Clone, Debug)]
pub struct CircuitBreakerConfig {
    /// Maximum consecutive losses before halting
    pub max_consecutive_losses: usize,
    /// Maximum daily drawdown percentage
    pub max_daily_drawdown_pct: f64,
    /// Cooldown duration after trigger
    pub cooldown_duration: Duration,
    /// Minimum trades before evaluating
    pub min_trades_for_evaluation: usize,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            max_consecutive_losses: 3,
            max_daily_drawdown_pct: 5.0,
            cooldown_duration: Duration::minutes(30),
            min_trades_for_evaluation: 5,
        }
    }
}

/// Circuit breaker for trading halts
#[derive(Clone, Debug)]
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    trade_history: VecDeque<TradeRecord>,
    consecutive_losses: usize,
    daily_pnl: f64,
    last_reset: DateTime<Utc>,
    triggered_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug)]
struct TradeRecord {
    timestamp: DateTime<Utc>,
    pnl: f64,
    was_profitable: bool,
}

impl CircuitBreaker {
    /// Create a new circuit breaker with default config
    pub fn new() -> Self {
        Self::with_config(CircuitBreakerConfig::default())
    }
    
    /// Create with custom config
    pub fn with_config(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            trade_history: VecDeque::new(),
            consecutive_losses: 0,
            daily_pnl: 0.0,
            last_reset: Utc::now(),
            triggered_at: None,
        }
    }
    
    /// Configure max consecutive losses
    pub fn max_consecutive_losses(mut self, max: usize) -> Self {
        self.config.max_consecutive_losses = max;
        self
    }
    
    /// Configure max daily drawdown
    pub fn max_daily_drawdown_pct(mut self, max: f64) -> Self {
        self.config.max_daily_drawdown_pct = max;
        self
    }
    
    /// Configure cooldown duration
    pub fn cooldown_duration(mut self, duration: Duration) -> Self {
        self.config.cooldown_duration = duration;
        self
    }
    
    /// Record a trade outcome
    pub fn record_trade(&mut self, pnl: f64) {
        let now = Utc::now();
        
        // Reset daily stats if needed
        if now.date_naive() != self.last_reset.date_naive() {
            self.daily_pnl = 0.0;
            self.last_reset = now;
        }
        
        self.daily_pnl += pnl;
        
        if pnl < 0.0 {
            self.consecutive_losses += 1;
        } else {
            self.consecutive_losses = 0;
        }
        
        self.trade_history.push_back(TradeRecord {
            timestamp: now,
            pnl,
            was_profitable: pnl >= 0.0,
        });
        
        // Trim old history (keep last 100)
        while self.trade_history.len() > 100 {
            self.trade_history.pop_front();
        }
    }
    
    /// Check if trading is allowed
    pub fn check(&self) -> RiskCheck {
        // Check if in cooldown
        if let Some(triggered) = self.triggered_at {
            let elapsed = Utc::now() - triggered;
            if elapsed < self.config.cooldown_duration {
                let remaining = self.config.cooldown_duration - elapsed;
                return RiskCheck::fail(
                    RiskLevel::Critical,
                    format!("Circuit breaker active. {}s remaining", remaining.num_seconds())
                );
            }
        }
        
        // Check consecutive losses
        if self.consecutive_losses >= self.config.max_consecutive_losses {
            return RiskCheck::fail(
                RiskLevel::Critical,
                format!(
                    "Max consecutive losses reached: {}",
                    self.consecutive_losses
                )
            );
        }
        
        // Check daily drawdown (would need starting balance for accurate %)
        // This is a simplified check
        
        RiskCheck::pass("Circuit breaker OK")
    }
    
    /// Check and trigger if needed
    pub fn check_and_trigger(&mut self) -> RiskCheck {
        let check = self.check();
        if !check.passed && self.triggered_at.is_none() {
            self.triggered_at = Some(Utc::now());
        }
        check
    }
    
    /// Manually trigger the circuit breaker
    pub fn trigger(&mut self, reason: impl Into<String>) -> RiskCheck {
        self.triggered_at = Some(Utc::now());
        RiskCheck::fail(RiskLevel::Critical, reason)
    }
    
    /// Reset the circuit breaker
    pub fn reset(&mut self) {
        self.triggered_at = None;
        self.consecutive_losses = 0;
    }
    
    /// Get current status
    pub fn status(&self) -> CircuitBreakerStatus {
        CircuitBreakerStatus {
            is_open: self.triggered_at.is_some() && self.check().passed == false,
            consecutive_losses: self.consecutive_losses,
            daily_pnl: self.daily_pnl,
            triggered_at: self.triggered_at,
        }
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new()
    }
}

/// Circuit breaker status
#[derive(Clone, Debug)]
pub struct CircuitBreakerStatus {
    pub is_open: bool,
    pub consecutive_losses: usize,
    pub daily_pnl: f64,
    pub triggered_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_consecutive_losses() {
        let mut cb = CircuitBreaker::new()
            .max_consecutive_losses(3);
        
        assert!(cb.check().passed);
        
        cb.record_trade(-10.0);
        cb.record_trade(-20.0);
        assert!(cb.check().passed);
        
        cb.record_trade(-15.0);
        assert!(!cb.check().passed); // 3 consecutive losses
    }
    
    #[test]
    fn test_reset_on_profit() {
        let mut cb = CircuitBreaker::new()
            .max_consecutive_losses(2);
        
        cb.record_trade(-10.0);
        cb.record_trade(-20.0);
        assert!(!cb.check().passed);
        
        cb.record_trade(5.0); // Reset
        assert!(cb.check().passed);
    }
    
    #[test]
    fn test_cooldown() {
        let mut cb = CircuitBreaker::new()
            .max_consecutive_losses(1)
            .cooldown_duration(Duration::seconds(0)); // Instant cooldown
        
        cb.record_trade(-10.0);
        assert!(!cb.check().passed);
        
        // Should pass after cooldown
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(cb.check().passed);
    }
}
