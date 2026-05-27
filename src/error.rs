//! Error types for arepy-steamworks.

use pyo3::exceptions::PyRuntimeError;
use pyo3::PyErr;

/// Errors that can occur during Steam operations.
#[derive(Debug, thiserror::Error)]
pub enum SteamError {
    /// Steam client initialization failed.
    #[error("Failed to initialize Steam: {0}")]
    InitFailed(String),

    /// A stat or achievement operation failed.
    #[error("Stat operation failed: {0}")]
    StatFailed(String),

    /// Cloud storage operation failed.
    #[error("Cloud operation failed: {0}")]
    CloudFailed(String),
}

impl From<SteamError> for PyErr {
    fn from(err: SteamError) -> Self {
        PyRuntimeError::new_err(err.to_string())
    }
}
