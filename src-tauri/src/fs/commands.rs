use std::{fs, path::PathBuf};
use base64::{encode_config, URL_SAFE};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::fs::{read, write};
use crate::fs::encryption::aes_decrypt;
use crate::fs::utilities::get_user_dir;
use crate::fs::encryption::aes_encrypt;

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
            let dir = dir.unwrap();
            if dir.name == new{FS.cd(dir);}
        }
    }
} 
#[tauri::command]
pub fn cd_back() {unsafe{FS.cd_back();}}

#[tauri::command]
pub fn ls() -> Vec<(String, String)> {
  unsafe {FS.current_dir.files.iter().map(|item| 
    match item{
        DirectoryItems::Directory(dir) => (dir.name.clone(), String::from("Directory")),
        DirectoryItems::File(file) => (file.name.clone(), String::from("File"))
    }).collect::<Vec<(String, String)>>()}
}
#[tauri::command]
pub fn upload_file(name: &str, password: &str, file_path: String) -> Result<(), String> {
    let data = read(file_path.clone());
    if data.is_err() {return Err(String::from("failed to read the uploaded file"));}
    let path = PathBuf::from(file_path);
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    let new_file = File::new(name, password,file_name);
    let result = new_file.save(name, password, &data.unwrap());
    if result.is_err(){return Err(result.unwrap_err().to_string())}
    unsafe{FS.current_dir.files.push(DirectoryItems::File(new_file))};

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
    
    pub fn cd_back(&mut self) {
        if self.path.len() > 1 {
            let prev_current_dir = self.current_dir.clone();
            self.path.pop();

            if let Some(second_last_dir) = self.path.last_mut() {
                second_last_dir.files.retain(|item| match item {
                    DirectoryItems::Directory(dir) => dir.name != prev_current_dir.name,
                    DirectoryItems::File(_) => true,
                });
            }
            self.current_dir = self.path.clone().into_iter().last().unwrap();
            self.current_dir.files.push(DirectoryItems::Directory(prev_current_dir));
        }
    }
    pub fn cd(&mut self, dir: Directory) {
        let files = &self.path.last().unwrap().files;
        let mut new_files = Vec::new();
        for file in self.current_dir.files.iter(){
            if !files.contains(&file){new_files.push(file.clone());}
        }
        self.path.last_mut().unwrap().files.append(&mut new_files);
        self.current_dir = dir.clone();
        self.path.push(dir);
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error>{return serde_json::to_vec(self);}
}

#[tauri::command]
pub fn mkdir(name: String) {unsafe{FS.current_dir.files.push(DirectoryItems::Directory(Directory::new(name)))};}
#[tauri::command]
pub fn mk(name: &str, password: &str, fileName: String) -> Result<(), String> {
    let new_file = File::new(name, password, fileName);
    unsafe{FS.current_dir.files.push(DirectoryItems::File(new_file))};
    Ok(())
}
#[tauri::command]
pub fn rm(file: String) -> Result<(), String>{
    let result = unsafe{FS.current_dir.delete_item(file)};
    if result.is_err() {Err(result.unwrap_err().to_string())}
    else{Ok(())}
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Directory{
    files: Vec<DirectoryItems>,
    name: String,
}

impl Directory{
    pub const fn new(name: String) -> Self{return Directory{name: name, files: Vec::new()}}
    pub fn delete_item(&mut self, remove: String) -> Result<(), std::io::Error> {
        let mut result = Ok(());
        self.files.retain(|itm| {
            if itm.name() == &remove{
                result = itm.remove();
                return false;
            } 
            else{
                return true;
            }
        });
        return result;
    }
    pub fn delete(&self) -> Result<(), std::io::Error>{
        for itm in self.files.iter(){
            itm.remove()?;
        }
        return Ok(());
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum DirectoryItems{File(File),Directory(Directory)}

impl DirectoryItems{
    pub fn name(&self) -> &String{
        match self{
            Self::Directory(dir) => &dir.name,
            Self::File(file) => &file.name,
        }
    }
    pub fn remove(&self) -> Result<(), std::io::Error>{
        match self{
            Self::Directory(dir) => dir.delete(),
            Self::File(file) => file.delete(),
        }
    }
    pub fn get_directory(&self) -> Option<Directory>{match self{Self::Directory(dir)=>{Some(dir.clone())} _=>{None}}}
    pub fn get_file(&self) -> Option<File>{match self{Self::File(file)=>{Some(file.clone())} _=>{None}}}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct File{
    name: String,
    location: PathBuf
}
impl File{
    pub fn new(name: &str, password: &str, file_name: String) -> Self {
        let location = get_user_dir(name, password);
        let location_name = encode_config(
            aes_encrypt(name, password, file_name.as_bytes()), 
            URL_SAFE);
        let location = location.join(location_name);
        let new_self = File {name: file_name, location};
        // i have more important things to work on
        // so just pretend this works with no issue
        let _ = new_self.save(name, password, b"");
        return new_self;
    }
    pub fn save(&self,name: &str, password: &str, data: &[u8]) -> Result<(), std::io::Error>{
        let encrypted_content = aes_encrypt(name, password, data);
        let save = write(self.location.as_path(),encrypted_content);
        if save.is_err() {return Err(save.unwrap_err());}
        return Ok(());
    }
    pub fn export(&self, name: &str, password: &str) -> Result<(), std::io::Error>{
        let encrypted_content = read(self.location.as_path());
        if encrypted_content.is_err(){return Err(encrypted_content.unwrap_err());}
        let _decrypted_content = aes_decrypt(name, password, &encrypted_content.unwrap());
        return Ok(());
    }
    pub fn delete(&self) -> Result<(), std::io::Error>{
        let result = fs::remove_file(self.location.as_path());
        return Ok(())
    }
    pub fn open(&self) -> Option<Vec<u8>> {
        let data = read(self.location.as_path());
        if data.is_err(){return None;}
        // i'l keep on working on it later
        Some(data.unwrap())
    }

}

#[test]
fn test_fs() {
    unsafe{FS.init_fs();}
    let name = "some";
    let password = "thing";
    let mut dir = Directory::new(String::from("Home"));
    let file = File::new(name, password, String::from("file"));
    dir.files.push(DirectoryItems::Directory(Directory::new(String::from("bin"))));
    dir.files.push(DirectoryItems::File(file));
    assert!(dir.delete_item(String::from("bin")).is_ok());
    assert!(unsafe{FS.current_dir.delete_item(String::from("bin"))}.is_ok());
    assert!(mk(name, password, String::from("file")).is_ok());
    assert_eq!(unsafe{&FS.current_dir}, &dir);
}