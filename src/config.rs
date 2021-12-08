use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::games::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Game {
    Ds1,
    Dsr,
    Ds2,
    Ds2Sotfs,
    Ds3,
    Sekiro,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub output_deaths_location: std::path::PathBuf,
    pub current_game: Game,
    pub dsr_config: dsr::DsrConfig,
    pub ds1_config: ds1::Ds1Config,
    pub ds2_config: ds2::Ds2Config,
    pub ds3_config: ds3::Ds3Config,
    pub sekiro_config: sekiro::SekiroConfig,
}

impl std::fmt::Display for ConfigFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigFile Error")
    }
}

pub fn new() -> ConfigFile {
    return ConfigFile {
        output_deaths_location: PathBuf::from("deaths.txt"),
        current_game: Game::Dsr,
        dsr_config: dsr::new(),
        ds1_config: ds1::new(),
        ds2_config: ds2::new(),
        ds3_config: ds3::new(),
        sekiro_config: sekiro::new(),
    }
}

pub fn load_config() -> Result<ConfigFile, Box<dyn Error>> {
    if !PathBuf::from("config.toml").exists() {
        let new_config = new();
        save_config(&new_config);
        // TODO return a message and close program so user can edit the file first.
        // Custom error type will need to be made for here
        panic!("Config file not found. GENERATING ONE FOR YOU!");
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
        Game::Ds1 => ds1::get_save_location().unwrap(),
        Game::Dsr => dsr::get_save_location().unwrap(),
        Game::Ds2 => ds2::get_save_location().unwrap(),
        Game::Ds2Sotfs => PathBuf::from(""),
        Game::Ds3 => ds3::get_save_location().unwrap(),
        Game::Sekiro => sekiro::get_save_location().unwrap(),
    };
}