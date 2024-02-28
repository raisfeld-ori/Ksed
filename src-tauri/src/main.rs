// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Runtime;
mod encryption;
use crate::encryption::encrypt;

#[allow(unused)]
#[tauri::command]
async fn first_init<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
    window.set_fullscreen(true);
    
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![first_init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
