use crate::dir;
use crate::fs::encryption::{aes_encrypt, aes_decrypt};
use crate::data::json::data_bytes;
use crate::data::json::init_user_data;
// use std::fs::{create_dir, read_dir, write};


#[tauri::command]
fn authenticate(name: &str, password: &str) -> bool {
    // for now, authentication is not possible since decryption is not ready
    return true;
}

#[tauri::command]
fn save_user(name: &str, password: &str){
    let dir_name = aes_encrypt(name, password, name.as_bytes().to_vec());
    let data = data_bytes();
    let encrypted_user_data = aes_encrypt(name, password, data.0);
    let encrypted_system_data = aes_encrypt(name, password, data.1);
    //println!("{:?}, {:?}", name, String::from_utf8(aes_decrypt(name, password, dir_name)));
}

#[test]
fn test_authentication(){
}
