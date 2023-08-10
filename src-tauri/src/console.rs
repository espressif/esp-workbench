
use tauri::Window;
use tauri::Manager;
use log::{Log, Record, Level, Metadata, SetLoggerError};

#[derive(Clone, serde::Serialize)]
struct ConsoleEvent {
    message: String,
}

pub struct TauriLogger {
    window: Window,
}

impl TauriLogger {
    pub fn new(window: Window) -> Self {
        Self { window }
    }
}

impl Log for TauriLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let event = ConsoleEvent {
                message: format!("{}", record.args()),
            };
            self.window.emit("rust-console", &event).unwrap();
            println!("{}", record.args()); // Also log to stdout
        }
    }

    fn flush(&self) {}
}

use fern::Dispatch;
use log::LevelFilter;

pub fn setup_logging(app: &tauri::App) {
  if let Some(window) = app.get_window("main") {
      let tauri_logger = TauriLogger::new(window);

      // Set the TauriLogger as the global logger.
      log::set_boxed_logger(Box::new(tauri_logger)).expect("Failed to set logger");
      log::set_max_level(log::LevelFilter::Info); // Set max level of logging

      // Fern setup for stdout logging
      let _ = Dispatch::new()
          .chain(std::io::stdout())
          .level(LevelFilter::Info) // Set desired log level here
          .apply();
  }
}

