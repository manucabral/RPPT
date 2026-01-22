use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug, Serialize, Clone)]
pub struct RawBrowser {
    pub name: String,
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CdpBrowserInfo {
    pub name: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub web_socket_debugger_url: Option<String>,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: Option<String>,
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
    pub raw: String,
}

#[derive(Deserialize)]
struct CdpVersionResponse {
    #[serde(rename = "User-Agent", alias = "userAgent")]
    user_agent: Option<String>,
    #[serde(rename = "Browser")]
    browser: Option<String>,
    #[serde(rename = "Protocol-Version", alias = "protocolVersion")]
    protocol_version: Option<String>,
    #[serde(rename = "webSocketDebuggerUrl")]
    web_socket_debugger_url: Option<String>,
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


fn _make_profile_dir(browser_path: &str, profile_name: &str) -> Result<String, String> {
    let exe_name = Path::new(browser_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("browser")
        .to_lowercase();

    let profile_dir = Path::new("profiles")
        .join(exe_name)
        .join(profile_name);

    std::fs::create_dir_all(&profile_dir)
        .map_err(|e| format!("Failed to create profile directory: {}", e))?;

    Ok(profile_dir.to_string_lossy().to_string())
}

fn _build_launch_command(exec_path: &str, profile_dir: &str) -> Vec<String> {
    let exe = Path::new(exec_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();

    if exe.contains("firefox") {
        vec![
            exec_path.to_string(),
            "-profile".to_string(),
            profile_dir.to_string(),
        ]
    } else {
        vec![
            exec_path.to_string(),
            format!("--user-data-dir={}", profile_dir),
        ]
    }
}

fn _add_remote_debug_flag(mut cmd: Vec<String>, exec_path: &str, port: u16) -> Vec<String> {
    if port == 0 {
        return cmd;
    }

    let exe = Path::new(exec_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();

    if exe.contains("firefox") {
        cmd.push("-start-debugger-server".to_string());
        cmd.push(port.to_string());
    } else {
        cmd.push(format!("--remote-debugging-port={}", port));
    }

    cmd
}

pub fn launch_browser(
    browser: &RawBrowser,
    profile_name: &str,
    dry_run: bool,
    remote_debug_port: Option<u16>,
    remote_allow_origins: Option<&str>,
) -> Result<(), String> {
    let path = browser.path.as_ref()
        .ok_or("Selected browser has no executable path.")?;

    if !Path::new(path).is_file() {
        return Err(format!("Executable not found: {}", path));
    }

    let profile_dir = _make_profile_dir(path, profile_name)?;
    let mut cmd = _build_launch_command(path, &profile_dir);

    if let Some(port) = remote_debug_port {
        cmd = _add_remote_debug_flag(cmd, path, port);
        
        let exe = Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        if !exe.contains("firefox") {
            let origins = remote_allow_origins
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("http://localhost:{}", port));
            cmd.push(format!("--remote-allow-origins={}", origins));
        }
    }

    println!("Launching {} with profile '{}':", browser.name, profile_name);
    println!("{}", cmd.join(" "));

    if dry_run {
        return Ok(());
    }

    Command::new(&cmd[0])
        .args(&cmd[1..])
        .spawn()
        .map_err(|e| format!("Failed to launch browser: {}", e))?
        .wait()
        .map_err(|e| format!("Failed to wait on browser process: {}", e))?;
    Ok(())
}

fn _extract_version(text: &str, token: &str) -> Option<String> {
    if text.is_empty() || !text.contains(token) {
        return None;
    }
    
    let idx = text.find(token)?;
    let mut tail = &text[idx + token.len()..];
    
    if tail.starts_with('/') {
        tail = &tail[1..];
    }
    
    for sep in [" ", ";", ")", "("] {
        if let Some(pos) = tail.find(sep) {
            tail = &tail[..pos];
            break;
        }
    }
    
    let result = tail.trim();
    if result.is_empty() {
        None
    } else {
        Some(result.to_string())
    }
}

pub async fn check_current_cdp_browser() -> Option<CdpBrowserInfo> {
    let url = "http://localhost:4969/json/version";

    let resp = reqwest::get(url).await.map_err(|err| {
        println!("Failed to fetch CDP version info: {}", err);
        err
    }).ok()?;

    if !resp.status().is_success() {
        return None;
    }
    let body_str = resp.bytes().await.map_err(|err| {
        println!("Failed to read response body: {}", err);
        err
    }).ok()?;
    
    let body_str = String::from_utf8(body_str.to_vec()).ok()?;
    let data: Value = serde_json::from_str(&body_str).ok()?;
    let cdp: CdpVersionResponse = serde_json::from_value(data.clone()).ok()?;
    
    let ua = cdp.user_agent.as_deref().unwrap_or("");
    let browser_hdr = cdp.browser.as_deref().unwrap_or("");
    
    let mut name: Option<String> = None;
    let mut version: Option<String> = None;
    
    // Check special browsers first
    let specials = [
        ("YaBrowser", "Yandex"),
        ("Edg", "Edge"),
        ("OPR", "Opera"),
        ("Opera", "Opera"),
        ("Brave", "Brave"),
        ("Vivaldi", "Vivaldi"),
        ("TorBrowser", "Tor"),
        ("Firefox", "Firefox"),
    ];
    
    for (token, label) in specials {
        if ua.contains(token) {
            name = Some(label.to_string());
            version = _extract_version(ua, token)
                .or_else(|| _extract_version(browser_hdr, token));
            break;
        }
    }
    
    if name.is_none() && !browser_hdr.is_empty() {
        if browser_hdr.contains("Chrome") && ua.contains("YaBrowser") {
            name = Some("Yandex".to_string());
            version = _extract_version(ua, "YaBrowser")
                .or_else(|| _extract_version(browser_hdr, "Chrome"));
        } else if browser_hdr.contains("Chrome") {
            name = Some("Chrome".to_string());
            version = _extract_version(browser_hdr, "Chrome");
        } else if browser_hdr.contains("Chromium") {
            name = Some("Chromium".to_string());
            version = _extract_version(browser_hdr, "Chromium");
        } else if browser_hdr.contains("Firefox") {
            name = Some("Firefox".to_string());
            version = _extract_version(browser_hdr, "Firefox");
        }
    }

    if name.is_none() && !ua.is_empty() {
        if ua.contains("Chrome") {
            name = Some("Chrome".to_string());
            version = _extract_version(ua, "Chrome");
        } else if ua.contains("Safari") && !ua.contains("Chrome") {
            name = Some("Safari".to_string());
        }
    }
    
    Some(CdpBrowserInfo {
        name,
        version,
        web_socket_debugger_url: cdp.web_socket_debugger_url,
        protocol_version: cdp.protocol_version,
        user_agent: cdp.user_agent,
        raw: serde_json::to_string(&data).unwrap_or_default(),
    })
}