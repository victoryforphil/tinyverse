use std::env;
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
struct CityConfig {
    california_cities: Vec<String>,
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let config_path = manifest_dir.join("data").join("california_cities.toml");
    let config_raw = fs::read_to_string(&config_path).expect("read california city config");
    let config: CityConfig = toml::from_str(&config_raw).expect("parse california city config");

    let cities: Vec<String> = config
        .california_cities
        .into_iter()
        .map(|city| city.trim().to_owned())
        .filter(|city| !city.is_empty())
        .collect();

    assert!(
        !cities.is_empty(),
        "california city config must define at least one city"
    );

    let mut generated = String::from("const DEFAULT_SESSION_CITIES: &[&str] = &[\n");
    for city in &cities {
        generated.push_str("    ");
        generated.push_str(&format!("{city:?},"));
        generated.push('\n');
    }
    generated.push_str("];\n");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    fs::write(out_dir.join("california_cities.rs"), generated).expect("write generated cities");

    println!("cargo:rerun-if-changed={}", config_path.display());
    println!("cargo:rerun-if-changed=build.rs");
}
