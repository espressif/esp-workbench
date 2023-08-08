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
    install_rustup(window, app, selected_variant).await?;
    Ok("Success".into())
}

pub async fn install_rustup(window: Window, app: AppHandle, selected_variant: Option<String>) -> Result<String, String> {

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
    println!("Selected variant: {:?}", selected_variant.unwrap_or("none".into()));

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
        let event = ConsoleEvent {
          message: stdout_str,
        };
        app.emit_all("rust-console", event).unwrap();
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
