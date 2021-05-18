use std::path::PathBuf;
use std::error::Error;
use serde::{Serialize, Deserialize};

const FILE_SIZE_NEW:usize = 4326432;
const FILE_SIZE_GFWL:usize = 4330480;
const SLOT_SIZE:usize = 393616;
const FIRST_SLOT_OFFSET:usize = 704;
const DEATHS_LOCATION:usize = 127272;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ds1Config {
    pub override_file_location: String,
    pub save_slot: usize,
}

impl std::fmt::Display for Ds1Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ds1Config Error")
    }
}

pub fn new() -> Ds1Config {
    return Ds1Config {
        override_file_location: String::from(""),
        save_slot: 0,
    }
}

pub fn parse(slot: usize, file_buffer: &Vec<u8>) -> u32 {
    // Noticed that old save files from GWFL-era are different sized from post GWFL-era.
    // Not sure what the difference is, both read the same for retrieving deaths.
    if file_buffer.len() != FILE_SIZE_NEW && file_buffer.len() != FILE_SIZE_GFWL {
        println!("ERROR: Save file size is invalid");
        return 0;
    }
    let slot_start = FIRST_SLOT_OFFSET + (slot * SLOT_SIZE);
    let slot_end = slot_start + SLOT_SIZE;
    let slot_data = &file_buffer[slot_start..slot_end];

    let mut deaths_arr = [0; 4];
    deaths_arr.copy_from_slice(&slot_data[DEATHS_LOCATION..DEATHS_LOCATION+4]);
    return u32::from_le_bytes(deaths_arr);
}

pub fn get_save_location() -> Result<PathBuf, Box<dyn Error>> {
    let mut save_location = dirs::document_dir().unwrap();
    save_location.push(r"NBGI/darksouls");

    // The save files are within a child folder that is named with the users unique ID or username. 
    // e.g. ~/Documents/NBDI/Dark Souls/[12345678]/DRAKS0005.sl2
    // Since I haven't figured out if there is a way to get this ID, we just take the first child folder.
    // TODO: Let the user view and select a user folder if there is more than one.
    return match std::fs::read_dir(&save_location) {
        Err(err) => { 
            println!("ERROR: Save File Not Found.");
            println!("Please open character creation first before starting this program.");
            println!("If you have already created a character and see this error, something went wrong.");
            Err(Box::new(err))
        },
        Ok(paths) => { 
            for path in paths {
                // Let's hope there is only one directory in here.
                save_location.push(path.unwrap().path());
                // There have been instances where the file name is all lowercase
                // TODO: Implement better checking for save file
                save_location.push("DRAKS0005.sl2");
                break;
            }
            Ok(save_location)
        },
    };
}
