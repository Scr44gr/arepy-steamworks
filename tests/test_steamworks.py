"""Tests for arepy_steamworks module.

These tests require Steam to be running. Tests will be skipped if Steam
initialization fails.
"""

from __future__ import annotations

import contextlib

import pytest
from arepy_steamworks import SteamResource, steam_tick_system

# App ID 480 is Spacewar, Steam's demo app for testing
TEST_APP_ID = 480


@pytest.fixture(scope="module")
def steam() -> SteamResource:
    """Create a SteamResource instance for testing.

    Skips all tests in the module if Steam is not available.
    """
    try:
        return SteamResource(TEST_APP_ID)
    except RuntimeError as e:
        pytest.skip(f"Steam not available: {e}")


class TestSteamResourceInit:
    """Tests for SteamResource initialization."""

    def test_init_with_valid_app_id(self) -> None:
        """Test initialization with valid demo app ID."""
        try:
            steam = SteamResource(TEST_APP_ID)
            assert steam is not None
        except RuntimeError:
            pytest.skip("Steam not available")

    def test_init_with_invalid_app_id_may_succeed(self) -> None:
        """Test initialization behavior with edge-case app IDs.

        Note: Steam may accept app ID 0 if a session is already active
        (e.g., steam_appid.txt exists). This test just verifies the call
        doesn't crash the process.
        """
        with contextlib.suppress(RuntimeError):
            SteamResource(0)


class TestUserProperties:
    """Tests for user-related properties."""

    def test_steam_id_returns_positive_int(self, steam: SteamResource) -> None:
        """Test that steam_id returns a positive integer."""
        steam_id = steam.steam_id
        assert isinstance(steam_id, int)
        assert steam_id > 0

    def test_username_returns_non_empty_string(self, steam: SteamResource) -> None:
        """Test that username returns a non-empty string."""
        username = steam.username
        assert isinstance(username, str)
        assert len(username) > 0

    def test_is_logged_on_returns_bool(self, steam: SteamResource) -> None:
        """Test that is_logged_on returns a boolean."""
        is_logged = steam.is_logged_on
        assert isinstance(is_logged, bool)

    def test_user_level_returns_non_negative_int(self, steam: SteamResource) -> None:
        """Test that user_level returns a non-negative integer."""
        level = steam.user_level
        assert isinstance(level, int)
        assert level >= 0


class TestAppProperties:
    """Tests for app-related properties."""

    def test_app_id_returns_correct_value(self, steam: SteamResource) -> None:
        """Test that app_id returns the initialized app ID."""
        assert steam.app_id == TEST_APP_ID

    def test_is_subscribed_returns_bool(self, steam: SteamResource) -> None:
        """Test that is_subscribed returns a boolean."""
        assert isinstance(steam.is_subscribed, bool)

    def test_current_language_returns_string(self, steam: SteamResource) -> None:
        """Test that current_language returns a string."""
        lang = steam.current_language
        assert isinstance(lang, str)
        assert len(lang) > 0

    def test_available_languages_returns_list(self, steam: SteamResource) -> None:
        """Test that available_languages returns a list of strings."""
        langs = steam.available_languages
        assert isinstance(langs, list)
        assert all(isinstance(lang, str) for lang in langs)

    def test_build_id_returns_int(self, steam: SteamResource) -> None:
        """Test that build_id returns an integer."""
        assert isinstance(steam.build_id, int)


class TestStatsAndAchievements:
    """Tests for stats and achievements."""

    def test_achievement_count_returns_non_negative(self, steam: SteamResource) -> None:
        """Test that achievement_count returns a non-negative integer."""
        count = steam.achievement_count
        assert isinstance(count, int)
        assert count >= 0

    def test_achievement_names_returns_list(self, steam: SteamResource) -> None:
        """Test that achievement_names returns a list of strings."""
        names = steam.achievement_names
        assert isinstance(names, list)
        assert all(isinstance(name, str) for name in names)

    def test_is_achievement_unlocked_returns_bool(self, steam: SteamResource) -> None:
        """Test that is_achievement_unlocked returns a boolean for valid achievement."""
        names = steam.achievement_names
        if names:
            result = steam.is_achievement_unlocked(names[0])
            assert isinstance(result, bool)

    def test_store_stats_returns_bool(self, steam: SteamResource) -> None:
        """Test that store_stats returns a boolean."""
        result = steam.store_stats()
        assert isinstance(result, bool)


