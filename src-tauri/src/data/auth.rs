use crate::dir;
use crate::fs::encryption::{encrypt, decrypt};
use crate::data::json::data_bytes;
use crate::data::json::init_user_data;
// use std::fs::{create_dir, read_dir, write};


#[tauri::command]
fn authenticate(name: &str, password: &str) -> bool {
    // for now, authentication is not possible since decryption is not possible
    return true;
}

#[tauri::command]
fn save_user(name: &str, password: &str){
    let dir_name = encrypt(name, password, name.as_bytes().to_vec());
    let data = data_bytes();
    let encrypted_user_data = encrypt(name, password, data.0);
    let encrypted_system_data = encrypt(name, password, data.1);
    println!("{:?}, {:?}", name, String::from_utf8(decrypt(name, password, dir_name)));
}

#[test]
fn test_authentication(){
    init_user_data();
    // if !dir().exists(){create_dir(dir().as_path()).expect("failed to create a data directory");}
    let name = "example name";
    let password = "example password";
    save_user(name, password);
}
