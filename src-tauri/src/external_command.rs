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

  let child_handle = thread::spawn(move || {
      emit_rust_console(&window, format!("Command: {} {}", cmd_name_owned, cmd_args_owned.join(" ")));
      // Launch the command
      let mut child = Command::new(&cmd_name_owned) // Use the owned data here
          .args(&cmd_args_owned)                    // And here
          .stdout(Stdio::piped())
          .stderr(Stdio::piped())
          .spawn()
          .expect("Failed to launch command");

      let stdout = child.stderr.take().unwrap();
      let reader = BufReader::new(stdout);

      // Read the command's output line by line
      for line in reader.lines() {
          let line = line.expect("Failed to read line");
          println!("{}", line);
          // window.emit(progress_event, line).unwrap();
          emit_rust_console(&window, line);

          // If is_abort_state is true, kill the command
          if is_abort_state(app.clone()) {
              child.kill().expect("Failed to kill command");
              break;
          }
      }

      // window.emit(progress_event, "Done".to_string()).unwrap();
      emit_rust_console(&window, "Done".to_string());

      // Wait for the child to exit completely
      child.wait().expect("Failed to wait on child");
  });

  // Wait for the child process to finish
  child_handle.join().unwrap();

  Ok("Success".to_string())
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
