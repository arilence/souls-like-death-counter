use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent, Result};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::PathBuf;
use crate::config;
use crate::config::{ConfigFile};

type CallbackOp = fn(config: &ConfigFile, save_location: &PathBuf);

pub fn start(config: &ConfigFile, callback_fn: CallbackOp) -> Result<()> {
    let save_location = config::get_save_location(config);

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx.clone(), Duration::from_secs(2))?;
    match watcher.watch(&save_location, RecursiveMode::NonRecursive) {
        Err(_) => {
            println!("ERROR: Save File Not Found.");
            println!("Please open character creation first before starting this program.");
            println!("If you have already created a character and see this error, something went wrong.");
            return Ok(());
        },
        Ok(_) => (),
    };

    println!("Started Successfully");
    let location = save_location.clone();
    callback_fn(config, &location);
    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    DebouncedEvent::Write(_) => {
                        callback_fn(config, &location);
                    },
                    _ => (),
                }
            },
            Err(e) => {
                println!("ERROR: Watching file failed: {:?}", e);
                break;
            },
        }
    }
    Ok(())
}
