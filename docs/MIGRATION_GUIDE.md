# Migration Guide

This guide helps you migrate from the old monolithic structure to the new modular architecture.

## 📁 Old vs New Structure

### Before (Monolithic)

```
crypto-trading/
├── prediction-markets/
│   ├── polymarket-btc/           # Standalone bot
│   ├── polymarket-copy-trading-bot/  # Python + Rust bots
│   ├── polymarket-trading-tools-telegram-bot/  # Multiple bots
│   └── trading-core/             # Kalshi trading
├── tools/
│   └── block-explorer/
│       └── whale-tracker/        # FastAPI + React app
└── ...
```

### After (Modular)

```
crypto-trading/
├── packages/                     # Reusable libraries
│   ├── telegram-control/         # Shared Telegram framework
│   ├── blockchain-clients/       # Shared blockchain clients
│   └── risk-management/          # Shared risk controls
│
├── apps/                         # Standalone applications
│   ├── polymarket-copy-trader/   # Uses packages/*
│   ├── kalshi-btc-trader/        # Uses packages/*
│   └── whale-tracker/            # Uses packages/*
│
└── tools/                        # CLI utilities
```

## 🔄 Component Mapping

| Old Location | New Location | Package |
|-------------|--------------|---------|
| `polymarket-trading-tools-telegram-bot/**/telegram/` | `packages/telegram-control/` | Extracted & generalized |
| `polymarket-btc/**/polymarket_client.py` | `packages/blockchain-clients/python/exchanges/polymarket.py` | Extracted |
| `trading-core/src/safety/` | `packages/risk-management/` | Extracted & improved |
| `polymarket-copy-trading-bot/` | `apps/polymarket-copy-trader/` | Uses packages |
| `whale-tracker/` | `apps/whale-tracker/` | Uses packages |

## 🚀 Migration Steps

### For Telegram Bot Users

#### Before (Old Code)

```python
# Old: Inline telegram handling
from telegram import Bot, Update

class MyTradingBot:
    def __init__(self):
        self.bot = Bot(token=TOKEN)
    
    def handle_status(self, update: Update):
        # Custom implementation
        pass
```

#### After (New Code)

```python
# New: Use telegram-control package
from telegram_control import Bot, AlertBuilder

bot = Bot(token=TOKEN)

@bot.command("status")
async def status(ctx):
    """Get trading status"""
    alert = AlertBuilder.info("System Status").build()
    await ctx.reply_markdown(alert)

bot.run()
```

### For Blockchain Client Users

#### Before (Old Code)

```python
# Old: Custom Polymarket client
class PolymarketClient:
    def __init__(self):
        self.session = requests.Session()
    
    def get_markets(self):
        # Custom implementation
        pass
```

#### After (New Code)

```python
# New: Use blockchain-clients package
from blockchain_clients import PolymarketClient, Chain

client = PolymarketClient(
    chain=Chain.POLYGON,
    rpc_url="https://polygon-rpc.com"
)
await client.authenticate(api_key, secret)
markets = await client.get_active_markets()
```

### For Risk Management

#### Before (Old Code)

```python
# Old: Inline risk checks
if consecutive_losses > 3:
    stop_trading()
```

#### After (New Code)

```python
# New: Use risk-management package
from risk_management import CircuitBreaker

cb = CircuitBreaker()
cb.max_consecutive_losses(3)

if not cb.check().passed:
    print("Trading halted")
```

## 📦 Using Packages

### Installation

#### Rust

```toml
# In your Cargo.toml
[dependencies]
telegram-control = { path = "../packages/telegram-control/rust" }
blockchain-clients = { path = "../packages/blockchain-clients/rust" }
risk-management = { path = "../packages/risk-management/rust" }
```

#### Python

```bash
# Install packages in development mode
pip install -e packages/telegram-control/python
pip install -e packages/blockchain-clients/python
pip install -e packages/risk-management/python
```

### Building an App

Here's how to build a new trading bot using the packages:

```rust
// apps/my-trading-bot/Cargo.toml
[package]
name = "my-trading-bot"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }

# Local packages
telegram-control = { path = "../../packages/telegram-control/rust" }
blockchain-clients = { path = "../../packages/blockchain-clients/rust" }
risk-management = { path = "../../packages/risk-management/rust" }
```

