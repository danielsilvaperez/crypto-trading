use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use teloxide::dispatching::{Dispatcher, UpdateFilterExt, HandlerExt};
use teloxide::prelude::*;
use teloxide::types::Update;
use teloxide::utils::command::BotCommands;
use tracing::{info, warn, error};

use crate::auth::AccessControl;
use crate::error::{Error, Result};
use crate::types::{CallbackContext, Context, MessageContext};

/// Handler function type
pub type HandlerFn = Arc<
    dyn Fn(Context) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync,
>;

/// Telegram Bot wrapper with trading-focused features
pub struct Bot {
    bot: teloxide::Bot,
    access_control: AccessControl,
    command_handlers: HashMap<String, HandlerFn>,
    callback_handlers: Vec<(String, HandlerFn)>, // pattern, handler
    default_handler: Option<HandlerFn>,
}

impl Bot {
    /// Create a new bot instance
    pub fn new(token: impl Into<String>) -> BotBuilder {
        BotBuilder::new(token)
    }
    
    /// Register a command handler
    pub fn on_command<F, Fut>(mut self, command: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let handler: HandlerFn = Arc::new(move |ctx| Box::pin(handler(ctx)));
        self.command_handlers.insert(command.into(), handler);
        self
    }
    
    /// Register a callback query handler with pattern matching
    pub fn on_callback<F, Fut>(mut self, pattern: impl Into<String>, handler: F) -> Self
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let handler: HandlerFn = Arc::new(move |ctx| Box::pin(handler(ctx)));
        self.callback_handlers.push((pattern.into(), handler));
        self
    }
    
    /// Set default handler for unmatched messages
    pub fn on_default<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.default_handler = Some(Arc::new(move |ctx| Box::pin(handler(ctx))));
        self
    }
    
    /// Run the bot (blocking)
    pub async fn run(self) -> Result<()> {
        info!("Starting Telegram bot...");
        
        let bot = self.bot.clone();
        let access_control = self.access_control.clone();
        let command_handlers = Arc::new(self.command_handlers);
        let callback_handlers = Arc::new(self.callback_handlers);
        let default_handler = self.default_handler;
        
        let handler = dptree::entry()
            .branch(
                Update::filter_message().endpoint(
                    move |bot: teloxide::Bot, msg: Message| {
                        let access_control = access_control.clone();
                        let handlers = command_handlers.clone();
                        let default = default_handler.clone();
                        
                        async move {
                            if let Some(user) = msg.from() {
                                let user_id = user.id.0 as i64;
                                
                                // Check authorization
                                if let Err(e) = access_control.authorize(user_id) {
                                    warn!("Unauthorized access attempt from user {}", user_id);
                                    let _ = bot.send_message(
                                        msg.chat.id,
                                        "⛔ You are not authorized to use this bot."
                                    ).await;
                                    return Ok(());
                                }
                                
                                let ctx = MessageContext {
                                    message: msg.clone(),
                                    user: user.clone(),
                                    chat_id: msg.chat.id.0,
                                    text: msg.text().map(|s| s.to_string()),
                                    bot: bot.clone(),
                                };
                                
                                // Try to match command
                                if let Some(text) = msg.text() {
                                    let parts: Vec<&str> = text.split_whitespace().collect();
                                    if let Some(cmd) = parts.first() {
                                        if let Some(handler) = handlers.get(*cmd) {
                                            if let Err(e) = handler(Context::Message(ctx)).await {
                                                error!("Handler error: {}", e);
                                                let _ = bot.send_message(
                                                    msg.chat.id,
                                                    format!("❌ Error: {}", e)
                                                ).await;
                                            }
                                            return Ok(());
                                        }
                                    }
                                }
                                
                                // Default handler
                                if let Some(handler) = default {
                                    if let Err(e) = handler(Context::Message(ctx)).await {
                                        error!("Default handler error: {}", e);
                                    }
                                }
                            }
                            Ok(())
                        }
                    }
                )
            )
            .branch(
                Update::filter_callback_query().endpoint(
                    move |bot: teloxide::Bot, q: CallbackQuery| {
                        let handlers = callback_handlers.clone();
                        
                        async move {
                            if let (Some(data), Some(user)) = (q.data.clone(), q.from) {
                                let ctx = CallbackContext {
                                    query: q.clone(),
                                    user: user.clone(),
                                    chat_id: q.message.as_ref()
                                        .map(|m| m.chat.id.0)
                                        .unwrap_or(user.id.0 as i64),
                                    data: data.clone(),
                                    bot: bot.clone(),
                                };
                                
                                // Find matching handler
                                for (pattern, handler) in handlers.iter() {
                                    if data.starts_with(pattern) {
                                        if let Err(e) = handler(Context::Callback(ctx)).await {
                                            error!("Callback handler error: {}", e);
                                        }
                                        return Ok(());
                                    }
                                }
                            }
                            Ok(())
                        }
                    }
                )
            );
        
        Dispatcher::builder(bot, handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
        
        Ok(())
    }
}

/// Builder for Bot configuration
pub struct BotBuilder {
    token: String,
    access_control: AccessControl,
}

impl BotBuilder {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            access_control: AccessControl::new(),
        }
    }
    
    pub fn with_whitelist(mut self, ids: Vec<i64>) -> Self {
        self.access_control = self.access_control.with_whitelist(ids);
        self
    }
    
    pub fn with_admins(mut self, ids: Vec<i64>) -> Self {
        self.access_control = self.access_control.with_admins(ids);
        self
    }
    
    pub fn build(self) -> Bot {
        Bot {
            bot: teloxide::Bot::new(self.token),
            access_control: self.access_control,
            command_handlers: HashMap::new(),
            callback_handlers: Vec::new(),
            default_handler: None,
        }
    }
}

impl From<BotBuilder> for Bot {
    fn from(builder: BotBuilder) -> Self {
        builder.build()
    }
}
