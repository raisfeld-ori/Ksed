


#[tauri::command]
async fn save_data<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
  Ok(())
}

#[test]
fn test_loading(){

}