use crate::dir;
use crate::fs::encryption::{aes_encrypt, aes_decrypt, aes_try_decrypt};
use crate::data::json::data_bytes;
use std::fs::{create_dir, read_dir, read_to_string, File, OpenOptions};
use std::io::{Read, Write};
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use crate::init_user_data;
use base64::{decode, encode};
use serde_json::Value;
use tauri::{Manager, Pixel, Runtime};
use std::ffi::OsStr;

pub fn init_dir() -> Result<(), std::io::Error>{if dir().exists() {Ok(())}else{create_dir(dir())}}
#[tauri::command]
pub fn update<R: Runtime>(app: tauri::AppHandle<R>) {app.trigger_global("rust_event", None)}
trait Encodable{fn to_bytes(&self) -> Vec<u8>;}
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
pub fn load_user(name: &str, password: &str){
    println!("{}, {}", name, password);
    let location: &[u8] = &aes_encrypt(name, password, name.as_bytes());
    let location = dir().join(format!("{:?}", location));

    if !location.exists() {
        println!("User directory does not exits");
        return;
    }
    for entry in read_dir(location).expect("failed to read directory"){
        let entry = entry.expect("failed to read file");
        let path = entry.path();
        let file_name = path.file_name().expect("failed to get file name").to_str().expect("failed to convert file name into string");
        let decoded_file_name = decode(file_name).unwrap();
        let decrypted_file_name = aes_decrypt(name, password, &decoded_file_name);
            
        let encrypted_content = read_to_string(&path).expect("failed to read file content");
        let decrypted_content = aes_decrypt(name, password, &decode(encrypted_content).expect("failed to decode encrypted file content"));

    }
    
}

#[tauri::command]
pub fn save_user(name: &str, password: &str){
    let location: &[u8] = &aes_encrypt(name, password, name.as_bytes());
    let location = dir().join(format!("{:?}", location));
    let data = data_bytes();
    let data_0: &[u8] = &data.0;
    let data_1: &[u8] = &data.1;
    let encrypted_user_data = aes_encrypt(name, password, data_0);
    let encrypted_user_name = format!("{:?}", aes_encrypt(name, password, b"user data"));
    let encrypted_system_data = aes_encrypt(name, password, data_1);
    let encrypted_user_data_base64 = base64::encode(&encrypted_user_data);
    let encrypted_system_data_base64 = base64::encode(&encrypted_system_data);
    let encrypted_system_name = format!("{:?}", aes_encrypt(name, password, b"system data"));
    if !location.exists(){create_dir(&location).expect("could not create a directory");}
    if location.join(&encrypted_user_name).exists(){
        OpenOptions::new() 
        .write(true)
        .append(true)
        .open(location.join(encrypted_user_name))
        .expect("failed to open an existing file")
        .write_all(&encrypted_user_data_base64.as_bytes())
        .expect("failed to write data");
    }
    else{ 
        File::create(location.join(encrypted_user_name))
        .expect("failed to create a file")
        .write_all(&encrypted_user_data_base64.as_bytes())
        .expect("failed to write data into file");
        
    }
    if location.join(&encrypted_system_name).exists(){
        OpenOptions::new()
        .write(true)
        .append(true)
        .open(location.join(encrypted_system_name))
        .expect("failed to open an existing file")
        .write_all(&encrypted_system_data_base64.as_bytes())
        .expect("failed to write data");
    }
    else{ 
        File::create(location.join(encrypted_system_name))
        .expect("failed to create a file")
        .write_all(&encrypted_system_data_base64.as_bytes())
        .expect("failed to write data into file");

    }

}

#[test]
fn test_authentication(){
    init_user_data();
    init_dir().expect("failed to create the main directory");
    let name = "non existed user";
    let password = "non existed user";
    save_user(name, password);
    authenticate_user(name, password);
}
