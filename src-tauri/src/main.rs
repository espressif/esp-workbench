// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

mod app_state;
use app_state::{AppState, BuilderState};

mod download;

mod console;
use console::setup_logging;
mod esp_idf;
use esp_idf::run_install_script;
mod external_command;
mod flasher;
mod monitor;
mod os;
use os::get_platform;
mod rust;
use rust::{check_rust_support, install_rust_support};

mod zip_archiver;
use zip_archiver::{unzip, zip_dir};

use tauri::{State, Window};

use serialport::available_ports;
use sysinfo::{DiskExt, System, SystemExt};

mod commands;
mod models;

use commands::*;
use models::*;

// Create a custom Error that we can return in Results
#[derive(Debug, thiserror::Error)]
enum Error {
    // Implement std::io::Error for our Error enum
    #[error(transparent)]
    Io(#[from] std::io::Error),
    // Add a PoisonError, but we implement it manually later
    // #[error("the mutex was poisoned")]
    // PoisonError(String),
}

// Command to compress directories into an archive file.
#[tauri::command]
async fn compress(
    window: Window,
    app: tauri::AppHandle,
    state_mutex: State<'_, Mutex<AppState>>,
    source_path: String,
    target_path: String,
) -> Result<String, ()> {
    let method = zip::CompressionMethod::Deflated;

    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Running;
    }

    let result = zip_dir(
        window,
        app.clone(),
        source_path.as_str(),
        target_path.as_str(),
        method,
    );
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Idle;
    }

    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(_) => Err(()),
    }
}

// Command to decompress an archive file into a directory.
#[tauri::command]
async fn decompress(
    window: Window,
    app: tauri::AppHandle,
    state_mutex: State<'_, Mutex<AppState>>,
    source_path: String,
    target_path: String,
) -> Result<String, ()> {
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Running;
    }

    let result = unzip(window, app.clone(), source_path, target_path);
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Idle;
    }

    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(_) => Err(()),
    }
}

// Command to run install shell script of ESP-IDF
#[tauri::command]
async fn run_esp_idf_install_script(
    window: Window,
    app: tauri::AppHandle,
    state_mutex: State<'_, Mutex<AppState>>,
    target_path: String,
) -> Result<String, ()> {
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Running;
    }

    let result = run_install_script(window, app.clone(), target_path);
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Idle;
    }

    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(_) => Err(()),
    }
}

// Command to download ESP-IDF to ZIP file
#[tauri::command]
async fn download_esp_idf(
    window: Window,
    app: tauri::AppHandle,
    state_mutex: State<'_, Mutex<AppState>>,
    version: String,
    target_path: String,
) -> Result<String, ()> {
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Running;
    }

    let download_handle =
        tokio::spawn(esp_idf::download_esp_idf(window, app, version, target_path));

    let result = download_handle.await;

    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Idle;
    }

    match result {
        Ok(result) => match result {
            Ok(_) => Ok("Download finished successfully".to_string()),
            Err(_) => Ok("Download failed".to_string()),
        },
        Err(_) => Ok("Download task panicked".to_string().to_string()),
    }
}

// Command to get list of ESP-IDF stored in users home directory as array of strings
#[tauri::command]
async fn get_esp_idf_list() -> Result<Vec<String>, ()> {
    let mut esp_idf_list: Vec<String> = Vec::new();

    let tools_dir = get_esp_idf_tools_dir().await.unwrap();
    let path = format!("{}/{}", tools_dir, "esp-idf");
    let paths = std::fs::read_dir(path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap().to_string();
        esp_idf_list.push(path);
    }

    Ok(esp_idf_list)
}

const GITHUB_REPOSITORY: &str = "espressif/esp-idf";

#[tauri::command]
async fn get_available_idf_versions() -> Result<String, String> {
    let url = "https://dl.espressif.com/dl/esp-idf/idf_versions.js".to_string();
    let client = reqwest::Client::builder()
        .user_agent("esp-workbench")
        .build()
        .map_err(|err| format!("Failed to create reqwest client: {}", err))?;
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|err| format!("Failed to make request: {}", err))?;
    let js_versions_file = response
        .text()
        .await
        .map_err(|err| format!("Failed to read response: {}", err))?;
    // Find the start and end index of the object within the JavaScript file
    let object_start = js_versions_file.find("{").ok_or("Object start not found")?;
    let object_end = js_versions_file.rfind("}").ok_or("Object end not found")? + 1;
    Ok(js_versions_file[object_start..object_end].to_string())
}

// Command to get the current user home
#[tauri::command]
async fn get_user_home() -> Result<String, ()> {
    match dirs::home_dir() {
        Some(path) => Ok(path.to_str().unwrap().to_string()),
        None => Err(()),
    }
}

