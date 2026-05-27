# arepy-steamworks

Steamworks integration for the [Arepy](https://github.com/scr44gr/arepy) ECS game engine. Built with Rust and PyO3 for performance and safety.

## Requirements

- Python 3.11+
- Steam client running
- Steamworks SDK redistributable (see [Setup](#setup))

## Installation

```bash
pip install arepy-steamworks
```

## Setup

The Steamworks SDK requires a native library to be available at runtime. You must obtain and place this file yourself for security and licensing reasons.

### Step 1: Download the Steamworks SDK

Download the SDK from [partner.steamgames.com](https://partner.steamgames.com/doc/sdk).

### Step 2: Copy the redistributable

Copy the appropriate library file to your project's working directory (where you run your game from):

| Platform | File |
|----------|------|
| Windows (64-bit) | `steam_api64.dll` |
| Windows (32-bit) | `steam_api.dll` |
| Linux | `libsteam_api.so` |
| macOS | `libsteam_api.dylib` |

These files are located in the SDK under `sdk/redistributable_bin/`.

### Step 3: Create steam_appid.txt (Development)

For development, create a `steam_appid.txt` file in your project root:

```
480
```

App ID `480` is Spacewar, Steam's demo app for testing.

### Production

When your game is launched through Steam, the SDK libraries are provided automatically by the Steam client.

## Usage

### Basic Integration

```python
from arepy import ArepyEngine, SystemPipeline
from arepy_steamworks import SteamResource, steam_tick_system

engine = ArepyEngine(title="My Game", width=800, height=600)
world = engine.create_world("main")

steam = SteamResource(app_id=480)
engine.add_resource(steam)

world.add_system(SystemPipeline.UPDATE, steam_tick_system)

engine.run()
```

The `steam_tick_system` processes Steam callbacks each frame. It must be registered in the `UPDATE` pipeline.

### Using Steam in Systems

Inject `SteamResource` into any Arepy system via type annotation:

```python
from arepy.ecs import Query, Entities, With
from arepy_steamworks import SteamResource

def achievement_system(steam: SteamResource, query: Query[Entities, With[Score]]):
    for entity, score in query.iter_entities_components(Score):
        if score.value >= 100:
            steam.unlock_achievement("HIGH_SCORE")
```

### Cloud Saves

```python
steam.cloud_write("save.dat", save_data)
data = steam.cloud_read("save.dat")
steam.cloud_delete("save.dat")
```

### Rich Presence

```python
steam.set_rich_presence("status", "In Menu")
steam.set_rich_presence("steam_display", "#StatusInMenu")
steam.clear_rich_presence()
```

## API Reference

### SteamResource

| Method / Property | Returns | Description |
|---|---|---|
| `SteamResource(app_id)` | - | Initialize with a Steam App ID |
| `run_callbacks()` | `None` | Process pending Steam callbacks |
| `steam_id` | `int` | Current user's Steam ID |
| `username` | `str` | Current user's display name |
| `is_logged_on` | `bool` | Whether the user is logged in |
| `user_level` | `int` | User's Steam level |
| `app_id` | `int` | Current App ID |
| `unlock_achievement(id)` | `bool` | Unlock an achievement |
| `is_achievement_unlocked(id)` | `bool` | Check if achievement is unlocked |
| `achievement_count` | `int` | Number of achievements |
| `achievement_names` | `list[str]` | All achievement names |
| `store_stats()` | `bool` | Flush stats to Steam servers |
| `get_stat_int(name)` | `int` | Get integer stat |
| `set_stat_int(name, value)` | `None` | Set integer stat |
| `get_stat_float(name)` | `float` | Get float stat |
| `set_stat_float(name, value)` | `None` | Set float stat |
| `reset_all_stats(achievements_too)` | `bool` | Reset all stats |
| `is_dlc_installed(dlc_app_id)` | `bool` | Check if DLC is installed |
| `is_subscribed` | `bool` | Whether user is subscribed |
| `current_language` | `str` | Current game language |
| `available_languages` | `list[str]` | Available languages |
| `build_id` | `int` | App build ID |
| `beta_name` | `str or None` | Beta branch name |
| `is_vac_banned` | `bool` | VAC ban status |
| `is_low_violence` | `bool` | Low violence mode |
| `set_rich_presence(key, value)` | `bool` | Set rich presence |
| `clear_rich_presence()` | `None` | Clear rich presence |
| `is_overlay_enabled` | `bool` | Overlay enabled status |
| `activate_overlay(dialog)` | `None` | Open overlay dialog |
| `activate_overlay_to_web_page(url)` | `None` | Open URL in overlay |
| `activate_overlay_to_store(app_id)` | `None` | Open store in overlay |
| `is_steam_deck` | `bool` | Running on Steam Deck |
| `is_big_picture_mode` | `bool` | Big Picture mode |
| `country_code` | `str` | User's country code |
| `ui_language` | `str` | Steam UI language |
| `server_time` | `int` | Steam server time |
| `set_notification_position(pos)` | `None` | Set notification position |
| `is_cloud_enabled` | `bool` | Cloud enabled for app |
| `is_cloud_enabled_for_account` | `bool` | Cloud enabled for account |
| `cloud_files()` | `list[tuple]` | List cloud files |
| `cloud_file_exists(filename)` | `bool` | Check if file exists |
| `cloud_read(filename)` | `bytes or None` | Read file from cloud |
| `cloud_write(filename, data)` | `bool` | Write file to cloud |
| `cloud_delete(filename)` | `bool` | Delete file from cloud |

## Development

### Prerequisites

- Rust toolchain (stable)
- Python 3.11+
- [uv](https://docs.astral.sh/uv/) package manager
- Steam client running

### Build from source

```bash
git clone https://github.com/scr44gr/arepy-steamworks.git
cd arepy-steamworks
uv sync
```

### Run tests

Tests require Steam to be running. Tests that require Steam will be skipped automatically if Steam is not available.

```bash
uv run pytest tests/ -v
```

### Lint and format

```bash
uv run ruff check python/ tests/ examples/
uv run ruff format python/ tests/ examples/
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all
```

## Acknowledgments

This project is built on top of excellent open-source work:

- [steamworks-rs](https://github.com/Noxime/steamworks-rs) - Rust bindings for the Steamworks SDK. Without this library, this integration would not be possible.
- [Arepy](https://github.com/scr44gr/arepy) - The ECS game engine this library is designed to integrate with.
- [PyO3](https://github.com/PyO3/pyo3) - Rust bindings for Python.
- [Maturin](https://github.com/PyO3/maturin) - Build tool for Rust-based Python packages.

## License

MIT
