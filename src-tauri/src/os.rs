

#[tauri::command]
pub fn get_platform() -> String {
  if cfg!(target_os = "windows") {
    "win32".to_string()
  } else if cfg!(target_os = "macos") {
    "darwin".to_string()
  } else if cfg!(target_os = "linux") {
    "linux".to_string()
  } else {
    "unknown".to_string()
  }
}
