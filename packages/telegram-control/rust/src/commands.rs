use async_trait::async_trait;
use crate::error::Result;
use crate::types::Context;

/// Trait for bot commands
#[async_trait]
pub trait Command: Sized {
    /// Parse command from string
    fn parse(input: &str) -> Result<Self>;
    
    /// Get command name
    fn name(&self) -> &'static str;
    
    /// Get command description
    fn description(&self) -> &'static str;
}

/// Command handler trait
#[async_trait]
pub trait CommandHandler<C: Command> {
    async fn handle(&self, ctx: Context, command: C) -> Result<()>;
}

/// Macro for defining commands
#[macro_export]
macro_rules! commands {
    (
        $name:ident {
            $(
                $variant:ident$(($($arg:ident: $arg_type:ty),*))? => $cmd:literal : $desc:literal
            ),*$(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub enum $name {
            $($variant$(($($arg_type),*))?),*
        }
        
        impl $crate::commands::Command for $name {
            fn parse(input: &str) -> $crate::error::Result<Self> {
                let parts: Vec<&str> = input.split_whitespace().collect();
                let cmd = parts.first().ok_or_else(|| {
                    $crate::error::Error::InvalidCommand("Empty command".to_string())
                })?;
                
                match *cmd {
                    $(
                        $cmd => {
                            let mut idx = 1;
                            Ok(Self::$variant$(($(
                                parts.get(idx)
                                    .ok_or_else(|| $crate::error::Error::InvalidCommand(
                                        format!("Missing argument: {}", stringify!($arg))
                                    ))?
                                    .parse::<$arg_type>()
                                    .map_err(|e| $crate::error::Error::InvalidCommand(
                                        format!("Invalid {}: {}", stringify!($arg), e)
                                    ))?
                            )$($({
                                idx += 1;
                                idx - 1
                            }))*))?)
                        }
                    ),*
                    _ => Err($crate::error::Error::InvalidCommand(cmd.to_string())),
                }
            }
            
            fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant$(($(_,)*))?) => $cmd),*
                }
            }
            
            fn description(&self) -> &'static str {
                match self {
                    $(Self::$variant$(($(_,)*))?) => $desc),*
                }
            }
        }
        
        impl $name {
            pub fn help() -> String {
                let mut help = String::from("Available commands:\n");
                $(
                    help.push_str(&format!("  {} - {}\n", $cmd, $desc));
                )*
                help
            }
        }
    };
}

/// Example usage:
/// ```rust
/// use telegram_control::commands;
///
/// commands! {
///     MyCommands {
///         Start => "/start": "Start the bot",
///         Status => "/status": "Get system status",
///         Trade(amount: f64, symbol: String) => "/trade": "Execute a trade",
///     }
/// }
/// ```
