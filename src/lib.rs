//! This module defines the CLI struct, command-line argument parsing,
//! and functions for requesting current weather information.

use clap::Parser;
use std::error::Error;
use weather::WeatherInfo;

pub mod weather;

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(override_usage = "weather-in <LOCATION> [OPTIONS] --api-key <API_KEY>")]
pub struct Cli {
    /// The location for which weather information is requested
    pub location: String,

    /// An optional ISO country code for more accurate weather results
    #[arg(short, long)]
    pub isocountry: Option<String>,

    /// The OpenWeather API key for accessing weather data
    #[arg(short, long)]
    pub api_key: String,
}

/// Function to request current weather information.
///
/// # Arguments
///
/// * `location` - The location for which weather information is requested.
/// * `country` - An optional country code for more accurate results.
/// * `api_key` - The API key for accessing weather data.
///
/// # Returns
///
/// Returns a Result containing WeatherInfo on success, or a Box<dyn Error> on failure.
pub fn request_current_weather(
    location: &String,
    country: &Option<String>,
    api_key: &String,
) -> Result<WeatherInfo, Box<dyn Error>> {
    let country = match country {
        Some(country) => country,
        None => "",
    };

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={location},{country}&appid={api_key}&units=metric"
    );

    let res = reqwest::blocking::get(url)?.json::<WeatherInfo>()?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_request_current_weather() {
        let _ = dotenvy::dotenv();

        let location = String::from("London");
        let country = Some(String::from("UK"));
        let api_key = env::var("API_KEY").unwrap();

        let res =
            request_current_weather(&location, &country, &api_key).expect("Error: No response");

        assert_eq!(res.cod, 200);
    }
}
