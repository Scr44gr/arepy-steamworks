//! Stats and achievements interface.

use pyo3::PyResult;
use steamworks::Client;

use crate::SteamError;

/// Unlock an achievement and store to Steam.
pub fn unlock_achievement(client: &Client, achievement_id: &str) -> PyResult<bool> {
    let stats = client.user_stats();
    let achievement = stats.achievement(achievement_id);

    achievement.set().map_err(|_| {
        SteamError::StatFailed(format!("Failed to set achievement: {}", achievement_id))
    })?;

    Ok(stats.store_stats().is_ok())
}

/// Check if an achievement is unlocked.
pub fn is_achievement_unlocked(client: &Client, achievement_id: &str) -> bool {
    let stats = client.user_stats();
    let achievement = stats.achievement(achievement_id);
    achievement.get().unwrap_or(false)
}

/// Get the number of achievements defined for this app.
pub fn achievement_count(client: &Client) -> PyResult<u32> {
    client
        .user_stats()
        .get_num_achievements()
        .map_err(|_| SteamError::StatFailed("Failed to get achievement count".to_string()).into())
}

/// Get all achievement names.
pub fn achievement_names(client: &Client) -> PyResult<Vec<String>> {
    client
        .user_stats()
        .get_achievement_names()
        .ok_or_else(|| SteamError::StatFailed("Failed to get achievement names".to_string()).into())
}

/// Store stats and achievements to Steam servers.
pub fn store_stats(client: &Client) -> bool {
    client.user_stats().store_stats().is_ok()
}

/// Get an integer stat value.
pub fn get_stat_int(client: &Client, name: &str) -> PyResult<i32> {
    client
        .user_stats()
        .get_stat_i32(name)
        .map_err(|_| SteamError::StatFailed(format!("Failed to get stat: {}", name)).into())
}

/// Set an integer stat value (in-memory only).
pub fn set_stat_int(client: &Client, name: &str, value: i32) -> PyResult<()> {
    client
        .user_stats()
        .set_stat_i32(name, value)
        .map_err(|_| SteamError::StatFailed(format!("Failed to set stat: {}", name)).into())
}

/// Get a float stat value.
pub fn get_stat_float(client: &Client, name: &str) -> PyResult<f32> {
    client
        .user_stats()
        .get_stat_f32(name)
        .map_err(|_| SteamError::StatFailed(format!("Failed to get stat: {}", name)).into())
}

/// Set a float stat value (in-memory only).
pub fn set_stat_float(client: &Client, name: &str, value: f32) -> PyResult<()> {
    client
        .user_stats()
        .set_stat_f32(name, value)
        .map_err(|_| SteamError::StatFailed(format!("Failed to set stat: {}", name)).into())
}

/// Reset all stats to their default values.
pub fn reset_all_stats(client: &Client, achievements_too: bool) -> bool {
    client
        .user_stats()
        .reset_all_stats(achievements_too)
        .is_ok()
}
