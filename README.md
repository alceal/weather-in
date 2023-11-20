# Weather-in CLI Tool

## Overview

The Weather-in CLI Tool is a command-line application for retrieving and displaying current weather information based on user-specified locations. It utilizes the OpenWeatherMap API to fetch real-time weather data and provides a simple and convenient way to check the weather from the terminal.

## Features

- **Command-Line Interface:** Easily accessible from the command line with support for various options and arguments.
- **Location Specific:** Retrieve weather information for a specific location and, optionally, an ISO country code for more accurate results.
- **API Integration:** Utilizes the OpenWeatherMap API to fetch current weather data.

## Installation

To use the Weather CLI Tool, follow these steps:

```bash
git clone https://github.com/alceal/weather-in.git
cd weather-in
cargo build --release
```

## Usage

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

## Contributing

If you find issues or have suggestions for improvements, feel free to open an issue or create a pull request. Contributions are welcome!

## License

This project is licensed under the MIT License.
