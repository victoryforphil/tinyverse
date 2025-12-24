//! Common types used across API clients

use serde::{Deserialize, Serialize};

/// Geographic coordinates (WGS84)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinates {
    /// Create new coordinates
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

/// Temperature unit for API responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl Default for TemperatureUnit {
    fn default() -> Self {
        Self::Celsius
    }
}

/// Wind speed unit for API responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindSpeedUnit {
    /// Kilometers per hour
    #[serde(rename = "kmh")]
    Kmh,
    /// Meters per second
    #[serde(rename = "ms")]
    Ms,
    /// Miles per hour
    #[serde(rename = "mph")]
    Mph,
    /// Knots
    #[serde(rename = "kn")]
    Kn,
}

impl Default for WindSpeedUnit {
    fn default() -> Self {
        Self::Kmh
    }
}

/// Precipitation unit for API responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrecipitationUnit {
    /// Millimeters
    #[serde(rename = "mm")]
    Mm,
    /// Inches
    #[serde(rename = "inch")]
    Inch,
}

impl Default for PrecipitationUnit {
    fn default() -> Self {
        Self::Mm
    }
}

/// Time format for API responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeFormat {
    #[serde(rename = "iso8601")]
    Iso8601,
    #[serde(rename = "unixtime")]
    Unixtime,
}

impl Default for TimeFormat {
    fn default() -> Self {
        Self::Iso8601
    }
}
