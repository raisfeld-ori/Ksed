// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::fs::utilities::*;
use crate::fs::commands::*;
use crate::data::json::*;
use crate::data::auth::*;
mod fs;
mod data;


fn main() {
   tauri::Builder::default().invoke_handler(tauri::generate_handler![
    first_init, console, user_get, authenticate_user, save_user, user_exists, load_user, ls, pwd, cd, create_user,
    create_value, mkdir, system_get, system_make, user_make, close_app, mk, upload_file, cd_back,
]).run(tauri::generate_context!()).expect("failed to run the code");
   }
