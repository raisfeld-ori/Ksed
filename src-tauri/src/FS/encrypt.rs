use tauri;
use tauri::Runtime;
use crypto::{aes, blockmodes};


pub async fn encrypt(name: &str, password: &str, data: Vec<u8>) 
-> Vec<u8> {
  todo!("command does not exist yet");
}

#[test]
fn test_encryption(){
  let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
  let iv = *b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\
  \x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F";

  aes::cbc_encryptor(aes::KeySize::KeySize128, key, &iv, blockmodes::PkcsPadding);

}