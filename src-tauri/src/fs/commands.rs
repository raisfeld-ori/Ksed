use std::{fs, path::PathBuf};
use base64::{encode_config, URL_SAFE};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fs::{read, write};
use crate::get_user_dir;

use super::encryption::aes_encrypt;

pub static mut FS: Home = Home::new();


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
        DirectoryItems::Directory(dir) => dir.name.clone(),
        DirectoryItems::File(file) => file.name.clone()
    }).collect::<Vec<String>>()}
}
#[tauri::command]
pub fn upload_file(name: &str, password: &str, file_path: String) -> Result<(), std::io::Error> {
  let file_content = read(file_path.clone());
  if file_content.is_err() {return Err(file_content.unwrap_err());}
  let encrypted_content = aes_encrypt(name, password, &file_content.unwrap());
  let path = PathBuf::from(file_path);
  let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
  let new_file = File::new(name, password,file_name, unsafe {&FS.current_dir});
  return Ok(());
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Home{
    path: Vec<Directory>,
    current_dir: Directory,
}

impl Home{
    pub const fn new() -> Self{return Home{path: Vec::new(), current_dir: Directory::new(String::new())}}
    pub fn init_fs(&mut self) {
        self.path = Vec::new();
        let home_dir = Directory::new(String::from("Home"));
        self.path.push(home_dir.clone());
        self.current_dir = home_dir;
        mkdir(String::from("bin"));
    }
    
    pub fn cd_back(&mut self) {if self.path.len() > 1 {self.path.pop();self.current_dir = self.path.last().unwrap().clone();}}
    pub fn cd(&mut self, dir: Directory) {self.current_dir = dir.clone();self.path.push(dir);}
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error>{return serde_json::to_vec(self);}
}

#[tauri::command]
pub fn mkdir(name: String) {unsafe{FS.current_dir.files.push(DirectoryItems::Directory(Directory::new(name)))};}
#[tauri::command]
pub fn mk(name: &str, password: &str, fileName: String) -> Result<(), String> {
    let new_file = File::new(name, password, fileName, unsafe{&FS.current_dir});
    if new_file.is_none() {return Err(String::from("a file with this name already exists"));}
    unsafe{FS.current_dir.files.push(DirectoryItems::File(new_file.unwrap()))};
    Ok(())
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Directory{
    files: Vec<DirectoryItems>,
    name: String,
}

impl Directory{
    pub const fn new(name: String) -> Self{return Directory{name: name, files: Vec::new()}}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum DirectoryItems{File(File),Directory(Directory)}

impl DirectoryItems{
    pub fn get_directory(&self) -> Option<Directory>{match self{Self::Directory(dir)=>{Some(dir.clone())} _=>{None}}}
    pub fn get_file(&self) -> Option<File>{match self{Self::File(file)=>{Some(file.clone())} _=>{None}}}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct File{
    name: String,
    location: PathBuf,
}
impl File{
    pub fn new(name: &str, password: &str, file_name: String, parent: &Directory) -> Option<Self> {
        let location = get_user_dir(name, password);
        let name = encode_config(aes_encrypt(name, password, file_name.as_bytes()), URL_SAFE);
        let location = location.join(name);
        // i will change this in the future
        // Ori what have you done!?
        if location.exists() {return None;}
        return Some(File {name: file_name, location});
    }
    pub fn save(&self,name: &str, password: &str, data: &[u8]) -> Result<(), std::io::Error>{
        let encrypted_content = aes_encrypt(name, password, data);
        let save = write(self.location.as_path(),encrypted_content);
        if save.is_err() {return Err(save.unwrap_err());}
        return Ok(());
    }
    pub fn open(&self) -> Option<Vec<u8>> {
        let data = read(self.location.as_path());
        if data.is_err(){return None;}
        // i'l keep on working on it later
        None
    }

}

#[test]
fn test_fs() {
    unsafe{FS.init_fs();}
    let name = "some";
    let password = "thing";
    let mut dir = Directory::new(String::from("Home"));
    let file = File::new(name, password, String::from("file"), &dir).unwrap();
    dir.files.push(DirectoryItems::Directory(Directory::new(String::from("bin"))));
    dir.files.push(DirectoryItems::File(file));
    assert!(mk(name, password, String::from("file")).is_ok());
    assert_eq!(unsafe{&FS.current_dir}, &dir);
    println!("Dir: {:?}",dir)
}
#[test]
fn test_upload(){
    unsafe{FS.init_fs();}
    let name = "some";
    let password = "thing";
    File::new(name, password, String::from("test.txt"), unsafe {&FS.current_dir});
    assert!(upload_file(name, password, String::from("/test.txt")).is_ok());
}