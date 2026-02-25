//! Polymarket CLOB client

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::types::{Order, OrderSide, OrderType, OrderStatus, Wallet, Chain, Token};

/// Polymarket API configuration
#[derive(Clone, Debug)]
pub struct PolymarketConfig {
    pub api_url: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub rpc_url: String,
    pub chain: Chain,
}

impl Default for PolymarketConfig {
    fn default() -> Self {
        Self {
            api_url: "https://clob.polymarket.com".to_string(),
            api_key: None,
            api_secret: None,
            rpc_url: "https://polygon-rpc.com".to_string(),
            chain: Chain::Polygon,
        }
    }
}

/// Polymarket CLOB client
#[derive(Clone, Debug)]
pub struct PolymarketClient {
    config: PolymarketConfig,
    http: Client,
    credentials: Option<Credentials>,
}

#[derive(Clone, Debug)]
struct Credentials {
    api_key: String,
    secret: String,
    expires_at: DateTime<Utc>,
}

/// Market data
#[derive(Clone, Debug, Deserialize)]
pub struct Market {
    pub id: String,
    pub condition_id: String,
    pub question: String,
    pub slug: String,
    #[serde(rename = "marketMakerAddress")]
    pub market_maker_address: String,
    pub tokens: Vec<MarketToken>,
    #[serde(rename = "active")]
    pub is_active: bool,
    #[serde(rename = "closed")]
    pub is_closed: bool,
    #[serde(rename = "closedTime")]
    pub closed_time: Option<DateTime<Utc>>,
}

/// Market token (outcome)
#[derive(Clone, Debug, Deserialize)]
pub struct MarketToken {
    pub token_id: String,
    pub outcome: String,
    pub price: f64,
}

/// Order book
#[derive(Clone, Debug, Deserialize)]
pub struct OrderBook {
    pub market: String,
    pub asset_id: String,
    pub bids: Vec<OrderBookEntry>,
    pub asks: Vec<OrderBookEntry>,
    pub timestamp: DateTime<Utc>,
}

/// Order book entry
#[derive(Clone, Debug, Deserialize)]
pub struct OrderBookEntry {
    pub price: f64,
    pub size: f64,
}

/// Trade execution result
#[derive(Clone, Debug, Deserialize)]
pub struct TradeResult {
    pub order_id: String,
    pub status: String,
    pub filled_size: f64,
    pub avg_price: f64,
    pub transaction_hash: Option<String>,
}

impl PolymarketClient {
    /// Create a new Polymarket client
    pub fn new(config: PolymarketConfig) -> Self {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self {
            config,
            http,
            credentials: None,
        }
    }
    
    /// Authenticate with API credentials
    pub async fn authenticate(&mut self, api_key: &str, api_secret: &str) -> Result<()> {
        // In real implementation, would exchange for a JWT or session token
        self.credentials = Some(Credentials {
            api_key: api_key.to_string(),
            secret: api_secret.to_string(),
            expires_at: Utc::now() + chrono::Duration::hours(24),
        });
        Ok(())
    }
    
    /// Check if authenticated
    pub fn is_authenticated(&self) -> bool {
        self.credentials.as_ref()
            .map(|c| c.expires_at > Utc::now())
            .unwrap_or(false)
    }
    
    /// Get active markets
    pub async fn get_active_markets(&self) -> Result<Vec<Market>> {
        let url = format!("{}/markets", self.config.api_url);
        
        let response = self.http
            .get(&url)
            .send()
            .await
            .map_err(Error::Network)?;
        
        if !response.status().is_success() {
            return Err(Error::Rpc(format!(
                "Failed to fetch markets: {}",
                response.status()
            )));
        }
        
        let markets: Vec<Market> = response.json().await.map_err(Error::Network)?;
        Ok(markets.into_iter().filter(|m| m.is_active && !m.is_closed).collect())
    }
    
    /// Get market by ID
    pub async fn get_market(&self, market_id: &str) -> Result<Market> {
        let url = format!("{}/markets/{}", self.config.api_url, market_id);
        
        let response = self.http
            .get(&url)
            .send()
            .await
            .map_err(Error::Network)?;
        
        if response.status().as_u16() == 404 {
            return Err(Error::MarketNotFound(market_id.to_string()));
        }
        
        response.json().await.map_err(Error::Network)
    }
    
    /// Get order book for market
    pub async fn get_order_book(&self, token_id: &str) -> Result<OrderBook> {
        let url = format!("{}/book/{}?side=buy&side=sell", self.config.api_url, token_id);
        
        let response = self.http
            .get(&url)
            .send()
            .await
            .map_err(Error::Network)?;
        
        response.json().await.map_err(Error::Network)
    }
    
    /// Place an order
    pub async fn place_order(&self, order: &OrderRequest) -> Result<TradeResult> {
        if !self.is_authenticated() {
            return Err(Error::Authentication("Not authenticated".to_string()));
        }
        
        let url = format!("{}/order", self.config.api_url);
        
        let response = self.http
            .post(&url)
            .json(order)
            .send()
            .await
            .map_err(Error::Network)?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::OrderRejected(error_text));
        }
        
        response.json().await.map_err(Error::Network)
    }
    
    /// Cancel an order
    pub async fn cancel_order(&self, order_id: &str) -> Result<bool> {
        if !self.is_authenticated() {
            return Err(Error::Authentication("Not authenticated".to_string()));
        }
        
        let url = format!("{}/order/{}", self.config.api_url, order_id);
        
        let response = self.http
            .delete(&url)
            .send()
            .await
            .map_err(Error::Network)?;
        
        Ok(response.status().is_success())
    }
    
    /// Get open orders
    pub async fn get_open_orders(&self, wallet: &Wallet) -> Result<Vec<Order>> {
        let url = format!(
            "{}/orders?address={}&status=OPEN",
            self.config.api_url, wallet.address
        );
        
        let response = self.http
            .get(&url)
            .send()
            .await
            .map_err(Error::Network)?;
        
        response.json().await.map_err(Error::Network)
    }
    
    /// Get fills/trades for wallet
    pub async fn get_fills(&self, wallet: &Wallet, limit: usize) -> Result<Vec<Fill>> {
        let url = format!(
            "{}/fills?address={}&limit={}",
            self.config.api_url, wallet.address, limit
        );
        
        let response = self.http
            .get(&url)
            .send()
            .await
            .map_err(Error::Network)?;
        
        response.json().await.map_err(Error::Network)
    }
}

/// Order request
#[derive(Clone, Debug, Serialize)]
pub struct OrderRequest {
    #[serde(rename = "tokenID")]
    pub token_id: String,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub size: f64,
    pub price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u64>,
}

impl OrderRequest {
    pub fn buy(token_id: impl Into<String>, size: f64, price: f64) -> Self {
        Self {
            token_id: token_id.into(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            size,
            price,
            nonce: None,
        }
    }
    
    pub fn sell(token_id: impl Into<String>, size: f64, price: f64) -> Self {
        Self {
            token_id: token_id.into(),
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            size,
            price,
            nonce: None,
        }
    }
    
    pub fn with_nonce(mut self, nonce: u64) -> Self {
        self.nonce = Some(nonce);
        self
    }
}

/// Fill/Trade record
#[derive(Clone, Debug, Deserialize)]
pub struct Fill {
    pub id: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    pub side: OrderSide,
    pub size: f64,
    pub price: f64,
    pub fee: f64,
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}
