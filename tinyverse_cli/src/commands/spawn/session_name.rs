use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{bail, Result};
use tinyverse_lib::SessionStore;

pub fn resolve_session_name(user_key: Option<&str>, store: &mut SessionStore) -> Result<String> {
    match user_key.and_then(normalize) {
        Some(key) => Ok(key.to_owned()),
        None => default_session_name(store),
    }
}

fn normalize(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }

    Some(trimmed)
}

fn default_session_name(store: &mut SessionStore) -> Result<String> {
    let mut used_keys = HashSet::new();
    for session in store.list_sessions()? {
        used_keys.insert(session.session_key);
    }

    let start_index = random_start_index(CALIFORNIA_CITIES.len());
    let maybe_city = select_available_city(start_index, &used_keys);
    match maybe_city {
        Some(city) => Ok(format!("tinyverse_{city}")),
        None => bail!(
            "no available default city names remain; pass --key to set an explicit session name"
        ),
    }
}

fn random_start_index(len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    (nanos % len as u128) as usize
}

fn select_available_city(start_index: usize, used_keys: &HashSet<String>) -> Option<&'static str> {
    for offset in 0..CALIFORNIA_CITIES.len() {
        let index = (start_index + offset) % CALIFORNIA_CITIES.len();
        let candidate = format!("tinyverse_{}", CALIFORNIA_CITIES[index]);
        if !used_keys.contains(&candidate) {
            return Some(CALIFORNIA_CITIES[index]);
        }
    }

    None
}

const CALIFORNIA_CITIES: [&str; 26] = [
    "anaheim",
    "bakersfield",
    "berkeley",
    "burbank",
    "carlsbad",
    "cupertino",
    "fresno",
    "glendale",
    "irvine",
    "longbeach",
    "losangeles",
    "malibu",
    "modesto",
    "monterey",
    "oakland",
    "pasadena",
    "redding",
    "sacramento",
    "san_bernardino",
    "sandiego",
    "sanfrancisco",
    "sanjose",
    "santabarbara",
    "santacruz",
    "santamonica",
    "stockton",
];

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{normalize, select_available_city, CALIFORNIA_CITIES};

    #[test]
    fn normalize_keeps_user_provided_key_when_present() {
        let result = normalize("my-session");
        assert_eq!(result, Some("my-session"));
    }

    #[test]
    fn normalize_trims_user_provided_key() {
        let result = normalize("  my-session  ");
        assert_eq!(result, Some("my-session"));
    }

    #[test]
    fn selects_first_available_city_from_start_index() {
        let used = HashSet::from([String::from("tinyverse_anaheim")]);
        let city = select_available_city(0, &used).expect("a city should be available");
        assert_eq!(city, "bakersfield");
    }

    #[test]
    fn returns_none_when_all_city_keys_are_taken() {
        let used: HashSet<String> = CALIFORNIA_CITIES
            .iter()
            .map(|city| format!("tinyverse_{city}"))
            .collect();
        let city = select_available_city(0, &used);
        assert!(city.is_none());
    }
}
