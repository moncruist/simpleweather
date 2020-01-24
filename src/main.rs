use std::env;

use simpleweather::cfg::{AppConfig, ConfigError};

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

}

fn login(api_key: &str) {
    let config = AppConfig::new(api_key);

    match config.save() {
        Ok(_) => println!("Successfully saved login information"),
        Err(err) => eprintln!("Cannot save configuration: {}", err)
    }
}