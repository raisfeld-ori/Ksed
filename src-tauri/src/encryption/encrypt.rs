use tauri;
use tauri::Runtime;

#[tauri::command]
pub async fn encrypt<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>, name: &str, password: &str) 
-> Result<(), String> {

  return Ok(());
}