use crypto::blockmodes::PaddingProcessor;
use tauri;
use crypto::{aes, blockmodes};
use crypto::buffer::{BufferResult, ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};



pub fn encrypt(name: &str, password: &str, data: Vec<u8>) 
-> Vec<u8> {
  let key = password.as_bytes();
  let iv = name.as_bytes();

  let mut cipher = aes::cbc_encryptor(aes::KeySize::KeySize128, key, iv, blockmodes::PkcsPadding);

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
  return  final_result;
  
  
}

#[test]
fn test_encryption(){
  let data = "secret";

  let result = encrypt("aviv", "digmas", data.as_bytes().to_vec());
  assert_ne!(data.as_bytes().to_vec(), result);
  
}