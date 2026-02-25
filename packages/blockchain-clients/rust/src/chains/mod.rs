//! Blockchain chain clients

pub mod evm;

#[cfg(feature = "solana")]
pub mod solana;

pub use evm::EvmClient;

#[cfg(feature = "solana")]
pub use solana::SolanaClient;
