use std::process::Stdio;
use std::sync::Mutex;

use crate::app_state::{AppState, BuilderState};
use tauri::{Manager, AppHandle};

use log::info;

fn is_abort_state(app: tauri::AppHandle) -> bool {
    let state_mutex = app.state::<Mutex<AppState>>();
    let state = state_mutex.lock().unwrap();
    matches!(state.builder, BuilderState::Abort)
}

use tokio::io::AsyncBufReadExt;
use tokio::process::Command;

fn log_to_frontend(app: &tauri::AppHandle, message: &str) {
    app.emit_all("command-log", message).unwrap();
}

pub async fn run_external_command_with_progress(
    app: tauri::AppHandle,
    cmd_name: &str,
    cmd_args: &[&str],
    _progress_event: &str,
) -> Result<String, ()> {
    let cmd_name_owned = cmd_name.to_string();
    let cmd_args_owned: Vec<String> = cmd_args.iter().map(|&s| s.to_string()).collect();

    info!("Command: {} {}", cmd_name_owned, cmd_args_owned.join(" "));
    log_to_frontend(&app, &format!("Command: {} {}", cmd_name_owned, cmd_args_owned.join(" ")));

    let child_result = Command::new(&cmd_name_owned)
        .args(&cmd_args_owned)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut child = match child_result {
        Ok(child) => child,
        Err(e) => {
            info!("Failed to launch command: {:?}", e);
            log_to_frontend(&app, &format!("Failed to launch command: {:?}", e));
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
                    log_to_frontend(&app, &stdout_buf);
                    stdout_buf.clear();
                }
            },
            _ = stderr.read_line(&mut stderr_buf) => {
                if !stderr_buf.is_empty() {
                    info!("{}", stderr_buf);
                    log_to_frontend(&app, &stderr_buf);
                    stderr_buf.clear();
                }
            },
            status = child.wait() => {
                match status {
                    Ok(status) if status.success() => {
                        info!("Done");
                        log_to_frontend(&app, "Child process completed successfully");
                        return Ok("Child process completed successfully".to_string());
                    },
                    Ok(_) => {
                        info!("Child process exited with an error");
                        log_to_frontend(&app, "Child process exited with an error");
                        return Err(());
                    },
                    Err(err) => {
                        info!("Child process encountered an error: {:?}", err);
                        log_to_frontend(&app, &format!("Child process encountered an error: {:?}", err));
                        return Err(());
                    },
                }
            },
            _ = tokio::time::sleep(poll_interval) => {
                if is_abort_state(app.clone()) {
                    info!("Aborting command due to external signal.");
                    log_to_frontend(&app, "Aborting command due to external signal.");
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
