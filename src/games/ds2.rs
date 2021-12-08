use std::path::PathBuf;
use std::error::Error;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::{ZeroPadding};
use serde::{Serialize, Deserialize};

// ZeroPadding (Pad with Zeros) opposed to pkcs7 found in other DS games.
type Aes128CbcZero = Cbc<Aes128, ZeroPadding>;

// DS2 Original
//const KEY:[u8; 16] = [0xB7, 0xFD, 0x46, 0x3E, 0x4A, 0x9C, 0x11, 0x02, 0xDF, 0x17, 0x39, 0xE5, 0xF3, 0xB2, 0xA5, 0x0F];
// DS2 SOTFS
const KEY:[u8; 16] = [0x59, 0x9F, 0x9B, 0x69, 0x96, 0x40, 0xA5, 0x52, 0x36, 0xEE, 0x2D, 0x70, 0x83, 0x5E, 0xC7, 0x44];

const AES_BLOCKLEN:usize = 16;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ds2Config {
    pub override_file_location: String,
    pub save_slot: usize,
}

impl std::fmt::Display for Ds2Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ds2Config Error")
    }
}

pub fn new() -> Ds2Config {
    return Ds2Config {
        override_file_location: String::from(""),
        save_slot: 0,
    }
}

pub fn parse(slot: usize, file_buffer: &Vec<u8>) -> u32 {
    // The first "file" in BND4 starts at 0x40, but in this case it seems to be a metadata file.
    // So we're skipping the first file and going to 0x60 which is the first character save slot.
    let meta_start = 0x60 + slot*0x20;

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
    let slot_start = slot_offset as usize + AES_BLOCKLEN;
    let slot_end = slot_start + data_length as usize - AES_BLOCKLEN;

    let slot_data = &file_buffer[slot_start..slot_end];
    let iv: Vec<u8> = file_buffer[slot_start-16..slot_start].to_vec();

    let cipher = Aes128CbcZero::new_from_slices(&KEY, &iv).unwrap();
    // TODO: Instead of println, return a custom error.
    let decrypted_slot_data = match cipher.decrypt_vec(&slot_data) {
        Err(e) => {
            println!("ERROR: Unable to decrypt save file {:?}", e);
            return 0;
        },
        Ok(b) => b,
    };

    //save_decrypted_file(slot, &decrypted_slot_data);

    let mut deaths_arr = [0; 4];
    let death_pointer = 0xCC;
    //println!("Offset: {}", (current_pointer - starting_pointer));
    deaths_arr.copy_from_slice(&decrypted_slot_data[death_pointer..death_pointer+4]);
    return u32::from_le_bytes(deaths_arr);
}

// TEMP
/*fn save_decrypted_file(slot: usize, decrypted_file_buffer: &Vec<u8>) {
    let file_name = format!("SAVESLOT{:03}", slot);
    let mut file = File::create(file_name).unwrap();
    file.write_all(decrypted_file_buffer).unwrap();
}*/

/*pub fn get_save_location() -> Result<PathBuf, Box<dyn Error>> {
    let save_location = PathBuf::from("DS2SOFS0000.sl2");
    return Ok(save_location);
}*/

pub fn get_save_location() -> Result<PathBuf, Box<dyn Error>> {
    let mut save_location = dirs::config_dir().unwrap();
    save_location.push(r"DarkSoulsII");

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
                save_location.push("DS2SOFS0000.sl2");
                break;
            }
            Ok(save_location)
        },
    };
}
