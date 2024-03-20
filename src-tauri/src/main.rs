// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod fs;
mod data;
use std::fs::read_dir;
use std::path::PathBuf;
use fs::encryption::aes_decrypt;
use tauri::Runtime;
use crate::data::json::{init_user_data, user_get, create_value};
use dirs::data_dir;
use base64::{decode_config, encode_config, URL_SAFE};
use crate::fs::encryption::aes_encrypt;
use crate::data::auth::{init_dir, save_user, authenticate_user, load_user, user_exists, create_user};
use crate::data::auth::Encodable;
use crate::fs::commands::{pwd, ls, FS, cd};

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

// VERY IMPORTANT: as of now (29/2/2024) there is no way to figure out the commands
// in the tauri handler, so this needs to be written manually
#[tauri::command]
async fn list_commands() -> String {
  return String::from(
r#"
commands:
list_commands - prints this text out
console - print something out to the terminal
first_init - initializes the app
user_get - returns a saved object
authenticate_user - makes sure the user's password and name are right
user_exists - returns 'true' if the user exists and 'false' if he doesn't
save_user - saves the current existing user
load_user - load an existing user
create_user - creates a new user using a name and a password
cd - enters a directory
cat - look what's inside a file if the file does not exists it creats the file
mkdir - creats a new directory
rmdir - removes a directory
pwd - shows your current path
"#)
}
fn main() {
   tauri::Builder::default().invoke_handler(tauri::generate_handler![
    first_init, list_commands, console, user_get, authenticate_user, save_user, user_exists, load_user, ls, pwd, cd, create_user,
    create_value,
]).run(tauri::generate_context!()).expect("failed to run the code");
   }
