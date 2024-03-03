use crypto::{aes, blockmodes};
use crypto::buffer::{BufferResult, ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::aes::cbc_decryptor;

pub fn encrypt(username: &str, password: &str, data: Vec<u8>) -> Vec<u8> {
  // The key and the iv have to be 16 bytes!!!
  let key = password.as_bytes();
  let iv = padding(username, password);

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

pub fn decrypt(username: &str, password: &str, data: Vec<u8>) -> Vec<u8> {
    // The key and the iv have to be 16 bytes!!!
    let key = password.as_bytes();
    let iv = padding(username, password);

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

    return final_result
}



fn padding(username: &str, password: &str) -> Vec<u8> {
  let mut padded_username = username.chars().collect::<Vec<char>>();
  let password_chars = password.chars().rev().collect::<Vec<char>>();

  while padded_username.len() < 16 {
      if password_chars.len() > 0 {
          padded_username.push(password_chars[padded_username.len() % password_chars.len()]);
      } else {
          break;
      }
  }
  // Convert the padded username and password to byte vectors
  let padded_username_bytes = padded_username.into_iter().map(|c| c as u8).collect::<Vec<u8>>();
  padded_username_bytes
}


#[test]
fn test_encryption(){
  let username = "aviv";
  let password = "digmas5avivsegal";
  let data = "secret messagess".as_bytes();
  let result2 = encrypt(username, password, data.to_vec());
  assert_ne!(data, result2);
  let result3 = decrypt(username, password, result2);
  assert_eq!(data, result3);

  
  
}