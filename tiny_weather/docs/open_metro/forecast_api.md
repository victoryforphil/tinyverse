# Weather Forecast API

Seamless integration of high-resolution weather models with up 16 days forecast

## Location and Time

### Coordinates
- **Latitude/Longitude**: WGS84 geographical coordinates (required)
- **Elevation**: Optional elevation for statistical downscaling
- **Multiple locations**: Comma-separated coordinates supported

### Time Parameters
- **Forecast days**: 0-16 days (default: 7)
- **Past days**: 0-92 days for historical data
- **Timezone**: Any timezone from tz database or "auto" for automatic detection

## Hourly Weather Variables

### Core Variables
- **Temperature (2m)**: Air temperature at 2 meters above ground
- **Relative Humidity (2m)**: Relative humidity percentage
- **Dewpoint (2m)**: Dew point temperature
- **Apparent Temperature**: Perceived temperature with wind chill
- **Precipitation Probability**: Probability of precipitation >0.1mm
- **Precipitation**: Total precipitation (rain + showers + snow)
- **Rain**: Rain from large scale weather systems
- **Showers**: Convective precipitation
- **Snowfall**: Snow amount in centimeters
- **Weather Code**: WMO weather interpretation codes

### Wind Variables
- **Wind Speed**: Available at 10m, 80m, 120m, 180m heights
- **Wind Direction**: Available at same heights as wind speed
- **Wind Gusts (10m)**: Maximum gusts in preceding hour

### Pressure & Cloud Variables
- **Sea Level Pressure**: Atmospheric pressure at mean sea level
- **Surface Pressure**: Pressure at surface level
- **Cloud Cover**: Total, low, mid, high level clouds
- **Visibility**: Viewing distance in meters

### Solar Radiation Variables
- **Shortwave Solar Radiation (GHI)**: Global horizontal irradiation
- **Direct Solar Radiation**: Direct solar radiation
- **Diffuse Solar Radiation (DHI)**: Diffuse horizontal irradiation
- **Direct Normal Irradiance (DNI)**: Direct normal irradiance
- **Global Tilted Radiation (GTI)**: For solar panels (requires tilt/azimuth)
- **Terrestrial Solar Radiation**: Available radiation

### Soil Variables
- **Soil Temperature**: At 0cm, 6cm, 18cm, 54cm depths
- **Soil Moisture**: Volumetric water content at various depths

### Additional Variables
- **UV Index**: UV radiation index
- **Sunshine Duration**: Hours of direct sunlight
- **CAPE**: Convective available potential energy
- **Evapotranspiration**: Water evaporation from land/plants
- **Reference Evapotranspiration (ET₀)**: FAO-56 standard

## 15-Minutely Weather Variables

Available for Central Europe and North America with high-resolution models:
- Temperature, humidity, precipitation
- Wind speed and direction
- Solar radiation variables
- Weather codes and visibility

## Daily Weather Variables

### Temperature Aggregations
- **Max/Min Temperature (2m)**: Daily temperature extremes
- **Max/Min Apparent Temperature**: Daily apparent temperature extremes
- **Mean Temperature**: Daily average temperature

### Precipitation Aggregations
- **Precipitation Sum**: Total daily precipitation
- **Rain Sum**: Daily rain total
- **Showers Sum**: Daily shower total  
- **Snowfall Sum**: Daily snowfall total
- **Precipitation Hours**: Hours with precipitation
- **Precipitation Probability Max**: Maximum daily precipitation probability

### Solar & Wind Aggregations
- **Sunrise/Sunset**: Sun rise and set times
- **Daylight Duration**: Total daylight hours
- **Sunshine Duration**: Direct sunshine hours
- **Shortwave Radiation Sum**: Daily solar energy total
- **Wind Speed Max**: Maximum daily wind speed
- **Wind Gusts Max**: Maximum daily wind gusts
- **Dominant Wind Direction**: Prevailing wind direction

## Current Weather

Real-time conditions based on 15-minutely model data:
- All hourly variables available as current conditions
- Based on latest model runs
- Updated frequently throughout the day

## Weather Models

