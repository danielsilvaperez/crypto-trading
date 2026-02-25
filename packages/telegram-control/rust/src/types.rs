use teloxide::types::{CallbackQuery, Message, User};

/// Context for message-based commands
#[derive(Clone, Debug)]
pub struct MessageContext {
    pub message: Message,
    pub user: User,
    pub chat_id: i64,
    pub text: Option<String>,
    pub bot: teloxide::Bot,
}

/// Context for callback queries (inline keyboard)
#[derive(Clone, Debug)]
pub struct CallbackContext {
    pub query: CallbackQuery,
    pub user: User,
    pub chat_id: i64,
    pub data: String,
    pub bot: teloxide::Bot,
}

/// Unified context type
#[derive(Clone, Debug)]
pub enum Context {
    Message(MessageContext),
    Callback(CallbackContext),
}

impl Context {
    pub fn chat_id(&self) -> i64 {
        match self {
            Context::Message(ctx) => ctx.chat_id,
            Context::Callback(ctx) => ctx.chat_id,
        }
    }
    
    pub fn user_id(&self) -> i64 {
        match self {
            Context::Message(ctx) => ctx.user.id.0 as i64,
            Context::Callback(ctx) => ctx.user.id.0 as i64,
        }
    }
    
    pub fn username(&self) -> Option<&str> {
        match self {
            Context::Message(ctx) => ctx.user.username.as_deref(),
            Context::Callback(ctx) => ctx.user.username.as_deref(),
        }
    }
}

impl From<MessageContext> for Context {
    fn from(ctx: MessageContext) -> Self {
        Context::Message(ctx)
    }
}

impl From<CallbackContext> for Context {
    fn from(ctx: CallbackContext) -> Self {
        Context::Callback(ctx)
    }
}
