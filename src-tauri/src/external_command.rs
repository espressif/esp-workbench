use std::io::BufReader;
use std::io::BufRead;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::path::Path;
use std::sync::{Mutex};

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

pub fn run_external_command_with_progress(
  window: Window,
  app: tauri::AppHandle,
  cmd_name: &str,
  cmd_args: &[&str],
  progress_event: &str
) -> Result<String, ()> {

    // Convert the references to owned data
    let cmd_name_owned = cmd_name.to_string();
    let cmd_args_owned: Vec<String> = cmd_args.iter().map(|&s| s.to_string()).collect();

    let child_handle = thread::spawn(move || -> Result<String, ()> {
        emit_rust_console(&window.clone(), format!("Command: {} {}", cmd_name_owned, cmd_args_owned.join(" ")));

        // Launch the command
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

        // Thread for stdout
        let stdout_handle = thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                let line = line.expect("Failed to read line from stdout");
                emit_rust_console(&window_clone_std, line);
            }
        });

        // Thread for stderr
        let stderr_handle = thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                let line = line.expect("Failed to read line from stderr");
                emit_rust_console(&window_clone_err, line);
            }
        });

        loop {
            if let Some(status) = child.try_wait().expect("Failed to wait on child process") {
                if status.success() {
                    break;
                } else {
                    emit_rust_console(&window, format!("Child process exited with {:?}", status));
                    return Err(());
                }
            }

            if is_abort_state(app.clone()) {
                child.kill().expect("Failed to kill command");
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        stdout_handle.join().unwrap();
        stderr_handle.join().unwrap();

        emit_rust_console(&window, "Done".to_string());
        // Wait for the child to exit completely
        child.wait().expect("Failed to wait on child");

        Ok("Child process completed successfully".to_string())
    });

    let result = child_handle.join().expect("Thread panicked");
    result // Return the result of the thread's execution to the caller of `run_external_command_with_progress`
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