```rust
// apps/my-trading-bot/src/main.rs
use telegram_control::{Bot, AlertBuilder, AlertLevel};
use blockchain_clients::{PolymarketClient, Chain};
use risk_management::{CircuitBreaker, KillSwitch, KellySizing};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize components
    let bot = Bot::new(std::env::var("TELEGRAM_TOKEN")?)
        .with_whitelist(vec![12345678]);
    
    let client = PolymarketClient::new(
        blockchain_clients::PolymarketConfig::default()
    );
    
    let mut circuit_breaker = CircuitBreaker::new()
        .max_consecutive_losses(3);
    
    let kill_switch = KillSwitch::new()
        .balance_floor(100.0);
    
    // Your trading logic
    bot.on_command("/trade", |ctx| async move {
        // Check risk controls
        if let Err(e) = kill_switch.check() {
            ctx.reply(format!("🛑 Risk check failed: {}", e)).await?;
            return Ok(());
        }
        
        // Execute trade
        // ...
        
        ctx.reply("✅ Trade executed").await?;
        Ok(())
    });
    
    bot.run().await?;
    Ok(())
}
```

## 🔧 Configuration Migration

### Environment Variables

Old environment variables are mostly compatible. Key changes:

| Old Variable | New Variable | Package |
|-------------|--------------|---------|
| `TELEGRAM_BOT_TOKEN` | `TELEGRAM_BOT_TOKEN` | telegram-control |
| `POLYMARKET_API_KEY` | `POLYMARKET_API_KEY` | blockchain-clients |
| `PRIVATE_KEY` | `WALLET_PRIVATE_KEY` | blockchain-clients |
| `MONGO_URI` | `DATABASE_URL` | app-specific |

### Config Files

Old JSON configs → New TOML configs:

```toml
# config.toml
[telegram]
token = "${TELEGRAM_BOT_TOKEN}"
whitelist = [12345678]

[trading]
max_position_size = 1000.0
daily_loss_limit = 500.0

[risk]
max_consecutive_losses = 3
balance_floor = 100.0
```

## 🧪 Testing Migration

### Running Old Tests

Tests in the old directories should still work during migration:

```bash
# Old tests
cd prediction-markets/polymarket-copy-trading-bot/rust
cargo test

# New tests
cargo test --workspace
```

### Migrating Tests

Move tests to the appropriate package:

```
# Before
prediction-markets/polymarket-copy-trading-bot/rust/tests/

# After
packages/blockchain-clients/rust/tests/  # If testing clients
apps/polymarket-copy-trader/tests/       # If testing app logic
```

## 📊 Feature Comparison

| Feature | Old | New |
|---------|-----|-----|
| Telegram alerts | ✅ | ✅ Better formatting |
| Risk management | ⚠️ Basic | ✅ Comprehensive |
| Blockchain clients | ⚠️ Coupled | ✅ Modular |
| Position sizing | ⚠️ Inline | ✅ Multiple strategies |
| Circuit breakers | ⚠️ Basic | ✅ Configurable |
| Testing | ⚠️ Limited | ✅ Comprehensive |
| Documentation | ⚠️ Sparse | ✅ Extensive |

## 🆘 Troubleshooting

### Common Issues

**Issue**: `cannot find package 'telegram-control'`

**Fix**: Ensure you've added the package to your dependencies:

```toml
[dependencies]
telegram-control = { path = "../../packages/telegram-control/rust" }
```

**Issue**: Python import errors

**Fix**: Install packages in editable mode:

```bash
pip install -e packages/telegram-control/python
```

**Issue**: Version conflicts

**Fix**: Use workspace dependencies in Rust:

```toml
[dependencies]
tokio = { workspace = true }
```

## 📚 Next Steps

1. Read package-specific documentation in `packages/*/README.md`
2. Check out example apps in `apps/`
3. Join community discussions for help

## ⏱️ Timeline

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Create packages | ✅ Done |
| 2 | Migrate core functionality | 🚧 In Progress |
| 3 | Update apps to use packages | 🚧 In Progress |
| 4 | Deprecate old code | 📋 Planned |
| 5 | Remove old code | 📋 Future |

---

Need help? Open a [GitHub Discussion](https://github.com/yourusername/crypto-trading/discussions).
