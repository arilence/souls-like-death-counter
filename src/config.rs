use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::games::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Game {
    DS1,
    DSR,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub output_deaths_location: std::path::PathBuf,
    pub current_game: Game,
    pub dsr_config: dsr::DsrConfig,
    pub ds1_config: ds1::Ds1Config,
}

impl std::fmt::Display for ConfigFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigFile Error")
    }
}

pub fn new() -> ConfigFile {
    return ConfigFile {
        output_deaths_location: PathBuf::from("deaths.txt"),
        current_game: Game::DSR,
        dsr_config: dsr::new(),
        ds1_config: ds1::new(),
    }
}

pub fn load_config() -> Result<ConfigFile, Box<dyn Error>> {
    if !PathBuf::from("config.toml").exists() {
        let new_config = new();
        save_config(&new_config);
        return Ok(new_config);
    }

    let mut file_buffer = String::new();
    let mut file = match File::open("config.toml") {
        Err(err) => {
            return Err(Box::new(err));
        },
        Ok(f) => f,
    };
    match file.read_to_string(&mut file_buffer) {
        Err(err) => {
            return Err(Box::new(err));
        },
        Ok(_n) => (),
    }
    let config: ConfigFile = match toml::from_str(file_buffer.as_str()) {
        Err(err) => {
            return Err(Box::new(err));
        },
        Ok(f) => f,
    };
    return Ok(config);
}

pub fn save_config(config: &ConfigFile) {
    match toml::to_string(config) {
        Err(err) => println!("{}", err),
        Ok(file_data) => {
            let mut file = File::create("config.toml").unwrap();
            file.write_all(file_data.as_bytes()).unwrap();
        },
    };
}

pub fn get_save_location(config: &ConfigFile) -> PathBuf {
    return match config.current_game {
        Game::DS1 => ds1::get_save_location().unwrap(),
        Game::DSR => dsr::get_save_location().unwrap(),
    };
}