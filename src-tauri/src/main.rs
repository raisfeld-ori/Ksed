// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod fs;
mod data;
use std::path::PathBuf;
use tauri::Runtime;
use crate::data::json::{init_user_data, user_get};
use dirs::data_dir;
use crate::data::auth::{init_dir, save_user, authenticate_user, update, load_user};
use crate::fs::fake::{pwd, ls, FS};

pub fn dir() -> PathBuf {data_dir().expect("failed to enter data directory").join("d_vault_data")}

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
update - calls the "rust_event" event (document.addEventListener('rust_event', () => {/*your code here*/}))
save_user - saves the current existing user
load_user - load an existing user
cd - enters a directory
cat - look what's inside a file if the file does not exists it creats the file
mkdir - creats a new directory
rmdir - removes a directory
pwd - shows your current path
"#)
}
fn main() {
   tauri::Builder::default().invoke_handler(tauri::generate_handler![
    first_init, list_commands, console, user_get, authenticate_user, update, save_user, load_user, ls, pwd
]).run(tauri::generate_context!()).expect("failed to run the code");
   }
