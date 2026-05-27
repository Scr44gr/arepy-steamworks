"""arepy-steamworks - Steamworks integration for Arepy ECS game engine.

This module provides a Python-friendly wrapper around the Steamworks API,
designed to work as an ECS Resource within Arepy's architecture.

Features:
    - User authentication and profile information
    - Achievements and stats management
    - Rich presence and overlay control
    - Cloud saves (Remote Storage)

GIL Management:
    Operations that may block (network calls, callbacks) release the Python GIL
    using `py.allow_threads()` to prevent blocking other Python threads.

Example:
    >>> from arepy import ArepyEngine, SystemPipeline
    >>> from arepy_steamworks import SteamResource, steam_tick_system
    >>>
    >>> engine = ArepyEngine(title="My Game", width=800, height=600)
    >>> world = engine.create_world("main")
    >>>
    >>> steam = SteamResource(app_id=480)
    >>> engine.add_resource(steam)
    >>> world.add_system(SystemPipeline.UPDATE, steam_tick_system)
"""

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from arepy_steamworks._native import SteamResource as SteamResource

try:
    from arepy_steamworks._native import SteamResource
except ImportError:
    SteamResource = None  # type: ignore[assignment,misc]

__all__ = [
    "CloudFileInfo",
    "NotificationPosition",
    "OverlayDialog",
    "SteamResource",
    "steam_tick_system",
]

# Type aliases for better documentation
CloudFileInfo = tuple[str, int]
"""Cloud file information: (filename, size_bytes)"""

OverlayDialog = str
"""
Overlay dialog types:
    - "friends": Friends list
    - "community": Community hub
    - "players": Players tab
    - "settings": Settings
    - "officialgamegroup": Official game group
    - "stats": Stats page
    - "achievements": Achievements page
"""

NotificationPosition = str
"""
Notification position options:
    - "top_left"
    - "top_right"
    - "bottom_left"
    - "bottom_right"
"""


def steam_tick_system(steam: SteamResource) -> None:
    """Process pending Steam callbacks.

    This system MUST be registered in the UPDATE pipeline to ensure Steam
    events are processed each frame. Without this, achievements, stats,
    and overlay updates may not work correctly.

    Args:
        steam: The SteamResource instance (injected by Arepy ECS).

    Example:
        >>> world.add_system(SystemPipeline.UPDATE, steam_tick_system)
    """
    steam.run_callbacks()
