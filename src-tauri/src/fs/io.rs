use std::fs::read_dir;
use std::path::PathBuf;
use dirs::data_dir;
pub use base64::{decode_config, encode_config, URL_SAFE};
use serde_json::Value;
pub use tauri::Runtime;
use crate::data::auth::{init_dir, Encodable};
use crate::data::json::init_user_data;
use crate::fs::encryption::{aes_encrypt, aes_decrypt};
use crate::fs::fs::FS;


pub fn dir() -> PathBuf {data_dir().expect("Failed to enter data directory").join("d_vault_data")}
pub fn get_user_dir(name: &str, password: &str) -> PathBuf{
  dir().join(encode_config(&aes_encrypt(name, password, name.as_bytes()), URL_SAFE))
}
#[allow(unused)]
pub fn open_file(name: &str, password: &str, target: String) -> Option<PathBuf>{
  let location = get_user_dir(name, password);
  if location.exists(){
    for file in read_dir(location).unwrap(){
      if file.is_err(){continue;}
      let file = file.unwrap();
      let file_name = decode_config(file.file_name().to_bytes(), URL_SAFE);
      if file_name.is_err() {continue;}
      if String::from_utf8(aes_decrypt(name, password, &file_name.unwrap())).unwrap() == target{
        return Some(file.path());
      }
    }
  }
  return None;
}

#[tauri::command]
pub fn bytes_to_string(bytes: Vec<u8>) -> Result<String, ()> {
  let result = String::from_utf8(bytes);
  if result.is_err() {return Err(());}
  else {return Ok(result.unwrap());}
}

#[tauri::command]
pub fn image_to_string(bytes: Vec<u8>) -> Result<(String, String), Value> {
    let reader = image::io::Reader::new(std::io::Cursor::new(bytes));
    let reader = reader.with_guessed_format();
    if reader.is_err(){return Err(Value::Null)}
    let reader = reader.unwrap();
    let format = reader.format();
    let result = reader.decode();
    if result.is_err() || format.is_none() {return Err(Value::Null)}
    let result = base64::encode(result.unwrap().as_bytes());
    return Ok((result, format.unwrap().to_mime_type().to_string()));
}

#[test]
fn test_general(){
  let name = "test";
  let password = "test";
  get_user_dir(name, password);
  open_file(name, password, String::from("something"));
}
#[allow(unused)]
#[tauri::command]
pub fn first_init<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
    window.set_fullscreen(true);
    init_user_data();
    init_dir();
    unsafe{FS.init_fs()};
    Ok(())
}
#[tauri::command]
pub fn console(value: String) {println!("{}", value)}
#[tauri::command]
pub fn close_app<R: Runtime>(window: tauri::Window<R>) -> tauri::Result<()> {window.close()}