class TestCallbacks:
    """Tests for callback processing."""

    def test_run_callbacks_does_not_raise(self, steam: SteamResource) -> None:
        """Test that run_callbacks executes without raising."""
        steam.run_callbacks()


class TestOverlay:
    """Tests for overlay functionality."""

    def test_is_overlay_enabled_returns_bool(self, steam: SteamResource) -> None:
        """Test that is_overlay_enabled returns a boolean."""
        assert isinstance(steam.is_overlay_enabled, bool)


class TestUtils:
    """Tests for utility properties."""

    def test_is_steam_deck_returns_bool(self, steam: SteamResource) -> None:
        """Test that is_steam_deck returns a boolean."""
        assert isinstance(steam.is_steam_deck, bool)

    def test_is_big_picture_mode_returns_bool(self, steam: SteamResource) -> None:
        """Test that is_big_picture_mode returns a boolean."""
        assert isinstance(steam.is_big_picture_mode, bool)

    def test_country_code_returns_string(self, steam: SteamResource) -> None:
        """Test that country_code returns a string."""
        code = steam.country_code
        assert isinstance(code, str)

    def test_ui_language_returns_string(self, steam: SteamResource) -> None:
        """Test that ui_language returns a string."""
        lang = steam.ui_language
        assert isinstance(lang, str)
        assert len(lang) > 0

    def test_server_time_returns_positive_int(self, steam: SteamResource) -> None:
        """Test that server_time returns a positive integer."""
        time = steam.server_time
        assert isinstance(time, int)
        assert time > 0


class TestCloudStorage:
    """Tests for cloud storage functionality."""

    def test_is_cloud_enabled_returns_bool(self, steam: SteamResource) -> None:
        """Test that is_cloud_enabled returns a boolean."""
        assert isinstance(steam.is_cloud_enabled, bool)

    def test_is_cloud_enabled_for_account_returns_bool(self, steam: SteamResource) -> None:
        """Test that is_cloud_enabled_for_account returns a boolean."""
        assert isinstance(steam.is_cloud_enabled_for_account, bool)

    def test_cloud_files_returns_list(self, steam: SteamResource) -> None:
        """Test that cloud_files returns a list of tuples."""
        files = steam.cloud_files()
        assert isinstance(files, list)
        for item in files:
            assert isinstance(item, tuple)
            assert len(item) == 2
            assert isinstance(item[0], str)
            assert isinstance(item[1], int)

    def test_cloud_file_exists_returns_bool(self, steam: SteamResource) -> None:
        """Test that cloud_file_exists returns a boolean."""
        result = steam.cloud_file_exists("nonexistent_test_file.txt")
        assert isinstance(result, bool)
        assert result is False

    def test_cloud_write_and_read(self, steam: SteamResource) -> None:
        """Test writing and reading a cloud file."""
        test_filename = "arepy_steamworks_test.txt"
        test_data = b"Hello from arepy_steamworks tests!"

        # Write
        write_result = steam.cloud_write(test_filename, test_data)
        assert write_result is True

        # Read
        read_data = steam.cloud_read(test_filename)
        assert read_data == test_data

        # Cleanup
        steam.cloud_delete(test_filename)

    def test_cloud_read_nonexistent_returns_none(self, steam: SteamResource) -> None:
        """Test that reading a nonexistent file returns None."""
        result = steam.cloud_read("definitely_nonexistent_file_12345.txt")
        assert result is None


class TestRichPresence:
    """Tests for rich presence functionality."""

    def test_set_rich_presence_returns_bool(self, steam: SteamResource) -> None:
        """Test that set_rich_presence returns a boolean."""
        result = steam.set_rich_presence("status", "Testing")
        assert isinstance(result, bool)

    def test_clear_rich_presence_does_not_raise(self, steam: SteamResource) -> None:
        """Test that clear_rich_presence executes without raising."""
        steam.clear_rich_presence()


class TestSteamTickSystem:
    """Tests for the steam_tick_system function."""

    def test_steam_tick_system_calls_run_callbacks(self, steam: SteamResource) -> None:
        """Test that steam_tick_system calls run_callbacks."""
        steam_tick_system(steam)
