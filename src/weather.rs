//! This module defines the data structures representing weather information.

use chrono::DateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Coor {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Weather {
    pub id: u16,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: u32,
    pub humidity: u32,
    #[serde(default)]
    pub sea_level: u32,
    #[serde(default)]
    pub grnd_level: u32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Wind {
    pub speed: f32,
    pub deg: i16,
    #[serde(default)]
    pub gust: f32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Clouds {
    pub all: u8,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Rain {
    #[serde(rename = "1h")]
    pub one_hour: f32,

    #[serde(rename = "3h")]
    pub three_hours: Option<f32>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Snow {
    #[serde(rename = "1h")]
    pub one_hour: f32,

    #[serde(rename = "3h")]
    pub three_hours: Option<f32>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Sys {
    pub r#type: u8,
    pub id: u32,
    pub country: String,
    pub sunrise: u32,
    pub sunset: u32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct WeatherInfo {
    pub coord: Coor,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: u16,
    pub wind: Wind,
    pub clouds: Clouds,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub dt: u32,
    pub sys: Sys,
    pub timezone: i16,
    pub id: u32,
    pub name: String,
    pub cod: u16,
}

impl WeatherInfo {
    pub fn print(&self) {
        let time = DateTime::from_timestamp(self.dt.into(), 0)
            .unwrap()
            .to_rfc2822();

        let sunrise = DateTime::from_timestamp(self.sys.sunrise.into(), 0)
            .unwrap()
            .to_rfc2822();

        let sunset = DateTime::from_timestamp(self.sys.sunset.into(), 0)
            .unwrap()
            .to_rfc2822();

        let message = format!(
            r###"
City: {}
Country: {}
Time: {}
Weather Conditions: {} - {}
Temperature: {} ºC
Pressure: {} hPa
Humidity: {}%
Visibility: {} m
Wind Speed: {} m/s
Wind Gust: {} m/s
Cloudiness: {}%
Sunrise: {}
Sunset: {}
"###,
            self.name,
            self.sys.country,
            time,
            self.weather[0].main,
            self.weather[0].description,
            self.main.temp,
            self.main.pressure,
            self.main.humidity,
            self.visibility,
            self.wind.speed,
            self.wind.gust,
            self.clouds.all,
            sunrise,
            sunset
        );

        let message = match &self.rain {
            Some(rain) => {
                let message = message + &format!("Rain:\n    1 hour: {}", rain.one_hour);

                let message = match rain.three_hours {
                    Some(three_hours) => {
                        let message = message + &format!("\n    3 hours: {}", three_hours);

                        message
                    }
                    None => message,
                };

                message
            }
            None => message,
        };

        let message = match &self.snow {
            Some(snow) => {
                let message = message + &format!("Snow:\n    1 hour: {}", snow.one_hour);

                let message = match snow.three_hours {
                    Some(three_hours) => {
                        let message = message + &format!("\n    3 hours: {}", three_hours);

                        message
                    }
                    None => message,
                };

                message
            }
            None => message,
        };

        println!("{message}");
    }
}
