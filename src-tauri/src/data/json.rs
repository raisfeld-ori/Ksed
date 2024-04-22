use serde::Serialize;
use serde_json::{Map, Number, Value};
use once_cell::sync::Lazy;

pub static mut USER_DATA: Lazy<Map<String, Value>> = Lazy::new(|| Map::new());

pub fn init_user_data() {
    unsafe {USER_DATA = Lazy::new(|| {
        let mut map = Map::new();
        map.insert(String::from("user"), Value::Object(Map::new()));
        let mut system_data = Map::new();
        system_data.insert(String::from("types"), Value::Object(Map::new()));
        map.insert(String::from("system"), Value::Object(system_data));
        
        return map;
    })}
}

pub fn set_data(user: Map<String, Value>, system: Map<String, Value>){
    unsafe {
        USER_DATA.insert(String::from("user"), Value::Object(user));
        USER_DATA.insert(String::from("system"), Value::Object(system));
    }
}

pub fn user<'a>() -> &'a Map<String, Value>{unsafe {USER_DATA.get("user").expect("USER_DATA WAS NOT INITIALIZED")}.as_object().expect("user was not an object")}
pub fn system<'a>() -> &'a Map<String, Value>{unsafe {USER_DATA.get("system").expect("USER_DATA WAS NOT INITIALIZED")}.as_object().expect("user was not an object")}
pub fn user_mut<'a>() -> &'a mut Map<String, Value> {unsafe {USER_DATA.get_mut("user").expect("USER_DATA WAS NOT INITIALIZED")}.as_object_mut().expect("user was not an object")}
pub fn system_mut<'a>() -> &'a mut Map<String, Value>{unsafe {USER_DATA.get_mut("system").expect("USER_DATA WAS NOT INITIALIZED")}.as_object_mut().expect("user was not an object")}


#[tauri::command]
pub fn user_get<'a>(key: String) -> &'a Value {
    return unsafe { 
        USER_DATA
        .get("user")
        .expect("USER_DATA was not initialized")
        .get(key)
        .unwrap_or(&Value::Null)
    };
}
#[tauri::command]
pub fn system_get<'a>(key: String) -> &'a Value  {
    return unsafe {
        USER_DATA
        .get("system")
        .expect("USER_DATA was not initialized")
        .get(key)
        .unwrap_or(&Value::Null)
    };
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn create_value(valType: String, val: String) -> Value {
    let result = match valType.to_ascii_lowercase().as_str(){
        "string" => {Value::String(val)},
        "int" => {Value::Number(val.parse().unwrap_or(Number::from(0)))},
        "null" => {Value::Null},
        _ => {Value::Null}
    };

    return result;
}

#[tauri::command]
pub fn user_make(key: String, data: Value) {user_mut().insert(key, data);}

#[tauri::command]
pub fn system_make(key: String, val: Value) {system_mut().insert(key, val);}

#[tauri::command]
pub fn gather_type(file: &str) -> &str {
    let file_extension = file.split(".").last();
    if file_extension.is_none(){return "Unknown";}
    match file_extension.unwrap(){
        "png" => {return "Image"}
        "txt" => {return "Text"}
        "html" => {return "HTML"}
        "mp4" => {return "Video"}
        "mov" => {return "Video"}
        "mp3" => {return "Audio"}
        _ => {return "Unknown"}
    }
}

#[derive(Serialize)]
struct SerializeUserData{
    user: Map<String, Value>,
    system: Map<String, Value>
}

pub fn data_bytes() -> (Vec<u8>, Vec<u8>){
    (
        serde_json::to_vec(user()).expect("failed to turn user into bytes"), 
        serde_json::to_vec(system()).expect("failed to turn user into bytes")
    )
}

#[test]
fn test_user_data(){
    init_user_data();
    let data = "example data";
    let data = create_value(String::from("string"), String::from(data));
    user_make(String::from("test"), data.clone());
    assert_eq!(&data, user_get(String::from("test")));
}

#[test]
fn test_file_extention(){
    init_user_data();
    assert_eq!(gather_type("file.png"), String::from("Image"));
    assert_eq!(gather_type("new_file.cool.mov"), String::from("Video"))
}