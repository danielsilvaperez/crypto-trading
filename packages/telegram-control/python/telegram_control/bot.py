"""Core bot implementation."""

import asyncio
import logging
from typing import Any, Awaitable, Callable, Dict, List, Optional, Pattern, Union

from telegram import Update
from telegram.ext import (
    Application,
    ApplicationBuilder,
    CallbackQueryHandler,
    CommandHandler,
    ContextTypes,
    MessageHandler,
    filters,
)

from .auth import AccessControl
from .types import CallbackContext, MessageContext

logger = logging.getLogger(__name__)

# Type aliases
HandlerFunc = Callable[[Any], Awaitable[None]]


class Bot:
    """Telegram Bot wrapper with trading-focused features."""
    
    def __init__(
        self,
        token: str,
        access_control: Optional[AccessControl] = None,
    ):
        self.token = token
        self.access_control = access_control or AccessControl()
        self._command_handlers: Dict[str, HandlerFunc] = {}
        self._callback_handlers: List[tuple] = []  # (pattern, handler)
        self._default_handler: Optional[HandlerFunc] = None
        self._application: Optional[Application] = None
    
    def with_whitelist(self, ids: List[int]) -> "Bot":
        """Restrict access to specific user IDs."""
        self.access_control.with_whitelist(ids)
        return self
    
    def with_admins(self, ids: List[int]) -> "Bot":
        """Set admin users."""
        self.access_control.with_admins(ids)
        return self
    
    def command(self, name: str):
        """Decorator for command handlers."""
        def decorator(func: HandlerFunc) -> HandlerFunc:
            self._command_handlers[f"/{name}"] = func
            self._command_handlers[name] = func
            return func
        return decorator
    
    def callback(self, pattern: Union[str, Pattern]):
        """Decorator for callback query handlers."""
        def decorator(func: HandlerFunc) -> HandlerFunc:
            self._callback_handlers.append((pattern, func))
            return func
        return decorator
    
    def default(self, func: HandlerFunc) -> HandlerFunc:
        """Set default handler for unmatched messages."""
        self._default_handler = func
        return func
    
    async def _handle_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle incoming commands."""
        if not update.message or not update.effective_user:
            return
        
        user_id = update.effective_user.id
        
        # Check authorization
        if not self.access_control.is_authorized(user_id):
            logger.warning(f"Unauthorized access attempt from user {user_id}")
            await update.message.reply_text("⛔ You are not authorized to use this bot.")
            return
        
        text = update.message.text or ""
        parts = text.split()
        cmd = parts[0] if parts else ""
        
        # Find handler
        handler = self._command_handlers.get(cmd)
        if handler:
            ctx = MessageContext(
                message=update.message,
                user=update.effective_user,
                chat_id=update.effective_chat.id if update.effective_chat else 0,
                text=text,
                bot=self,
            )
            try:
                await handler(ctx)
            except Exception as e:
                logger.exception("Command handler error")
                await update.message.reply_text(f"❌ Error: {e}")
        elif self._default_handler:
            ctx = MessageContext(
                message=update.message,
                user=update.effective_user,
                chat_id=update.effective_chat.id if update.effective_chat else 0,
                text=text,
                bot=self,
            )
            try:
                await self._default_handler(ctx)
            except Exception as e:
                logger.exception("Default handler error")
                await update.message.reply_text(f"❌ Error: {e}")
    
    async def _handle_callback(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle callback queries."""
        if not update.callback_query or not update.effective_user:
            return
        
        query = update.callback_query
        data = query.data or ""
        
        # Find matching handler
        for pattern, handler in self._callback_handlers:
            import re
            if isinstance(pattern, str):
                if data.startswith(pattern):
                    ctx = CallbackContext(
                        query=query,
                        user=update.effective_user,
                        chat_id=update.effective_chat.id if update.effective_chat else 0,
                        data=data,
                        bot=self,
                    )
                    try:
                        await handler(ctx)
                    except Exception as e:
                        logger.exception("Callback handler error")
                        await query.answer(f"Error: {e}")
                    return
            elif isinstance(pattern, Pattern):
                if pattern.match(data):
                    ctx = CallbackContext(
                        query=query,
                        user=update.effective_user,
                        chat_id=update.effective_chat.id if update.effective_chat else 0,
                        data=data,
                        bot=self,
                    )
                    try:
                        await handler(ctx)
                    except Exception as e:
                        logger.exception("Callback handler error")
                        await query.answer(f"Error: {e}")
                    return
    
    def build(self) -> Application:
        """Build the Application instance."""
        app = ApplicationBuilder().token(self.token).build()
        
        # Add command handler
        app.add_handler(
            MessageHandler(filters.COMMAND | filters.TEXT, self._handle_command)
        )
        
        # Add callback handler
        if self._callback_handlers:
            app.add_handler(CallbackQueryHandler(self._handle_callback))
        
        self._application = app
        return app
    
    def run(self, poll_interval: float = 1.0):
        """Run the bot (blocking)."""
        app = self.build()
        logger.info("Starting Telegram bot...")
        app.run_polling(poll_interval=poll_interval)
    
    async def run_async(self, poll_interval: float = 1.0):
        """Run the bot (async)."""
        app = self.build()
        logger.info("Starting Telegram bot...")
        await app.initialize()
        await app.start()
        await app.updater.start_polling(poll_interval=poll_interval)
    
    async def send_message(self, chat_id: int, text: str, **kwargs):
        """Send a message to a chat."""
        if self._application:
            await self._application.bot.send_message(chat_id=chat_id, text=text, **kwargs)
    
    async def send_alert(self, chat_id: int, text: str, **kwargs):
        """Send an alert (formatted message)."""
        await self.send_message(chat_id, text, parse_mode="Markdown", **kwargs)
