
use std::{fs, io};
use crate::dir;
use serde::{de, Deserialize, Serialize};
use tauri::{api::file, App};
use base64::{self, decode, encode};

use super::encryption::aes_encrypt;

pub static mut FS: Home = Home::new();

#[tauri::command]
pub async fn save_user_dir(name: &str, password: &str ,home: Home, path: String) -> Result<(), String> {
  let location = dir().join(format!("{}", path));
  let serialized_home = serde_json::to_vec(&home).map_err(|e| e.to_string())?;
  let encrypted_file_name = aes_encrypt(name, password, &serialized_home);
  let serialized_home_base64 = encode(encrypted_file_name);
  location.join(serialized_home_base64);
  Ok(())
 
}
#[tauri::command]
pub async fn load_user_dir(path: String) -> Result<Vec<Home>, String> {
    let file = fs::File::open(&path).map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);
    let deserializer = serde_json::Deserializer::from_reader(reader);
    let iterator = deserializer.into_iter::<Home>();

    let mut homes = Vec::new();
    for item in iterator{
        match item{
            Ok(home) => homes.push(home),
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(homes)
}
#[tauri::command]
pub async fn cd(dir: String) -> Option<Directory> {
  unsafe {let cd = FS.cd(dir);
  return cd.cloned();}
}
#[tauri::command]
pub async fn cat(file_name: String, content: Option<String>) -> Option<()> {
    unsafe {if let Some(content) = content {
        FS.add_file(file_name, content.into_bytes().to_vec());
        Some(())
    } else {
        match FS.find_file(&file_name) {
            Some(file) => {
                let content = String::from_utf8(file.data.clone()).unwrap();
                println!("{}", content);
                Some(())
            },
            None => None,
        }
    }
}
}
#[tauri::command]
pub async fn mkdir(dir: String){
    unsafe {FS.directories.push(Directory::new(dir))}
}
#[tauri::command]
pub async fn rmdir(dir: String) -> Result<(), Option<String>> {
    unsafe {let index = FS.directories.iter().position(|d| d.name == dir);
        match index{
            Some(idx) => {
                FS.directories.remove(idx);
                if FS.path.ends_with(&dir) {
                    FS.path.truncate(FS.path.len() - dir.len());
                }
                Ok(())
            },
            None => Err(Some(format!("Directory '{}' not found", dir)))
        }

    }
}
#[tauri::command]
pub async fn pwd() -> String {
  unsafe {"home".to_string() + FS.path.as_str()}
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct File{
    pub name: String,
    pub extension: String,
    data: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Directory{
    pub name: String,
    pub directories: Box<Vec<Directory>>,
    pub files: Vec<File>,
}
// impl for Directory with function that only Directory have.
impl Directory {
    fn new(name: String) -> Self{Self{name, directories: Box::new(Vec::new()), files: Vec::new()}}
    fn find_file(&self, file_name: &str) -> Option<&File> {
        self.files.iter().find(|file| file.name == file_name)
    }
    fn add_file(&mut self, file_name: String, content: Vec<u8>) -> Option<()> {
        let extension = self.get_file_extension(&file_name).unwrap_or("".to_string());
        let new_file = File {
            name: file_name,
            extension: extension,
            data: content,
        };
        self.files.push(new_file);
        Some(())
    }
    fn get_file_extension(&self,file_name: &str) -> Option<String> {
        let last_period_pos = file_name.rfind('.')?;
        Some(file_name[last_period_pos + 1..].to_string())
    }
}

// impl the directory trait for Directory struct because he needs these commands.
    
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Home{
    pub directories: Vec<Directory>,
    pub files: Vec<File>,
    pub path: String,
    pub current_dir: String,
}
// impl for home with functions that only Home have.
impl Home{
    pub const fn new() -> Self{Self { directories: Vec::new(), files: Vec::new(), path: String::new(), current_dir: String::new()}}
    pub fn name(&self) -> String {String::from("Home")}
    pub fn pwd(&self) -> String{"Home".to_string() + self.path.as_str()}
}
// impl the directory trait for Home because Home needs these commands.
impl DirTrait for Home {
    fn mkdir(&mut self, dir: String) {self.directories.push(Directory::new(dir))}
    fn rmdir(&mut self, dir: String) -> Result<(), Option<String>> {
        let index = self.directories.iter().position(|d| d.name == dir);
        match index{
            Some(idx) => {
                self.directories.remove(idx);
                if self.path.ends_with(&dir) {
                    self.path.truncate(self.path.len() - dir.len());
                }
                Ok(())
            },
            None => Err(Some(format!("Directory '{}' not found", dir)))
        }

    }
    fn cd(&mut self, dir: String) -> Option<&Directory> {
        for directory in &mut self.directories.iter() {
            if directory.name == dir{
                Some(directory);
                self.path += "/";
                self.path += directory.name.as_str();
                self.current_dir = directory.name.clone();
            }
        }
        None
    }
    fn cat(&mut self, file_name: String, content: Option<String>) -> Option<()> {
        if let Some(content) = content {
            self.add_file(file_name, content.into_bytes().to_vec());
            Some(())
        } else {
            match self.find_file(&file_name) {
                Some(file) => {
                    let content = String::from_utf8(file.data.clone()).unwrap();
                    println!("{}", content);
                    Some(())
                },
                None => None,
            }
        }
    }
    fn add_file(&mut self, file_name: String, content: Vec<u8>) -> Option<()> {
        self.directories.iter_mut().find(|dir| dir.name == self.current_dir.clone())
        .map(|dir| dir.add_file(file_name.clone(), content.clone()))
        .unwrap_or_else(|| {
            self.directories.push(Directory::new(self.current_dir.clone()));
            let mut new_dir = Directory::new(self.current_dir.clone());
            new_dir.add_file(file_name, content);
            Some(())
        })
    }
    fn find_file(&self, file_name: &str) -> Option<&File> {
        for directory in &self.directories {
            if let Some(file) = directory.find_file(file_name) {
                return Some(file);
            }
        }
        None
    }
}
// Trait with commands for all directory structs.
trait DirTrait {
    fn mkdir(&mut self, dir: String);
    fn rmdir(&mut self, dir: String) -> Result<(), Option<String>>;
    fn cd(&mut self, dir: String) -> Option<&Directory>;
    fn cat(&mut self, file_name: String, content: Option<String>) -> Option<()>;
    fn add_file(&mut self, file_name: String, content: Vec<u8>) -> Option<()>;
    fn find_file(&self, file_name: &str) -> Option<&File>;
}

// Testing the directory.
#[test]
fn test_directory(){
    let mut home = Home::new();
    let dir_name = String::from("ori's_feetpics_directory");
    // Unit testing for directory commands.
    home.mkdir(dir_name.clone());
    assert!(home.rmdir(dir_name.clone()).is_ok());
    assert!(home.rmdir(dir_name.clone()).is_err());
    home.cd(dir_name);
    println!("{}", home.pwd());
    
}
#[test]
fn test_cat() {
    let mut home = Home::new();
    let file_name = String::from("avivs.txt");
    let dir_name = String::from("digma");
    let content = None;
    home.mkdir(dir_name.clone());
    home.cd(dir_name);
    home.cat(file_name.clone(), Some(String::from("hello, world")));
    println!("{}", file_name);
    home.cat(file_name, content);
    println!("{}", home.pwd());
    
}
