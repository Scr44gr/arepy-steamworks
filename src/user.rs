//! User interface methods.

use steamworks::Client;

/// Get the current user's Steam ID.
pub fn steam_id(client: &Client) -> u64 {
    client.user().steam_id().raw()
}

/// Get the current user's display name.
pub fn username(client: &Client) -> String {
    client.friends().name()
}

/// Check if the user is logged into Steam.
pub fn is_logged_on(client: &Client) -> bool {
    client.user().logged_on()
}

/// Get the user's Steam level.
pub fn level(client: &Client) -> u32 {
    client.user().level()
}
