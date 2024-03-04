use serde::{Serialize, Deserialize};

pub static mut FS: Home = Home::new();

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct File{
    pub name: String,
    pub extension: String,
    data: Vec<u8>,
}

pub struct Directory{
    pub name: String,
    pub directories: Box<Vec<Directory>>,
    pub files: Vec<File>,
}

pub struct Home{
    pub directories: Vec<Directory>,
    pub files: Vec<File>,
}

impl Home{
    pub const fn new() -> Self{Self { directories: Vec::new(), files: Vec::new() }}
    pub fn name(&self) -> String {String::from("Home")}
}