use crypto::{aes, blockmodes};
use crypto::buffer::{BufferResult, ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::aes::cbc_decryptor;

pub fn aes_encrypt(username: &str, password: &str, data: &[u8]) -> Vec<u8> {
  let key: &[u8] = &pad(password.as_bytes());
  let iv = padding(username, password, 16);
  let data = pad(data);
  let mut cipher = aes::cbc_encryptor(aes::KeySize::KeySize128, key, &iv, blockmodes::PkcsPadding);
  let mut read_buffer = RefReadBuffer::new(&data);
  let mut buffer = [0; 4096];
  let mut write_buffer = RefWriteBuffer::new(&mut buffer);

  let mut final_result = Vec::<u8>::new();

  loop {
    let encryption_result = cipher.encrypt(&mut read_buffer, &mut write_buffer, true);
    
    match encryption_result {
        Ok(BufferResult::BufferUnderflow) => break,
        Ok(BufferResult::BufferOverflow) => {
          final_result.extend(write_buffer.take_read_buffer().take_remaining());
        }
        Err(e) => panic!("{:?}", e),
    }
  }
  final_result.extend(write_buffer.take_read_buffer().take_remaining());
  return final_result;
  
  
}

pub fn aes_decrypt(username: &str, password: &str, data: &[u8]) -> Vec<u8> {
    let key: &[u8] = &pad(password.as_bytes());
    let iv = padding(username, password, 16);

    let mut cipher = cbc_decryptor(aes::KeySize::KeySize128, key, &iv, blockmodes::PkcsPadding);

    let mut read_buffer = RefReadBuffer::new(&data);
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    let mut final_result = Vec::<u8>::new();

    loop {
        let decryption_result = cipher.decrypt(&mut read_buffer, &mut write_buffer, false);
        
        match decryption_result {
            Ok(BufferResult::BufferUnderflow) => break,
            Ok(BufferResult::BufferOverflow) => {
                final_result.extend(write_buffer.take_read_buffer().take_remaining());
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    final_result.extend(write_buffer.take_read_buffer().take_remaining());

    return unpad(&final_result)
}

fn pad(data: &[u8]) -> Vec<u8> {
  let padding_size = 16 - (data.len() % 16);
  let mut padded_data = data.to_vec();
  padded_data.extend(vec![padding_size as u8; padding_size]);
  padded_data
}

fn unpad(data: &[u8]) -> Vec<u8> {
  let padding_size = data.last().unwrap();
  data[..data.len() - *padding_size as usize].to_vec()
}

fn xor_encrypt(data: Vec<u8>, key: &[u8]) -> Vec<u8>{
  let mut encrypted = Vec::new();

  for (i, &byte) in data.iter().enumerate() {
    encrypted.push(byte ^ key[i % key.len()])
  }

  encrypted
}
fn xor_dencrypt(encrypted_data: Vec<u8>, key: &[u8]) -> Vec<u8>{
  let mut decrypted = Vec::new();

  for(i, &byte) in encrypted_data.iter().enumerate() {
    decrypted.push(byte ^ key[i % key.len()])
  }
  decrypted
}


fn padding(username: &str, password: &str, size: usize) -> Vec<u8> {
  let mut padded_username = username.chars().collect::<Vec<char>>();
  let password_chars = password.chars().rev().collect::<Vec<char>>();

  while padded_username.len() < size {
      if password_chars.len() > 0 {
          padded_username.push(password_chars[padded_username.len() % password_chars.len()]);
      } else {
        // if the password length is 0.
          break;
      }
  }
  // Convert the padded username to byte vector and this is the IV.
  let padded_username_bytes = padded_username.into_iter().map(|c| c as u8).collect::<Vec<u8>>();
  padded_username_bytes
}


#[test]
fn test_xor_encryption(){
  let key = b"scrt";
  let data = b"hello, world!";
  let encrypted = xor_encrypt(data.to_vec(), key);
  let decrypted: &[u8] = &xor_dencrypt(encrypted, key);
  assert_eq!(data, decrypted);
}

#[test]
fn test_aes_encryption(){
  let username = "name";
  let password = "passwordpasswor";
  let data = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
  let encrypted: &[u8] = &aes_encrypt(username, password, data);
  let decrypted = aes_decrypt(username, password, encrypted);
  assert_eq!(data.to_vec(), decrypted);
}