use std::io::prelude::*;
use std::path::PathBuf;
use std::thread;

mod config;
mod games;
mod deaths;
mod watcher;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    println!("Souls-Like Death Counter v{}", VERSION.unwrap_or("-unknown"));

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
        config::Game::Ds1 => {
            let save_slot = config.ds1_config.save_slot;
            games::ds1::parse(save_slot, &file_buffer)
        },
        config::Game::Dsr => {
            let save_slot = config.dsr_config.save_slot;
            games::dsr::parse(save_slot, &file_buffer)
        },
        config::Game::Ds2 => {
            let save_slot = config.ds2_config.save_slot;
            games::ds2::parse(save_slot, &file_buffer)
        },
        config::Game::Ds2Sotfs => {
            println!("Game not supported yet.");
            return;
        },
        config::Game::Ds3 => {
            let save_slot = config.ds3_config.save_slot;
            games::ds3::parse(save_slot, &file_buffer)
        },
        config::Game::Sekiro => {
            let save_slot = config.sekiro_config.save_slot;
            games::sekiro::parse(save_slot, &file_buffer)
        }
    };

    deaths::save(&config.output_deaths_location, deaths, 0);
}
