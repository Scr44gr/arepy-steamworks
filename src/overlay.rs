//! Overlay interface methods.

use steamworks::{AppId, Client};

/// Check if the Steam overlay is enabled.
pub fn is_enabled(client: &Client) -> bool {
    client.utils().is_overlay_enabled()
}

/// Activate the Steam overlay to a specific dialog.
pub fn activate(client: &Client, dialog: &str) {
    client.friends().activate_game_overlay(dialog);
}

/// Activate the Steam overlay to a web page.
pub fn activate_to_web_page(client: &Client, url: &str) {
    client.friends().activate_game_overlay_to_web_page(url);
}

/// Activate the Steam overlay to a store page.
pub fn activate_to_store(client: &Client, app_id: u32) {
    client
        .friends()
        .activate_game_overlay_to_store(AppId(app_id), steamworks::OverlayToStoreFlag::None);
}
