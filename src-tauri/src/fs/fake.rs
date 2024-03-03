use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct File{
    pub name: String,
    pub extension: String,
    data: Vec<u8>,
}

pub struct Directory{

}