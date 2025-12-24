//! Error types for Open-Meteo API clients

use thiserror::Error;

/// Errors that can occur when interacting with Open-Meteo APIs
#[derive(Error, Debug)]
pub enum OpenMeteoError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    /// API returned an error response
    #[error("API error: {reason}")]
    ApiError { reason: String },

    /// Failed to deserialize API response
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(#[from] serde_json::Error),

    /// Invalid parameter provided to API call
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// No results found
    #[error("No results found for query")]
    NoResults,
}

/// Result type alias for Open-Meteo operations
pub type Result<T> = std::result::Result<T, OpenMeteoError>;
