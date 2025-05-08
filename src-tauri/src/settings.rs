use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{AppHandle};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    github_username: Option<String>,
    developer_portal_repo_path: Option<String>,
}

const SETTINGS_PATH: &str = ".espressif/esp-workbench.json";

#[tauri::command]
pub async fn load_settings(app: AppHandle) -> Result<Settings, String> {
    let path = dirs::home_dir().unwrap().join(SETTINGS_PATH);
    if path.exists() {
        let settings = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let settings: Settings = serde_json::from_str(&settings).map_err(|e| e.to_string())?;
        Ok(settings)
    } else {
        Ok(Settings { github_username: None, developer_portal_repo_path: None })
    }
}

#[tauri::command]
pub async fn save_settings(github_username: Option<String>, developer_portal_repo_path: Option<String>, app: AppHandle) -> Result<(), String> {
    let path = dirs::home_dir().unwrap().join(SETTINGS_PATH);
    let settings = Settings {
        github_username,
        developer_portal_repo_path,
    };
    fs::write(path, serde_json::to_string(&settings).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    Ok(())
}
