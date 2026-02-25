//! EVM chain client

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::types::{Chain, Transaction, Wallet};

/// EVM client configuration
#[derive(Clone, Debug)]
pub struct EvmClientConfig {
    pub rpc_url: String,
    pub chain: Chain,
    pub api_key: Option<String>,
    pub max_retries: u32,
    pub timeout_secs: u64,
}

impl EvmClientConfig {
    pub fn new(rpc_url: impl Into<String>, chain: Chain) -> Self {
        Self {
            rpc_url: rpc_url.into(),
            chain,
            api_key: None,
            max_retries: 3,
            timeout_secs: 30,
        }
    }
    
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
}

/// EVM chain client
#[derive(Clone, Debug)]
pub struct EvmClient {
    config: EvmClientConfig,
}

impl EvmClient {
    /// Create a new EVM client
    pub fn new(config: EvmClientConfig) -> Self {
        Self { config }
    }
    
    /// Get client configuration
    pub fn config(&self) -> &EvmClientConfig {
        &self.config
    }
    
    /// Get chain
    pub fn chain(&self) -> Chain {
        self.config.chain
    }
    
    /// Get native balance for address
    pub async fn get_balance(&self, address: &str) -> Result<f64> {
        // Placeholder implementation
        // In real implementation, use ethers-rs or similar
        Ok(0.0)
    }
    
    /// Get token balance
    pub async fn get_token_balance(&self, address: &str, token_address: &str) -> Result<f64> {
        // Placeholder implementation
        Ok(0.0)
    }
    
    /// Get transaction by hash
    pub async fn get_transaction(&self, tx_hash: &str) -> Result<Transaction> {
        // Placeholder implementation
        Err(Error::msg("Not implemented"))
    }
    
    /// Get latest block number
    pub async fn get_block_number(&self) -> Result<u64> {
        // Placeholder implementation
        Ok(0)
    }
    
    /// Send raw transaction
    pub async fn send_transaction(&self, signed_tx: &str) -> Result<String> {
        // Placeholder implementation
        Err(Error::msg("Not implemented"))
    }
    
    /// Estimate gas for transaction
    pub async fn estimate_gas(
        &self,
        from: &str,
        to: &str,
        data: Option<&str>,
        value: Option<&str>,
    ) -> Result<u64> {
        // Placeholder implementation
        Ok(21000)
    }
}

/// ERC20 token interface
pub struct Erc20Token {
    client: EvmClient,
    address: String,
}

impl Erc20Token {
    pub fn new(client: EvmClient, address: impl Into<String>) -> Self {
        Self {
            client,
            address: address.into(),
        }
    }
    
    /// Get token name
    pub async fn name(&self) -> Result<String> {
        // Placeholder
        Ok("Unknown".to_string())
    }
    
    /// Get token symbol
    pub async fn symbol(&self) -> Result<String> {
        // Placeholder
        Ok("UNK".to_string())
    }
    
    /// Get token decimals
    pub async fn decimals(&self) -> Result<u8> {
        // Placeholder
        Ok(18)
    }
    
    /// Get total supply
    pub async fn total_supply(&self) -> Result<f64> {
        // Placeholder
        Ok(0.0)
    }
    
    /// Get balance of address
    pub async fn balance_of(&self, address: &str) -> Result<f64> {
        self.client.get_token_balance(address, &self.address).await
    }
}

/// Common RPC methods
#[derive(Debug, Serialize)]
#[serde(tag = "method", content = "params")]
pub enum RpcMethod {
    #[serde(rename = "eth_getBalance")]
    GetBalance { address: String, block: String },
    #[serde(rename = "eth_getTransactionByHash")]
    GetTransaction { hash: String },
    #[serde(rename = "eth_blockNumber")]
    BlockNumber,
    #[serde(rename = "eth_sendRawTransaction")]
    SendRawTransaction { data: String },
    #[serde(rename = "eth_estimateGas")]
    EstimateGas { transaction: serde_json::Value },
}

#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<T>,
    pub error: Option<RpcError>,
}

#[derive(Debug, Deserialize)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
}
