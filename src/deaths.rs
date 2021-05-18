use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn save(output_location: &PathBuf, new_death_count: u32, current_death_count: u32) {
    if new_death_count == current_death_count {
        return;
    }

    let deaths_str = new_death_count.to_string();
    let mut deaths_file = match File::create(output_location) {
        Err(_) => {
            println!("Unable to open deaths.txt");
            return;
        },
        Ok(f) => f,
    };
    match deaths_file.write_all(&deaths_str.as_bytes()) {
        Err(_) => {
            println!("Couldn't write to deaths.txt");
            return;
        },
        Ok(_) => (),
    };
}
