use tauri::Manager;
use winreg::enums::*;
use winreg::RegKey;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct RawBrowser {
    pub name: String,
    pub path: Option<String>,
}

pub fn _read_smi() -> () {
    // let mut _items: Vec<RawBrowser> = Vec::new();

    let roots = vec![
        (HKEY_CURRENT_USER, r"SOFTWARE\Clients\StartMenuInternet"),
        (HKEY_LOCAL_MACHINE, r"SOFTWARE\Clients\StartMenuInternet"),
        (HKEY_LOCAL_MACHINE, r"SOFTWARE\WOW6432Node\Clients\StartMenuInternet")
    ];

    for (hive, base) in roots {
        let key = RegKey::predef(hive).open_subkey(base);
        if let Err(err) = key {
            eprintln!("Failed to open registry key {}: {}", base, err);
            continue;
        }
        let key = key.unwrap();
        
        for subkey in key.enum_keys().filter_map(Result::ok) {
            println!("Found browser key: {}", subkey);

            let sk_path = format!(r"{}\{}", base, subkey);
            let sk = RegKey::predef(hive).open_subkey(&sk_path);
            if let Err(err) = sk {
                eprintln!("Failed to open subkey {}: {}", sk_path, err);
                continue;
            }

            let sk = sk.unwrap();
            let caps_path = format!(r"{}\Capabilities", sk_path);
            let caps = RegKey::predef(hive).open_subkey(&caps_path).ok();
            
            let mut app_name = if let Some(ref cap) = caps {
                cap.get_value::<String, &str>("ApplicationName").ok()
            } else {
                None
            };
            if app_name.is_none() {
                app_name = sk.get_value::<String, &str>("").ok();
            }

            

        }
    }
    
}
