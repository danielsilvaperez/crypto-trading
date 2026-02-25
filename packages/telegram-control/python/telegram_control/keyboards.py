"""Keyboard builders for Telegram bots."""

from typing import List

from telegram import InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, ReplyKeyboardMarkup


class InlineKeyboardBuilder:
    """Builder for inline keyboards."""
    
    def __init__(self):
        self._rows: List[List[InlineKeyboardButton]] = []
        self._current_row: List[InlineKeyboardButton] = []
    
    def button(self, text: str, callback_data: str) -> "InlineKeyboardBuilder":
        """Add a callback button to current row."""
        self._current_row.append(
            InlineKeyboardButton(text=text, callback_data=callback_data)
        )
        return self
    
    def url_button(self, text: str, url: str) -> "InlineKeyboardBuilder":
        """Add a URL button to current row."""
        self._current_row.append(InlineKeyboardButton(text=text, url=url))
        return self
    
    def row(self) -> "InlineKeyboardBuilder":
        """Finish current row and start new one."""
        if self._current_row:
            self._rows.append(self._current_row)
            self._current_row = []
        return self
    
    def build(self) -> InlineKeyboardMarkup:
        """Build the keyboard."""
        if self._current_row:
            self._rows.append(self._current_row)
        return InlineKeyboardMarkup(self._rows)


class ReplyKeyboardBuilder:
    """Builder for reply keyboards."""
    
    def __init__(self):
        self._rows: List[List[KeyboardButton]] = []
        self._current_row: List[KeyboardButton] = []
        self._resize = True
        self._one_time = False
    
    def button(self, text: str) -> "ReplyKeyboardBuilder":
        """Add a button to current row."""
        self._current_row.append(KeyboardButton(text=text))
        return self
    
    def row(self) -> "ReplyKeyboardBuilder":
        """Finish current row and start new one."""
        if self._current_row:
            self._rows.append(self._current_row)
            self._current_row = []
        return self
    
    def resize(self, resize: bool = True) -> "ReplyKeyboardBuilder":
        """Set resize option."""
        self._resize = resize
        return self
    
    def one_time(self, one_time: bool = True) -> "ReplyKeyboardBuilder":
        """Set one_time option."""
        self._one_time = one_time
        return self
    
    def build(self) -> ReplyKeyboardMarkup:
        """Build the keyboard."""
        if self._current_row:
            self._rows.append(self._current_row)
        return ReplyKeyboardMarkup(
            self._rows,
            resize_keyboard=self._resize,
            one_time_keyboard=self._one_time,
        )


def confirm_keyboard(action: str) -> InlineKeyboardMarkup:
    """Confirmation keyboard (Yes/No)."""
    return (
        InlineKeyboardBuilder()
        .button("✅ Yes", f"{action}:yes")
        .button("❌ No", f"{action}:no")
        .build()
    )


def pagination_keyboard(current: int, total: int, prefix: str) -> InlineKeyboardMarkup:
    """Pagination keyboard."""
    builder = InlineKeyboardBuilder()
    
    if current > 0:
        builder.button("⬅️ Prev", f"{prefix}:page:{current - 1}")
    
    builder.button(f"{current + 1}/{total}", f"{prefix}:noop")
    
    if current < total - 1:
        builder.button("Next ➡️", f"{prefix}:page:{current + 1}")
    
    return builder.build()


def trading_actions_keyboard(symbol: str) -> InlineKeyboardMarkup:
    """Trading action keyboard."""
    return (
        InlineKeyboardBuilder()
        .button(f"📈 Buy {symbol}", f"trade:buy:{symbol}")
        .button(f"📉 Sell {symbol}", f"trade:sell:{symbol}")
        .row()
        .button("📊 View Chart", f"chart:{symbol}")
        .button("⚙️ Settings", "settings:open")
        .build()
    )


def main_menu_keyboard() -> ReplyKeyboardMarkup:
    """Main menu keyboard."""
    return (
        ReplyKeyboardBuilder()
        .button("📊 Dashboard")
        .button("💼 Portfolio")
        .row()
        .button("📈 Markets")
        .button("⚙️ Settings")
        .build()
    )
