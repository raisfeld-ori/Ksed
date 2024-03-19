use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::fs::read;

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
        let mut home_dir = Directory::new(String::from("Home"));
        let bin_dir = Directory::new(String::from("bin"));
        home_dir.files.push(DiretoryItems::Directory(bin_dir));
        self.path.push(home_dir);
        println!("path:{:?}", self.path);
    }
   
    pub fn cd_back(&mut self) {if self.path.len() > 1 {self.path.pop();self.current_dir = self.path.last().unwrap().clone();}}
    pub fn cd(&mut self, dir: Directory) {self.current_dir = dir.clone();self.path.push(dir);}
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
    let mut home = Home::new();
    home.init_fs();

}