use std::process::Command;

use tauri::{AppHandle, Manager};
use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct ConsoleEvent {
    message: String,
}


pub fn emit_rust_console(window: &Window, message: String) {
  let event = ConsoleEvent {
      message: message,
  };
  window.emit("rust-console", event).unwrap();
}

pub fn get_tool_version(command: &str, flags: &[&str], keyword: Option<&str>) -> Option<String> {
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


#[tauri::command]
pub async fn install_rust_support(window: Window, app: AppHandle, selected_variant: Option<String>) -> Result<String, String> {
    install_rustup(window.clone(), selected_variant.as_ref()).await?;
    install_espup(window, app, selected_variant.as_ref()).await?;
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



use std::fs::File;
use std::io::copy;
use reqwest::blocking::get;

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

    // Download the binary using reqwest
    let response = get(url).map_err(|e| format!("Failed to download espup: {}", e))?;
    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .ok_or("Failed to extract filename from URL")?;

        let output_dir = dirs::home_dir().ok_or("Failed to get home directory")?.join(".cargo/bin");
        File::create(output_dir.join(fname)).map_err(|e| format!("Failed to create file: {}", e))?
    };
    copy(&mut response.bytes().map_err(|e| format!("Failed to read response bytes: {}", e))?.as_ref(), &mut dest)
        .map_err(|e| format!("Failed to copy content: {}", e))?;

    // Change permission for Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = dest.metadata().unwrap().permissions();
        permissions.set_mode(0o755); // rwxr-xr-x
        dest.set_permissions(permissions).unwrap();
    }

    emit_rust_console(&window, "espup downloaded successfully!".into());

    // // Run "espup install", pass variant if on Windows
    // let mut command = Command::new(dest.path());
    // command.arg("install");
    // if cfg!(target_os = "windows") && selected_variant.is_some() {
    //     command.arg(&selected_variant.unwrap());
    // }
    // let output = command.output().map_err(|_| "Failed to run 'espup install'".to_string())?;
    // let stdout_str = String::from_utf8_lossy(&output.stdout).into_owned();
    // emit_rust_console(&window, stdout_str);

    // emit_rust_console(&window, "espup installed successfully!".into());

    Ok("espup installed successfully!".into())
}
