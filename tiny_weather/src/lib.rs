//! Tiny Weather - A simple, clean Rust client for Open-Meteo APIs
//!
//! This crate provides clients for:
//! - Weather Forecast API (hourly, daily, current conditions)
//! - Geocoding API (location search)

pub mod client;
pub mod args;
pub use client::{ForecastClient, GeocodingClient, OpenMeteoClient};
