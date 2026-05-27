//! # arepy-steamworks
//!
//! Steamworks integration for the Arepy ECS game engine.
//!
//! This crate provides a Python-friendly wrapper around the Steamworks API,
//! designed to work as an ECS Resource within Arepy's architecture.
//!
//! ## Features
//!
//! - User authentication and profile information
//! - Achievements and stats management
//! - Rich presence and overlay control
//! - Leaderboards (async operations)
//! - Cloud saves (Remote Storage)
//!
//! ## GIL Management
//!
//! Operations that may block (network calls, callbacks) release the Python GIL
//! using `py.detach()` to prevent blocking other Python threads.

use pyo3::prelude::*;

mod apps;
mod cloud;
mod error;
mod friends;
mod leaderboard;
mod overlay;
mod stats;
mod user;
mod utils;

pub use error::SteamError;

/// Steam Resource for Arepy ECS integration.
///
/// This struct wraps the Steamworks Client and provides methods for accessing
/// Steam features. It should be registered as a global Resource in Arepy.
///
/// # Example
///
/// ```python
/// from arepy_steamworks import SteamResource, steam_tick_system
///
/// steam = SteamResource(app_id=480)
/// engine.add_resource(steam)
/// world.add_system(SystemPipeline.UPDATE, steam_tick_system)
/// ```
#[pyclass]
pub struct SteamResource {
    pub(crate) client: steamworks::Client,
}

#[pymethods]
impl SteamResource {
    /// Initialize Steam with the given App ID.
    ///
    /// # Arguments
    ///
    /// * `app_id` - The Steam App ID (use 480 for Spacewar demo/testing)
    ///
    /// # Errors
    ///
    /// Raises `RuntimeError` if Steam client is not running or initialization fails.
    #[new]
    fn new(app_id: u32) -> PyResult<Self> {
        let client = steamworks::Client::init_app(steamworks::AppId(app_id))
            .map_err(|e| SteamError::InitFailed(e.to_string()))?;

        Ok(Self { client })
    }

    /// Process pending Steam callbacks.
    ///
    /// This method MUST be called once per frame to process Steam events.
    /// Releases the GIL during execution to avoid blocking Python.
    ///
    /// # Example
    ///
    /// ```python
    /// def steam_tick_system(steam: SteamResource) -> None:
    ///     steam.run_callbacks()
    /// ```
    fn run_callbacks(&self, py: Python<'_>) {
        let client = self.client.clone();
        py.detach(|| {
            client.run_callbacks();
        });
    }

    /// Get the current user's Steam ID.
    #[getter]
    fn steam_id(&self) -> u64 {
        user::steam_id(&self.client)
    }

    /// Get the current user's display name.
    #[getter]
    fn username(&self) -> String {
        user::username(&self.client)
    }

    /// Check if the user is logged into Steam.
    #[getter]
    fn is_logged_on(&self) -> bool {
        user::is_logged_on(&self.client)
    }

    /// Get the user's Steam level.
    #[getter]
    fn user_level(&self) -> u32 {
        user::level(&self.client)
    }

    /// Unlock an achievement by ID.
    ///
    /// # Arguments
    ///
    /// * `achievement_id` - The achievement identifier (e.g., "ACH_WIN_ONE_GAME")
    ///
    /// # Returns
    ///
    /// `True` if the achievement was unlocked and stored successfully.
    ///
    /// # Errors
    ///
    /// Raises `RuntimeError` if the operation fails.
    fn unlock_achievement(&self, achievement_id: &str) -> PyResult<bool> {
        stats::unlock_achievement(&self.client, achievement_id)
    }

    /// Check if an achievement is unlocked.
    ///
    /// # Arguments
    ///
    /// * `achievement_id` - The achievement identifier
    ///
    /// # Returns
    ///
    /// `True` if the achievement is unlocked, `False` otherwise.
    fn is_achievement_unlocked(&self, achievement_id: &str) -> bool {
        stats::is_achievement_unlocked(&self.client, achievement_id)
    }

    /// Get the number of achievements defined for this app.
    #[getter]
    fn achievement_count(&self) -> PyResult<u32> {
        stats::achievement_count(&self.client)
    }

    /// Get all achievement names.
    ///
    /// # Returns
    ///
    /// A list of achievement name strings.
    #[getter]
    fn achievement_names(&self) -> PyResult<Vec<String>> {
        stats::achievement_names(&self.client)
    }

