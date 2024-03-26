use std::fs::read_dir;
use std::path::PathBuf;
use dirs::data_dir;
pub use base64::{decode_config, encode_config, URL_SAFE};
pub use tauri::Runtime;
pub use crate::data::json::{init_user_data, user_get, create_value, user_make, system_get, system_make};
pub use crate::data::auth::{init_dir, save_user, authenticate_user, load_user, user_exists, create_user};
pub use crate::fs::commands::{pwd, ls, FS, cd, mkdir, mk};
pub use crate::fs::encryption::{aes_decrypt, aes_encrypt};
pub use crate::data::auth::Encodable;


pub fn dir() -> PathBuf {data_dir().expect("failed to enter data directory").join("d_vault_data")}
pub fn get_user_dir(name: &str, password: &str) -> PathBuf{
  dir().join(encode_config(&aes_encrypt(name, password, name.as_bytes()), URL_SAFE))
}
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
