//! Cloud storage (Remote Storage) interface methods.

use pyo3::PyResult;
use std::io::{Read, Write};
use steamworks::Client;

use crate::SteamError;

/// Check if cloud storage is enabled for this app.
pub fn is_enabled_for_app(client: &Client) -> bool {
    client.remote_storage().is_cloud_enabled_for_app()
}

/// Check if cloud storage is enabled for the user's account.
pub fn is_enabled_for_account(client: &Client) -> bool {
    client.remote_storage().is_cloud_enabled_for_account()
}

/// List all files in cloud storage.
///
/// Returns a list of tuples: (filename, size_bytes)
pub fn list_files(client: &Client) -> Vec<(String, u64)> {
    client
        .remote_storage()
        .files()
        .into_iter()
        .map(|f| (f.name, f.size))
        .collect()
}

/// Check if a cloud file exists.
pub fn file_exists(client: &Client, filename: &str) -> bool {
    client.remote_storage().file(filename).exists()
}

/// Read a file from cloud storage.
///
/// Returns the file contents as bytes, or `None` if the file doesn't exist.
pub fn read_file(client: &Client, filename: &str) -> PyResult<Option<Vec<u8>>> {
    let file = client.remote_storage().file(filename);

    if !file.exists() {
        return Ok(None);
    }

    let mut reader = file.read();
    let mut contents = Vec::new();
    reader
        .read_to_end(&mut contents)
        .map_err(|_| SteamError::CloudFailed(format!("Failed to read file: {}", filename)))?;

    Ok(Some(contents))
}

/// Write a file to cloud storage.
pub fn write_file(client: &Client, filename: &str, data: &[u8]) -> PyResult<bool> {
    let file = client.remote_storage().file(filename);
    let mut writer = file.write();

    writer
        .write_all(data)
        .map_err(|_| SteamError::CloudFailed(format!("Failed to write file: {}", filename)))?;

    Ok(true)
}

/// Delete a file from cloud storage.
pub fn delete_file(client: &Client, filename: &str) -> bool {
    client.remote_storage().file(filename).delete()
}