// Command to get ESP-IDF Tools directory which is specific for each operating system.
#[tauri::command]
async fn get_esp_idf_tools_dir() -> Result<String, ()> {
    #[cfg(unix)]
    match dirs::home_dir() {
        Some(path) => Ok(format!("{}/{}", path.to_str().unwrap(), ".espressif")),
        None => Err(()),
    }

    #[cfg(windows)]
    Ok("C:\\Espressif".to_string())
}

use crate::monitor::monitor_port;

#[tauri::command]
async fn start_monitor(
    window: Window,
    app: tauri::AppHandle,
    state_mutex: State<'_, Mutex<AppState>>,
    port: String,
) -> Result<String, ()> {
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Running;
    }

    let monitor_handle = tokio::spawn(monitor_port(window, app, port));

    let result = monitor_handle.await;

    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Idle;
    }

    match result {
        Ok(result) => match result {
            Ok(_) => Ok("Monitoring finished successfully".to_string()),
            Err(_) => Ok("Monitoring failed".to_string()),
        },
        Err(_) => Ok("Monitoring task panicked".to_string().to_string()),
    }
}

#[tauri::command]
async fn stop_monitor(state_mutex: State<'_, Mutex<AppState>>) -> Result<String, ()> {
    let mut state = state_mutex.lock().unwrap();
    state.builder = BuilderState::Abort;
    Ok("ok".to_string())
}

// async fn monitor_port(window: Window, app: tauri::AppHandle, port: String) -> Result<(), ()> {
//   let state_mutex = app.get_state::<Mutex<AppState>>().unwrap();
//   let state = state_mutex.lock().await;

//   // code to open the port and monitor it, dispatching events to frontend...
//   // check state.monitor occasionally to see if we need to stop monitoring...
// }

#[tauri::command]
async fn start_flash(
    window: Window,
    app: tauri::AppHandle,
    state_mutex: State<'_, Mutex<AppState>>,
    port: String,
    file_path: String,
    flash_offset: u32,
) -> Result<String, ()> {
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Running;
    }

    let flasher_handle = tokio::spawn(flasher::flash_file(
        window,
        app,
        port,
        file_path,
        flash_offset,
    ));

    let result = flasher_handle.await;

    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Idle;
    }

    match result {
        Ok(result) => match result {
            Ok(_) => Ok("Flashing finished successfully".to_string()),
            Err(_) => Ok("flashing failed".to_string()),
        },
        Err(_) => Ok("Flashing task panicked".to_string().to_string()),
    }
}

#[tauri::command]
async fn stop_flash(state_mutex: State<'_, Mutex<AppState>>) -> Result<String, ()> {
    let mut state = state_mutex.lock().unwrap();
    state.builder = BuilderState::Abort;
    Ok("ok".to_string())
}

#[tauri::command]
async fn get_disk_usage() -> Result<Vec<String>, ()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let disk_info: Vec<String> = sys
        .disks()
        .iter()
        .map(|disk| {
            // convert from bytes to GB
            let capacity_gb = disk.available_space() / 1_000_000_000;
            format!("{} GB", capacity_gb)
        })
        .collect();

    Ok(disk_info)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ConnectedPort {
    port_name: String,
    product: String,
    pid: u16,
    vid: u16,
}

#[tauri::command]
async fn get_connected_serial_devices() -> Vec<ConnectedPort> {
    let mut esp32s = vec![];
    if let Ok(ports) = available_ports() {
        for p in ports {
            if let serialport::SerialPortType::UsbPort(info) = p.port_type {
                // if info.manufacturer.is_some() && (info.vid == 4292 || info.vid == 1027) {
                // 4292 = 0x10C4 (Silabs CP210x)
                // 1027 = 0x0403 (FTDI)
                esp32s.push(ConnectedPort {
                    port_name: p.port_name,
                    product: info.product.unwrap_or("".to_string()),
                    pid: info.pid,
                    vid: info.vid,
                });
                // }
            }
        }
    }
    esp32s
}

#[tauri::command]
async fn execute_command(command: String) -> Result<String, String> {
    use std::process::Command;

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &command])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
    };

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            if output.status.success() {
                Ok(stdout)
            } else {
                Err(stderr)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn check_devportal() -> Result<bool, String> {
  // Implement the logic to check if the devportal directory exists
  // Example: Check if ~/.espressif/devportal exists
  Ok(std::path::Path::new(&dirs::home_dir().unwrap().join(".espressif/devportal")).exists())
}



fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            compress,
            decompress,
            delete_author,
            download_esp_idf,
            execute_command,
            get_connected_serial_devices,
            get_disk_usage,
            get_user_home,
            get_esp_idf_list,
            get_esp_idf_tools_dir,
            get_available_idf_versions,
            run_esp_idf_install_script,
            start_flash,
            stop_flash,
            start_monitor,
            stop_monitor,
            check_rust_support,
            install_rust_support,
            get_platform,
            check_devportal,
            get_authors,
            save_author,
            launch_hugo,
            restart_hugo,
        ])
        .setup(|app| {
            // Initialize the logging system
            setup_logging(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
