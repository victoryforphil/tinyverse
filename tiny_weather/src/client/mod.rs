//! Open-Meteo API clients
//!
//! This module provides clients for interacting with Open-Meteo APIs:
//! - `ForecastClient`: Weather forecast data (hourly, daily, current)
//! - `GeocodingClient`: Location search and geocoding
//! - `OpenMeteoClient`: Unified client combining both APIs

pub mod error;
pub mod forecast;
pub mod geocoding;
pub mod types;

pub use error::{OpenMeteoError, Result};
pub use forecast::ForecastClient;
pub use geocoding::GeocodingClient;

/// Unified client for all Open-Meteo APIs
///
/// This is a convenience wrapper that combines the forecast and geocoding clients.
#[derive(Debug, Clone)]
pub struct OpenMeteoClient {
    forecast: ForecastClient,
    geocoding: GeocodingClient,
}

impl OpenMeteoClient {
    /// Create a new unified Open-Meteo client
    pub fn new() -> Self {
        Self {
            forecast: ForecastClient::new(),
            geocoding: GeocodingClient::new(),
        }
    }

    /// Create a new unified client with a custom HTTP client
    pub fn with_client(client: reqwest::Client) -> Self {
        Self {
            forecast: ForecastClient::with_client(client.clone()),
            geocoding: GeocodingClient::with_client(client),
        }
    }

    /// Get the forecast client
    pub fn forecast(&self) -> &ForecastClient {
        &self.forecast
    }

    /// Get the geocoding client
    pub fn geocoding(&self) -> &GeocodingClient {
        &self.geocoding
    }
}

impl Default for OpenMeteoClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_client_creation() {
        let client = OpenMeteoClient::new();
        assert!(client.forecast().base_url.contains("forecast"));
        assert!(client.geocoding().base_url.contains("geocoding"));
    }
}
