//! Friends interface methods.

use steamworks::Client;

/// Set a rich presence key-value pair.
pub fn set_rich_presence(client: &Client, key: &str, value: &str) -> bool {
    client.friends().set_rich_presence(key, Some(value))
}

/// Clear all rich presence data.
pub fn clear_rich_presence(client: &Client) {
    client.friends().clear_rich_presence();
}
