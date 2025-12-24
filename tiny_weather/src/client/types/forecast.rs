//! Forecast API request and response types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::common::{PrecipitationUnit, TemperatureUnit, TimeFormat, WindSpeedUnit};

/// Request builder for weather forecast API
#[derive(Debug, Clone)]
pub struct ForecastRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: Option<f64>,
    pub hourly: Option<Vec<String>>,
    pub daily: Option<Vec<String>>,
    pub current: Option<Vec<String>>,
    pub temperature_unit: TemperatureUnit,
    pub wind_speed_unit: WindSpeedUnit,
    pub precipitation_unit: PrecipitationUnit,
    pub timeformat: TimeFormat,
    pub timezone: Option<String>,
    pub past_days: Option<u8>,
    pub forecast_days: Option<u8>,
    pub models: Option<String>,
}

impl ForecastRequest {
    /// Create a new forecast request for the given coordinates
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            elevation: None,
            hourly: None,
            daily: None,
            current: None,
            temperature_unit: TemperatureUnit::default(),
            wind_speed_unit: WindSpeedUnit::default(),
            precipitation_unit: PrecipitationUnit::default(),
            timeformat: TimeFormat::default(),
            timezone: None,
            past_days: None,
            forecast_days: None,
            models: None,
        }
    }

    /// Set elevation for statistical downscaling (meters)
    pub fn elevation(mut self, elevation: f64) -> Self {
        self.elevation = Some(elevation);
        self
    }

    /// Set hourly weather variables to fetch
    pub fn hourly(mut self, variables: &[&str]) -> Self {
        self.hourly = Some(variables.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Set daily weather variables to fetch
    pub fn daily(mut self, variables: &[&str]) -> Self {
        self.daily = Some(variables.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Set current weather variables to fetch
    pub fn current(mut self, variables: &[&str]) -> Self {
        self.current = Some(variables.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Set temperature unit
    pub fn temperature_unit(mut self, unit: TemperatureUnit) -> Self {
        self.temperature_unit = unit;
        self
    }

    /// Set wind speed unit
    pub fn wind_speed_unit(mut self, unit: WindSpeedUnit) -> Self {
        self.wind_speed_unit = unit;
        self
    }

    /// Set precipitation unit
    pub fn precipitation_unit(mut self, unit: PrecipitationUnit) -> Self {
        self.precipitation_unit = unit;
        self
    }

    /// Set time format
    pub fn timeformat(mut self, format: TimeFormat) -> Self {
        self.timeformat = format;
        self
    }

    /// Set timezone (e.g., "America/New_York" or "auto")
    pub fn timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = Some(timezone.into());
        self
    }

    /// Set number of past days to include (0-92)
    pub fn past_days(mut self, days: u8) -> Self {
        self.past_days = Some(days);
        self
    }

    /// Set number of forecast days (1-16)
    pub fn forecast_days(mut self, days: u8) -> Self {
        self.forecast_days = Some(days);
        self
    }

    /// Set specific weather model
    pub fn models(mut self, models: impl Into<String>) -> Self {
        self.models = Some(models.into());
        self
    }
}

/// Weather forecast API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly: Option<HourlyData>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_units: Option<HashMap<String, String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<DailyData>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_units: Option<HashMap<String, String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<CurrentData>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_units: Option<HashMap<String, String>>,
}

/// Hourly weather data with flexible variable support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyData {
    pub time: Vec<String>,
    
    #[serde(flatten)]
    pub variables: HashMap<String, Vec<Option<f64>>>,
}

/// Daily weather data with flexible variable support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyData {
    pub time: Vec<String>,
    
    #[serde(flatten)]
    pub variables: HashMap<String, Vec<Option<f64>>>,
}

/// Current weather conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentData {
    pub time: String,
    
    #[serde(flatten)]
    pub variables: HashMap<String, Option<f64>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forecast_request_builder() {
        let request = ForecastRequest::new(52.52, 13.41)
            .hourly(&["temperature_2m", "precipitation"])
            .daily(&["temperature_2m_max"])
            .timezone("auto");

        assert_eq!(request.latitude, 52.52);
        assert_eq!(request.longitude, 13.41);
        assert_eq!(request.hourly.as_ref().unwrap().len(), 2);
        assert_eq!(request.daily.as_ref().unwrap().len(), 1);
        assert_eq!(request.timezone.as_ref().unwrap(), "auto");
    }

    #[test]
    fn test_forecast_response_deserialization() {
        let json = r#"{
            "latitude": 52.52,
            "longitude": 13.419,
            "elevation": 44.812,
            "generationtime_ms": 2.2119,
            "utc_offset_seconds": 7200,
            "timezone": "Europe/Berlin",
            "timezone_abbreviation": "CEST",
            "hourly": {
                "time": ["2022-07-01T00:00", "2022-07-01T01:00"],
                "temperature_2m": [13.0, 12.7]
            },
            "hourly_units": {
                "temperature_2m": "°C"
            }
        }"#;

        let response: ForecastResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.latitude, 52.52);
        assert_eq!(response.timezone, "Europe/Berlin");
        
        let hourly = response.hourly.unwrap();
        assert_eq!(hourly.time.len(), 2);
        assert_eq!(hourly.variables.get("temperature_2m").unwrap().len(), 2);
    }
}
