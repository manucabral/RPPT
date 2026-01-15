use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug, Serialize)]
pub struct RawBrowser {
    pub name: String,
    pub path: Option<String>,
}

fn _clean_path(raw_path: &str) -> Option<String> {
    let trimmed = raw_path.trim();
    if trimmed.is_empty() {
        return None;
    }
    let unquoted = trimmed.trim_matches('"').trim_matches('\'');
    let expanded = shellexpand::env(unquoted).ok()?;
    let path = Path::new(expanded.as_ref());
    if path.exists() {
        Some(path.to_string_lossy().to_string())
    } else {
        Some(unquoted.to_string())
    }
}

fn _extract_path_from_cmd(cmd: &str) -> Option<String> {
    if cmd.contains('"') {
        let parts: Vec<&str> = cmd.split('"').collect();
        if parts.len() >= 2 {
            return _clean_path(parts[1]);
        }
    }
    if let Some(first_token) = cmd.split_whitespace().next() {
        return _clean_path(first_token);
    }
    None
}

fn _read_smi() -> Vec<RawBrowser> {
    let mut results: Vec<RawBrowser> = Vec::new();

    let roots = vec![
        (HKEY_CURRENT_USER, r"SOFTWARE\Clients\StartMenuInternet"),
        (HKEY_LOCAL_MACHINE, r"SOFTWARE\Clients\StartMenuInternet"),
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\WOW6432Node\Clients\StartMenuInternet",
        ),
    ];

    for (hive, base) in roots {
        let key = RegKey::predef(hive).open_subkey(base);
        if let Err(_) = key {
            continue;
        }
        let key = key.unwrap();

        for subkey in key.enum_keys().filter_map(Result::ok) {
            let sk_path = format!(r"{}\{}", base, subkey);
            let sk = RegKey::predef(hive).open_subkey(&sk_path);
            if let Err(_) = sk {
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

            // appname
            if app_name.is_none() {
                app_name = sk.get_value::<String, &str>("").ok();
            }
            let app_name = app_name.unwrap_or_else(|| subkey.clone());

            // shell\open\command
            let mut path: Option<String> = None;
            let cmd_path = format!(r"{}\shell\open\command", sk_path);
            if let Ok(cmdkey) = RegKey::predef(hive).open_subkey(&cmd_path) {
                if let Ok(cmd) = cmdkey.get_value::<String, &str>("") {
                    path = _extract_path_from_cmd(&cmd);
                }
            }
            results.push(RawBrowser {
                name: app_name,
                path,
            });
        }
    }
    results
}

fn _is_executable(path: &str) -> bool {
    let temp_path = Path::new(path);
    temp_path.exists() 
        && temp_path.is_file() 
        && path.to_lowercase().ends_with(".exe")
}

fn _is_browser_name(name: &str) -> bool {
    let lower = name.to_lowercase();
    let browser_keywords = [
        "chrome", "firefox", "edge", "opera", "brave", "vivaldi", "tor", "yandex", "chromium",
    ];
    browser_keywords.iter().any(|&kw| lower.contains(kw))
}

pub fn installed_browsers() -> Vec<RawBrowser> {
    let raw_browsers = _read_smi();
    let mut seen: HashMap<String, RawBrowser> = HashMap::new();
    for mut browser in raw_browsers {
        let name = browser.name.trim().to_string();
        let path = browser.path.as_deref().and_then(_clean_path);
        browser.path = path.clone();
        let keep = if let Some(ref temp_path) = path {
            _is_executable(temp_path)
        } else {
            _is_browser_name(&name)
        };
        if !keep {
            continue;
        }
        let dedupe_key = if let Some(ref temp_path) = path {
             Path::new(temp_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&name)
                .to_lowercase()
        } else {
            name.to_lowercase()
        };
        if let Some(existing) = seen.get(&dedupe_key) {
            let ex_path = existing.path.as_deref();
            let cur_has = path.as_deref().map_or(false, _is_executable);
            let ex_has = ex_path.map_or(false, _is_executable);
            if cur_has && !ex_has {
                seen.insert(dedupe_key, browser);
            } 
            else if cur_has == ex_has {
                if let (Some(ref p), Some(ref ex_p)) = (&path, ex_path) {
                    if p.len() > ex_p.len() {
                        seen.insert(dedupe_key, browser);
                    }
                }
            }
        } else {
            seen.insert(dedupe_key, browser);
        }
    }
    seen.into_values().collect()
}
