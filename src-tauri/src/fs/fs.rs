use std::{fs, path::PathBuf};
use base64::{encode_config, URL_SAFE};
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use std::fs::{read, write};
use crate::fs::encryption::aes_decrypt;
use crate::fs::io::get_user_dir;
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
#[tauri::command(async)]
pub fn upload_file(name: &str, password: &str, file_path: String) -> Result<(), String> {
    let data = read(file_path.clone());
    if data.is_err() {return Err(String::from("Failed to read the uploaded file"));}
    let path = PathBuf::from(file_path);
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    let existing_file_names: Vec<&String> = unsafe{FS.current_dir.files.iter()}
    .map(|file|{return file.name();}).collect();
    fn unique_file_name(name: String, all_names: Vec<&String>, num: i32) -> String{
        let num_str = num.to_string();
        let num_str = num_str.as_str();
        if all_names.contains(&&(name.clone() + num_str)){
            let num = num + 1;
            return unique_file_name(name, all_names, num);
        }
        else if num == 1 {return name;}
        else {return name + num_str;}
    }
    let num = 1;
    let file_name = unique_file_name(file_name, existing_file_names, num);
    let new_file = File::new(name, password,file_name);
    if unsafe{FS.current_dir.files.iter().any(|other_file| {other_file.name() == &new_file.name})} {
        return Err(String::from("A file with this name already exists"));
    }
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
    pub fn get_item(&self, name: &str) -> Option<&DirectoryItems>{
        self.current_dir.files.iter().find(|file| file.name() == name)
    }
    #[allow(unused)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error>{return serde_json::to_vec(self);}
}

#[tauri::command]
pub fn mkdir(name: String) {unsafe{FS.current_dir.files.push(DirectoryItems::Directory(Directory::new(name)))};}
#[allow(non_snake_case)]
#[tauri::command]
pub fn mk(name: &str, password: &str, fileName: String) -> Result<(), String> {
    let new_file = File::new(name, password, fileName);
    unsafe{FS.current_dir.files.push(DirectoryItems::File(new_file))};
    Ok(())
}
#[tauri::command]
pub fn rename(file_name: &str, new: String) {
    for file in unsafe{FS.current_dir.files.iter_mut()}{
        if file.name() == file_name{
            file.rename(new);
            return;
        }
    }
}
#[tauri::command(async)]
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
    pub fn rename(&mut self, new: String){self.name = new;}
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
    pub fn rename(&mut self, new: String){
        match self{
            Self::Directory(dir) => dir.rename(new),
            Self::File(file) => file.rename(new),
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
        let _ = new_self.save(name, password, b"");
        return new_self;
    }
    pub fn save(&self,name: &str, password: &str, data: &[u8]) -> Result<(), std::io::Error>{
        let encrypted_content = aes_encrypt(name, password, data);
        let save = write(self.location.as_path(),encrypted_content);
        if save.is_err() {return Err(save.unwrap_err());}
        return Ok(());
    }
    pub fn export(&self, name: &str, password: &str, location: String) -> bool{
        let encrypted_content = read(self.location.as_path());
        if encrypted_content.is_err(){return false;}
        let decrypted_content = aes_decrypt(name, password, &encrypted_content.unwrap());
        let result = write(location, decrypted_content);
        if result.is_err(){return true;}
        return true;
    }
    pub fn delete(&self) -> Result<(), std::io::Error>{
        let _result = fs::remove_file(self.location.as_path());
        return Ok(())
    }
    pub fn rename(&mut self, new: String){self.name = new;}
    pub fn read(&self, name: &str, password: &str) -> Option<Vec<u8>> {
        let data = read(self.location.as_path());
        if data.is_err(){return None;}
        let data = data.unwrap();
        let data = aes_decrypt(name, password, &data);
        return Some(data);
    }

}

#[tauri::command(async)]
pub fn read_file(file: &str, name: &str, password: &str) -> Result<Vec<u8>, Value> {
    let file = unsafe{FS.get_item(file)};
    if file.is_none() {return Err(Value::Null);}
    let file = file.unwrap().get_file();
    if file.is_none() {return Err(Value::Null);}
    let file  = file.unwrap().read(name, password);
    if file.is_none() {return Err(Value::Null);}
    else{return Ok(file.unwrap())}
}

#[tauri::command(async)]
pub fn export_file(name: &str, password: &str,file: &str, location: String) -> bool {
    let item = unsafe{FS.get_item(file)};
    if item.is_none() {return false}
    let item = item.unwrap().get_file();
    if item.is_none() {return false}
    item.unwrap().export(name, password, location)
}


#[test]
fn test_file_reading(){
    use crate::{authenticate_user, bytes_to_string, create_user};
    use crate::data::json::init_user_data;
    use crate::init_dir;
    let name = "names";
    let password = "passwords";
    let file = "example_file.txt";
    assert!(std::fs::write(&file, "foo, bar").is_ok());
    init_user_data();
    assert!(init_dir().is_ok());
    unsafe{FS.init_fs()};
    if !authenticate_user(name, password){
        assert!(create_user(name, password).is_ok());
    }
    assert!(upload_file(name, password, file.to_string()).is_ok());
    let data = read_file(file, name, password);
    assert!(data.is_ok());
    assert_eq!("foo, bar", bytes_to_string(data.unwrap()).unwrap_or("".to_string()));
}

#[tauri::command]
pub fn file_exists(file_name: String) -> bool {
  for file in unsafe{FS.current_dir.files.iter()}{
    if file.name() == &file_name{
        return true;
    }
  }
  return false;
}

#[test]
fn test_fs() {
    unsafe{FS.init_fs();}
    let name = "some";
    let password = "thing";
    let mut dir = Directory::new(String::from("Home"));
    let file = File::new(name, password, String::from("file"));
    dir.files.push(DirectoryItems::File(file));
    assert!(dir.delete_item(String::from("bin")).is_ok());
    assert!(unsafe{FS.current_dir.delete_item(String::from("bin"))}.is_ok());
    assert!(mk(name, password, String::from("file")).is_ok());
    assert_eq!(unsafe{&FS.current_dir}, &dir);
}