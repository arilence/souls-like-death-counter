use std::path::PathBuf;
use async_std::task;

mod config;
mod games;
mod deaths;
mod watcher;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    println!("Souls-Like Death Counter v{}", VERSION.unwrap_or("-unknown"));

    let value = false;
    match value {
        true => {
            println!("Failed to find a configuration file. Generating one for you.");
            println!("Please edit config.toml and restart this application.");
        },
        false => {
            println!("Game Selected: Sekiro");
            println!("Save Slot: 0 (Smiddy)");
            println!("Outputting to: deaths.txt");
            println!("Successfully watching your death count :)");
        }
    }

    // Try to load configuration file
    // If file cannot be found, create one and walk user through first-time config wizard
    // If file is found, start normally with config values

    let paths = vec![
        PathBuf::from("C:/Users/Anthony/AppData/Roaming/Sekiro/76561198009628201/S0000.sl2"),
        PathBuf::from("./config.toml"),
    ];
    task::spawn(watcher::watch(paths));

    loop {
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        println!("{}", buffer.trim());
    }
}
