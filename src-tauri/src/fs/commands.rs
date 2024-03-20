use std::path::PathBuf;
use base64::{encode_config, URL_SAFE};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fs::read;

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
            if dir.name == new{unsafe{FS.cd(dir);}}
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
pub fn mkdir(name: String) {unsafe{FS.current_dir.files.push(DiretoryItems::Directory(Directory::new(name)))};}
#[tauri::command]
fn mk(name: &str, password: &str, file_name: String) -> Result<(), String> {
    let new_file = File::new(name, password, file_name, unsafe{&FS.current_dir});
    if new_file.is_none() {return Err(String::from("a file with this name already exists"));}
    unsafe{FS.current_dir.files.push(DiretoryItems::File(new_file.unwrap()))};
    Ok(())
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
        if location.exists() {return None;}
        return Some(File {name: file_name, location});
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
    let mut dir = Directory::new(String::from("home"));
    let file = File::new(name, password, String::from("file"), &dir).unwrap();
    dir.files.push(DiretoryItems::Directory(Directory::new(String::from("bin"))));
    dir.files.push(DiretoryItems::File(file));
    assert!(mk(name, password, String::from("file")).is_ok());
    assert_eq!(unsafe{&FS.current_dir}, &dir);
}