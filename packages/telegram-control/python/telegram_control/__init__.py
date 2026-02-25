"""
Telegram Control - A reusable Telegram bot framework for crypto trading.

Example:
    >>> from telegram_control import Bot, Context
    >>> 
    >>> bot = Bot(token="YOUR_BOT_TOKEN")
    >>> 
    >>> @bot.command("status")
    >>> async def status(ctx: Context):
    >>>     await ctx.reply("System operational!")
    >>> 
    >>> bot.run()
"""

from .alerts import AlertBuilder, AlertLevel, format_price
from .auth import AccessControl
from .bot import Bot
from .commands import Command, command
from .keyboards import InlineKeyboardBuilder, ReplyKeyboardBuilder
from .types import CallbackContext, Context, MessageContext

__version__ = "0.1.0"
__all__ = [
    "AlertBuilder",
    "AlertLevel",
    "AccessControl",
    "Bot",
    "Command",
    "CallbackContext",
    "Context",
    "InlineKeyboardBuilder",
    "MessageContext",
    "ReplyKeyboardBuilder",
    "command",
    "format_price",
]
