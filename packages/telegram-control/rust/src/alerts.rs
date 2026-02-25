use chrono::{DateTime, Utc};
use std::fmt::Write;

/// Alert severity levels
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertLevel {
    Info,
    Success,
    Warning,
    Error,
    Critical,
}

impl AlertLevel {
    pub fn emoji(&self) -> &'static str {
        match self {
            AlertLevel::Info => "ℹ️",
            AlertLevel::Success => "✅",
            AlertLevel::Warning => "⚠️",
            AlertLevel::Error => "❌",
            AlertLevel::Critical => "🚨",
        }
    }
}

/// Builder for trading alerts
#[derive(Clone, Debug)]
pub struct AlertBuilder {
    level: AlertLevel,
    title: String,
    fields: Vec<(String, String)>,
    timestamp: DateTime<Utc>,
}

impl AlertBuilder {
    pub fn new(level: AlertLevel, title: impl Into<String>) -> Self {
        Self {
            level,
            title: title.into(),
            fields: Vec::new(),
            timestamp: Utc::now(),
        }
    }
    
    pub fn info(title: impl Into<String>) -> Self {
        Self::new(AlertLevel::Info, title)
    }
    
    pub fn success(title: impl Into<String>) -> Self {
        Self::new(AlertLevel::Success, title)
    }
    
    pub fn warning(title: impl Into<String>) -> Self {
        Self::new(AlertLevel::Warning, title)
    }
    
    pub fn error(title: impl Into<String>) -> Self {
        Self::new(AlertLevel::Error, title)
    }
    
    pub fn critical(title: impl Into<String>) -> Self {
        Self::new(AlertLevel::Critical, title)
    }
    
    /// Add a field to the alert
    pub fn field(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.push((name.into(), value.into()));
        self
    }
    
    /// Add a price field with formatting
    pub fn price(self, name: impl Into<String>, value: f64, currency: &str) -> Self {
        self.field(name, format_price(value, currency))
    }
    
    /// Add a percentage field
    pub fn percentage(self, name: impl Into<String>, value: f64) -> Self {
        let emoji = if value >= 0.0 { "📈" } else { "📉" };
        self.field(name, format!("{} {:.2}%", emoji, value))
    }
    
    /// Build the alert message
    pub fn build(self) -> String {
        let mut msg = String::new();
        writeln!(&mut msg, "{} **{}**", self.level.emoji(), self.title).unwrap();
        
        for (name, value) in self.fields {
            writeln!(&mut msg, "• *{}*: {}", name, escape_markdown(&value)).unwrap();
        }
        
        msg
    }
}

/// Format a price value
pub fn format_price(value: f64, currency: &str) -> String {
    match currency.to_uppercase().as_str() {
        "USD" | "USDC" | "USDT" => format!("${:.2}", value),
        "BTC" => format!("₿{:.8}", value),
        "ETH" => format!("Ξ{:.6}", value),
        _ => format!("{:.4} {}", value, currency),
    }
}

/// Format a percentage change
pub fn format_percentage(value: f64) -> String {
    let sign = if value >= 0.0 { "+" } else { "" };
    format!("{}{:.2}%", sign, value)
}

/// Escape markdown special characters
fn escape_markdown(text: &str) -> String {
    text.replace('_', "\\_")
        .replace('*', "\\*")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .replace('~', "\\~")
        .replace('`', "\\`")
        .replace('>', "\\>")
        .replace('#', "\\#")
        .replace('+', "\\+")
        .replace('-', "\\-")
        .replace('=', "\\=")
        .replace('|', "\\|")
        .replace('{', "\\{")
        .replace('}', "\\}")
        .replace('.', "\\.")
        .replace('!', "\\!")
}

/// Pre-built alert templates for trading
pub mod templates {
    use super::*;
    
    pub fn trade_executed(
        side: &str,
        symbol: &str,
        amount: f64,
        price: f64,
        total: f64,
    ) -> String {
        AlertBuilder::success("Trade Executed")
            .field("Side", side)
            .field("Symbol", symbol)
            .price("Amount", amount, "")
            .price("Price", price, "USD")
            .price("Total", total, "USD")
            .build()
    }
    
    pub fn price_alert(symbol: &str, price: f64, change_pct: f64, threshold: f64) -> String {
        let level = if change_pct.abs() >= threshold * 2.0 {
            AlertLevel::Critical
        } else if change_pct.abs() >= threshold {
            AlertLevel::Warning
        } else {
            AlertLevel::Info
        };
        
        AlertBuilder::new(level, &format!("{} Price Alert", symbol))
            .price("Current Price", price, "USD")
            .percentage("Change", change_pct)
            .build()
    }
    
    pub fn risk_alert(metric: &str, value: f64, threshold: f64) -> String {
        AlertBuilder::warning("Risk Threshold Reached")
            .field("Metric", metric)
            .field("Current Value", format!("{:.2}", value))
            .field("Threshold", format!("{:.2}", threshold))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_alert_builder() {
        let alert = AlertBuilder::success("Test Alert")
            .field("Key", "Value")
            .build();
        
        assert!(alert.contains("✅ **Test Alert**"));
        assert!(alert.contains("Key"));
        assert!(alert.contains("Value"));
    }
    
    #[test]
    fn test_format_price() {
        assert_eq!(format_price(100.5, "USD"), "$100.50");
        assert_eq!(format_price(0.00123456, "BTC"), "₿0.00123456");
    }
}
