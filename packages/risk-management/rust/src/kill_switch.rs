use chrono::{DateTime, Utc};

use crate::types::{RiskCheck, RiskLevel};

/// Kill switch configuration
#[derive(Clone, Debug)]
pub struct KillSwitchConfig {
    /// Minimum balance before kill
    pub balance_floor: f64,
    /// Maximum open positions
    pub max_open_positions: usize,
    /// Maximum consecutive API errors
    pub max_api_errors: usize,
    /// Enable manual override
    pub manual_override: bool,
}

impl Default for KillSwitchConfig {
    fn default() -> Self {
        Self {
            balance_floor: 100.0,
            max_open_positions: 10,
            max_api_errors: 5,
            manual_override: true,
        }
    }
}

/// Kill switch for emergency stops
#[derive(Clone, Debug)]
pub struct KillSwitch {
    config: KillSwitchConfig,
    current_balance: f64,
    open_positions: usize,
    consecutive_errors: usize,
    manually_triggered: bool,
    triggered_at: Option<DateTime<Utc>>,
    trigger_reason: Option<String>,
}

/// Conditions that can trigger kill switch
#[derive(Clone, Debug)]
pub enum KillSwitchCondition {
    BalanceFloor(f64),
    MaxPositions(usize),
    ApiErrors(usize),
    Manual(String),
}

impl KillSwitch {
    /// Create a new kill switch
    pub fn new() -> Self {
        Self::with_config(KillSwitchConfig::default())
    }
    
    /// Create with custom config
    pub fn with_config(config: KillSwitchConfig) -> Self {
        Self {
            config,
            current_balance: 0.0,
            open_positions: 0,
            consecutive_errors: 0,
            manually_triggered: false,
            triggered_at: None,
            trigger_reason: None,
        }
    }
    
    /// Configure balance floor
    pub fn balance_floor(mut self, floor: f64) -> Self {
        self.config.balance_floor = floor;
        self
    }
    
    /// Configure max positions
    pub fn max_positions(mut self, max: usize) -> Self {
        self.config.max_open_positions = max;
        self
    }
    
    /// Update current state
    pub fn update_state(&mut self, balance: f64, open_positions: usize) {
        self.current_balance = balance;
        self.open_positions = open_positions;
    }
    
    /// Record API error
    pub fn record_error(&mut self) {
        self.consecutive_errors += 1;
    }
    
    /// Clear errors (on successful operation)
    pub fn clear_errors(&mut self) {
        self.consecutive_errors = 0;
    }
    
    /// Check all conditions
    pub fn check(&self) -> RiskCheck {
        // Check if already triggered
        if self.triggered_at.is_some() {
            return RiskCheck::fail(
                RiskLevel::Critical,
                format!(
                    "Kill switch already triggered: {}",
                    self.trigger_reason.as_deref().unwrap_or("Unknown")
                )
            );
        }
        
        // Check balance floor
        if self.current_balance < self.config.balance_floor {
            return RiskCheck::fail(
                RiskLevel::Critical,
                format!(
                    "Balance below floor: {} < {}",
                    self.current_balance,
                    self.config.balance_floor
                )
            );
        }
        
        // Check max positions
        if self.open_positions > self.config.max_open_positions {
            return RiskCheck::fail(
                RiskLevel::Critical,
                format!(
                    "Max positions exceeded: {} > {}",
                    self.open_positions,
                    self.config.max_open_positions
                )
            );
        }
        
        // Check API errors
        if self.consecutive_errors >= self.config.max_api_errors {
            return RiskCheck::fail(
                RiskLevel::Critical,
                format!(
                    "Max API errors reached: {}",
                    self.consecutive_errors
                )
            );
        }
        
        // Check manual trigger
        if self.manually_triggered {
            return RiskCheck::fail(
                RiskLevel::Critical,
                "Manual kill switch triggered"
            );
        }
        
        RiskCheck::pass("Kill switch OK")
    }
    
    /// Check and trigger if needed
    pub fn check_and_trigger(&mut self) -> RiskCheck {
        let check = self.check();
        if !check.passed && self.triggered_at.is_none() {
            self.trigger(check.message.clone());
        }
        check
    }
    
    /// Trigger kill switch
    pub fn trigger(&mut self, reason: impl Into<String>) {
        self.triggered_at = Some(Utc::now());
        self.trigger_reason = Some(reason.into());
    }
    
