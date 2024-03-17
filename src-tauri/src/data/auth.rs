use crate::dir;
use crate::fs::encryption::{aes_encrypt, aes_decrypt, aes_try_decrypt};
use crate::data::json;
use crate::fs::commands::{FS, Home};
use std::error::Error;
use std::fs::{create_dir, read_dir, read_to_string, File, OpenOptions};
use std::io::{Read, Write};
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_os = "linux")]
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use crate::init_user_data;
use base64::{decode, encode};
use serde::Serialize;
use std::ffi::OsStr;
use crate::get_user_dir;

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
        self.as_encoded_bytes().to_vec()
    }
}


#[tauri::command]
pub fn authenticate_user(name: &str, password: &str) -> bool{
    let location = encode(aes_encrypt(name, password, name.as_bytes()));
    let location = dir().join(location);
    if !location.exists(){return false;}
    for entry in read_dir(location).unwrap(){
        if entry.is_err(){continue;}
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let entry = decode(entry.file_name().to_bytes());
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
    println!("{:?}", location);
    if !location.exists() {
        return Err(String::from("user directory was not initialized"));
    }
    for entry in read_dir(location).expect("failed to read directory"){
        let entry = entry.expect("failed to read file");
        let path = entry.path();
        let entry = decode(entry.file_name().to_bytes()).unwrap();
        let entry = aes_decrypt(name, password, &entry);
        let entry = String::from_utf8(entry).unwrap();
        match entry.as_str(){
            "user data" => {
                let file_content = read_to_string(&path).unwrap();
                let content = decode(file_content).unwrap();
                let original_content = aes_decrypt(name, password, &content);
                
            },
            "system data" => {
                let file_content = read_to_string(&path).unwrap();
                let content = decode(file_content).unwrap();
                let original_content = aes_decrypt(name, password, &content);
            },
            _ => {}

        }

        let encrypted_content = read_to_string(&path).expect("failed to read file content");
        let decrypted_content = aes_decrypt(name, password, &decode(encrypted_content).expect("failed to decode encrypted file content"));
    }
    return Ok(());
}

#[tauri::command]
pub fn save_user(username: &str, password: &str) -> Result<(), String> {
    fn save_data(username: &str, password: &str, data_name: String, data: Vec<u8>) -> Result<(), String>{
        let location = encode(aes_encrypt(username, password, &data_name.into_bytes())).replace('/', "_");
        let location = get_user_dir(username, password).join(location);
        if location.exists(){
            let err = File::open(location.as_path()).unwrap().write_all(&data);
            if err.is_err(){return Err(String::from("failed to write into the file"));}
        }
        else{
            let err = File::create(location.as_path()).unwrap().write_all(&data);
            if err.is_err() {return Err(String::from("failed to write into the file"));}
        }
        return Ok(());
    }
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

#[test]
fn test_authentication(){
    init_user_data();
    init_dir().expect("failed to create the main directory");
    let name = "non oe user";
    let password = "non oe user";
    let err = save_user(name, password);
    println!("test: {}", err.unwrap_err());
    //if err.is_err() {panic!("save user failed");}
    authenticate_user(name, password);
    let err = load_user(name, password);
}
