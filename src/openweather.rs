extern crate reqwest;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Error as ReqwestError;
use serde::Deserialize;
use std::fmt;
use std::fmt::{Display, Formatter};

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

#[derive(Debug)]
pub struct CityWeather {
    name: String,
    condition: String,
    temp: f64,
    temp_min: f64,
    temp_max: f64,
}

impl CityWeather {
    fn from(json_resp: &CityWeatherResponse) -> CityWeather {
        let condition = if json_resp.weather.len() > 0 {
            let weather = &json_resp.weather[0];
            weather.main.clone()
        } else {
            String::from("None")
        };

        CityWeather {
            name: json_resp.name.clone(),
            condition,
            temp: kelvin_to_celcius(json_resp.main.temp),
            temp_min: kelvin_to_celcius(json_resp.main.temp_min),
            temp_max: kelvin_to_celcius(json_resp.main.temp_max),
        }
    }
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

    Ok(CityWeather::from(&data))
}

fn kelvin_to_celcius(deg: f64) -> f64 {
    deg - 273.15
}
