//! Apps interface methods.

use steamworks::{AppId, Client};

/// Check if a DLC is installed.
pub fn is_dlc_installed(client: &Client, dlc_app_id: u32) -> bool {
    client.apps().is_dlc_installed(AppId(dlc_app_id))
}

/// Check if the user is subscribed to the app.
pub fn is_subscribed(client: &Client) -> bool {
    client.apps().is_subscribed()
}

/// Get the current game language.
pub fn current_language(client: &Client) -> String {
    client.apps().current_game_language()
}

/// Get the list of available game languages.
pub fn available_languages(client: &Client) -> Vec<String> {
    client.apps().available_game_languages()
}

/// Get the app build ID.
pub fn build_id(client: &Client) -> i32 {
    client.apps().app_build_id()
}

/// Check if the app is running in low violence mode.
pub fn is_low_violence(client: &Client) -> bool {
    client.apps().is_low_violence()
}

/// Check if the user has a VAC ban.
pub fn is_vac_banned(client: &Client) -> bool {
    client.apps().is_vac_banned()
}

/// Get the current beta branch name, if any.
pub fn beta_name(client: &Client) -> Option<String> {
    client.apps().current_beta_name()
}
