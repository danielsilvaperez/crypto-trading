"""Access control for Telegram bot users."""

from typing import Optional, Set


class AccessControl:
    """Access control for bot users."""
    
    def __init__(
        self,
        whitelist: Optional[Set[int]] = None,
        admins: Optional[Set[int]] = None
    ):
        self._whitelist = whitelist
        self._admins = admins or set()
    
    def with_whitelist(self, ids: list[int]) -> "AccessControl":
        """Restrict access to specific user IDs."""
        self._whitelist = set(ids)
        return self
    
    def with_admins(self, ids: list[int]) -> "AccessControl":
        """Set admin users."""
        self._admins = set(ids)
        return self
    
    def is_authorized(self, user_id: int) -> bool:
        """Check if user is authorized."""
        if self._whitelist is None:
            return True  # No whitelist = open access
        return user_id in self._whitelist
    
    def is_admin(self, user_id: int) -> bool:
        """Check if user is an admin."""
        return user_id in self._admins
    
    def authorize(self, user_id: int) -> None:
        """Authorize a user, raising error if not authorized."""
        if not self.is_authorized(user_id):
            raise PermissionError(f"User {user_id} is not authorized")
