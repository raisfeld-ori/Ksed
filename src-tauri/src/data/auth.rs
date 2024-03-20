use crate::dir;
use crate::fs::encryption::{aes_encrypt, aes_decrypt, aes_try_decrypt};
use crate::data::json;
use crate::fs::commands::{Home, FS};
use std::fs::{create_dir, read_dir, read, File};
use std::io::{Read, Write};
#[cfg(target_os = "windows")]
use std::os::windows::prelude::*;
#[cfg(target_os = "linux")]
use std::os::unix::ffi::OsStrExt;
use crate::get_user_dir;
use base64::{decode_config, encode_config, URL_SAFE};
use std::ffi::OsStr;
use serde_json::{Map, Value};
use super::json::set_data;
pub fn init_dir() -> Result<(), std::io::Error>{if dir().exists() {Ok(())}else{create_dir(dir())}}

#[tauri::command]
pub fn user_exists(name: &str, password: &str) -> bool {get_user_dir(name, password).exists()}

pub trait Encodable{fn to_bytes(&self) -> Vec<u8>;}
impl Encodable for OsStr{
    #[cfg(target_os = "windows")]
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for w_byte in self.encode_wide() {
            if let Some(c) = char::from_u32(w_byte as u32) {
                let mut buf = [0; 4]; // Buffer to hold the UTF-8 encoded character
                let encoded = c.encode_utf8(&mut buf);
                bytes.extend_from_slice(encoded.as_bytes());
            }
        }
        bytes
    }
    #[cfg(target_os = "linux")]
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}


#[tauri::command]
pub fn authenticate_user(name: &str, password: &str) -> bool{
    let location = encode_config(aes_encrypt(name, password, name.as_bytes()), URL_SAFE);
    let location = dir().join(location);
    if !location.exists(){return false;}
    for entry in read_dir(location).unwrap(){
        if entry.is_err(){continue;}
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let entry = decode_config(entry.file_name().to_bytes(), URL_SAFE);
        if entry.is_err() {continue;}
        let entry = aes_decrypt(name, password, &entry.unwrap());
        let entry = String::from_utf8(entry);
        if entry.is_err() {continue;}
        match entry.unwrap().as_str(){
            "auth" => {
                let mut auth_data = File::open(entry_path).unwrap();
                let mut buffer: Vec<u8> = Vec::new();
                let result = auth_data.read_to_end(&mut buffer);
                if result.is_err() {return false;}
                return aes_try_decrypt(name, password, &buffer);
            }
            _ => {continue;}
        }
    }
    return false;
}

#[tauri::command]
pub fn load_user(name: &str, password: &str) -> Result<(), String>{
    let location = get_user_dir(name, password);
    if !location.exists() {
        return Err(String::from("user directory was not initialized"));
    }
    for entry in read_dir(location).expect("failed to read directory"){
        let entry = entry.expect("failed to read file");
        let path = entry.path();
        let entry = decode_config(entry.file_name().to_bytes(), URL_SAFE).unwrap();
        let entry = aes_decrypt(name, password, &entry);
        let entry = String::from_utf8(entry).unwrap();
        let mut system_data = Map::new();
        let mut user_data = Map::new();
        match entry.as_str(){
            "user_json" => {
                let file_content = read(&path).unwrap();
                let original = aes_decrypt(name, password, &file_content);
                let original_user_data: Map<String, Value> = serde_json::from_slice(&original).unwrap();
                user_data = original_user_data;
            },
            "sys_json" => {
                let file_content = read(&path).unwrap();
                let original = aes_decrypt(name, password, &file_content);
                let original_system_data: Map<String, Value> = serde_json::from_slice(&original).unwrap();
                system_data = original_system_data;
            },
            "fs" => {
                let file_content = read(&path).unwrap();
                let original = aes_decrypt(name, password, &file_content);
                let original = serde_json::from_slice(&original).unwrap();
                unsafe{FS = original};
            }
            _ => {}

        }
        set_data(user_data, system_data);
    }
    return Ok(());
}

fn save_data(username: &str, password: &str, data_name: String, data: Vec<u8>) -> Result<(), String>{
    let location = encode_config(aes_encrypt(username, password, &data_name.into_bytes()), URL_SAFE);
    let location = get_user_dir(username, password).join(location);
    let file = File::create(location.as_path());
    if file.is_err() {return Err(String::from("failed to create the file"));}
    let file = file.unwrap().write_all(&data);
    if file.is_err() {return Err(String::from("failed to write into the file"))}
    return Ok(());
}

#[tauri::command]
pub fn save_user(username: &str, password: &str) -> Result<(), String> {
    let (user_json, sys_json) = json::data_bytes();
    let user_json = aes_encrypt(username, password, &user_json);
    let err = save_data(username, password, String::from("user_json"), user_json);
    if err.is_err(){return err;}
    let sys_json = aes_encrypt(username, password, &sys_json);
    let err = save_data(username, password, String::from("sys_json"), sys_json);
    if err.is_err(){return err;}
    let fs = unsafe{serde_json::to_string(&FS)};
    if fs.is_err(){return Err(String::from("failed to save the user data"));}
    let fs = aes_encrypt(username, password, fs.unwrap().as_bytes());
    let err = save_data(username, password, String::from("fs"), fs);
    if err.is_err(){return err;}
    return Ok(());
}

#[tauri::command]
pub fn create_user(name: &str, password: &str) -> Result<(), String> {
  let location = get_user_dir(name, password);
    if location.exists(){return Err(String::from("user already exists"));}
    else {
        let err = create_dir(location.as_path());
        if err.is_err() {return Err(err.unwrap_err().to_string());}
    }
    return save_data(name, password, String::from("auth"), b"arg arg mbc mbc".to_vec());
}

#[test]
fn test_authentication(){
    use crate::data::json::init_user_data;
    init_user_data();
    init_dir().expect("failed to create the main directory");
    let name = "a";
    let password = "a";
    if user_exists(name, password) {}
    else {assert!(create_user(name, password).is_ok());}
    assert!(save_user(name, password).is_ok());
    assert!(authenticate_user(name, password));
    assert!(load_user(name, password).is_ok());
}
