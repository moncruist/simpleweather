// Simpleweather
// Copyright (C) 2020  Konstantin Zhukov
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use reqwest;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Error as ReqwestError;
use serde::Deserialize;
use std::fmt;
use std::fmt::{Display, Formatter};

use crate::CityWeather;

struct OpenWeatherRequest<'a> {
    city: String,
    api_key: &'a str,
}

impl<'a> OpenWeatherRequest<'a> {
    fn new(city: &str, api_key: &'a str) -> OpenWeatherRequest<'a> {
        OpenWeatherRequest {
            city: String::from(city),
            api_key,
        }
    }

    fn build_request(&self) -> reqwest::blocking::RequestBuilder {
        let client = reqwest::blocking::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        let request = client
            .get("https://api.openweathermap.org/data/2.5/weather")
            .query(&[("q", self.city.as_str()), ("APPID", self.api_key)])
            .headers(headers);
        request
    }
}

#[derive(Debug, Deserialize)]
struct WeatherInternal {
    id: u32,
    main: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct TemperatureInternal {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: u32,
    humidity: u16,
}

#[derive(Debug, Deserialize)]
struct WindInternal {
    speed: f64,
    deg: u16,
}

#[derive(Debug, Deserialize)]
struct RainInternal {
    #[serde(rename = "1h")]
    one_hour: i32,

    #[serde(rename = "3h")]
    three_hours: i32,
}

#[derive(Debug, Deserialize)]
struct SnowInternal {
    #[serde(rename = "1h")]
    one_hour: u32,

    #[serde(rename = "3h")]
    three_hours: u32,
}

#[derive(Debug, Deserialize)]
struct CityWeatherResponse {
    name: String,
    weather: Vec<WeatherInternal>,
    main: TemperatureInternal,
    rain: Option<RainInternal>,
    snow: Option<SnowInternal>,
}

impl CityWeatherResponse {
    fn to_city_weather(&self) -> CityWeather {
        let condition = if self.weather.len() > 0 {
            let weather = &self.weather[0];
            weather.main.clone()
        } else {
            String::from("None")
        };

        CityWeather {
            name: self.name.clone(),
            condition,
            temp: kelvin_to_celcius_i16(self.main.temp),
            temp_min: kelvin_to_celcius_i16(self.main.temp_min),
            temp_max: kelvin_to_celcius_i16(self.main.temp_max),
        }
    }
}

fn kelvin_to_celcius_i16(deg: f64) -> i16 {
    let celcius = deg - 273.15;
    celcius.round() as i16
}


#[derive(Debug)]
pub enum OpenWeatherError {
    Io(ReqwestError),
    DeserializationError,
}

impl Display for OpenWeatherError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OpenWeatherError::Io(err) => err.fmt(f),
            OpenWeatherError::DeserializationError => write!(f, "Cannot deserialize the response"),
        }
    }
}

pub fn get_city_current_weather(
    city: &str,
    api_key: &str,
) -> Result<CityWeather, OpenWeatherError> {
    let request = OpenWeatherRequest::new(city, api_key).build_request();

    let response = match request.send() {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Failed to send request: {}", e);
            return Err(OpenWeatherError::Io(e));
        }
    };

    let data: CityWeatherResponse = match response.json() {
        Ok(data) => data,
        Err(_) => return Err(OpenWeatherError::DeserializationError),
    };

    Ok(data.to_city_weather())
}
