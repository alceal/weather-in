//! The main entry point for the Weather CLI tool.
//!
//! This module parses command-line arguments using the `clap` crate,
//! retrieves current weather information using the `weather_in` module,
//! and prints the weather details.

use clap::Parser;
use weather_in::{request_current_weather, Cli};

fn main() {
    let cli = Cli::parse();
    let current_weather =
        request_current_weather(&cli.location, &cli.isocountry, &cli.api_key).unwrap();

    current_weather.print();
}
