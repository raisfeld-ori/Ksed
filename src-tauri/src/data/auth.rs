use crate::dir;
use crate::fs::encryption::{aes_encrypt, aes_decrypt, aes_try_decrypt};
use crate::data::json::data_bytes;
use std::fs::{create_dir, read_dir, read_to_string, File, OpenOptions};
use std::io::{Read, Write};
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_os = "linux")]
use std::os::unix::ffi::OsStrExt;
use crate::user_dir;
use base64::{decode, encode};
use std::ffi::OsStr;

pub fn init_dir() -> Result<(), std::io::Error>{if dir().exists() {Ok(())}else{create_dir(dir())}}

#[cfg(target_os = "windows")]
pub trait Encodable{fn to_bytes(&self) -> Vec<u8>;}
impl Encodable for OsStr{
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
}
#[cfg(target_os = "linux")]
impl Encodable for OsStr {
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
        println!("test here 2");
        let entry = decode(entry.file_name().to_bytes());
        if entry.is_err() {continue;}
        let entry = aes_decrypt(name, password, &entry.unwrap());
        let entry = String::from_utf8(entry);
        if entry.is_err() {continue;}
        println!("test here 3");
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
pub fn user_exists(name: &str, password: &str) -> bool {
    let location = user_dir(name, password);
    if location.exists(){true}
    else{false}
}

#[tauri::command]
pub fn load_user(name: &str, password: &str){
    let location = encode(aes_encrypt(name, password, name.as_bytes()));
    let location: &[u8] = &aes_encrypt(name, password, name.as_bytes());
    let location = dir().join(format!("{:?}", location));

    if !location.exists() {
        println!("User directory does not exits");
        return;
    }
    for entry in read_dir(location).expect("failed to read directory"){
        let entry = entry.expect("failed to read file");
        let path = entry.path();
        let entry = decode(entry.file_name().to_bytes()).unwrap();
        println!("ni: {:?}", path);
        let entry = aes_decrypt(name, password, &entry);
        let entry = String::from_utf8(entry).unwrap();
        println!("file name: {}", entry);
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
    
}

#[tauri::command]
pub fn save_user(name: &str, password: &str) {
    

}

#[test]
fn test_authentication(){
    init_user_data();
    init_dir().expect("failed to create the main directory");
    let name = "non oe user";
    let password = "non oe user";
    save_user(name, password);
    authenticate_user(name, password);
    load_user(name, password);
}
