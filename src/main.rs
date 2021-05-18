use std::io::prelude::*;
use std::path::PathBuf;
use std::thread;

mod config;
mod games;
mod deaths;
mod watcher;

fn main() {
    println!("Souls-Like Death Counter v0.3.0");

    let config = config::load_config().unwrap();
    println!("Game Selected: {:?}", config.current_game);

    // Not using a special thread shutdown communication channel here. Just praying it gets killed when the parent process ends.
    // Gist: I don't know enough about threads. 
    thread::spawn(move || {
        watcher::start(&config, callback).unwrap();
    });

    println!("q + enter to quit");
    loop {
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        let b: u8 = buffer.trim().as_bytes()[0];
        match b as char {
            'q' => {
                break;
            },
            _ => (),
        }
    }
}

fn callback(config: &config::ConfigFile, save_file_location: &PathBuf) {
    let mut file_buffer = Vec::new();
    {
        let mut save_file = match std::fs::File::open(save_file_location) {
            Err(_) => {
                println!("ERROR: Couldn't open save file");
                return;
            },
            Ok(f) => f,
        };
        match save_file.read_to_end(&mut file_buffer) {
            Err(_) => {
                println!("ERROR: Couldn't read save file");
                return;
            },
            Ok(_) => (),
        }
    }

    let deaths = match config.current_game  {
        config::Game::DS1 => {
            let save_slot = config.ds1_config.save_slot;
            games::ds1::parse(save_slot, &file_buffer)
        },
        config::Game::DSR => {
            let save_slot = config.dsr_config.save_slot;
            games::dsr::parse(save_slot, &file_buffer)
        },
    };

    deaths::save(&config.output_deaths_location, deaths, 0);
}