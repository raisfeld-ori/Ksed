use crate::dir;
use crate::fs::encryption::{aes_encrypt, aes_decrypt};
use crate::data::json::data_bytes;
use crate::data::json::init_user_data;
use std::fs::{create_dir, read_dir, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use base64::{encode, decode};
use serde_json::Value;
use tauri::{Manager, Runtime};

pub fn init_dir() -> Result<(), std::io::Error>{if dir().exists() {Ok(())}else{create_dir(dir())}}
#[tauri::command]
pub fn update<R: Runtime>(app: tauri::AppHandle<R>) {app.trigger_global("rust_event", None)}

#[tauri::command]
pub fn authenticate_user(name: &str, password: &str) -> bool {
    // for now, authentication is not possible since decryption is not ready
    return true;
}

#[tauri::command]
async fn load_user<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
    
    Ok(())
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
    let encrypted_system_name = format!("{:?}", aes_encrypt(name, password, b"system data"));
    create_dir(&location).expect("could not create a directory");
    println!("{:?}", location);
    File::create(location.join(encrypted_user_name))
    .expect("failed to create a file")
    .write_all(&encrypted_user_data)
    .expect("failed to write data into file");
    File::create(location.join(encrypted_system_name))
    .expect("failed to create a file")
    .write_all(&encrypted_system_data)
    .expect("failed to write data into file");
}

#[test]
fn test_authentication(){
    init_user_data();
    init_dir().expect("failed to create the main directory");
    let name = "test";
    let password = "test";
    save_user(name, password);
}
