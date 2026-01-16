mod browser;
use once_cell::sync::Lazy;
use std::sync::RwLock;

static INSTALLED_BROWSERS: Lazy<RwLock<Vec<browser::RawBrowser>>> =
    Lazy::new(|| RwLock::new(Vec::new()));
static CURRENT_CDP_BROWSER: Lazy<RwLock<Option<browser::CdpBrowserInfo>>> =
    Lazy::new(|| RwLock::new(None));

#[tauri::command]
async fn refresh_installed_browsers() -> Result<(), String> {
    let browsers = tauri::async_runtime::spawn_blocking(|| browser::installed_browsers())
        .await
        .map_err(|e| format!("spawn failed: {e}"))?;

    let mut lock = INSTALLED_BROWSERS.write().unwrap_or_else(|e| e.into_inner());
    *lock = browsers;
    Ok(())
}

#[tauri::command]
fn get_installed_browsers() -> Result<Vec<browser::RawBrowser>, String> {
    let lock = INSTALLED_BROWSERS.read().unwrap_or_else(|e| e.into_inner());
    Ok(lock.clone())
}


#[tauri::command]
async fn refresh_current_cdp_browser() {
    let cdp_browser = browser::check_current_cdp_browser();
    let mut lock = CURRENT_CDP_BROWSER.write().unwrap_or_else(|e| e.into_inner());
    *lock = cdp_browser;
}

#[tauri::command]
fn get_current_cdp_browser() -> Option<browser::CdpBrowserInfo> {
    let lock = CURRENT_CDP_BROWSER.read().unwrap_or_else(|e| e.into_inner());
    lock.clone()
}

#[tauri::command]
async fn launch_browser_by_name(
    browser_name: String,
    profile_name: String,
    dry_run: bool,
    remote_debug_port: Option<u16>,
    remote_allow_origins: Option<String>,
) -> Result<(), String> {
    if let Some(port) = remote_debug_port {
        if port == 0 || port < 1024 {
            return Err("remote_debug_port must be >= 1024".into());
        }
    }
    let maybe = {
        let lock = INSTALLED_BROWSERS.read().unwrap_or_else(|e| e.into_inner());
        lock.iter().find(|b| b.name == browser_name).cloned()
    };

    let browser = maybe.ok_or_else(|| format!("Browser '{}' not found", browser_name))?;

    tauri::async_runtime::spawn_blocking(move || {
        browser::launch_browser(
            &browser,
            &profile_name,
            dry_run,
            remote_debug_port,
            remote_allow_origins.as_deref(),
        )
    })
    .await
    .map_err(|e| format!("spawn failed: {e}"))?
    .map_err(|e| format!("launch failed: {e}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            refresh_installed_browsers,
            get_installed_browsers,
            refresh_current_cdp_browser,
            get_current_cdp_browser,
            launch_browser_by_name
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
