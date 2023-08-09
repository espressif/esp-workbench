use std::process::Command;

use tauri::{AppHandle, Manager};
use tauri::Window;


use external_command::{run_external_command_with_progress, emit_rust_console};

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
      println!("command failed: {:?}", output);
      return None;
  }

  let stdout = String::from_utf8_lossy(&output.stdout);
  println!("stdout: {:?}", stdout);

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
      println!("command failed: {:?}", output);
      return None;
  }

  let stdout = String::from_utf8_lossy(&output.stdout);
  println!("stdout: {:?}", stdout);

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


    println!("riscv: {:?}", riscv_version);
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

    install_rustup(window.clone(), selected_variant.as_ref()).await?;
    install_espup(window.clone(), app.clone(), selected_variant.as_ref()).await?;
    install_rust_toolchain(window, app, selected_variant.as_ref());
    Ok("Success".into())
}

pub async fn install_rustup(window: Window, selected_variant: Option<&String>) -> Result<String, String> {

    // Check if rustup is already installed
    match Command::new("rustup").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                emit_rust_console(&window, "Rustup already installed".into());
                return Ok("Rustup already installed".into());
            }
        },
        Err(_) => {}
    }

    emit_rust_console(&window, "Installing rustup...".into());

    // Install rustup based on the OS
    #[cfg(target_os = "windows")]
    {
        let mut cmd = Command::new("rustup-init.exe");
        cmd.arg("install").arg("-y");

        if let Some(variant) = &selected_variant {
            let host = match variant.as_str() {
                "msvc" => "--default-host x86_64-pc-windows-msvc",
                "mingw" => "--default-host x86_64-pc-windows-gnu",
                _ => return Err("Invalid variant".into()),
            };
            cmd.arg(host);
        }

        let output = cmd.output().map_err(|_| "Failed to run rustup-init.exe".to_string())?;
        let stdout_str = String::from_utf8_lossy(&output.stdout).into_owned();
        emit_rust_console(&window, stdout_str);
    }

    #[cfg(unix)]
    {
        let output = Command::new("sh")
            .arg("./rustup-init.sh")
            .arg("-y")
            .output()
            .map_err(|_| "Failed to run rustup-init.sh".to_string())?;

        let stdout_str = String::from_utf8_lossy(&output.stdout).into_owned();
        emit_rust_console(&window, stdout_str);
    }

    emit_rust_console(&window, "Rustup installed or already present".into());
    Ok("Rustup installed or already present".into())
}



use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::external_command;
use crate::external_command::set_exec_permission;

async fn install_espup(window: Window, app: AppHandle, selected_variant: Option<&String>) -> Result<String, String> {
    emit_rust_console(&window, "Installing espup...".into());

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

    // Download the binary using reqwest's async API
    let response = reqwest::get(url).await.map_err(|e| format!("Failed to download espup: {}", e))?;

    #[cfg(unix)]
    let fname = "espup";
    #[cfg(windows)]
    let fname = "espup.exe";

    let bytes = response.bytes().await.map_err(|e| format!("Failed to read response bytes: {}", e))?;

    let output_dir = dirs::home_dir().ok_or("Failed to get home directory")?.join(".cargo/bin");
    let output_path = output_dir.join(fname);
    let mut dest = fs::File::create(&output_path)
       .await
       .map_err(|e| format!("Failed to create file: {}", e))?;

    dest.write_all(&bytes).await.map_err(|e| format!("Failed to write to file: {}", e))?;

    // Set execute permission for the binary on Unix-based systems
    #[cfg(unix)]
    set_exec_permission(&output_path).map_err(|e| format!("Failed to set execute permissions: {}", e))?;

    emit_rust_console(&window, "espup downloaded successfully!".into());

    Ok("espup installed successfully!".into())
}


fn install_rust_toolchain(window: Window, app: AppHandle, selected_variant: Option<&String>) -> Result<String, String> {
    emit_rust_console(&window, "Installing Rust toolchain via espup... (this might take a while)".into());

    let espup_path = dirs::home_dir().ok_or("Failed to get home directory")?.join(".cargo/bin/espup").to_str().unwrap().to_string();

    let mut args = vec!["install"];
    // If there's a variant specified for Windows, pass it as a parameter
    #[cfg(target_os = "windows")]
    if let Some(variant) = selected_variant {
        args.push(variant);
    }

    let result = run_external_command_with_progress(
        window.clone(),
        app.clone(),
        &espup_path,
        &args,
        "PROGRESS_EVENT"
    );

    match result {
        Ok(_) => {
            emit_rust_console(&window, "Rust toolchain installed successfully via espup.".into());
            Ok("Rust toolchain installed successfully!".into())
        }
        Err(_) => {
            emit_rust_console(&window, "Failed to install Rust toolchain via espup.".into());
            Err("Failed to install Rust toolchain via espup.".into())
        }
    }
}


#[cfg(target_os = "windows")]
async fn install_vc_tools_and_sdk(window: Window, app: tauri::AppHandle) -> Result<String, String> {
    emit_rust_console(&window.clone(), "Downloading Visual Studio Build Tools and Windows SDK...".into());

    // Download vs_buildtools.exe
    let url = "https://aka.ms/vs/17/release/vs_buildtools.exe";
    let response = reqwest::get(url).await.map_err(|e| format!("Failed to download VS Build Tools: {}", e))?;
    let bytes = response.bytes().await.map_err(|e| format!("Failed to read response bytes: {}", e))?;

    // Save to a temporary location
    use std::env;
    let tmp_dir = env::temp_dir();
    let file_path = tmp_dir.join("vs_buildtools.exe");
    fs::write(&file_path, &bytes).await;
    emit_rust_console(&window, format!("Starting installer at {:?}", &file_path));

    // Run the installer with the necessary components
    let args = [
        "--passive",
        "--wait",
        "--add", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
        "--add", "Microsoft.VisualStudio.Component.Windows11SDK.22621"
    ];
    run_external_command_with_progress(window.clone(), app, &file_path.to_string_lossy(), &args, "Installing Visual Studio Build Tools and Windows SDK...");

    emit_rust_console(&window, "Visual Studio Build Tools and Windows SDK installed successfully!".into());

    Ok("Visual Studio Build Tools and Windows SDK installed successfully!".into())
}
