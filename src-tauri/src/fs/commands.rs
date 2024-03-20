use std::{fs, path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fs::{write,read};

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
#[tauri::command]
pub fn create_file(file_name: String, location: PathBuf) {
  unsafe {
    let new_file = File::new(file_name, location);
    FS.current_dir.files.push(DiretoryItems::File(new_file))
   
  }
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
    }
    
    pub fn cd_back(&mut self) {if self.path.len() > 1 {self.path.pop();self.current_dir = self.path.last().unwrap().clone();}}
    pub fn cd(&mut self, dir: Directory) {self.current_dir = dir.clone();self.path.push(dir);}
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error>{return serde_json::to_vec(self);}
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
    pub fn new(name: String,location: PathBuf) -> Self {return File {name, location}}
    pub fn add_data(&self, data: Vec<String>) -> Option<()>{
        let content = data.join("\n");
        let data = fs::write(self.location.as_path(), content);
        if data.is_err(){return None;}
        // Ori will work on it later
        None
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
    let mut home = Home::new();
    let mut dir = Directory::new(String::from("man"));
    home.cd(dir.clone());
    let location = PathBuf::from("Home/man/");
    let new_file = File::new(String::from("salves"), location.clone());
    dir.files.push(DiretoryItems::File(new_file));
    println!("dir files: {:?}", home.path);
}