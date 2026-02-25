# Risk Management

A modular risk management library for algorithmic trading. Provides circuit breakers, position sizing, kill switches, and other safety mechanisms.

## Features

- 🛑 **Circuit Breakers**: Halt trading on adverse conditions
- 📊 **Position Sizing**: Kelly criterion, fixed fractional, volatility-based
- ⚡ **Kill Switch**: Emergency stops for various conditions
- 📈 **Volatility Guards**: Adjust to market conditions
- 💰 **Drawdown Limits**: Maximum loss protection
- 🔔 **Alert Integration**: Notify when limits hit

## Quick Start

### Rust

```rust
use risk_management::{
    CircuitBreaker, KillSwitch, PositionSizer, 
    KellySizing, RiskGuard
};

// Circuit breaker on consecutive losses
let mut cb = CircuitBreaker::new()
    .max_consecutive_losses(3)
    .cooldown_duration(Duration::minutes(30));

// Check before trading
if cb.check().is_ok() {
    // Execute trade
} else {
    // Trading halted
}

// Kelly position sizing
let kelly = KellySizing::new()
    .win_rate(0.55)
    .avg_win(100.0)
    .avg_loss(50.0);
    
let position_size = kelly.calculate(half_kelly = true);
```

### Python

```python
from risk_management import CircuitBreaker, KellySizer, KillSwitch

# Circuit breaker
cb = CircuitBreaker(
    max_consecutive_losses=3,
    cooldown_minutes=30
)

if cb.check():
    # Execute trade
    pass
else:
    print(f"Trading halted: {cb.status()}")

# Kelly sizing
sizer = KellySizer(win_rate=0.55, avg_win=100, avg_loss=50)
position_size = sizer.calculate(half_kelly=True)
```

## Components

### CircuitBreaker

Halts trading based on:
- Consecutive losses
- Daily drawdown limits
- Volatility spikes
- Time-based cooldowns

### KillSwitch

Emergency stops for:
- Balance floor breach
- Maximum open positions
- API errors threshold
- Manual override

### Position Sizing

Strategies available:
- **Kelly Criterion**: Optimal growth
- **Fixed Fractional**: Risk fixed % per trade
- **Volatility-Based**: ATR-adjusted sizing
- **Martingale/Anti-Martingale**: Progressive sizing

## License

MIT
