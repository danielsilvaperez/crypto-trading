use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Risk level classification
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Normal operation
    Normal = 0,
    /// Elevated risk - proceed with caution
    Elevated = 1,
    /// High risk - consider halting
    High = 2,
    /// Critical - halt trading
    Critical = 3,
}

impl RiskLevel {
    /// Check if trading should be allowed
    pub fn allows_trading(&self) -> bool {
        matches!(self, RiskLevel::Normal | RiskLevel::Elevated)
    }
    
    /// Get emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            RiskLevel::Normal => "🟢",
            RiskLevel::Elevated => "🟡",
            RiskLevel::High => "🟠",
            RiskLevel::Critical => "🔴",
        }
    }
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Risk check result
#[derive(Clone, Debug)]
pub struct RiskCheck {
    pub passed: bool,
    pub level: RiskLevel,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl RiskCheck {
    /// Create a passing check
    pub fn pass(message: impl Into<String>) -> Self {
        Self {
            passed: true,
            level: RiskLevel::Normal,
            message: message.into(),
            timestamp: Utc::now(),
        }
    }
    
    /// Create a failing check
    pub fn fail(level: RiskLevel, message: impl Into<String>) -> Self {
        Self {
            passed: false,
            level,
            message: message.into(),
            timestamp: Utc::now(),
        }
    }
    
    /// Create from boolean
    pub fn from_bool(passed: bool, message: impl Into<String>) -> Self {
        if passed {
            Self::pass(message)
        } else {
            Self::fail(RiskLevel::High, message)
        }
    }
}

/// Comprehensive risk report
#[derive(Clone, Debug, Default)]
pub struct RiskReport {
    pub overall_level: RiskLevel,
    pub checks: Vec<(String, RiskCheck)>,
    pub timestamp: DateTime<Utc>,
}

impl RiskReport {
    pub fn new() -> Self {
        Self {
            overall_level: RiskLevel::Normal,
            checks: Vec::new(),
            timestamp: Utc::now(),
        }
    }
    
    /// Add a check result
    pub fn add_check(mut self, name: impl Into<String>, check: RiskCheck) -> Self {
        let name = name.into();
        // Update overall level to highest seen
        if check.level > self.overall_level {
            self.overall_level = check.level;
        }
        self.checks.push((name, check));
        self
    }
    
    /// Check if all checks passed
    pub fn all_passed(&self) -> bool {
        self.checks.iter().all(|(_, check)| check.passed)
    }
    
    /// Get failed checks
    pub fn failed_checks(&self) -> Vec<&(String, RiskCheck)> {
        self.checks.iter()
            .filter(|(_, check)| !check.passed)
            .collect()
    }
}

/// Trading limits configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradingLimits {
    pub max_position_size: f64,
    pub max_daily_loss: f64,
    pub max_drawdown_pct: f64,
    pub max_open_positions: usize,
    pub max_consecutive_losses: usize,
}

impl Default for TradingLimits {
    fn default() -> Self {
        Self {
            max_position_size: 1000.0,
            max_daily_loss: 500.0,
            max_drawdown_pct: 10.0,
            max_open_positions: 5,
            max_consecutive_losses: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::Normal < RiskLevel::Critical);
        assert!(RiskLevel::High.allows_trading() == false);
        assert!(RiskLevel::Normal.allows_trading() == true);
    }
    
    #[test]
    fn test_risk_report() {
        let report = RiskReport::new()
            .add_check("Balance", RiskCheck::pass("Sufficient"))
            .add_check("Drawdown", RiskCheck::fail(RiskLevel::High, "Exceeded"));
        
        assert!(!report.all_passed());
        assert_eq!(report.overall_level, RiskLevel::High);
        assert_eq!(report.failed_checks().len(), 1);
    }
}
