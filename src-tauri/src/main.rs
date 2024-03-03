// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod fs;
mod data;
use std::path::PathBuf;
use tauri::Runtime;
use crate::data::json::{init_user_data, user_get};
use dirs::data_dir;
use crate::data::auth::{init_dir, save_user, authenticate};

pub fn dir() -> PathBuf {data_dir().expect("failed to enter data directory").join("d_vault_data")}

#[allow(unused)]
#[tauri::command]
async fn first_init<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
    window.set_fullscreen(true);
    init_user_data();
    init_dir();
    Ok(())
}

#[tauri::command]
async fn printf(value: String) {println!("{}", value)}

// VERY IMPORTANT: as of now (29/2/2024) there is no way to figure out the commands
// in the tauri handler, so this needs to be written manually
#[tauri::command]
async fn list_commands() -> String {
  return String::from(
r#"
commands:
list_commands - prints this text out
printf - print something out to the terminal
first_init - initializes the app
user_get - returns a saved object

"#)
}
fn main() {
   tauri::Builder::default().invoke_handler(tauri::generate_handler![
    first_init, list_commands, printf, user_get
]).run(tauri::generate_context!()).expect("failed to run the code");
   }
