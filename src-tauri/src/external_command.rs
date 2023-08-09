use std::process::Stdio;
use std::sync::Mutex;

use tauri::Manager;
use tauri::Window;
use crate::app_state::{AppState, BuilderState};

#[derive(Clone, serde::Serialize)]
struct ConsoleEvent {
    message: String,
}

fn is_abort_state(app: tauri::AppHandle) -> bool {
  let state_mutex = app.state::<Mutex<AppState>>();
  let mut state = state_mutex.lock().unwrap();
  match state.builder {
      BuilderState::Abort => true,
      _ => false
  }
}

pub fn emit_rust_console(window: &Window, message: String) {
  let event = ConsoleEvent {
      message: message,
  };
  window.emit("rust-console", event).unwrap();
}

use tokio::process::{Command, ChildStdout, ChildStderr};
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn run_external_command_with_progress(
    window: Window,
    app: tauri::AppHandle,
    cmd_name: &str,
    cmd_args: &[&str],
    progress_event: &str
) -> Result<String, ()> {
    let cmd_name_owned = cmd_name.to_string();
    let cmd_args_owned: Vec<String> = cmd_args.iter().map(|&s| s.to_string()).collect();

    emit_rust_console(&window.clone(), format!("Command: {} {}", cmd_name_owned, cmd_args_owned.join(" ")));

    let mut child = Command::new(&cmd_name_owned)
        .args(&cmd_args_owned)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to launch command");

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let window_clone_std = window.clone();
    let window_clone_err = window.clone();

    let stdout_task = tokio::spawn(async move {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await.expect("Failed to read line from stdout") {
            emit_rust_console(&window_clone_std, line);
        }
    });

    let stderr_task = tokio::spawn(async move {
        let reader = BufReader::new(stderr);
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await.expect("Failed to read line from stderr") {
            emit_rust_console(&window_clone_err, line);
        }
    });

    let child_task = tokio::spawn(async move {
        child.wait().await.expect("Child process encountered an error")
    });

    tokio::select! {
        _ = stdout_task => {},
        _ = stderr_task => {},
        status = child_task => {
            if let Err(err) = status {
                emit_rust_console(&window, format!("Child process exited with {:?}", err));
                return Err(());
            }
        }
    }

    emit_rust_console(&window, "Done".to_string());
    Ok("Child process completed successfully".to_string())
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
