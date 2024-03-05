use crate::dir;
use crate::fs::encryption::{aes_encrypt, aes_decrypt, aes_try_decrypt};
use crate::data::json::data_bytes;
use crate::data::json::init_user_data;
use std::fs::{create_dir, read_dir, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use serde_json::Value;
use tauri::{App, AppHandle, Manager, Runtime};

pub fn init_dir() -> Result<(), std::io::Error>{if dir().exists() {Ok(())}else{create_dir(dir())}}
#[tauri::command]
pub fn update<R: Runtime>(app: tauri::AppHandle<R>) {app.trigger_global("rust_event", None)}

#[tauri::command]
pub fn authenticate_user(name: &str, password: &str) -> bool {
    let location: &[u8] = &aes_encrypt(name, password, name.as_bytes());
    let location = dir().join(format!("{:?}", location));
    println!("{:?}", location.read_dir().iter().nth(1));

    return true;
}

#[tauri::command]
pub fn load_user<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>, name: &str, password: &str) -> bool {
    println!("{}, {}", name, password);
    let location: &[u8] = &aes_encrypt(name, password, name.as_bytes());
    let location = dir().join(format!("{:?}", location));
    window.emit("rust_event", 0);

    return true;
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
    if !location.exists(){create_dir(&location).expect("could not create a directory");}
    if location.join(&encrypted_user_name).exists(){
        File::open(location.join(encrypted_user_name))
        .expect("failed to open an existing file")
        .write_all(&encrypted_user_data)
        .expect("failed to write data");
    }
    else{ 
        File::create(location.join(encrypted_user_name))
        .expect("failed to create a file")
        .write_all(&encrypted_user_data)
        .expect("failed to write data into file");
    }
    if location.join(&encrypted_system_name).exists(){
        File::open(location.join(encrypted_system_name))
        .expect("failed to open an existing file")
        .write_all(&encrypted_system_data)
        .expect("failed to write data");
    }
    else{ 
        File::create(location.join(encrypted_system_name))
        .expect("failed to create a file")
        .write_all(&encrypted_system_data)
        .expect("failed to write data into file");
    }
}

#[test]
fn test_authentication(){
    init_user_data();
    init_dir().expect("failed to create the main directory");
    let name = "test";
    let password = "test";
    // save_user(name, password);
    // authenticate_user(name, password);
}
