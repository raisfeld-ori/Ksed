extern crate dirs;
use tauri::Runtime;
use std::path::PathBuf;

pub fn load_dir() -> PathBuf{
    return dirs::data_local_dir().expect("failed to read file system");
}

#[tauri::command]
async fn load_data<R: Runtime>(_app: tauri::AppHandle<R>, _window: tauri::Window<R>) -> Result<(), String> {
  Ok(())
}

#[test]
fn test_loading(){
    load_dir();
}