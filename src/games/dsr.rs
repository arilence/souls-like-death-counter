use std::path::PathBuf;
use std::error::Error;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use serde::{Serialize, Deserialize};

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

const KEY:[u8; 16] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10];
const FILE_SIZE:usize = 4326608;
const SLOT_SIZE:usize = 393264;
const FIRST_SLOT_OFFSET:usize = 704;
const AES_BLOCKLEN:usize = 16;

#[derive(Serialize, Deserialize, Debug)]
pub struct DsrConfig {
    pub override_file_location: String,
    pub save_slot: usize,
}

impl std::fmt::Display for DsrConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DsrConfig Error")
    }
}

pub fn new() -> DsrConfig {
    return DsrConfig {
        override_file_location: String::from(""),
        save_slot: 0,
    }
}

pub fn parse(slot: usize, file_buffer: &Vec<u8>) -> u32 {
    if file_buffer.len() != FILE_SIZE {
        println!("ERROR: Save file size is invalid");
        return 0;
    }
    let slot_start = FIRST_SLOT_OFFSET + AES_BLOCKLEN + (slot * SLOT_SIZE);
    let slot_end = slot_start + SLOT_SIZE - AES_BLOCKLEN;
    let mut slot_data = &file_buffer[slot_start..slot_end];
    let iv: Vec<u8> = slot_data[0..16].to_vec();

    let cipher = Aes128Cbc::new_from_slices(&KEY, &iv).unwrap();
    // TODO: Instead of println, return a custom error.
    let decrypted_slot_data = match cipher.decrypt_vec(&mut slot_data) {
        Err(_) => {
            println!("ERROR: Unable to decrypt save file");
            return 0;
        },
        Ok(b) => b,
    };

    // Figured out that the offset from 0x1E4F0 to 4 0x00's in a row,
    // is the same offset from 0x1F1C0.
    let mut running: bool = true;
    let starting_pointer: usize = 124144;
    let mut current_pointer: usize = 124144;
    while running {
        if decrypted_slot_data[current_pointer] == 0u8
        && decrypted_slot_data[current_pointer+1] == 0u8
        && decrypted_slot_data[current_pointer+2] == 0u8
        && decrypted_slot_data[current_pointer+3] == 0u8 {
            if current_pointer - starting_pointer != 0 {
                current_pointer += 1;
            }
            running = false;
        } else {
            current_pointer += 1;
        }
    }

    let mut deaths_arr = [0; 4];
    let death_pointer = 127424 + (current_pointer - starting_pointer);
    //println!("Offset: {}", (current_pointer - starting_pointer));
    deaths_arr.copy_from_slice(&decrypted_slot_data[death_pointer..death_pointer+4]);
    return u32::from_le_bytes(deaths_arr);
}

pub fn get_save_location() -> Result<PathBuf, Box<dyn Error>> {
    let mut save_location = dirs::document_dir().unwrap();
    save_location.push(r"NBGI/DARK SOULS REMASTERED");

    // The save files are within a child folder that is named with the users unique ID or username. 
    // e.g. ~/Documents/NBDI/DARK SOULS REMASTERED/[12345678]/DRAKS0005.sl2
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
                save_location.push("DRAKS0005.sl2");
                break;
            }
            Ok(save_location)
        },
    };
}
