// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod fs;
mod data;
use std::fs::read_dir;
use std::path::PathBuf;
use fs::encryption::aes_decrypt;
use tauri::{Manager, Runtime};
use crate::data::json::{init_user_data, user_get, create_value, user_make, system_get, system_make};
use dirs::data_dir;
use base64::{decode_config, encode_config, URL_SAFE};
use crate::fs::encryption::aes_encrypt;
use crate::data::auth::{init_dir, save_user, authenticate_user, load_user, user_exists, create_user};
use crate::data::auth::Encodable;
use crate::fs::commands::{pwd, ls, FS, cd, mkdir, mk};

pub fn dir() -> PathBuf {data_dir().expect("failed to enter data directory").join("d_vault_data")}
pub fn get_user_dir(name: &str, password: &str) -> PathBuf{
  dir().join(encode_config(&aes_encrypt(name, password, name.as_bytes()), URL_SAFE))
}
pub fn open_file(name: &str, password: &str, target: String) -> Option<PathBuf>{
  let location = get_user_dir(name, password);
  if location.exists(){
    for file in read_dir(location).unwrap(){
      if file.is_err(){continue;}
      let file = file.unwrap();
      let file_name = decode_config(file.file_name().to_bytes(), URL_SAFE);
      if file_name.is_err() {continue;}
      if String::from_utf8(aes_decrypt(name, password, &file_name.unwrap())).unwrap() == target{
        return Some(file.path());
      }
    }
  }
  return None;
}

#[test]
fn test_general(){
  let name = "test";
  let password = "test";
  get_user_dir(name, password);
  open_file(name, password, String::from("something"));
}
#[allow(unused)]
#[tauri::command]
async fn first_init<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
    window.set_fullscreen(true);
    init_user_data();
    init_dir();
    unsafe{FS.init_fs()};
    Ok(())
}

#[tauri::command]
async fn console(value: String) {println!("{}", value)}
#[tauri::command]
async fn close_app<R: Runtime>(window: tauri::Window<R>) -> tauri::Result<()> {window.close()}

fn main() {
   tauri::Builder::default().invoke_handler(tauri::generate_handler![
    first_init, console, user_get, authenticate_user, save_user, user_exists, load_user, ls, pwd, cd, create_user,
    create_value, mkdir, system_get, system_make, user_make, close_app, mk
]).run(tauri::generate_context!()).expect("failed to run the code");
   }
