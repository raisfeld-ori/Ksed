use serde::{Serialize, Deserialize};

pub static mut FS: Home = Home::new();

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

impl Directory{
    pub fn new(name: String) -> Self{Self{name, directories: Box::new(Vec::new()), files: Vec::new()}}
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Home{
    pub directories: Vec<Directory>,
    pub files: Vec<File>,
}

impl Home{
    pub const fn new() -> Self{Self { directories: Vec::new(), files: Vec::new() }}
    pub fn name(&self) -> String {String::from("Home")}
    pub fn mkdir(&mut self, dir: String) {self.directories.push(Directory::new(dir))}
}