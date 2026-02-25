# Blockchain Clients

A collection of reusable blockchain client libraries for interacting with various networks and services. Provides unified interfaces for DeFi protocols, exchanges, and data providers.

## Supported Networks & Services

### Blockchains
- 🔷 **Ethereum** (and all EVM chains: Polygon, BSC, Arbitrum, Optimism, Base)
- ⚡ **Solana**

### Exchanges/DEXs
- 🎯 **Polymarket** (CLOB)
- 📊 **Kalshi**
- 🦄 **1inch** (aggregation)
- 🥞 **PancakeSwap**

### Data Providers
- 🔍 **DeBank** (portfolio tracking)
- 🐋 **Helius** (Solana)
- 📈 **Etherscan**

## Quick Start

### Rust

```rust
use blockchain_clients::{PolymarketClient, Wallet, Chain};

// Initialize client
let client = PolymarketClient::new(Chain::Polygon)
    .with_rpc("https://polygon-rpc.com")
    .with_credentials(api_key, api_secret)?;

// Get market data
let markets = client.get_active_markets().await?;

// Place order
let order = client.place_order(
    MarketOrder::buy("0x...", 100.0, 0.55)
).await?;
```

### Python

```python
from blockchain_clients import PolymarketClient, Chain

# Initialize client
client = PolymarketClient(
    chain=Chain.POLYGON,
    rpc_url="https://polygon-rpc.com",
)
client.authenticate(api_key, api_secret)

# Get markets
markets = await client.get_active_markets()

# Place order
order = await client.place_order(
    side="BUY",
    token_id="0x...",
    size=100.0,
    price=0.55,
)
```

## Architecture

```
blockchain-clients/
├── rust/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── chains/          # Chain-specific clients
│   │   │   ├── evm.rs       # Ethereum & EVM chains
│   │   │   └── solana.rs    # Solana
│   │   ├── exchanges/       # Exchange clients
│   │   │   ├── polymarket.rs
│   │   │   ├── kalshi.rs
│   │   │   └── oneinch.rs
│   │   ├── providers/       # Data providers
│   │   │   ├── debank.rs
│   │   │   ├── helius.rs
│   │   │   └── etherscan.rs
│   │   └── types.rs         # Shared types
│   └── Cargo.toml
│
├── python/
│   └── blockchain_clients/
│       └── ... (mirrors Rust structure)
│
└── README.md
```

## Installation

### Rust

```toml
[dependencies]
blockchain-clients = { path = "../../packages/blockchain-clients/rust" }
```

### Python

```bash
pip install -e ../../packages/blockchain-clients/python
```

## API Reference

### Common Types

| Type | Description |
|------|-------------|
| `Chain` | Blockchain network enum |
| `Wallet` | Wallet abstraction |
| `Token` | Token metadata |
| `Order` | Order representation |
| `Position` | Position tracking |

### Client Traits

All clients implement these common traits:

- `Authenticator` - API authentication
- `RateLimited` - Rate limiting handling
- `Retryable` - Automatic retries

## Examples

See `examples/` directory:
- `polymarket_basic.rs/py` - Basic Polymarket operations
- `wallet_monitor.rs/py` - Wallet monitoring
- `portfolio_tracker.rs/py` - Portfolio tracking

## License

MIT - See LICENSE file
