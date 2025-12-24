//! Type definitions for Open-Meteo API clients

pub mod common;
pub mod forecast;
pub mod geocoding;

pub use common::{Coordinates, PrecipitationUnit, TemperatureUnit, TimeFormat, WindSpeedUnit};
pub use forecast::{
    CurrentData, DailyData, ForecastRequest, ForecastResponse, HourlyData,
};
pub use geocoding::{GeocodingRequest, GeocodingResponse, Location};
