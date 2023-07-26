// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn compress(source_path: String, target_path:String) -> Result<String, ()> {
    // format!("Source: {}\nTarget: {}", source_path, target_path);
    Ok("Compression finished".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compress])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
