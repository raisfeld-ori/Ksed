

fn main() {
  let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or("unknown OS".to_string());
  match target_os.as_str() {
    "windows" => {
    let mut res = tauri_winres::WindowsResource::new();
    res.set_icon("./icons/icon.ico");
    res.compile().unwrap();
    },
    "linux" => {
        println!("Hello, Root-World!");
    }
    _ => {},
  }
  let resource_file = "d_vault/src-tauri/resource.rc";
  let macros = vec![
        "MY_MACRO",
        "MY_OTHER_MACRO=VALUE",
    ];
    tauri_build::build();
    embed_resource::compile(resource_file, macros)
    
}

