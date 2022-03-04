use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::games::*;

pub struct Game {
    pub override_file_location: String,
    pub save_slot: usize,
}

pub trait DeathCount {
    fn get_save_location() -> PathBuf;
    fn parse(slot: usize, file_buffer: &Vec<u8>) -> u32;
    fn new_config() -> Self;
}

// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub enum Game {
//     Ds1,
//     Dsr,
//     Ds2,
//     Ds2Sotfs,
//     Ds3,
//     Sekiro,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ConfigFile {
//     pub output_deaths_location: std::path::PathBuf,
//     pub current_game: Game,
//     pub dsr_config: dsr::DsrConfig,
//     pub ds1_config: ds1::Ds1Config,
//     pub ds2_config: ds2::Ds2Config,
//     pub ds3_config: ds3::Ds3Config,
//     pub sekiro_config: sekiro::SekiroConfig,
// }