    /// Store stats and achievements to Steam servers.
    ///
    /// This method releases the GIL as it may involve network communication.
    ///
    /// # Returns
    ///
    /// `True` if stats were stored successfully.
    fn store_stats(&self, py: Python<'_>) -> bool {
        let client = self.client.clone();
        py.detach(move || stats::store_stats(&client))
    }

    /// Get an integer stat value.
    ///
    /// # Arguments
    ///
    /// * `name` - The stat name as defined in Steamworks
    ///
    /// # Errors
    ///
    /// Raises `RuntimeError` if the stat doesn't exist or retrieval fails.
    fn get_stat_int(&self, name: &str) -> PyResult<i32> {
        stats::get_stat_int(&self.client, name)
    }

    /// Set an integer stat value (in-memory only).
    ///
    /// Call `store_stats()` to persist to Steam servers.
    ///
    /// # Arguments
    ///
    /// * `name` - The stat name
    /// * `value` - The integer value to set
    ///
    /// # Errors
    ///
    /// Raises `RuntimeError` if the stat doesn't exist.
    fn set_stat_int(&self, name: &str, value: i32) -> PyResult<()> {
        stats::set_stat_int(&self.client, name, value)
    }

    /// Get a float stat value.
    ///
    /// # Arguments
    ///
    /// * `name` - The stat name
    ///
    /// # Errors
    ///
    /// Raises `RuntimeError` if the stat doesn't exist.
    fn get_stat_float(&self, name: &str) -> PyResult<f32> {
        stats::get_stat_float(&self.client, name)
    }

    /// Set a float stat value (in-memory only).
    ///
    /// Call `store_stats()` to persist to Steam servers.
    ///
    /// # Arguments
    ///
    /// * `name` - The stat name
    /// * `value` - The float value to set
    ///
    /// # Errors
    ///
    /// Raises `RuntimeError` if the stat doesn't exist.
    fn set_stat_float(&self, name: &str, value: f32) -> PyResult<()> {
        stats::set_stat_float(&self.client, name, value)
    }

    /// Reset all stats to their default values.
    ///
    /// # Arguments
    ///
    /// * `achievements_too` - If `True`, also reset all achievements
    ///
    /// # Returns
    ///
    /// `True` if reset was successful.
    fn reset_all_stats(&self, achievements_too: bool) -> bool {
        stats::reset_all_stats(&self.client, achievements_too)
    }

    /// Get the current App ID.
    #[getter]
    fn app_id(&self) -> u32 {
        utils::app_id(&self.client)
    }

    /// Check if a DLC is installed.
    ///
    /// # Arguments
    ///
    /// * `dlc_app_id` - The DLC's App ID
    fn is_dlc_installed(&self, dlc_app_id: u32) -> bool {
        apps::is_dlc_installed(&self.client, dlc_app_id)
    }

    /// Check if the user is subscribed to the app.
    #[getter]
    fn is_subscribed(&self) -> bool {
        apps::is_subscribed(&self.client)
    }

    /// Get the current game language.
    #[getter]
    fn current_language(&self) -> String {
        apps::current_language(&self.client)
    }

    /// Get the list of available game languages.
    #[getter]
    fn available_languages(&self) -> Vec<String> {
        apps::available_languages(&self.client)
    }

    /// Get the app build ID.
    #[getter]
    fn build_id(&self) -> i32 {
        apps::build_id(&self.client)
    }

    /// Check if the app is running in low violence mode.
    #[getter]
    fn is_low_violence(&self) -> bool {
        apps::is_low_violence(&self.client)
    }

    /// Check if the user has a VAC ban.
    #[getter]
    fn is_vac_banned(&self) -> bool {
        apps::is_vac_banned(&self.client)
    }

    /// Get the current beta branch name, if any.
    #[getter]
    fn beta_name(&self) -> Option<String> {
        apps::beta_name(&self.client)
    }

    /// Set a rich presence key-value pair.
    ///
    /// Rich presence is displayed to friends in the Steam friends list.
    ///
    /// # Arguments
    ///
    /// * `key` - The presence key (e.g., "steam_display", "status")
    /// * `value` - The presence value
    ///
    /// # Returns
    ///
    /// `True` if the presence was set successfully.
    fn set_rich_presence(&self, key: &str, value: &str) -> bool {
        friends::set_rich_presence(&self.client, key, value)
    }

