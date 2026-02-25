# Architecture Overview

This document describes the high-level architecture of the Crypto Trading Toolkit.

## 🏗️ System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Applications                             │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐           │
│  │ Polymarket   │ │ Kalshi       │ │ Whale        │           │
│  │ Copy Trader  │ │ BTC Trader   │ │ Tracker      │           │
│  └──────────────┘ └──────────────┘ └──────────────┘           │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                          Packages                                │
│  ┌─────────────────┬─────────────────┬─────────────────┐       │
│  │ telegram-control│blockchain-clients│risk-management  │       │
│  │                 │                 │                 │       │
│  │ • Bot framework │ • EVM clients   │ • Circuit       │       │
│  │ • Alerts        │ • Exchange APIs │   breakers      │       │
│  │ • Keyboards     │ • Data providers│ • Position      │       │
│  │ • Auth          │ • Unified types │   sizing        │       │
│  └─────────────────┴─────────────────┴─────────────────┘       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     External Services                            │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐          │
│  │ Telegram │ │Polygon/  │ │ Kalshi   │ │ DeBank   │          │
│  │ API      │ │Ethereum  │ │ API      │ │ API      │          │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘          │
└─────────────────────────────────────────────────────────────────┘
```

## 📦 Package Architecture

### telegram-control

```
telegram-control/
├── rust/
│   ├── src/
│   │   ├── bot.rs         # Core bot abstraction
│   │   ├── commands.rs    # Command routing
│   │   ├── alerts.rs      # Alert formatting
│   │   ├── keyboards.rs   # UI components
│   │   └── auth.rs        # Access control
│   └── Cargo.toml
│
└── python/
    └── telegram_control/
        ├── bot.py
        ├── alerts.py
        ├── keyboards.py
        └── auth.py
```

**Key Design Patterns:**
- Builder pattern for configuration
- Handler registration for commands/callbacks
- Unified Context type for messages and callbacks

### blockchain-clients

```
blockchain-clients/
├── rust/
│   ├── src/
│   │   ├── chains/        # EVM, Solana clients
│   │   ├── exchanges/     # Polymarket, Kalshi
│   │   ├── providers/     # DeBank, Etherscan
│   │   └── types.rs       # Shared types
│   └── Cargo.toml
│
└── python/
    └── blockchain_clients/
        ├── chains/
        ├── exchanges/
        └── types.py
```

**Key Design Patterns:**
- Trait-based abstraction for common operations
- Feature flags for optional dependencies
- Unified error types

### risk-management

```
risk-management/
├── rust/
│   ├── src/
│   │   ├── circuit_breaker.rs
│   │   ├── kill_switch.rs
│   │   ├── position_sizing.rs
│   │   └── types.rs
│   └── Cargo.toml
│
└── python/
    └── risk_management/
        ├── circuit_breaker.py
        ├── kill_switch.py
        └── position_sizing.py
```

**Key Design Patterns:**
- Strategy pattern for position sizing
- State machines for circuit breakers
- Composable risk guards

## 🔌 Integration Patterns

### Dependency Injection

```rust
// Apps compose packages through dependency injection
pub struct TradingBot {
    telegram: Bot,
    exchange: Box<dyn Exchange>,
    risk_guard: RiskGuard,
}

impl TradingBot {
    pub fn new(
        telegram: Bot,
        exchange: impl Exchange + 'static,
        risk_guard: RiskGuard,
    ) -> Self {
        Self {
            telegram,
            exchange: Box::new(exchange),
            risk_guard,
        }
    }
}
```

### Event-Driven Architecture

```
Trade Event → Risk Check → Execute → Notify
                ↓ (if failed)
            Circuit Break
```

### Error Handling

```rust
// Unified error types across packages
pub enum TradingError {
    Risk(RiskError),
    Exchange(ExchangeError),
    Telegram(TelegramError),
}

// Propagate with context
result.map_err(|e| TradingError::Exchange(e))?;
```

## 🔄 Data Flow

### Trade Execution Flow

```
1. Market Data → Exchange Client
      ↓
2. Signal Generated
      ↓
3. Risk Check → Risk Management
      ↓
4. If passed:
      ↓
5. Submit Order → Exchange Client
      ↓
6. Confirm & Log
      ↓
7. Notify → Telegram Control
```

### Alert Flow

```
Event Detected
      ↓
AlertBuilder
      ↓
Format Message
      ↓
Send via Telegram API
```

## 🧪 Testing Architecture

### Unit Tests

Each package has comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_feature() {
        // Test implementation
    }
}
```

### Integration Tests

Integration tests verify package interactions:

```rust
// tests/integration_test.rs
use telegram_control::Bot;
use blockchain_clients::MockExchange;

#[tokio::test]
async fn test_full_trade_flow() {
    // Test end-to-end flow
}
```

### Mock Implementations

```rust
// Mock exchange for testing
pub struct MockExchange {
    orders: Vec<Order>,
}

#[async_trait]
impl Exchange for MockExchange {
    async fn place_order(&self, order: Order) -> Result<TradeResult> {
        // Mock implementation
    }
}
```

## 📊 Performance Considerations

### Async Runtime

All packages use Tokio for async execution:

```rust
#[tokio::main]
async fn main() {
    // Single runtime for all async operations
}
```

### Connection Pooling

HTTP clients use connection pools for efficiency:

```rust
let client = Client::builder()
    .pool_max_idle_per_host(10)
    .build()?;
```

### Caching

Market data is cached to reduce API calls:

```rust
pub struct CachedClient<C> {
    inner: C,
    cache: Arc<RwLock<HashMap<String, CachedValue>>>,
}
```

## 🔒 Security Architecture

### Secrets Management

```
┌──────────────┐
│ Environment  │ ← .env (gitignored)
│ Variables    │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Config       │ ← Runtime config
│ Loader       │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Services     │ ← No hardcoded secrets
└──────────────┘
```

### Access Control

```rust
pub struct AccessControl {
    whitelist: Option<HashSet<i64>>,
    admins: HashSet<i64>,
}
```

### Rate Limiting

Built-in rate limiting for all external APIs:

```rust
pub struct RateLimiter {
    requests: Arc<Mutex<Vec<Instant>>>,
    max_requests: usize,
    window: Duration,
}
```

## 🚀 Deployment Architecture

### Docker

```dockerfile
# Multi-stage build
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/app /usr/local/bin/
CMD ["app"]
```

### Configuration

Environment-based configuration:

```rust
pub struct Config {
    #[envvar("TELEGRAM_TOKEN")]
    pub telegram_token: String,
    
    #[envvar("RISK_ENABLED", default = true)]
    pub risk_enabled: bool,
}
```

## 📈 Future Architecture

Planned improvements:

1. **Plugin System**: Dynamic loading of strategies
2. **Event Bus**: Centralized event handling
3. **Metrics**: Prometheus/OpenTelemetry integration
4. **Web Dashboard**: Real-time monitoring UI

## 📚 Additional Reading

- [Package Design Principles](design-principles.md)
- [API Guidelines](api-guidelines.md)
- [Deployment Guide](../deployment/README.md)
