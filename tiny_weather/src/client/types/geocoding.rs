//! Geocoding API request and response types

use serde::{Deserialize, Serialize};

/// Request for geocoding search
#[derive(Debug, Clone)]
pub struct GeocodingRequest {
    pub name: String,
    pub count: Option<u8>,
    pub language: Option<String>,
    pub format: Option<String>,
}

impl GeocodingRequest {
    /// Create a new geocoding request for the given location name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            count: None,
            language: None,
            format: None,
        }
    }

    /// Set maximum number of results (default: 10)
    pub fn count(mut self, count: u8) -> Self {
        self.count = Some(count);
        self
    }

    /// Set language for results (e.g., "en", "de", "fr")
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Set response format (default: "json")
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
}

/// Geocoding API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeocodingResponse {
    #[serde(default)]
    pub results: Vec<Location>,
}

/// A geographic location result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: u64,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin1: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin2: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin3: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin4: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub population: Option<u64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postcodes: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geocoding_request_builder() {
        let request = GeocodingRequest::new("Berlin")
            .count(5)
            .language("en");

        assert_eq!(request.name, "Berlin");
        assert_eq!(request.count, Some(5));
        assert_eq!(request.language.as_ref().unwrap(), "en");
    }

    #[test]
    fn test_geocoding_response_deserialization() {
        let json = r#"{
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
        }"#;

        let response: GeocodingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.results.len(), 1);
        
        let location = &response.results[0];
        assert_eq!(location.name, "Berlin");
        assert_eq!(location.latitude, 52.52437);
        assert_eq!(location.country_code.as_ref().unwrap(), "DE");
    }

    #[test]
    fn test_empty_results() {
        let json = r#"{"results": []}"#;
        let response: GeocodingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.results.len(), 0);
    }
}
