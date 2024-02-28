// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Runtime;

#[allow(unused)]
#[tauri::command]
async fn first_init<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
    // setting up CONFIG is unsafe, but it doesn't break anything
    Ok(())
}

#[tauri::command]
fn printf(text: &str) {println!("{}", text);}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![printf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
