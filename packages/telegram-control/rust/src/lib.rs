//! # Telegram Control
//!
//! A reusable Telegram bot framework for crypto trading applications.
//!
//! ## Example
//!
//! ```rust,no_run
//! use telegram_control::{Bot, CommandHandler, Context};
//!
//! #[tokio::main]
//! async fn main() {
//!     let bot = Bot::new(std::env::var("TELEGRAM_TOKEN").unwrap())
//!         .with_whitelist(vec![12345678]);
//!     
//!     bot.on_command("/status", |ctx: Context| async move {
//!         ctx.reply("System operational").await
//!     });
//!     
//!     bot.run().await;
//! }
//! ```

pub mod alerts;
pub mod auth;
pub mod bot;
pub mod commands;
pub mod error;
pub mod keyboards;
pub mod types;

pub use bot::{Bot, BotBuilder};
pub use commands::{Command, CommandHandler};
pub use error::{Error, Result};
pub use types::{CallbackContext, Context, MessageContext};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{
        alerts::*,
        auth::*,
        bot::{Bot, BotBuilder},
        commands::{Command, CommandHandler},
        keyboards::*,
        types::*,
    };
}