### Global Models
- **ECMWF IFS**: European Centre forecasts (9km-25km resolution)
- **NCEP GFS**: US Global Forecast System (11km-25km)
- **CMA GRAPES**: China Meteorological Administration
- **BOM ACCESS**: Australian Bureau of Meteorology

### Regional High-Resolution Models
- **DWD ICON**: German Weather Service (2-11km)
- **NOAA HRRR**: US high-resolution (3km)
- **Météo-France AROME**: French high-resolution (1km)
- **UK Met Office**: British forecasts (2-10km)

## API Endpoint

**Base URL**: `https://api.open-meteo.com/v1/forecast`

### Required Parameters
- `latitude`: Floating point WGS84 latitude
- `longitude`: Floating point WGS84 longitude

### Optional Parameters
- `elevation`: Elevation for downscaling (meters)
- `hourly`: Comma-separated list of hourly variables
- `daily`: Comma-separated list of daily variables  
- `current`: Comma-separated list of current variables
- `temperature_unit`: celsius (default) or fahrenheit
- `wind_speed_unit`: kmh (default), ms, mph, kn
- `precipitation_unit`: mm (default) or inch
- `timeformat`: iso8601 (default) or unixtime
- `timezone`: Timezone name or "auto"
- `past_days`: 0-92 days of historical data
- `forecast_days`: 1-16 days of forecast
- `models`: Specific weather model selection

### Example Request

```
https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&hourly=temperature_2m,relative_humidity_2m,precipitation&daily=temperature_2m_max,temperature_2m_min,precipitation_sum&timezone=auto
```

## Response Format

```json
{
    "latitude": 52.52,
    "longitude": 13.419,
    "elevation": 44.812,
    "generationtime_ms": 2.2119,
    "utc_offset_seconds": 0,
    "timezone": "Europe/Berlin",
    "timezone_abbreviation": "CEST",
    "hourly": {
        "time": ["2022-07-01T00:00", "2022-07-01T01:00", ...],
        "temperature_2m": [13, 12.7, 12.7, 12.5, ...]
    },
    "hourly_units": {
        "temperature_2m": "°C"
    },
    "daily": {
        "time": ["2022-07-01", "2022-07-02", ...],
        "temperature_2m_max": [22.1, 24.3, ...],
        "temperature_2m_min": [11.2, 13.1, ...]
    },
    "daily_units": {
        "temperature_2m_max": "°C",
        "temperature_2m_min": "°C"
    }
}
```

## WMO Weather Codes

| Code | Description |
|------|-------------|
| 0 | Clear sky |
| 1,2,3 | Mainly clear, partly cloudy, overcast |
| 45,48 | Fog and depositing rime fog |
| 51,53,55 | Drizzle: Light, moderate, dense |
| 56,57 | Freezing drizzle: Light, dense |
| 61,63,65 | Rain: Slight, moderate, heavy |
| 66,67 | Freezing rain: Light, heavy |
| 71,73,75 | Snow fall: Slight, moderate, heavy |
| 77 | Snow grains |
| 80,81,82 | Rain showers: Slight, moderate, violent |
| 85,86 | Snow showers: Slight, heavy |
| 95 | Thunderstorm: Slight or moderate |
| 96,99 | Thunderstorm with slight/heavy hail |

## Error Handling

API returns JSON error objects with HTTP 400 status for invalid requests:

```json
{
    "error": true,
    "reason": "Cannot initialize WeatherVariable from invalid String value temperature_2m for key hourly"
}
```

## Usage Limits

- **Non-Commercial**: Free tier with rate limiting
- **Commercial**: Requires API key and subscription
- **Self-Hosted**: Available for enterprise use

## Model Update Frequency

| Provider | Model | Update Frequency | Resolution | Forecast Length |
|----------|-------|------------------|------------|-----------------|
| DWD | ICON | Every 3 hours | 2-11 km | 7.5 days |
| NOAA | GFS/HRRR | Every hour | 3-25 km | 16 days |
| ECMWF | IFS | Every 6 hours | 25 km | 15 days |
| Météo-France | ARPEGE/AROME | Every hour | 1-25 km | 4 days |

## Data Sources

Weather forecast APIs combine multiple national weather providers for global coverage. The "Best Match" option automatically selects optimal models for each location worldwide.