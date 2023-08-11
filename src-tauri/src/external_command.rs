use std::process::Stdio;
use std::sync::Mutex;

use crate::app_state::{AppState, BuilderState};
use tauri::Manager;
use tauri::Window;

use log::info;

fn is_abort_state(app: tauri::AppHandle) -> bool {
    let state_mutex = app.state::<Mutex<AppState>>();
    let mut state = state_mutex.lock().unwrap();
    match state.builder {
        BuilderState::Abort => true,
        _ => false,
    }
}

use tokio::io::AsyncBufReadExt;
use tokio::process::Command;

pub async fn run_external_command_with_progress(
    window: Window,
    app: tauri::AppHandle,
    cmd_name: &str,
    cmd_args: &[&str],
    progress_event: &str,
) -> Result<String, ()> {
    let cmd_name_owned = cmd_name.to_string();
    let cmd_args_owned: Vec<String> = cmd_args.iter().map(|&s| s.to_string()).collect();

    info!("Command: {} {}", cmd_name_owned, cmd_args_owned.join(" "));

    let child_result = Command::new(&cmd_name_owned)
        .args(&cmd_args_owned)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut child = match child_result {
        Ok(child) => child,
        Err(e) => {
            info!("Failed to launch command: {:?}", e);
            return Err(());
        }
    };

    let mut stdout = tokio::io::BufReader::new(child.stdout.take().unwrap());
    let mut stderr = tokio::io::BufReader::new(child.stderr.take().unwrap());

    let mut stdout_buf = String::new();
    let mut stderr_buf = String::new();

    let poll_interval = tokio::time::Duration::from_millis(100); // adjust as necessary

    loop {
        tokio::select! {
            _ = stdout.read_line(&mut stdout_buf) => {
                if !stdout_buf.is_empty() {
                    info!("{}", stdout_buf);
                    stdout_buf.clear();
                }
            },
            _ = stderr.read_line(&mut stderr_buf) => {
                if !stderr_buf.is_empty() {
                    info!("{}", stderr_buf);
                    stderr_buf.clear();
                }
            },
            status = child.wait() => {
                match status {
                    Ok(status) if status.success() => {
                        info!("Done");
                        return Ok("Child process completed successfully".to_string());
                    },
                    Ok(_) => {
                        info!("Child process exited with an error");
                        return Err(());
                    },
                    Err(err) => {
                        info!("Child process encountered an error: {:?}", err);
                        return Err(());
                    },
                }
            },
            _ = tokio::time::sleep(poll_interval) => {
                if is_abort_state(app.clone()) {
                    info!("Aborting command due to external signal.");
                    let _ = child.kill();
                    return Err(());
                }
            }
        }
    }
}

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
pub fn set_exec_permission(path: &std::path::Path) -> std::io::Result<()> {
    use std::fs;

    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(0o755); // rwxr-xr-x
    fs::set_permissions(path, perms)
}

#[cfg(windows)]
pub fn set_exec_permission(path: &std::path::Path) -> std::io::Result<()> {
    todo!()
}
