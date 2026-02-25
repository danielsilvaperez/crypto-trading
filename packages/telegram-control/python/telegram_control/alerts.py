"""Alert formatting utilities for trading notifications."""

from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
from typing import List, Tuple


class AlertLevel(Enum):
    """Alert severity levels."""
    
    INFO = "ℹ️"
    SUCCESS = "✅"
    WARNING = "⚠️"
    ERROR = "❌"
    CRITICAL = "🚨"


@dataclass
class AlertBuilder:
    """Builder for trading alerts."""
    
    level: AlertLevel
    title: str
    fields: List[Tuple[str, str]] = field(default_factory=list)
    timestamp: datetime = field(default_factory=datetime.utcnow)
    
    @classmethod
    def info(cls, title: str) -> "AlertBuilder":
        """Create an info alert."""
        return cls(level=AlertLevel.INFO, title=title)
    
    @classmethod
    def success(cls, title: str) -> "AlertBuilder":
        """Create a success alert."""
        return cls(level=AlertLevel.SUCCESS, title=title)
    
    @classmethod
    def warning(cls, title: str) -> "AlertBuilder":
        """Create a warning alert."""
        return cls(level=AlertLevel.WARNING, title=title)
    
    @classmethod
    def error(cls, title: str) -> "AlertBuilder":
        """Create an error alert."""
        return cls(level=AlertLevel.ERROR, title=title)
    
    @classmethod
    def critical(cls, title: str) -> "AlertBuilder":
        """Create a critical alert."""
        return cls(level=AlertLevel.CRITICAL, title=title)
    
    def field(self, name: str, value: str) -> "AlertBuilder":
        """Add a field to the alert."""
        self.fields.append((name, value))
        return self
    
    def price(self, name: str, value: float, currency: str = "USD") -> "AlertBuilder":
        """Add a price field with formatting."""
        return self.field(name, format_price(value, currency))
    
    def percentage(self, name: str, value: float) -> "AlertBuilder":
        """Add a percentage field."""
        emoji = "📈" if value >= 0 else "📉"
        return self.field(name, f"{emoji} {value:.2f}%")
    
    def build(self) -> str:
        """Build the alert message."""
        lines = [f"{self.level.value} **{self.title}**"]
        
        for name, value in self.fields:
            lines.append(f"• *{name}*: {escape_markdown(value)}")
        
        return "\n".join(lines)
    
    def __str__(self) -> str:
        """String representation."""
        return self.build()


def format_price(value: float, currency: str = "USD") -> str:
    """Format a price value."""
    currency = currency.upper()
    
    if currency in ("USD", "USDC", "USDT"):
        return f"${value:.2f}"
    elif currency == "BTC":
        return f"₿{value:.8f}"
    elif currency == "ETH":
        return f"Ξ{value:.6f}"
    else:
        return f"{value:.4f} {currency}"


def format_percentage(value: float) -> str:
    """Format a percentage change."""
    sign = "+" if value >= 0 else ""
    return f"{sign}{value:.2f}%"


def escape_markdown(text: str) -> str:
    """Escape markdown special characters."""
    chars = ["_", "*", "[", "]", "(", ")", "~", "`", ">", "#", "+", "-", "=", "|", "{", "}", ".", "!"]
    for char in chars:
        text = text.replace(char, f"\\{char}")
    return text


# Pre-built alert templates
def trade_executed(
    side: str,
    symbol: str,
    amount: float,
    price: float,
    total: float
) -> str:
    """Alert for executed trade."""
    return (
        AlertBuilder.success("Trade Executed")
        .field("Side", side)
        .field("Symbol", symbol)
        .field("Amount", str(amount))
        .price("Price", price)
        .price("Total", total)
        .build()
    )


def price_alert(symbol: str, price: float, change_pct: float, threshold: float) -> str:
    """Price movement alert."""
    if abs(change_pct) >= threshold * 2:
        level = AlertLevel.CRITICAL
    elif abs(change_pct) >= threshold:
        level = AlertLevel.WARNING
    else:
        level = AlertLevel.INFO
    
    return (
        AlertBuilder(level, f"{symbol} Price Alert")
        .price("Current Price", price)
        .percentage("Change", change_pct)
        .build()
    )


def risk_alert(metric: str, value: float, threshold: float) -> str:
    """Risk threshold alert."""
    return (
        AlertBuilder.warning("Risk Threshold Reached")
        .field("Metric", metric)
        .field("Current Value", f"{value:.2f}")
        .field("Threshold", f"{threshold:.2f}")
        .build()
    )
