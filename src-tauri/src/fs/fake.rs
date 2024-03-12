
use std::{fs, io};
use crate::dir;
use serde::{Deserialize, Serialize};
use tauri::{App};
use base64::{self, decode, encode};

use super::encryption::aes_encrypt;

pub static mut FS: Home = Home::new();

#[tauri::command]
pub fn pwd() -> String{return unsafe {
    let result = FS.path.iter().map(|dir| dir.name.clone() + "/").collect::<String>();
    if result == String::from(""){String::from("Home/")}
    else {result}
}}

#[tauri::command]
pub fn ls() -> Vec<String> {
  unsafe {FS.current_dir.Files.iter().map(|item| 
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
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Directory{
    Files: Vec<DiretoryItems>,
    name: String,
}

impl Directory{
    pub const fn new(name: String) -> Self{return Directory{name: name, Files: Vec::new()}}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum DiretoryItems{File(File),Directory(Directory)}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct File{
    name: String,
}