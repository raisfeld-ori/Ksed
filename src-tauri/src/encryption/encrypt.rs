use tauri;

#[tauri::command]
pub async fn command_name<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>, name: &str, password: &str) 
-> Result<(), String> {
    
  return Ok(());
}