use tauri_build::WindowsAttributes;

fn main() {
    let win_attr = WindowsAttributes::new();
    let win_attr = win_attr.app_manifest(
r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
 <assemblyIdentity version="1.0.0.0" name="MyApplication.app"/>
 <trustInfo xmlns="urn:schemas-microsoft-com:asm.v2">
    <security>
      <requestedPrivileges xmlns="urn:schemas-microsoft-com:asm.v3">
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
      </requestedPrivileges>
    </security>
 </trustInfo>
</assembly>
"#
    );
    let attrs =  tauri_build::Attributes::new().windows_attributes(win_attr);
    tauri_build::try_build(attrs).expect("error in compilation, the windows manifest was wrong");
}
