//! Example usage of the tiny_weather Open-Meteo API client

use tiny_weather::client::types::{ForecastRequest, GeocodingRequest};
use tiny_weather::client::{ForecastClient, GeocodingClient};
use tiny_weather::args::TinyWeatherArgs;
use tracing::info;
use anyhow::Result;
use clap::Parser;
#[tokio::main]
async fn main() -> Result<()> {

    let args = TinyWeatherArgs::parse();
    info!(target: "tiny_weather", "args: {:?}", args);

    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Example 1: Geocoding - Search for a location
    println!("=== Geocoding Example ===");
    let geocoding_client = GeocodingClient::new();

    match geocoding_client
        .search(
            GeocodingRequest::new("San Francisco")
                .count(1)
                .language("en"),
        )
        .await
    {
        Ok(response) => {
            if let Some(location) = response.results.first() {
                println!(
                    "Found: {} ({}, {})",
                    location.name, location.latitude, location.longitude
                );
                println!("Country: {:?}", location.country);
                println!("Timezone: {:?}\n", location.timezone);

                // Example 2: Get weather forecast for the found location
                println!("=== Forecast Example ===");
                let forecast_client = ForecastClient::new();

                match forecast_client
                    .get(
                        ForecastRequest::new(location.latitude, location.longitude)
                            .hourly(&["temperature_2m", "precipitation"])
                            .daily(&["temperature_2m_max", "temperature_2m_min"])
                            .timezone("auto")
                            .temperature_unit(
                                tiny_weather::client::types::TemperatureUnit::Fahrenheit,
                            )
                            .forecast_days(3),
                    )
                    .await
                {
                    Ok(forecast) => {
                        println!("Weather forecast for {}", location.name);
                        println!("Timezone: {}", forecast.timezone);

                        if let Some(daily) = forecast.daily {
                            println!("\nDaily forecast:");
                            for (i, date) in daily.time.iter().enumerate() {
                                let temp_max = daily
                                    .variables
                                    .get("temperature_2m_max")
                                    .and_then(|v| v.get(i))
                                    .and_then(|v| *v);
                                let temp_min = daily
                                    .variables
                                    .get("temperature_2m_min")
                                    .and_then(|v| v.get(i))
                                    .and_then(|v| *v);

                                println!(
                                    "  {}: {:.1}°C - {:.1}°C",
                                    date,
                                    temp_min.unwrap_or(0.0),
                                    temp_max.unwrap_or(0.0)
                                );
                            }
                        }
                    }
                    Err(e) => eprintln!("Forecast error: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Geocoding error: {}", e),
    }

    Ok(())
}
