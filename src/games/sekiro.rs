use std::path::PathBuf;
use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SekiroConfig {
    pub override_file_location: String,
    pub save_slot: usize,
}

pub fn new() -> SekiroConfig {
    return SekiroConfig {
        override_file_location: String::from(""),
        save_slot: 0,
    }
}

pub fn parse(slot: usize, file_buffer: &Vec<u8>) -> u32 {
    // The first "file" in BND4 starts at 0x40, but in this case it seems to be a metadata file.
    // So we're skipping the first file and going to 0x60 which is the first character save slot.
    let meta_start = 0x40 + slot*0x20;

    // When reading the 32 bytes (0x20), the data layout is as follows
    // ID (4 bytes)
    // Flag (4 bytes)
    // Data Length (4 Bytes)
    // --unused-- (4 Bytes)
    // Data Offset (4 Bytes)
    // Name Offset (4 Bytes)
    // --unused-- (4 Bytes)
    // --unused-- (4 Bytes)
    let data_length_vec = &file_buffer[meta_start+8..meta_start+12];
    let mut data_length = [0; 4];
    data_length.copy_from_slice(&data_length_vec[0..4]);
    let data_length = u32::from_le_bytes(data_length);
    let slot_offset_vec = &file_buffer[meta_start+16..meta_start+20];
    let mut slot_offset = [0; 4];
    slot_offset.copy_from_slice(&slot_offset_vec[0..4]);
    let slot_offset = u32::from_le_bytes(slot_offset);
    let slot_start = slot_offset as usize;
    let slot_end = slot_start + data_length as usize;

    let slot_data = &file_buffer[slot_start..slot_end];

    let mut deaths_arr = [0; 4];
    let death_pointer = 0x33F60;
    //println!("Offset: {}", (current_pointer - starting_pointer));
    deaths_arr.copy_from_slice(&slot_data[death_pointer..death_pointer+4]);
    return u32::from_le_bytes(deaths_arr);
}

pub fn get_save_location() -> Result<PathBuf, Box<dyn Error>> {
    let mut save_location = dirs::data_dir().unwrap();
    save_location.push(r"Sekiro");

    // The save files are within a child folder that is named with the users unique ID or username.
    // e.g. %APPDATA%/Sekiro/[12345678]/S0000.sl2
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
                // TODO: Implement better checking for save file
                save_location.push("S0000.sl2");
                break;
            }
            Ok(save_location)
        },
    };
}
