use std::process::Command;

use tauri::{AppHandle, Window};

use external_command::run_external_command_with_progress;

use log::info;

use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::external_command;
use crate::external_command::set_exec_permission;
use crate::download::download_file;

#[cfg(windows)]
use std::os::windows::process::CommandExt;
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000; // Windows specific constant to hide console window

pub fn get_tool_version(command: &str, flags: &[&str], keyword: Option<&str>) -> Option<String> {
  let mut cmd = Command::new(command);
  for flag in flags {
      cmd.arg(flag);
  }

  #[cfg(windows)]
  cmd.creation_flags(CREATE_NO_WINDOW);

  let output = cmd.output().ok()?;

  if !output.status.success() {
      info!("command failed: {:?}", output);
      return None;
  }

  let stdout = String::from_utf8_lossy(&output.stdout);
  info!("stdout: {:?}", stdout);

  // Split by newline and take the first line.
  let binding = stdout.split('\n').collect::<Vec<&str>>();
  let line = binding.get(0)?;

  // If a keyword is provided, look for it in the line. If not found, return None.
  if let Some(keyword) = keyword {
      if !line.contains(keyword) {
          return None;
      }
  }

  // Extract just the version part for other cases.
  line.split_whitespace().nth(1).map(|s| s.to_string())
}


pub fn get_tool_version_xtensa(command: &str, flags: &[&str], keyword: Option<&str>) -> Option<String> {
  let mut cmd = Command::new(command);
  for flag in flags {
      cmd.arg(flag);
  }

  let output = cmd.output().ok()?;

  if !output.status.success() {
      info!("command failed: {:?}", output);
      return None;
  }

  let stdout = String::from_utf8_lossy(&output.stdout);
  info!("stdout: {:?}", stdout);

  // Split by newline and take the first line.
  let binding = stdout.split('\n').collect::<Vec<&str>>();
  let line = binding.get(0)?;

  // If a keyword is provided, look for it in the line. If not found, return None.
  if let Some(keyword) = keyword {
      if !line.contains(keyword) {
          return None;
      }
  }

  // Extract just the version part for other cases.
  line.split_whitespace().nth(4).map(|s| s.trim_matches(')').trim_matches('(').to_string())
}


#[derive(serde::Serialize)]
pub struct RustSupportResponse {
    xtensa: Option<String>,
    riscv: Option<String>,
    cargo: Option<String>,
}

#[tauri::command]
pub fn check_rust_support() -> Result<RustSupportResponse, String> {
  let cargo_version = get_tool_version("cargo", &["--version"], None);
  let riscv_version = get_tool_version("rustc", &["+nightly", "--version"], Some("rustc"));
  let xtensa_version = get_tool_version_xtensa("rustc", &["+esp", "--version"], Some("rustc"));


    info!("riscv: {:?}", riscv_version);
    Ok(RustSupportResponse {
      xtensa: xtensa_version,
      riscv: riscv_version,
      cargo: cargo_version,
  })
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RustInstallOptions {
    selected_variant: Option<String>,
    install_msvc: bool,
    install_mingw: bool,
}

#[tauri::command]
pub async fn install_rust_support(window: Window, app: AppHandle, install_options: RustInstallOptions) -> Result<String, String> {
    let selected_variant = install_options.selected_variant;
    #[cfg(target_os = "windows")]
    {
        if install_options.install_msvc {
            install_vc_tools_and_sdk(window.clone(), app.clone()).await?;
        }
    }

    download_rustup(window.clone(), app.clone()).await?;
    install_rustup(window.clone(), app.clone(), selected_variant.as_ref()).await?;
    install_espup(window.clone(), app.clone(), selected_variant.as_ref()).await?;
    install_rust_toolchain(window, app, selected_variant.as_ref()).await?;
    Ok("Success".into())
}

pub async fn install_rustup(window: Window, app: tauri::AppHandle, selected_variant: Option<&String>) -> Result<String, String> {

    #[cfg(windows)]
    let fname = "rustup-init.exe";
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let fname = "rustup-init.sh";

    let output_dir = dirs::home_dir().ok_or("Failed to get home directory")?.join(".cargo").join("bin");
    let rustup_path = output_dir.join(fname).to_str().unwrap().to_string();

    info!("Installing rustup...");

    #[cfg(target_os = "windows")]
    {
        let mut args = vec!["-y"];

        if let Some(variant) = selected_variant {
            args.push("--default-host");
            args.push(variant);
        }

        run_external_command_with_progress(window.clone(), app, &rustup_path, &args, "PROGRESS_EVENT").await;
    }

    #[cfg(unix)]
    {
        let args = vec!["-y"];
        run_external_command_with_progress(window.clone(), app, &rustup_path, &args, "PROGRESS_EVENT").await;
    }

    info!("Rustup installed or already present");
    Ok("Rustup installed or already present".into())
}

async fn download_rustup(window: Window, app: AppHandle) -> Result<String, String> {
    info!("Downloading rustup...");

    let url: &'static str;
    #[cfg(target_os = "windows")]
    {
        url = "https://win.rustup.rs/x86_64";
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        url = "https://sh.rustup.rs";
    }

    #[cfg(windows)]
    let fname = "rustup-init.exe";
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let fname = "rustup-init.sh";

    let output_dir = dirs::home_dir().ok_or("Failed to get home directory")?.join(".cargo").join("bin");
    let output_path = output_dir.join(fname);


    // Check if rustup is already installed
    match Command::new(output_path.clone()).arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                info!("Rustup already installed");
                return Ok("Rustup already installed".into());
            }
        },
        Err(_) => {}
    }

    // Use the download_file function to download the file
    download_file(window.clone(), app.clone(), url, &output_path).await.map_err(|e| format!("Failed to download rustup: {:?}", e))?;

    // Set execute permission for the binary on Unix-based systems
    #[cfg(unix)]
    set_exec_permission(&output_path).map_err(|e| format!("Failed to set execute permissions: {}", e))?;

    info!("rustup downloaded successfully!");

    Ok("rustup installed successfully!".into())
}


