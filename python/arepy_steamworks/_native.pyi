"""Type stubs for the native Rust module."""

class SteamResource:
    """Steam Resource for Arepy ECS integration.

    This class wraps the Steamworks Client and provides methods for accessing
    Steam features. It should be registered as a global Resource in Arepy.

    Attributes:
        steam_id: The current user's Steam ID.
        username: The current user's display name.
        is_logged_on: Whether the user is logged into Steam.
        user_level: The user's Steam level.
        achievement_count: Number of achievements defined for this app.
        achievement_names: List of all achievement names.
        app_id: The current App ID.
        is_subscribed: Whether the user is subscribed to the app.
        current_language: The current game language.
        available_languages: List of available game languages.
        build_id: The app build ID.
        is_low_violence: Whether the app is in low violence mode.
        is_vac_banned: Whether the user has a VAC ban.
        beta_name: The current beta branch name, if any.
        is_overlay_enabled: Whether the Steam overlay is enabled.
        is_steam_deck: Whether running on Steam Deck.
        is_big_picture_mode: Whether running in Big Picture mode.
        country_code: The user's country code (based on IP).
        ui_language: The Steam UI language.
        server_time: The Steam server time (Unix timestamp).
        is_cloud_enabled: Whether cloud storage is enabled for this app.
        is_cloud_enabled_for_account: Whether cloud storage is enabled for the account.
    """

    def __init__(self, app_id: int) -> None:
        """Initialize Steam with the given App ID.

        Args:
            app_id: The Steam App ID (use 480 for Spacewar demo/testing).

        Raises:
            RuntimeError: If Steam client is not running or initialization fails.
        """

    def run_callbacks(self) -> None:
        """Process pending Steam callbacks.

        This method MUST be called once per frame to process Steam events.
        Releases the GIL during execution to avoid blocking Python.
        """

    @property
    def steam_id(self) -> int:
        """Get the current user's Steam ID."""

    @property
    def username(self) -> str:
        """Get the current user's display name."""

    @property
    def is_logged_on(self) -> bool:
        """Check if the user is logged into Steam."""

    @property
    def user_level(self) -> int:
        """Get the user's Steam level."""

    def unlock_achievement(self, achievement_id: str) -> bool:
        """Unlock an achievement by ID.

        Args:
            achievement_id: The achievement identifier (e.g., "ACH_WIN_ONE_GAME").

        Returns:
            True if the achievement was unlocked and stored successfully.

        Raises:
            RuntimeError: If the operation fails.
        """

    def is_achievement_unlocked(self, achievement_id: str) -> bool:
        """Check if an achievement is unlocked.

        Args:
            achievement_id: The achievement identifier.

        Returns:
            True if the achievement is unlocked, False otherwise.
        """

    @property
    def achievement_count(self) -> int:
        """Get the number of achievements defined for this app.

        Raises:
            RuntimeError: If retrieval fails.
        """

    @property
    def achievement_names(self) -> list[str]:
        """Get all achievement names.

        Returns:
            A list of achievement name strings.

        Raises:
            RuntimeError: If retrieval fails.
        """

    def store_stats(self) -> bool:
        """Store stats and achievements to Steam servers.

        This method releases the GIL as it may involve network communication.

        Returns:
            True if stats were stored successfully.
        """

    def get_stat_int(self, name: str) -> int:
        """Get an integer stat value.

        Args:
            name: The stat name as defined in Steamworks.

        Returns:
            The stat value.

        Raises:
            RuntimeError: If the stat doesn't exist or retrieval fails.
        """

    def set_stat_int(self, name: str, value: int) -> None:
        """Set an integer stat value (in-memory only).

        Call store_stats() to persist to Steam servers.

        Args:
            name: The stat name.
            value: The integer value to set.

        Raises:
            RuntimeError: If the stat doesn't exist.
        """

    def get_stat_float(self, name: str) -> float:
        """Get a float stat value.

        Args:
            name: The stat name.

        Returns:
            The stat value.

        Raises:
            RuntimeError: If the stat doesn't exist.
        """

    def set_stat_float(self, name: str, value: float) -> None:
        """Set a float stat value (in-memory only).

        Call store_stats() to persist to Steam servers.

        Args:
            name: The stat name.
            value: The float value to set.

        Raises:
            RuntimeError: If the stat doesn't exist.
        """

    def reset_all_stats(self, achievements_too: bool) -> bool:
        """Reset all stats to their default values.

        Args:
            achievements_too: If True, also reset all achievements.

        Returns:
            True if reset was successful.
        """

    @property
    def app_id(self) -> int:
        """Get the current App ID."""

    def is_dlc_installed(self, dlc_app_id: int) -> bool:
        """Check if a DLC is installed.

        Args:
            dlc_app_id: The DLC's App ID.

        Returns:
            True if the DLC is installed.
        """

    @property
    def is_subscribed(self) -> bool:
        """Check if the user is subscribed to the app."""

    @property
    def current_language(self) -> str:
        """Get the current game language."""

    @property
    def available_languages(self) -> list[str]:
        """Get the list of available game languages."""

    @property
    def build_id(self) -> int:
        """Get the app build ID."""

    @property
    def is_low_violence(self) -> bool:
        """Check if the app is running in low violence mode."""

    @property
    def is_vac_banned(self) -> bool:
        """Check if the user has a VAC ban."""

    @property
    def beta_name(self) -> str | None:
        """Get the current beta branch name, if any."""

    def set_rich_presence(self, key: str, value: str) -> bool:
        """Set a rich presence key-value pair.

        Rich presence is displayed to friends in the Steam friends list.

        Args:
            key: The presence key (e.g., "steam_display", "status").
            value: The presence value.

        Returns:
            True if the presence was set successfully.
        """

    def clear_rich_presence(self) -> None:
        """Clear all rich presence data."""

    @property
    def is_overlay_enabled(self) -> bool:
        """Check if the Steam overlay is enabled."""

    def activate_overlay(self, dialog: str) -> None:
        """Activate the Steam overlay to a specific dialog.

        Args:
            dialog: The dialog to show: "friends", "community", "players",
                "settings", "officialgamegroup", "stats", "achievements".
        """

    def activate_overlay_to_web_page(self, url: str) -> None:
        """Activate the Steam overlay to a web page.

        Args:
            url: The URL to open in the overlay browser.
        """

    def activate_overlay_to_store(self, app_id: int) -> None:
        """Activate the Steam overlay to a store page.

        Args:
            app_id: The App ID to show in the store.
        """

    @property
    def is_steam_deck(self) -> bool:
        """Check if running on Steam Deck."""

    @property
    def is_big_picture_mode(self) -> bool:
        """Check if running in Big Picture mode."""

    @property
    def country_code(self) -> str:
        """Get the user's country code (based on IP)."""

    @property
    def ui_language(self) -> str:
        """Get the Steam UI language."""

    @property
    def server_time(self) -> int:
        """Get the Steam server time (Unix timestamp)."""

    def set_notification_position(self, position: str) -> None:
        """Set the overlay notification position.

        Args:
            position: Position: "top_left", "top_right", "bottom_left", "bottom_right".
        """

    @property
    def is_cloud_enabled(self) -> bool:
        """Check if cloud storage is enabled for this app."""

    @property
    def is_cloud_enabled_for_account(self) -> bool:
        """Check if cloud storage is enabled for the user's account."""

    def cloud_files(self) -> list[tuple[str, int]]:
        """List all files in cloud storage.

        Returns:
            A list of tuples: (filename, size_bytes).
        """

    def cloud_file_exists(self, filename: str) -> bool:
        """Check if a cloud file exists.

        Args:
            filename: The file name to check.

        Returns:
            True if the file exists.
        """

    def cloud_read(self, filename: str) -> bytes | None:
        """Read a file from cloud storage.

        Args:
            filename: The file name to read.

        Returns:
            The file contents as bytes, or None if the file doesn't exist.

        Note:
            This operation may block internally. Use with caution in
            performance-critical code.
        """

    def cloud_write(self, filename: str, data: bytes) -> bool:
        """Write a file to cloud storage.

        Args:
            filename: The file name to write.
            data: The file contents as bytes.

        Returns:
            True if the write was successful.
        """

    def cloud_delete(self, filename: str) -> bool:
        """Delete a file from cloud storage.

        Args:
            filename: The file name to delete.

        Returns:
            True if the deletion was successful.
        """
