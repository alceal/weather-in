# Weather-in CLI Tool

## Introduction

The Weather-in CLI Tool serves as a demonstration of making API requests using Rust. This project showcases my proficiency in Rust development by implementing a command-line application that fetches real-time weather data from the OpenWeatherMap API and presents it conveniently in the terminal.

## Dependencies

This project leverages several crates from crates.io to achieve its functionality:

- clap: For parsing command-line arguments in a structured manner.
- reqwest: To perform HTTP requests and interact with APIs.
- serde: For serializing and deserializing JSON data.
- chrono: Utilized for date and time operations.

## Installation

To use the Weather-in CLI Tool, follow these simple installation steps:

```bash
git clone <https://github.com/alceal/weather-in.git>
cd weather-in
cargo build --release
```

Alternatively, you can install the tool directly using Cargo:

```bash
cargo install weather-in
```

# Usage

The Weather-in CLI Tool is designed to retrieve current weather information based on user-provided locations and API keys. Here's an example of how to use it:

```bash
weather-in <LOCATION> [--isocountry <COUNTRY_CODE>] --api-key <API_KEY>
```

- `<LOCATION>`: Specifies the location for which weather information is requested.
- `--isocountry`: (Optional) Provides an ISO country code for more accurate weather results.
- `--api-key`: Requires the OpenWeatherMap API key for accessing weather data.

Example:

```bash
weather-in "New York" --isocountry US --api-key YOUR_API_KEY
```

# License

This project is licensed under the MIT License.
