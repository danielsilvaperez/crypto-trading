/// Position sizing strategies for trading

/// Sizing strategy trait
pub trait SizingStrategy: Send + Sync {
    /// Calculate position size given available capital
    fn calculate(&self, capital: f64) -> f64;
    
    /// Get strategy name
    fn name(&self) -> &'static str;
}

/// Position sizer with configurable strategy
pub struct PositionSizer {
    strategy: Box<dyn SizingStrategy>,
    min_size: f64,
    max_size: f64,
}

impl PositionSizer {
    /// Create with a strategy
    pub fn new(strategy: impl SizingStrategy + 'static) -> Self {
        Self {
            strategy: Box::new(strategy),
            min_size: 0.0,
            max_size: f64::MAX,
        }
    }
    
    /// Set minimum position size
    pub fn with_min_size(mut self, min: f64) -> Self {
        self.min_size = min;
        self
    }
    
    /// Set maximum position size
    pub fn with_max_size(mut self, max: f64) -> Self {
        self.max_size = max;
        self
    }
    
    /// Calculate position size with bounds
    pub fn calculate(&self, capital: f64) -> f64 {
        let size = self.strategy.calculate(capital);
        size.clamp(self.min_size, self.max_size)
    }
}

/// Kelly Criterion sizing
/// Optimal growth: f* = (p*b - q) / b
/// where p = win rate, q = loss rate, b = avg win / avg loss
#[derive(Clone, Debug)]
pub struct KellySizing {
    win_rate: f64,
    avg_win: f64,
    avg_loss: f64,
}

impl KellySizing {
    pub fn new() -> Self {
        Self {
            win_rate: 0.5,
            avg_win: 1.0,
            avg_loss: 1.0,
        }
    }
    
    pub fn win_rate(mut self, rate: f64) -> Self {
        self.win_rate = rate.clamp(0.0, 1.0);
        self
    }
    
    pub fn avg_win(mut self, win: f64) -> Self {
        self.avg_win = win.max(0.01);
        self
    }
    
    pub fn avg_loss(mut self, loss: f64) -> Self {
        self.avg_loss = loss.max(0.01);
        self
    }
    
    /// Calculate full Kelly fraction
    pub fn kelly_fraction(&self) -> f64 {
        let loss_rate = 1.0 - self.win_rate;
        let b = self.avg_win / self.avg_loss;
        
        let f = (self.win_rate * b - loss_rate) / b;
        f.max(0.0)
    }
    
    /// Calculate position size
    pub fn calculate_size(&self, capital: f64, half_kelly: bool) -> f64 {
        let kelly = self.kelly_fraction();
        let fraction = if half_kelly { kelly * 0.5 } else { kelly };
        capital * fraction
    }
}

impl SizingStrategy for KellySizing {
    fn calculate(&self, capital: f64) -> f64 {
        self.calculate_size(capital, false)
    }
    
    fn name(&self) -> &'static str {
        "Kelly Criterion"
    }
}

impl Default for KellySizing {
    fn default() -> Self {
        Self::new()
    }
}

/// Fixed fractional sizing (risk fixed % per trade)
#[derive(Clone, Debug)]
pub struct FixedFractionalSizing {
    risk_per_trade_pct: f64,
}

impl FixedFractionalSizing {
    pub fn new(risk_per_trade_pct: f64) -> Self {
        Self {
            risk_per_trade_pct: risk_per_trade_pct.clamp(0.0, 100.0),
        }
    }
    
    /// Conservative: 1% risk per trade
    pub fn conservative() -> Self {
        Self::new(1.0)
    }
    
    /// Moderate: 2% risk per trade
    pub fn moderate() -> Self {
        Self::new(2.0)
    }
    
    /// Aggressive: 5% risk per trade
    pub fn aggressive() -> Self {
        Self::new(5.0)
    }
}

impl SizingStrategy for FixedFractionalSizing {
    fn calculate(&self, capital: f64) -> f64 {
        capital * (self.risk_per_trade_pct / 100.0)
    }
    
    fn name(&self) -> &'static str {
        "Fixed Fractional"
    }
}

/// Volatility-based sizing (ATR-based)
#[derive(Clone, Debug)]
pub struct VolatilityBasedSizing {
    atr_period: usize,
    risk_per_trade_pct: f64,
    current_atr: f64,
}

impl VolatilityBasedSizing {
    pub fn new(atr: f64) -> Self {
        Self {
            atr_period: 14,
            risk_per_trade_pct: 2.0,
            current_atr: atr.max(0.0001),
        }
    }
    
