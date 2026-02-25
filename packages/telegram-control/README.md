# Telegram Control

A reusable, extensible Telegram bot framework for crypto trading applications. Provides a clean abstraction over `teloxide` (Rust) and `python-telegram-bot` (Python) with built-in support for trading-specific features.

## Features

- 🚀 **Multi-language**: Rust and Python implementations
- 🔔 **Alert System**: Real-time trading alerts with rich formatting
- 📊 **Interactive Keyboards**: Inline and reply keyboards for trading UIs
- 💬 **Command Handling**: Structured command routing
- 🔒 **Access Control**: User whitelist and admin roles
- 📈 **Price Formatting**: Built-in crypto/fiat formatting utilities
- 📝 **Logging**: Structured logging integration

## Quick Start

### Rust

```rust
use telegram_control::{Bot, Command, Context};

#[derive(Command)]
enum MyCommands {
    Start,
    Status,
    Trade { amount: f64, symbol: String },
}

let bot = Bot::new(&env::var("TELEGRAM_BOT_TOKEN")?)
    .with_whitelist(vec![12345678])
    .on_command(MyCommands::Start, |ctx| async move {
        ctx.reply("Welcome!").await
    })
    .on_command(MyCommands::Status, |ctx| async move {
        let status = get_system_status().await;
        ctx.reply_markdown(format!("**Status**: {}", status)).await
    });

bot.run().await;
```

### Python

```python
from telegram_control import Bot, Command, Context

bot = Bot(token=os.environ["TELEGRAM_BOT_TOKEN"])

@bot.command("status")
async def status(ctx: Context):
    """Get trading bot status"""
    status = await get_system_status()
    await ctx.reply_markdown(f"**Status**: {status}")

@bot.command("trade")
async def trade(ctx: Context, amount: float, symbol: str):
    """Execute a trade"""
    result = await execute_trade(symbol, amount)
    await ctx.reply_alert(f"✅ Trade executed: {result}")

bot.run()
```

## Installation

### Rust

```toml
[dependencies]
telegram-control = { path = "../../packages/telegram-control/rust" }
```

### Python

```bash
pip install -e ../../packages/telegram-control/python
```

## Architecture

```
telegram-control/
├── rust/                      # Rust implementation (teloxide-based)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── bot.rs            # Core bot abstraction
│   │   ├── commands.rs       # Command routing
││   ├── alerts.rs           # Alert formatting
│   │   ├── keyboards.rs      # Keyboard builders
│   │   └── auth.rs           # Access control
│   └── Cargo.toml
│
├── python/                    # Python implementation
│   ├── telegram_control/
│   │   ├── __init__.py
│   │   ├── bot.py
│   │   ├── commands.py
│   │   ├── alerts.py
│   │   ├── keyboards.py
│   │   └── auth.py
│   └── pyproject.toml
│
└── README.md
```

## API Reference

### Bot Builder Pattern

Both implementations use a builder pattern for configuration:

| Method | Description |
|--------|-------------|
| `new(token)` | Create bot instance |
| `with_whitelist(ids)` | Restrict to user IDs |
| `with_admins(ids)` | Set admin users |
| `on_command(cmd, handler)` | Register command handler |
| `on_callback(pattern, handler)` | Register callback handler |
| `with_middleware(mw)` | Add middleware |

### Alert Types

- `price_alert` - Price movement notifications
- `trade_alert` - Trade execution alerts
- `risk_alert` - Risk threshold warnings
- `system_alert` - System status updates

## Examples

See `examples/` directory for complete usage examples:
- `basic_bot.rs/py` - Simple echo bot
- `trading_alerts.rs/py` - Trading alert system
- `interactive_keyboard.rs/py` - Complex UI with keyboards

## License

MIT - See LICENSE file
