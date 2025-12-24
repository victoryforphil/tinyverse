use serde::{Deserialize, Serialize};
use clap::Parser;

#[derive(Debug, Parser, Clone, PartialEq, Eq, Hash)]
pub struct TinyWeatherArgs {
    // Flatten the config into the args
    #[command(flatten)]
    pub config: TinyWeatherConfig,
}

#[derive(Debug, Deserialize, Serialize, Parser, Clone, PartialEq, Eq, Hash)]
pub struct TinyWeatherConfig {
    #[arg(short, long)] 
    pub location: String,
}