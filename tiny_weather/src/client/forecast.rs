//! Weather Forecast API client

use reqwest::Client;
use tracing::{debug, trace, warn};

use crate::client::error::{OpenMeteoError, Result};
use crate::client::types::forecast::{ForecastRequest, ForecastResponse};

const FORECAST_API_BASE: &str = "https://api.open-meteo.com/v1/forecast";

/// Client for the Open-Meteo Weather Forecast API
#[derive(Debug, Clone)]
pub struct ForecastClient {
    client: Client,
    pub(crate) base_url: String,
}

impl ForecastClient {
    /// Create a new forecast client with default settings
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: FORECAST_API_BASE.to_string(),
        }
    }

    /// Create a new forecast client with a custom HTTP client
    pub fn with_client(client: Client) -> Self {
        Self {
            client,
            base_url: FORECAST_API_BASE.to_string(),
        }
    }

    /// Create a new forecast client with a custom base URL (for testing)
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
        }
    }

    /// Get weather forecast for the given request
    pub async fn get(&self, request: ForecastRequest) -> Result<ForecastResponse> {
        let mut params = vec![
            ("latitude", request.latitude.to_string()),
            ("longitude", request.longitude.to_string()),
        ];

        if let Some(elevation) = request.elevation {
            params.push(("elevation", elevation.to_string()));
        }

        if let Some(hourly) = &request.hourly {
            params.push(("hourly", hourly.join(",")));
        }

        if let Some(daily) = &request.daily {
            params.push(("daily", daily.join(",")));
        }

        if let Some(current) = &request.current {
            params.push(("current", current.join(",")));
        }

        params.push((
            "temperature_unit",
            format!("{:?}", request.temperature_unit).to_lowercase(),
        ));
        params.push((
            "wind_speed_unit",
            format!("{:?}", request.wind_speed_unit).to_lowercase(),
        ));
        params.push((
            "precipitation_unit",
            format!("{:?}", request.precipitation_unit).to_lowercase(),
        ));
        params.push((
            "timeformat",
            if matches!(request.timeformat, crate::client::types::TimeFormat::Iso8601) {
                "iso8601"
            } else {
                "unixtime"
            }
            .to_string(),
        ));

        if let Some(timezone) = &request.timezone {
            params.push(("timezone", timezone.clone()));
        }

        if let Some(past_days) = request.past_days {
            params.push(("past_days", past_days.to_string()));
        }

        if let Some(forecast_days) = request.forecast_days {
            params.push(("forecast_days", forecast_days.to_string()));
        }

        if let Some(models) = &request.models {
            params.push(("models", models.clone()));
        }

        let url = reqwest::Url::parse_with_params(&self.base_url, &params)
            .map_err(|e| OpenMeteoError::InvalidParameter(e.to_string()))?;

        debug!("Fetching forecast from: {}", url);

        let response = self.client.get(url.clone()).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            warn!("API error ({}): {}", status, error_text);

            // Try to parse as API error
            if let Ok(error) = serde_json::from_str::<serde_json::Value>(&error_text) {
                if let Some(reason) = error.get("reason").and_then(|r| r.as_str()) {
                    return Err(OpenMeteoError::ApiError {
                        reason: reason.to_string(),
                    });
                }
            }

            return Err(OpenMeteoError::ApiError {
                reason: format!("HTTP {}: {}", status, error_text),
            });
        }

        let body = response.text().await?;
        trace!("Response body: {}", body);

        let forecast: ForecastResponse = serde_json::from_str(&body)?;

        debug!(
            "Successfully fetched forecast for ({}, {})",
            forecast.latitude, forecast.longitude
        );

        Ok(forecast)
    }
}

impl Default for ForecastClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forecast_client_creation() {
        let client = ForecastClient::new();
        assert_eq!(client.base_url, FORECAST_API_BASE);
    }

    #[test]
    fn test_forecast_client_custom_base_url() {
        let client = ForecastClient::with_base_url("https://example.com/api");
        assert_eq!(client.base_url, "https://example.com/api");
    }

    // Note: Integration tests with actual API calls should be in separate test files
    // and marked with #[ignore] to avoid hitting the API during normal test runs
}
