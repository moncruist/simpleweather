pub mod cfg;
pub mod openweather;

#[derive(Debug)]
pub struct CityWeather {
    name: String,
    condition: String,
    temp: f64,
    temp_min: f64,
    temp_max: f64,
}
