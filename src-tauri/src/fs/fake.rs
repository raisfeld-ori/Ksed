
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
// impl for Directory with function that only Directory have.
impl Directory {
    fn new(name: String) -> Self{Self{name, directories: Box::new(Vec::new()), files: Vec::new()}}

}
// impl the directory trait for Directory struct because he needs these commands.
impl DirTrait for Directory{
    fn mkdir(&mut self, dir: String) {self.directories.push(Directory::new(dir))}

    fn rmdir(&mut self, dir: String) -> Result<(), Option<String>> {
        let index = self.directories.iter().position(|d| d.name == dir);
        match index {
            Some(idx) => {
                self.directories.remove(idx);
                Ok(())
            },
            None => Err(Some(format!("Directory '{}' not found", dir)))
        }
    }
    fn cd(&mut self, dir: String) -> Result<&Directory, String> {
        for directory in &mut self.directories.iter() {
            if directory.name == dir {
                return Ok(directory);
            }
        }
        Err(format!("Directory '{}' not found", dir))
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Home{
    pub directories: Vec<Directory>,
    pub files: Vec<File>,
    pub path: String,
}
// impl for home with functions that only Home have.
impl Home{
    pub const fn new() -> Self{Self { directories: Vec::new(), files: Vec::new(), path: String::new()}}
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
    fn cd(&mut self, dir: String) -> Result<&Directory, String> {
        for directory in &mut self.directories.iter() {
            if directory.name == dir{
                Some(directory);
                self.path += "/";
                self.path += directory.name.as_str();
                return Ok(directory);
            }
        }
        Err(format!("Directory '{}' not found", dir))
    }
}
// Trait with commands for all directory structs.
trait DirTrait {
    fn mkdir(&mut self, dir: String);
    fn rmdir(&mut self, dir: String) -> Result<(), Option<String>>;
    fn cd(&mut self, dir: String) -> Result<&Directory, String>;
        
}

// Testing the directory.
#[test]
fn test_directory(){
    let mut home = Home::new();
    let dir_name = String::from("ori's_secret_directory");
    // Unit testing for directory commands.
    home.mkdir(dir_name.clone());
    assert!(home.rmdir(dir_name.clone()).is_ok());
    assert!(home.rmdir(dir_name.clone()).is_err());
    match home.cd(dir_name) {
        Ok(dir) => println!("Successfully changed to directory: {}", dir.name),
        Err(e) => println!("Error {}", e)
    }
    println!("{}", home.pwd());
    
}