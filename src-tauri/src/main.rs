// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod fs;
use fs::utilities::*;
use fs::commands::*;
mod data;
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
    create_value, mkdir, system_get, system_make, user_make, close_app, mk, upload_file
]).run(tauri::generate_context!()).expect("failed to run the code");
   }
