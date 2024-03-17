use std::{fs::{self, read}, io::Write, path::PathBuf};
use crate::dir;
use serde::{Deserialize, Serialize};
use serde_json::to_vec;
use base64::{self, decode, encode};
use tauri::api::file;

use crate::fs::encryption::{aes_decrypt, xor_encrypt};

pub static mut FS: Home = Home::new();

pub fn save_fs(name: String, password: String) -> Result<(), Box<dyn Error>>{
    let dir = dir();
    let fs_bytes = unsafe{to_vec(&FS)?};
    let dir = dir.join(encode(xor_encrypt(b"encrypt".to_vec(), &fs_bytes)));
    if dir.exists(){
        let mut writer = fs::File::open(dir.as_path()).unwrap();
        writer.write_all(&fs_bytes)?;
    }
    else {
        let mut writer = fs::File::create(dir.as_path()).unwrap();
        writer.write_all(&fs_bytes)?;
    }
    println!("{:?}", dir.as_path());
    Ok(())
}
pub fn load_fs(name: String, password: String) -> Result<(), String>{
    let dir = dir();
    let fs_bytes = unsafe{to_vec(&FS)};
    if fs_bytes.is_err(){return Err(format!("{}", fs_bytes.unwrap_err()));}
    let fs_bytes = fs_bytes.unwrap();
    let file_path = dir.join(encode(xor_encrypt(b"encrypt".to_vec(), &fs_bytes)));
    let mut file = fs::File::open(&file_path).map_err(|e| format!("failed to open file: {}", e))?;
    let mut content = Vec::new();
    file.read_to_end(&mut content).map_err(|e|format!("failed to read file: {}", e))?;

    let decoded = decode(&content).map_err(|e| format!("failed to decode: {}", e))?;
    let decrypted = xor_encrypt(decoded, &password.as_bytes());

    let home: Home = serde_json::from_slice(&decrypted).map_err(|e| format!("failed to deserialize data: {}", e))?;
    unsafe{FS = home;}
    println!("Loaded file system state from: {:?}", dir.as_path());
    Ok(())
}

#[tauri::command]
pub fn pwd() -> String{return unsafe {
    let result = FS.path.iter().map(|dir| dir.name.clone() + "/").collect::<String>();
    if result == String::from(""){String::from("Home/")}
    else {result}
}}

#[tauri::command]
pub fn cd(new: String) {
    unsafe{
        for item in FS.current_dir.files.iter(){
            let dir = item.get_directory();
            if dir.is_none(){continue}
            // if you use an else block then dir will need to be mutable
            let dir = dir.unwrap();
            if dir.name == new{FS.cd(dir);}
        }
    }
} 

#[tauri::command]
pub fn ls() -> Vec<String> {
  unsafe {FS.current_dir.files.iter().map(|item| 
    match item{
        DiretoryItems::Directory(dir) => dir.name.clone(),
        DiretoryItems::File(file) => file.name.clone()
    }).collect::<Vec<String>>()}
}


#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Home{
    path: Vec<Directory>,
    current_dir: Directory,
}

impl Home{
    pub const fn new() -> Self{return Home{path: Vec::new(), current_dir: Directory::new(String::new())}}
    pub fn init_fs(&mut self) {self.path = Vec::new();self.current_dir = Directory::new(String::from("Home"));}
    pub fn cd_back(&mut self) {if self.path.len() > 1 {self.path.pop();self.current_dir = self.path.last().unwrap().clone();}}
    pub fn cd(&mut self, new: Directory) {self.current_dir = new.clone();self.path.push(new);}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Directory{
    files: Vec<DiretoryItems>,
    name: String,
}

impl Directory{
    pub const fn new(name: String) -> Self{return Directory{name: name, files: Vec::new()}}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum DiretoryItems{File(File),Directory(Directory)}

impl DiretoryItems{
    pub fn get_directory(&self) -> Option<Directory>{match self{Self::Directory(dir)=>{Some(dir.clone())} _=>{None}}}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct File{
    name: String,
    location: PathBuf,
}
impl File{
    pub fn open(&self) -> Option<Vec<u8>> {
        let data = read(self.location.as_path());
        if data.is_err(){return None;}
        // i'l keep on working on it later
        None
    }
}

#[test]
fn test_fs(){
    let err = save_fs(String::from("test"), String::from("test"));
    println!("{}", err);
}