    /// Clear all rich presence data.
    fn clear_rich_presence(&self) {
        friends::clear_rich_presence(&self.client);
    }

    /// Check if the Steam overlay is enabled.
    #[getter]
    fn is_overlay_enabled(&self) -> bool {
        overlay::is_enabled(&self.client)
    }

    /// Activate the Steam overlay to a specific dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog` - The dialog to show: "friends", "community", "players",
    ///   "settings", "officialgamegroup", "stats", "achievements"
    fn activate_overlay(&self, dialog: &str) {
        overlay::activate(&self.client, dialog);
    }

    /// Activate the Steam overlay to a web page.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to open in the overlay browser
    fn activate_overlay_to_web_page(&self, url: &str) {
        overlay::activate_to_web_page(&self.client, url);
    }

    /// Activate the Steam overlay to a store page.
    ///
    /// # Arguments
    ///
    /// * `app_id` - The App ID to show in the store
    fn activate_overlay_to_store(&self, app_id: u32) {
        overlay::activate_to_store(&self.client, app_id);
    }

    /// Check if running on Steam Deck.
    #[getter]
    fn is_steam_deck(&self) -> bool {
        utils::is_steam_deck(&self.client)
    }

    /// Check if running in Big Picture mode.
    #[getter]
    fn is_big_picture_mode(&self) -> bool {
        utils::is_big_picture_mode(&self.client)
    }

    /// Get the user's country code (based on IP).
    #[getter]
    fn country_code(&self) -> String {
        utils::country_code(&self.client)
    }

    /// Get the Steam UI language.
    #[getter]
    fn ui_language(&self) -> String {
        utils::ui_language(&self.client)
    }

    /// Get the Steam server time (Unix timestamp).
    #[getter]
    fn server_time(&self) -> u32 {
        utils::server_time(&self.client)
    }

    /// Set the overlay notification position.
    ///
    /// # Arguments
    ///
    /// * `position` - Position: "top_left", "top_right", "bottom_left", "bottom_right"
    fn set_notification_position(&self, position: &str) {
        utils::set_notification_position(&self.client, position);
    }

    /// Check if cloud storage is enabled for this app.
    #[getter]
    fn is_cloud_enabled(&self) -> bool {
        cloud::is_enabled_for_app(&self.client)
    }

    /// Check if cloud storage is enabled for the user's account.
    #[getter]
    fn is_cloud_enabled_for_account(&self) -> bool {
        cloud::is_enabled_for_account(&self.client)
    }

    /// List all files in cloud storage.
    ///
    /// # Returns
    ///
    /// A list of tuples: (filename, size_bytes)
    fn cloud_files(&self) -> Vec<(String, u64)> {
        cloud::list_files(&self.client)
    }

    /// Check if a cloud file exists.
    ///
    /// # Arguments
    ///
    /// * `filename` - The file name to check
    fn cloud_file_exists(&self, filename: &str) -> bool {
        cloud::file_exists(&self.client, filename)
    }

    /// Read a file from cloud storage.
    ///
    /// # Arguments
    ///
    /// * `filename` - The file name to read
    ///
    /// # Returns
    ///
    /// The file contents as bytes, or `None` if the file doesn't exist.
    ///
    /// # Note
    ///
    /// This operation may block internally. Use with caution in performance-critical code.
    fn cloud_read(&self, py: Python<'_>, filename: &str) -> PyResult<Option<Vec<u8>>> {
        let client = self.client.clone();
        let filename = filename.to_string();
        py.detach(move || cloud::read_file(&client, &filename))
    }

    /// Write a file to cloud storage.
    ///
    /// # Arguments
    ///
    /// * `filename` - The file name to write
    /// * `data` - The file contents as bytes
    ///
    /// # Returns
    ///
    /// `True` if the write was successful.
    fn cloud_write(&self, filename: &str, data: &[u8]) -> PyResult<bool> {
        cloud::write_file(&self.client, filename, data)
    }

    /// Delete a file from cloud storage.
    ///
    /// # Arguments
    ///
    /// * `filename` - The file name to delete
    ///
    /// # Returns
    ///
    /// `True` if the deletion was successful.
    fn cloud_delete(&self, filename: &str) -> bool {
        cloud::delete_file(&self.client, filename)
    }
}

#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SteamResource>()?;
    Ok(())
}
