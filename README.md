# 🚀 Crypto Trading Toolkit

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/python-3670A0?style=flat&logo=python&logoColor=ffdd54)](https://www.python.org)
[![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=flat&logo=typescript&logoColor=white)](https://www.typescriptlang.org)

A **modular, open-source toolkit** for building crypto trading bots, market makers, and DeFi automation tools. Designed to be **reusable, extensible, and production-ready**.

## 🎯 Philosophy

This project is built on the principle that trading infrastructure should be:

- **🧩 Modular**: Use only what you need, replace what you don't
- **🔌 Pluggable**: Easy to integrate with your own systems
- **📚 Well-documented**: Clear APIs and examples
- **🧪 Tested**: Reliable code you can trust with your funds
- **🌍 Multi-language**: Rust for performance, Python for rapid development, TypeScript for web

## 📦 Packages

### Core Libraries

| Package | Language | Description |
|---------|----------|-------------|
| [`telegram-control`](packages/telegram-control) | Rust/Python | Reusable Telegram bot framework with trading-specific features |
| [`blockchain-clients`](packages/blockchain-clients) | Rust/Python | Unified clients for EVM chains, Solana, Polymarket, Kalshi |
| [`risk-management`](packages/risk-management) | Rust/Python | Circuit breakers, position sizing, kill switches |

### Trading Bots (Apps)

| App | Description | Status |
|-----|-------------|--------|
| `polymarket-copy-trader` | Copy successful traders on Polymarket | 🚧 Migrating |
| `kalshi-btc-trader` | BTC volatility trading on Kalshi | 🚧 Migrating |
| `whale-tracker` | Multi-chain whale monitoring with alerts | 🚧 Migrating |
| `arbitrage-bot` | Cross-exchange arbitrage | 🚧 Planned |

### Tools

| Tool | Purpose |
|------|---------|
| `wallet-analyzer` | Analyze wallet performance |
| `market-scanner` | Find trading opportunities |
| `backtester` | Strategy backtesting framework |

## 🚀 Quick Start

### Using a Package

#### Rust

```rust
// Add to Cargo.toml
[dependencies]
telegram-control = { git = "https://github.com/yourusername/crypto-trading", subdir = "packages/telegram-control/rust" }

// Use it
use telegram_control::{Bot, AlertBuilder};

let bot = Bot::new(&env::var("TELEGRAM_TOKEN")?)
    .with_whitelist(vec![12345678]);
```

#### Python

```python
# Install
pip install git+https://github.com/yourusername/crypto-trading#subdirectory=packages/telegram-control/python

# Use it
from telegram_control import Bot, AlertBuilder

bot = Bot(token=os.environ["TELEGRAM_TOKEN"])

@bot.command("status")
async def status(ctx):
    await ctx.reply("System operational!")
```

### Building a Custom Bot

```rust
use telegram_control::Bot;
use blockchain_clients::{PolymarketClient, Chain};
use risk_management::{CircuitBreaker, KellySizing};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize components
    let bot = Bot::new(&env::var("TELEGRAM_TOKEN")?);
    let client = PolymarketClient::new(Chain::Polygon);
    let circuit_breaker = CircuitBreaker::new()
        .max_consecutive_losses(3);
    
    // Your trading logic here
    
    Ok(())
}
```

## 🏗️ Architecture

```
crypto-trading/
├── packages/              # Reusable libraries
│   ├── telegram-control/  # Telegram bot framework
│   ├── blockchain-clients/# Blockchain/exchange clients
│   └── risk-management/   # Risk controls
│
├── apps/                  # Standalone applications
│   ├── polymarket-copy-trader/
│   ├── kalshi-btc-trader/
│   └── whale-tracker/
│
├── tools/                 # CLI utilities
│   ├── wallet-analyzer/
│   ├── market-scanner/
│   └── backtester/
│
└── docs/                  # Documentation
    ├── architecture/
    ├── api-reference/
    └── examples/
```

## 🛠️ Development

### Prerequisites

- **Rust** 1.75+ ([install](https://rustup.rs/))
- **Python** 3.10+ with pip
- **Node.js** 20+ with pnpm
- **Git**

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/crypto-trading.git
cd crypto-trading

# Build all Rust packages
cargo build --workspace

# Install Python packages
pip install -e packages/telegram-control/python
pip install -e packages/blockchain-clients/python
pip install -e packages/risk-management/python

# Install Node.js dependencies
pnpm install
```

### Running Tests

```bash
# Rust tests
cargo test --workspace

# Python tests
pytest packages/

# All tests
make test
```

## 📚 Documentation

- [Architecture Overview](docs/architecture/README.md)
- [API Reference](docs/api-reference/README.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Security Best Practices](docs/security.md)

## 🔒 Security

⚠️ **IMPORTANT**: This software involves financial risk. Please read:

- Start with **small amounts** for testing
- Always use **testnet/paper trading** first
- Never share your **private keys**
- Review all **smart contract interactions**
- Enable **all safety features** (kill switches, circuit breakers)

See [SECURITY.md](SECURITY.md) for detailed security guidelines.

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:

- Code style guidelines
- Testing requirements
- Pull request process
- Development workflow

### Quick Contribution Guide

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/crypto-trading.git

# Create a branch
git checkout -b feature/my-feature

# Make changes, add tests
# ...

# Run checks
cargo test && cargo clippy
pytest

# Commit and push
git commit -m "feat: add my feature"
git push origin feature/my-feature

# Open a Pull Request
```

## 📜 License

This project is licensed under the **MIT License** - see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- Original code authors for the trading algorithms
- Open source libraries: teloxide, ethers-rs, python-telegram-bot
- The crypto trading community for feedback and testing

## 📞 Support

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/yourusername/crypto-trading/issues)
- 💡 **Feature Requests**: [GitHub Discussions](https://github.com/yourusername/crypto-trading/discussions)
- 💬 **Community**: [Discord/Telegram links]

---

⚠️ **Disclaimer**: This software is for **educational and research purposes only**. Trading involves significant financial risk. The authors assume no responsibility for any losses incurred while using this software. Always do your own research and never trade with money you cannot afford to lose.
