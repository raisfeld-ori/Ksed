use serde::Serialize;
use serde_json::{Map, Value, to_vec};
use once_cell::sync::Lazy;


pub static mut USER_DATA: Lazy<Map<String, Value>> = Lazy::new(|| Map::new());

pub fn init_user_data() {
    unsafe {USER_DATA = Lazy::new(|| {
        let mut map = Map::new();
        map.insert(String::from("user"), Value::Object(Map::new()));
        map.insert(String::from("system"), Value::Object(Map::new()));
        
        return map;
    })}
}

fn user<'a>() -> &'a Map<String, Value>{unsafe {USER_DATA.get("user").expect("USER_DATA WAS NOT INITIALIZED")}.as_object().expect("user was not an object")}
fn system<'a>() -> &'a Map<String, Value>{unsafe {USER_DATA.get("system").expect("USER_DATA WAS NOT INITIALIZED")}.as_object().expect("user was not an object")}


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
    assert_eq!(user_get(String::from("test")), &Value::String(String::from("test")));
}