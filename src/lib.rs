pub mod cfg;
pub mod openweather;

#[derive(Debug)]
pub struct CityWeather {
    name: String,
    condition: String,
    temp: i16,
    temp_min: i16,
    temp_max: i16,
}