    pub fn with_risk_pct(mut self, pct: f64) -> Self {
        self.risk_per_trade_pct = pct.clamp(0.1, 10.0);
        self
    }
    
    pub fn update_atr(&mut self, atr: f64) {
        self.current_atr = atr.max(0.0001);
    }
    
    /// Calculate position size based on ATR stop
    pub fn calculate_with_stop(&self, capital: f64, entry_price: f64, stop_price: f64) -> f64 {
        let risk_amount = capital * (self.risk_per_trade_pct / 100.0);
        let price_risk = (entry_price - stop_price).abs();
        
        if price_risk <= 0.0 {
            return 0.0;
        }
        
        risk_amount / price_risk
    }
}

impl SizingStrategy for VolatilityBasedSizing {
    fn calculate(&self, capital: f64) -> f64 {
        // Simplified: reduce size as volatility increases
        let volatility_factor = 1.0 / (1.0 + self.current_atr);
        capital * (self.risk_per_trade_pct / 100.0) * volatility_factor
    }
    
    fn name(&self) -> &'static str {
        "Volatility-Based"
    }
}

/// Anti-martingale (increase size on wins, decrease on losses)
#[derive(Clone, Debug)]
pub struct AntiMartingaleSizing {
    base_size: f64,
    consecutive_wins: usize,
    consecutive_losses: usize,
    win_multiplier: f64,
    loss_divisor: f64,
    max_multiplier: f64,
}

impl AntiMartingaleSizing {
    pub fn new(base_size: f64) -> Self {
        Self {
            base_size,
            consecutive_wins: 0,
            consecutive_losses: 0,
            win_multiplier: 1.5,
            loss_divisor: 2.0,
            max_multiplier: 4.0,
        }
    }
    
    pub fn record_result(&mut self, is_win: bool) {
        if is_win {
            self.consecutive_wins += 1;
            self.consecutive_losses = 0;
        } else {
            self.consecutive_losses += 1;
            self.consecutive_wins = 0;
        }
    }
    
    fn current_multiplier(&self) -> f64 {
        let multiplier = (self.win_multiplier.powi(self.consecutive_wins as i32))
            / (self.loss_divisor.powi(self.consecutive_losses as i32));
        
        multiplier.clamp(0.25, self.max_multiplier)
    }
}

impl SizingStrategy for AntiMartingaleSizing {
    fn calculate(&self, capital: f64) -> f64 {
        let _ = capital; // unused
        self.base_size * self.current_multiplier()
    }
    
    fn name(&self) -> &'static str {
        "Anti-Martingale"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_kelly_criterion() {
        let kelly = KellySizing::new()
            .win_rate(0.6)
            .avg_win(100.0)
            .avg_loss(50.0);
        
        // f* = (0.6 * 2 - 0.4) / 2 = 0.4
        assert!((kelly.kelly_fraction() - 0.4).abs() < 0.001);
        
        // Half-Kelly on $10k capital
        let size = kelly.calculate_size(10000.0, true);
        assert!((size - 2000.0).abs() < 0.001);
    }
    
    #[test]
    fn test_fixed_fractional() {
        let sizer = FixedFractionalSizing::moderate();
        assert_eq!(sizer.calculate(10000.0), 200.0);
    }
    
    #[test]
    fn test_position_sizer_bounds() {
        let sizer = PositionSizer::new(FixedFractionalSizing::aggressive())
            .with_min_size(100.0)
            .with_max_size(300.0);
        
        // Should be clamped to max
        assert_eq!(sizer.calculate(10000.0), 300.0);
        
        // Should be clamped to min
        assert_eq!(sizer.calculate(1000.0), 100.0);
    }
    
    #[test]
    fn test_volatility_sizing() {
        let sizer = VolatilityBasedSizing::new(0.1)
            .with_risk_pct(2.0);
        
        let size = sizer.calculate(10000.0);
        // Higher volatility = smaller position
        assert!(size < 200.0);
    }
    
    #[test]
    fn test_anti_martingale() {
        let mut sizer = AntiMartingaleSizing::new(100.0);
        assert_eq!(sizer.calculate(1000.0), 100.0);
        
        sizer.record_result(true);
        sizer.record_result(true);
        // 1.5^2 = 2.25x
        assert_eq!(sizer.calculate(1000.0), 225.0);
        
        sizer.record_result(false);
        // Reset to base
        assert_eq!(sizer.calculate(1000.0), 50.0);
    }
}