async fn install_espup(window: Window, app: AppHandle, selected_variant: Option<&String>) -> Result<String, String> {
    info!("Installing espup...");

    let url: &'static str;
    #[cfg(target_os = "linux")]
    #[cfg(target_arch = "aarch64")]
    {
        url = "https://github.com/esp-rs/espup/releases/latest/download/espup-aarch64-unknown-linux-gnu";
    }
    #[cfg(target_os = "linux")]
    #[cfg(target_arch = "x86_64")]
    {
        url = "https://github.com/esp-rs/espup/releases/latest/download/espup-x86_64-unknown-linux-gnu";
    }
    #[cfg(target_os = "macos")]
    #[cfg(target_arch = "aarch64")]
    {
        url = "https://github.com/esp-rs/espup/releases/latest/download/espup-aarch64-apple-darwin";
    }
    #[cfg(target_os = "macos")]
    #[cfg(target_arch = "x86_64")]
    {
        url = "https://github.com/esp-rs/espup/releases/latest/download/espup-x86_64-apple-darwin";
    }
    #[cfg(target_os = "windows")]
    {
        url = "https://github.com/esp-rs/espup/releases/latest/download/espup-x86_64-pc-windows-msvc.exe";
    }

    #[cfg(unix)]
    let fname = "espup";
    #[cfg(windows)]
    let fname = "espup.exe";

    let output_dir = dirs::home_dir().ok_or("Failed to get home directory")?.join(".cargo").join("bin");
    let output_path = output_dir.join(fname);

    // Use the download_file function to download the file
    download_file(window.clone(), app.clone(), url, &output_path).await.map_err(|e| format!("Failed to download espup: {:?}", e))?;

    // Set execute permission for the binary on Unix-based systems
    #[cfg(unix)]
    set_exec_permission(&output_path).map_err(|e| format!("Failed to set execute permissions: {}", e))?;

    info!("espup downloaded successfully!");

    Ok("espup installed successfully!".into())
}


async fn install_rust_toolchain(window: Window, app: AppHandle, selected_variant: Option<&String>) -> Result<String, String> {
    info!("Installing Rust toolchain via espup... (this might take a while)");

    #[cfg(unix)]
    let fname = "espup";
    #[cfg(windows)]
    let fname = "espup.exe";
    let espup_path = dirs::home_dir().ok_or("Failed to get home directory")?.join(".cargo").join("bin").join(fname).to_str().unwrap().to_string();

    let mut args = vec!["install"];
    // If there's a variant specified for Windows, pass it as a parameter
    #[cfg(target_os = "windows")]
    if let Some(variant) = selected_variant {
        args.push("--default-host");
        args.push(variant);
    }

    let result = run_external_command_with_progress(
        window.clone(),
        app.clone(),
        &espup_path,
        &args,
        "PROGRESS_EVENT"
    ).await;

    match result {
        Ok(_) => {
            info!("Rust toolchain installed successfully via espup.");
            Ok("Rust toolchain installed successfully!".into())
        }
        Err(_) => {
            info!("Failed to install Rust toolchain via espup.");
            Err("Failed to install Rust toolchain via espup.".into())
        }
    }
}


#[cfg(target_os = "windows")]
async fn install_vc_tools_and_sdk(window: Window, app: tauri::AppHandle) -> Result<String, String> {
    info!("Downloading Visual Studio Build Tools and Windows SDK...");

    // Download vs_buildtools.exe
    let url = "https://aka.ms/vs/17/release/vs_buildtools.exe";
    let response = reqwest::get(url).await.map_err(|e| format!("Failed to download VS Build Tools: {}", e))?;
    let bytes = response.bytes().await.map_err(|e| format!("Failed to read response bytes: {}", e))?;

    // Save to a temporary location
    use std::env;
    let tmp_dir = env::temp_dir();
    let file_path = tmp_dir.join("vs_buildtools.exe");
    fs::write(&file_path, &bytes).await;
    info!("Starting installer at {:?}", &file_path.display());

    // Run the installer with the necessary components
    let args = [
        "--passive",
        "--wait",
        "--add", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
        "--add", "Microsoft.VisualStudio.Component.Windows11SDK.22621"
    ];
    run_external_command_with_progress(window.clone(), app, &file_path.to_string_lossy(), &args, "Installing Visual Studio Build Tools and Windows SDK...").await;

    info!("Visual Studio Build Tools and Windows SDK installed successfully!");

    Ok("Visual Studio Build Tools and Windows SDK installed successfully!".into())
}
