"""Basic usage example for arepy_steamworks.

This example demonstrates how to integrate Steam with Arepy ECS.
Requires Steam to be running.
"""

from __future__ import annotations

from arepy import ArepyEngine, SystemPipeline
from arepy.ecs import Component, Entities, Query, With
from arepy_steamworks import SteamResource, steam_tick_system


class Score(Component):
    """Player score component."""

    def __init__(self, value: int = 0) -> None:
        """Initialize score with a value.

        Args:
            value: Initial score value.
        """
        self.value = value


def achievement_system(
    steam: SteamResource, query: Query[Entities, With[Score]]
) -> None:
    """Check player scores and unlock achievements."""
    for entity, score in query.iter_entities_components(Score):
        if score.value >= 100 and steam.unlock_achievement("HIGH_SCORE"):
            print(f"Unlocked HIGH_SCORE for entity {entity}")


def main() -> None:
    """Run the Steam integration example."""
    engine = ArepyEngine(
        title="Steam Example", width=800, height=600, max_frame_rate=60
    )
    world = engine.create_world("game")

    try:
        steam = SteamResource(app_id=480)
        engine.add_resource(steam)

        # User info
        print(f"Logged in: {steam.is_logged_on}")
        print(f"Username: {steam.username}")
        print(f"Steam ID: {steam.steam_id}")
        print(f"User Level: {steam.user_level}")

        # App info
        print(f"App ID: {steam.app_id}")
        print(f"Language: {steam.current_language}")
        print(f"Available languages: {steam.available_languages}")
        print(f"Build ID: {steam.build_id}")

        # Platform detection
        print(f"Steam Deck: {steam.is_steam_deck}")
        print(f"Big Picture: {steam.is_big_picture_mode}")
        print(f"Overlay enabled: {steam.is_overlay_enabled}")

        # Achievements
        print(f"Achievement count: {steam.achievement_count}")
        print(f"First 5 achievements: {steam.achievement_names[:5]}")

        # Cloud storage
        print(f"Cloud enabled: {steam.is_cloud_enabled}")
        print(f"Cloud files: {steam.cloud_files()}")

        # Register systems
        world.add_system(SystemPipeline.UPDATE, steam_tick_system)
        world.add_system(SystemPipeline.UPDATE, achievement_system)

        # Create test entity
        world.create_entity().with_component(Score(value=150)).build()

        engine.run()

    except RuntimeError as e:
        print(f"Steam not available: {e}")
        print(
            "Make sure Steam is running and you have steam_appid.txt with your app ID"
        )


if __name__ == "__main__":
    main()
