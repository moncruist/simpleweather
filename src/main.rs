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
use std::env;

use simpleweather::cfg::{AppConfig, ConfigError};
use simpleweather::openweather;
use simpleweather::pretty_printer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Too few args");
        return;
    }

    let command = &args[1];
    if command == "get" {
        if args.len() != 3 {
            eprintln!("Command 'get' requires exact 1 parameter");
            return;
        }
        get_weather(&args[2]);
    } else if command == "login" {
        if args.len() != 3 {
            eprintln!("Command 'login' requires exact 1 parameter");
            return;
        }
        login(&args[2]);
    } else {
        eprintln!("Unknown command '{}'", command);
        return;
    }
}

fn get_weather(location: &str) {
    let config = match AppConfig::load() {
        Ok(config) => config,
        Err(err) => match err {
            ConfigError::FileError(io_err) => {
                eprintln!("Cannot load configuration: {}", io_err);
                return;
            },
            ConfigError::Missing => {
                eprintln!("Missing API key. Use 'simpleweather login <api key>'");
                return;
            }
        }
    };

    let weather = openweather::get_city_current_weather(location, config.api_key()).unwrap();

    pretty_printer::print_current_weather(&weather);
}

fn login(api_key: &str) {
    let config = AppConfig::new(api_key);

    match config.save() {
        Ok(_) => println!("Successfully saved login information"),
        Err(err) => eprintln!("Cannot save configuration: {}", err)
    }
}