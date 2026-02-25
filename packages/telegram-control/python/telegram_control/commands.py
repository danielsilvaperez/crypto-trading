"""Command handling utilities."""

import functools
import re
from typing import Any, Callable, Dict, List, Optional, get_type_hints


class Command:
    """Represents a bot command."""
    
    def __init__(
        self,
        name: str,
        description: str,
        handler: Callable,
        args: Optional[List[tuple]] = None,
    ):
        self.name = name
        self.description = description
        self.handler = handler
        self.args = args or []
    
    def __call__(self, *args, **kwargs):
        return self.handler(*args, **kwargs)


def command(name: str, description: str = ""):
    """Decorator to register a command."""
    def decorator(func: Callable) -> Command:
        # Extract argument types from type hints
        type_hints = get_type_hints(func)
        args = []
        
        # Skip 'ctx' parameter
        for param_name, param_type in list(type_hints.items())[1:]:
            args.append((param_name, param_type))
        
        return Command(
            name=name,
            description=description,
            handler=func,
            args=args,
        )
    return decorator


class CommandRegistry:
    """Registry for bot commands."""
    
    def __init__(self):
        self._commands: Dict[str, Command] = {}
    
    def register(self, cmd: Command) -> None:
        """Register a command."""
        self._commands[cmd.name] = cmd
    
    def get(self, name: str) -> Optional[Command]:
        """Get a command by name."""
        return self._commands.get(name)
    
    def parse(self, text: str) -> Optional[tuple]:
        """
        Parse a command from text.
        
        Returns:
            Tuple of (command, args) or None if not found.
        """
        parts = text.split()
        if not parts:
            return None
        
        cmd_name = parts[0]
        if cmd_name.startswith("/"):
            cmd_name = cmd_name[1:]
        
        cmd = self._commands.get(cmd_name)
        if not cmd:
            return None
        
        # Parse arguments
        args = parts[1:]
        parsed_args = []
        
        for i, (arg_name, arg_type) in enumerate(cmd.args):
            if i >= len(args):
                break
            
            try:
                if arg_type == int:
                    parsed_args.append(int(args[i]))
                elif arg_type == float:
                    parsed_args.append(float(args[i]))
                elif arg_type == bool:
                    parsed_args.append(args[i].lower() in ("true", "1", "yes"))
                else:
                    parsed_args.append(args[i])
            except ValueError:
                raise ValueError(f"Invalid value for {arg_name}: {args[i]}")
        
        return cmd, parsed_args
    
    def help_text(self) -> str:
        """Generate help text for all commands."""
        lines = ["Available commands:"]
        
        for cmd in self._commands.values():
            args_str = " ".join([f"<{name}>" for name, _ in cmd.args])
            lines.append(f"  /{cmd.name} {args_str} - {cmd.description}")
        
        return "\n".join(lines)