    /// Manually trigger
    pub fn manual_trigger(&mut self, reason: impl Into<String>) {
        if self.config.manual_override {
            self.manually_triggered = true;
            self.trigger(format!("Manual: {}", reason.into()));
        }
    }
    
    /// Reset kill switch
    pub fn reset(&mut self) {
        self.triggered_at = None;
        self.trigger_reason = None;
        self.manually_triggered = false;
        self.consecutive_errors = 0;
    }
    
    /// Check if triggered
    pub fn is_triggered(&self) -> bool {
        self.triggered_at.is_some()
    }
    
    /// Get trigger reason
    pub fn trigger_reason(&self) -> Option<&str> {
        self.trigger_reason.as_deref()
    }
    
    /// Get status
    pub fn status(&self) -> KillSwitchStatus {
        KillSwitchStatus {
            is_triggered: self.is_triggered(),
            triggered_at: self.triggered_at,
            reason: self.trigger_reason.clone(),
            current_balance: self.current_balance,
            open_positions: self.open_positions,
            consecutive_errors: self.consecutive_errors,
        }
    }
}

impl Default for KillSwitch {
    fn default() -> Self {
        Self::new()
    }
}

/// Kill switch status
#[derive(Clone, Debug)]
pub struct KillSwitchStatus {
    pub is_triggered: bool,
    pub triggered_at: Option<DateTime<Utc>>,
    pub reason: Option<String>,
    pub current_balance: f64,
    pub open_positions: usize,
    pub consecutive_errors: usize,
}

/// Composite risk guard combining multiple safety mechanisms
pub struct RiskGuard {
    kill_switch: KillSwitch,
    checks: Vec<Box<dyn Fn() -> RiskCheck + Send + Sync>>,
}

impl RiskGuard {
    /// Create a new risk guard
    pub fn new(kill_switch: KillSwitch) -> Self {
        Self {
            kill_switch,
            checks: Vec::new(),
        }
    }
    
    /// Add a custom check
    pub fn add_check<F>(&mut self, check: F)
    where
        F: Fn() -> RiskCheck + Send + Sync + 'static,
    {
        self.checks.push(Box::new(check));
    }
    
    /// Run all checks
    pub fn check_all(&self) -> Vec<RiskCheck> {
        let mut results = Vec::new();
        
        // Kill switch first
        results.push(self.kill_switch.check());
        
        // Custom checks
        for check_fn in &self.checks {
            results.push(check_fn());
        }
        
        results
    }
    
    /// Check if all passed
    pub fn all_passed(&self) -> bool {
        self.check_all().iter().all(|c| c.passed)
    }
    
    /// Get first failure
    pub fn first_failure(&self) -> Option<RiskCheck> {
        self.check_all().into_iter().find(|c| !c.passed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_balance_floor() {
        let mut ks = KillSwitch::new()
            .balance_floor(100.0);
        
        ks.update_state(150.0, 0);
        assert!(ks.check().passed);
        
        ks.update_state(50.0, 0);
        assert!(!ks.check().passed);
    }
    
    #[test]
    fn test_max_positions() {
        let mut ks = KillSwitch::new()
            .max_positions(3);
        
        ks.update_state(1000.0, 2);
        assert!(ks.check().passed);
        
        ks.update_state(1000.0, 4);
        assert!(!ks.check().passed);
    }
    
    #[test]
    fn test_api_errors() {
        let mut ks = KillSwitch::new();
        
        for _ in 0..5 {
            ks.record_error();
        }
        
        assert!(!ks.check().passed);
    }
    
    #[test]
    fn test_manual_trigger() {
        let mut ks = KillSwitch::new();
        
        ks.manual_trigger("Emergency stop");
        assert!(ks.is_triggered());
        assert!(ks.trigger_reason().unwrap().contains("Emergency"));
    }
    
    #[test]
    fn test_reset() {
        let mut ks = KillSwitch::new()
            .balance_floor(100.0);
        
        ks.update_state(50.0, 0);
        ks.check_and_trigger();
        assert!(ks.is_triggered());
        
        ks.reset();
        assert!(!ks.is_triggered());
    }
    
    #[test]
    fn test_risk_guard() {
        let ks = KillSwitch::new();
        let mut guard = RiskGuard::new(ks);
        
        guard.add_check(|| RiskCheck::pass("Custom OK"));
        
        assert!(guard.all_passed());
        
        guard.add_check(|| RiskCheck::fail(RiskLevel::High, "Custom fail"));
        
        assert!(!guard.all_passed());
        assert!(guard.first_failure().is_some());
    }
}
