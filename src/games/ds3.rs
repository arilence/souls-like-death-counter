use std::path::PathBuf;
use std::error::Error;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use serde::{Serialize, Deserialize};

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

const KEY:[u8; 16] = [0xFD, 0x46, 0x4D, 0x69, 0x5E, 0x69, 0xA3, 0x9A, 0x10, 0xE3, 0x19, 0xA7, 0xAC, 0xE8, 0xB7, 0xFA];
const AES_BLOCKLEN:usize = 16;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ds3Config {
    pub override_file_location: String,
    pub save_slot: usize,
}

impl std::fmt::Display for Ds3Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ds3Config Error")
    }
}

pub fn new() -> Ds3Config {
    return Ds3Config {
        override_file_location: String::from(""),
        save_slot: 0,
    }
}

pub fn parse(slot: usize, file_buffer: &Vec<u8>) -> u32 {
    let meta_start = 0x40 + slot*0x20;
    let data_length_vec = &file_buffer[meta_start+8..meta_start+12];
    let mut data_length = [0; 4];
    data_length.copy_from_slice(&data_length_vec[0..4]);
    let data_length = u32::from_le_bytes(data_length);
    let slot_offset_vec = &file_buffer[meta_start+16..meta_start+20];
    let mut slot_offset = [0; 4];
    slot_offset.copy_from_slice(&slot_offset_vec[0..4]);
    let slot_offset = u32::from_le_bytes(slot_offset);
    let slot_start = slot_offset as usize + AES_BLOCKLEN;
    let slot_end = slot_start + data_length as usize - AES_BLOCKLEN;
    let mut slot_data = &file_buffer[slot_start..slot_end];
    let iv: Vec<u8> = file_buffer[slot_start-16..slot_start].to_vec();

    //println!("Start: {}, Size: {}", slot_start, data_length);

    let cipher = Aes128Cbc::new_from_slices(&KEY, &iv).unwrap();
    // TODO: Instead of println, return a custom error.
    let decrypted_slot_data = match cipher.decrypt_vec(&mut slot_data) {
        Err(e) => {
            println!("ERROR: Unable to decrypt save file {}", e);
            return 0;
        },
        Ok(b) => b,
    };

    //save_decrypted_file(slot, &decrypted_slot_data);

    let mut data_pointer = [0; 4];
    data_pointer.copy_from_slice(&decrypted_slot_data[0x24..0x28]);
    let data_offset = u32::from_le_bytes(data_pointer);

    let mut deaths_arr = [0;4];
    let deaths_offset:usize = 0x6B;
    deaths_arr.copy_from_slice(&decrypted_slot_data[data_offset as usize+deaths_offset..data_offset as usize+deaths_offset + 4]);
    return u32::from_le_bytes(deaths_arr);
}

pub fn get_save_location() -> Result<PathBuf, Box<dyn Error>> {
    let mut save_location = dirs::config_dir().unwrap();
    save_location.push(r"DarkSoulsIII");

    // The save files are within a child folder that is named with the users unique ID or username. 
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
                save_location.push("DS30000.sl2");
                break;
            }
            Ok(save_location)
        },
    };
}
