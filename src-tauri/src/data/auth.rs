use crate::dir;
use crate::fs::encryption::{aes_encrypt, aes_decrypt};
use crate::data::json::data_bytes;
use crate::data::json::init_user_data;
use std::fs::{create_dir, read_dir, write};
use std::path::Path;
use base64::{encode, decode};

pub fn init_dir() -> Result<(), std::io::Error>{if dir().exists() {Ok(())}else{create_dir(dir())}}

#[tauri::command]
pub fn authenticate(name: &str, password: &str) -> bool {
    // for now, authentication is not possible since decryption is not ready
    return true;
}

#[tauri::command]
pub fn save_user(name: &str, password: &str){
    let location: &[u8] = &aes_encrypt(name, password, name.as_bytes());
    println!("{:?}", location);
    let location = aes_decrypt(name, password, location);
    //let location = dir().join(location);
    let data = data_bytes();
    let data_0: &[u8] = &data.0;
    let data_1: &[u8] = &data.1;
    let encrypted_user_data = aes_encrypt(name, password, data_0);
    let encrypted_system_data = aes_encrypt(name, password, data_1);
    //let d = aes_decrypt(name, password, decode(location.file_stem().unwrap().as_encoded_bytes()).unwrap());
    println!("{:?}", location);
    //create_dir(location).expect("could not create a directory");
}

#[test]
fn test_authentication(){
    init_user_data();
    init_dir().expect("failed to create the main directory");
    let name = "test";
    let password = "test";
    save_user(name, password);
}
