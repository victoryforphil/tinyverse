//! Geocoding API client for location search

use reqwest::Client;
use tracing::{debug, trace, warn};

use crate::client::error::{OpenMeteoError, Result};
use crate::client::types::geocoding::{GeocodingRequest, GeocodingResponse};

const GEOCODING_API_BASE: &str = "https://geocoding-api.open-meteo.com/v1/search";

/// Client for the Open-Meteo Geocoding API
#[derive(Debug, Clone)]
pub struct GeocodingClient {
    client: Client,
    pub(crate) base_url: String,
}

impl GeocodingClient {
    /// Create a new geocoding client with default settings
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: GEOCODING_API_BASE.to_string(),
        }
    }

    /// Create a new geocoding client with a custom HTTP client
    pub fn with_client(client: Client) -> Self {
        Self {
            client,
            base_url: GEOCODING_API_BASE.to_string(),
        }
    }

    /// Create a new geocoding client with a custom base URL (for testing)
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }

    /// Search for locations by name
    pub async fn search(&self, request: GeocodingRequest) -> Result<GeocodingResponse> {
        let mut params = vec![("name", request.name.clone())];

        if let Some(count) = request.count {
            params.push(("count", count.to_string()));
        }

        if let Some(language) = &request.language {
            params.push(("language", language.clone()));
        }

        if let Some(format) = &request.format {
            params.push(("format", format.clone()));
        }

        let url = reqwest::Url::parse_with_params(&self.base_url, &params)
            .map_err(|e| OpenMeteoError::InvalidParameter(e.to_string()))?;

        debug!("Searching for location: {} at {}", request.name, url);

        let response = self.client.get(url.clone()).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            warn!("API error ({}): {}", status, error_text);

            return Err(OpenMeteoError::ApiError {
                reason: format!("HTTP {}: {}", status, error_text),
            });
        }

        let body = response.text().await?;
        trace!("Response body: {}", body);

        let geocoding: GeocodingResponse = serde_json::from_str(&body)?;

        if geocoding.results.is_empty() {
            debug!("No results found for location: {}", request.name);
            return Err(OpenMeteoError::NoResults);
        }

        debug!(
            "Found {} location(s) for query: {}",
            geocoding.results.len(),
            request.name
        );

        Ok(geocoding)
    }
}

impl Default for GeocodingClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geocoding_client_creation() {
        let client = GeocodingClient::new();
        assert_eq!(client.base_url, GEOCODING_API_BASE);
    }

    #[test]
    fn test_geocoding_client_custom_base_url() {
        let client = GeocodingClient::with_base_url("https://example.com/api");
        assert_eq!(client.base_url, "https://example.com/api");
    }

    // Note: Integration tests with actual API calls should be in separate test files
    // and marked with #[ignore] to avoid hitting the API during normal test runs
}
