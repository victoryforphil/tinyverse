use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Result, bail};

use crate::session_store::SessionStore;

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

    let cities = default_session_cities();
    let start_index = random_start_index(cities.len());
    let maybe_city = select_available_city(cities, start_index, &used_keys);
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

fn select_available_city<'a>(
    cities: &'a [&'a str],
    start_index: usize,
    used_keys: &HashSet<String>,
) -> Option<&'a str> {
    for offset in 0..cities.len() {
        let index = (start_index + offset) % cities.len();
        let candidate = format!("tinyverse_{}", cities[index]);
        if !used_keys.contains(&candidate) {
            return Some(cities[index]);
        }
    }

    None
}

fn default_session_cities() -> &'static [&'static str] {
    DEFAULT_SESSION_CITIES
}

include!(concat!(env!("OUT_DIR"), "/california_cities.rs"));

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{default_session_cities, normalize, random_start_index, select_available_city};

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
        let cities = ["anaheim", "bakersfield"];
        let city = select_available_city(&cities, 0, &used).expect("a city should be available");
        assert_eq!(city, "bakersfield");
    }

    #[test]
    fn returns_none_when_all_city_keys_are_taken() {
        let cities = ["anaheim", "bakersfield"];
        let used: HashSet<String> = cities
            .into_iter()
            .map(|city| format!("tinyverse_{city}"))
            .collect();
        let city = select_available_city(&cities, 0, &used);
        assert!(city.is_none());
    }

    #[test]
    fn wraps_around_to_find_available_city() {
        let cities = ["anaheim", "bakersfield", "berkeley"];
        let used = HashSet::from([
            String::from("tinyverse_berkeley"),
            String::from("tinyverse_anaheim"),
        ]);
        let city = select_available_city(&cities, 2, &used).expect("a city should be available");
        assert_eq!(city, "bakersfield");
    }

    #[test]
    fn default_city_list_is_not_empty() {
        assert!(!default_session_cities().is_empty());
    }

    #[test]
    fn random_start_index_is_zero_for_empty_lists() {
        assert_eq!(random_start_index(0), 0);
    }
}
