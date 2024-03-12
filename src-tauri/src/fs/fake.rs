
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
pub fn cd(new: String) {
    unsafe{
        for item in FS.current_dir.Files.iter(){
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
    pub fn cd_back(&mut self) {if self.path.len() > 1 {self.path.pop();self.current_dir = self.path.last().unwrap().clone();}}
    pub fn cd(&mut self, new: Directory) {self.current_dir = new.clone();self.path.push(new);}
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

impl DiretoryItems{
    pub fn get_directory(&self) -> Option<Directory>{match self{Self::Directory(dir)=>{Some(dir.clone())} _=>{None}}}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct File{
    name: String,
}