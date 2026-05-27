//! Utils interface methods.

use steamworks::{Client, NotificationPosition};

/// Get the current App ID.
pub fn app_id(client: &Client) -> u32 {
    client.utils().app_id().0
}

/// Check if running on Steam Deck.
pub fn is_steam_deck(client: &Client) -> bool {
    client.utils().is_steam_running_on_steam_deck()
}

/// Check if running in Big Picture mode.
pub fn is_big_picture_mode(client: &Client) -> bool {
    client.utils().is_steam_in_big_picture_mode()
}

/// Get the user's country code (based on IP).
pub fn country_code(client: &Client) -> String {
    client.utils().ip_country()
}

/// Get the Steam UI language.
pub fn ui_language(client: &Client) -> String {
    client.utils().ui_language()
}

/// Get the Steam server time (Unix timestamp).
pub fn server_time(client: &Client) -> u32 {
    client.utils().get_server_real_time()
}

/// Set the overlay notification position.
pub fn set_notification_position(client: &Client, position: &str) {
    let pos = match position {
        "top_left" => NotificationPosition::TopLeft,
        "top_right" => NotificationPosition::TopRight,
        "bottom_left" => NotificationPosition::BottomLeft,
        "bottom_right" => NotificationPosition::BottomRight,
        _ => NotificationPosition::BottomRight,
    };
    client.utils().set_overlay_notification_position(pos);
}
