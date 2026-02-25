"""Type definitions for telegram-control."""

from dataclasses import dataclass
from typing import TYPE_CHECKING, Optional, Union

if TYPE_CHECKING:
    from telegram import CallbackQuery, Message, Update, User
    from telegram.ext import CallbackContext as ExtCallbackContext


@dataclass
class MessageContext:
    """Context for message-based commands."""
    
    message: "Message"
    user: "User"
    chat_id: int
    text: Optional[str]
    bot: "Bot"
    
    async def reply(
        self,
        text: str,
        parse_mode: Optional[str] = None,
        reply_markup=None,
        **kwargs
    ):
        """Reply to the message."""
        return await self.message.reply_text(
            text=text,
            parse_mode=parse_mode,
            reply_markup=reply_markup,
            **kwargs
        )
    
    async def reply_markdown(self, text: str, **kwargs):
        """Reply with Markdown formatting."""
        return await self.reply(text, parse_mode="Markdown", **kwargs)
    
    async def reply_html(self, text: str, **kwargs):
        """Reply with HTML formatting."""
        return await self.reply(text, parse_mode="HTML", **kwargs)


@dataclass
class CallbackContext:
    """Context for callback queries (inline keyboards)."""
    
    query: "CallbackQuery"
    user: "User"
    chat_id: int
    data: str
    bot: "Bot"
    
    async def answer(self, text: Optional[str] = None, **kwargs):
        """Answer the callback query."""
        return await self.query.answer(text=text, **kwargs)
    
    async def edit_message(
        self,
        text: str,
        parse_mode: Optional[str] = None,
        reply_markup=None,
        **kwargs
    ):
        """Edit the message associated with this callback."""
        if self.query.message:
            return await self.query.message.edit_text(
                text=text,
                parse_mode=parse_mode,
                reply_markup=reply_markup,
                **kwargs
            )


Context = Union[MessageContext, CallbackContext]
