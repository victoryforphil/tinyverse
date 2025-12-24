# Geocoding APIs for Location to Coordinates Conversion

This document outlines various APIs that can convert location strings (city names, zip codes, addresses) to latitude/longitude coordinates for use with the Open-Meteo weather API.

## Open-Meteo Geocoding API (Recommended)

**Perfect companion to Open-Meteo weather API**

- **Base URL**: `https://geocoding-api.open-meteo.com/v1/search`
- **Cost**: Free for non-commercial use
- **Rate Limits**: Reasonable limits for small to medium applications
- **Data Source**: GeoNames database
- **Advantages**: Same provider as weather API, consistent terms

### Features
- Global location search in any language
- Fuzzy matching (3+ characters)
- Exact matching (2 characters)
- Postal code support
- Country filtering
- Multiple language support
- Administrative area breakdown
- Population data
- Timezone information

### Example Request
```
https://geocoding-api.open-meteo.com/v1/search?name=Berlin&count=1&language=en
```

### Example Response
```json
{
    "results": [
        {
            "id": 2950159,
            "name": "Berlin",
            "latitude": 52.52437,
            "longitude": 13.41053,
            "elevation": 74.0,
            "timezone": "Europe/Berlin",
            "country_code": "DE",
            "country": "Deutschland",
            "population": 3426354,
            "postcodes": ["10967", "13347"]
        }
    ]
}
```

## Nominatim (OpenStreetMap)

**Open source geocoding with OpenStreetMap data**

- **Base URL**: `https://nominatim.openstreetmap.org/search`
- **Cost**: Free
- **Rate Limits**: 1 request/second for bulk usage
- **Data Source**: OpenStreetMap
- **License**: Open Database License

### Features
- Global coverage
- Forward and reverse geocoding
- Structured queries
- Multiple languages
- Address component breakdown
- Self-hostable

### Example Request
```
https://nominatim.openstreetmap.org/search?q=Berlin&format=json&limit=1
```

### Usage Policy
- Maximum 1 request per second for bulk usage
- Must provide valid User-Agent
- No heavy usage without permission
- Consider self-hosting for high volume

## Google Maps Geocoding API

**Commercial high-accuracy geocoding**

- **Base URL**: `https://maps.googleapis.com/maps/api/geocode/json`
- **Cost**: $5 per 1000 requests after 40,000 free monthly requests
- **Rate Limits**: 50 requests/second
- **Data Source**: Google Maps data

### Features
- Extremely high accuracy
- Address validation
- Component filtering
- Multiple result types
- Global coverage
- Place ID support

### Example Request
```
https://maps.googleapis.com/maps/api/geocode/json?address=Berlin&key=YOUR_API_KEY
```

### Pricing (2024)
- First 40,000 requests/month: Free
- Additional requests: $5.00 per 1,000

## OpenCage Geocoding API

**Easy open data geocoding**

- **Base URL**: `https://api.opencagedata.com/geocode/v1/json`
- **Cost**: 2,500 free requests/day, then paid plans
- **Rate Limits**: 1 request/second on free tier
- **Data Source**: Multiple open sources (OpenStreetMap, GeoNames, etc.)

### Features
- Multiple open data sources
- Address formatting
- Data annotations (timezone, currency, etc.)
- Confidence scores
- Bounds information
- 40+ language SDKs

### Example Request
```
https://api.opencagedata.com/geocode/v1/json?q=Berlin&key=YOUR_API_KEY
```

### Pricing (2024)
- **Free**: 2,500 requests/day
- **Starter**: €30/month (10K requests/day)  
- **Standard**: €80/month (100K requests/day)

## LocationIQ

**Affordable geocoding service**

- **Base URL**: `https://api.locationiq.com/v1/search.php`
- **Cost**: 5,000 free requests/day, then paid plans
- **Rate Limits**: 2 requests/second on free tier
- **Data Source**: OpenStreetMap via Nominatim

### Features
- Forward and reverse geocoding
- Autocomplete
- Multiple languages
- Nearby places
- Timezone lookup

### Example Request
```
https://api.locationiq.com/v1/search.php?key=YOUR_API_KEY&q=Berlin&format=json
```

### Pricing (2024)
- **Free**: 5,000 requests/day
- **Freelancer**: $39/month (100K requests)
- **Startup**: $99/month (500K requests)

## Comparison Matrix

| Provider | Free Tier | Cost After Free | Rate Limit | Data Source | Self-Host |
|----------|-----------|-----------------|------------|-------------|-----------|
| **Open-Meteo** | Non-commercial use | Commercial license | Reasonable | GeoNames | No |
| **Nominatim** | Unlimited | Free | 1 req/sec | OpenStreetMap | Yes |
| **Google Maps** | 40K/month | $5/1K requests | 50 req/sec | Google | No |
| **OpenCage** | 2.5K/day | €30+/month | 1 req/sec | Multiple open | No |
| **LocationIQ** | 5K/day | $39+/month | 2 req/sec | OpenStreetMap | No |

## Recommendations by Use Case

### Small Personal Projects
- **Open-Meteo Geocoding API**: Same provider as weather API, simple integration
- **Nominatim**: Completely free, good for learning

### Medium Commercial Projects  
- **OpenCage**: Good balance of features and pricing
- **LocationIQ**: Generous free tier, reasonable pricing

### High-Volume Commercial
- **Google Maps**: Most accurate, best for production apps
- **Self-hosted Nominatim**: Complete control, one-time setup cost

### Open Source Projects
- **Open-Meteo**: Consistent with weather API choice
- **Nominatim**: Fully open source stack

## Implementation Considerations

### Error Handling
All APIs can return:
- Rate limit exceeded errors
- Invalid query errors  
- No results found
- Service unavailable errors

### Caching Strategy
- Cache successful geocoding results locally
- Implement reasonable cache expiration (24-48 hours)
- Respect API terms regarding caching

### Rate Limiting
- Implement client-side rate limiting
- Add retry logic with exponential backoff
- Consider request queuing for batch operations

### Data Quality
- Always validate coordinates are reasonable
- Handle multiple results appropriately
- Consider confidence scores when available
- Implement fallback APIs for critical applications

## Integration with Weather API

### Typical Flow
1. User inputs location string (city, address, zip code)
2. Call geocoding API to get lat/lon coordinates
3. Use coordinates with Open-Meteo weather API
4. Cache geocoding results to minimize API calls

### Example Integration Pattern
```rust
// Pseudo-code example
async fn get_weather_by_location(location: &str) -> Result<WeatherData> {
    // 1. Check cache for coordinates
    if let Some(coords) = cache.get_coordinates(location) {
        return get_weather(coords.lat, coords.lon).await;
    }
    
    // 2. Geocode the location
    let coords = geocoding_api.search(location).await?;
    
    // 3. Cache the result
    cache.store_coordinates(location, coords);
    
    // 4. Get weather data
    get_weather(coords.lat, coords.lon).await
}
```

## Best Practices

1. **Use Open-Meteo's geocoding API** for consistency with weather API
2. **Implement caching** to reduce API calls and improve performance  
3. **Handle multiple results** - let users choose when ambiguous
4. **Validate inputs** before making API calls
5. **Implement fallback** APIs for critical applications
6. **Respect rate limits** and terms of service
7. **Cache negative results** briefly to avoid repeated failed queries
8. **Use appropriate timeouts** for API calls
9. **Log errors** for debugging and monitoring
10. **Consider user privacy** when caching location data