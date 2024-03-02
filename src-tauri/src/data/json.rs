use serde_json::{Map, Value};
use once_cell::sync::Lazy;


pub static mut USER_DATA: Lazy<Map<String, Value>> = Lazy::new(|| Map::new());

pub fn init_user_data() -> Result<(), String> {
    unsafe {USER_DATA = Lazy::new(|| {
        let mut map = Map::new();
        map.insert(String::from("user"), Value::Object(Map::new()));
        map.insert(String::from("system"), Value::Object(Map::new()));
        
        return map;
    })}
    Ok(())
}

// this is a bad function that takes a lot of ram, since
// there is no way to return the user without cloning it.
// i should fix it, but i am working a short time frame and this would take a while to fix
fn user_clone() -> Map<String, Value>{
    let usr = unsafe {USER_DATA.get("user").expect("USER_DATA WAS NOT INITIALIZED")};
    match usr{
        Value::Object(obj) => {obj.clone()}
        _ => {panic!(r#""user" is not an object, even though it should be"#)}
    }
}

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
pub fn user_make<'a>(key: String, value: Value){
    let mut user = user_clone();
    user.insert(key, value);
    unsafe {USER_DATA.insert(String::from("user"), Value::Object(user));}
}

#[test]
fn test_user_data(){
    init_user_data().expect("FAILED TO INITIALIZE THE USER DATA");
    user_make(String::from("test"), Value::String(String::from("test")));
    assert_eq!(user_get(String::from("test")), &Value::String(String::from("test")));
}