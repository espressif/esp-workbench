use std::path::PathBuf;
use std::fs::read;
use espflash::flasher::Flasher;
use espflash::flasher::ProgressCallbacks;
use espflash::interface::Interface;
use tauri::Window;
use tauri::AppHandle;

#[derive(Clone, serde::Serialize)]
struct Payload {
    pct: String,
}

#[derive(Clone, serde::Serialize)]
struct FlashProgressEvent {
    count: usize,
    total: usize,
}

struct FlashProgress {
    window: Window,
    current: usize,
    total: usize,
}

impl ProgressCallbacks for FlashProgress {

    fn init(&mut self, addr: u32, total: usize) {
        println!("init: addr: {:x}, total: {}", addr, total);
        self.current = 0;
        self.total = total;
        let flash_payload = FlashProgressEvent {
          count: self.current,
          total
        };
        self.window.emit("flash-update", flash_payload).unwrap();
    }

    fn update(&mut self, current: usize) {
        println!("update: current: {}", current);
        self.current = current;
        let flash_payload = FlashProgressEvent {
          count: current,
          total: self.total
        };
        self.window.emit("flash-update", flash_payload).unwrap();
    }

    fn finish(&mut self) {
        println!("finish");
        let flash_payload = FlashProgressEvent {
          count: self.total,
          total: self.total
        };
        self.window.emit("flash-finish", flash_payload).unwrap();
    }
}

use serialport::{SerialPortInfo, UsbPortInfo};
use serialport::available_ports;
use std::io;

pub fn get_serial_port_info(port_name: &str) -> io::Result<SerialPortInfo> {
  let ports = available_ports()?;
  for p in ports {
      if p.port_name == port_name {
          return Ok(p);
      }
  }
  Err(io::Error::new(io::ErrorKind::NotFound, "Port not found"))
}

pub fn emit_error(window: &Window, error: &str) {
  let error_payload = Payload {
      pct: format!("Error: {}", error),
  };
  window.emit("error", error_payload).unwrap();
}

pub async fn flash_file(window: Window, app: AppHandle, port: String, file_path: String, flash_offset: u32)  -> Result<(), String> {

    // let file_metadata = std::fs::metadata(&file_path);
    // match file_metadata {
    //     Ok(metadata) => {
    //         if metadata.len() > 1500000 {
    //             emit_error(&window, "File size cannot be greater than 1.5MB. Limitation: https://github.com/esp-rs/espflash/issues/453");
    //             return Ok(());
    //         }
    //     }
    //     Err(e) => {
    //         emit_error(&window, &format!("Failed to get file metadata: {}", e));
    //         return Ok(());
    //     }
    // }

    let binary_file = PathBuf::from(file_path);

    let mut data = read(&binary_file).unwrap();

    let dtr = Some(1);
    let rts = Some(0);

    // let port_info = get_serial_port_info(port.as_str()).unwrap();

    println!("port: {}", port);
    let serial_port_info = get_serial_port_info(port.as_str()).unwrap();
    let port_info = match &serial_port_info.port_type {
        serialport::SerialPortType::UsbPort(info) => Some(info.clone()),
        _ => return Err(("Port is not a USB port".to_string() ))
    };
    let mut serial = Interface::new(&serial_port_info, dtr, rts).unwrap();

    println!("Connecting to port...");
    let mut flasher = Flasher::connect(serial, port_info.unwrap(), None, false).unwrap();

    // Emit the line to the frontend
    let payload = Payload {
        pct: "Start flashing...".to_string(),
    };
    window.emit("flash-event", payload).unwrap();

    let mut progress = FlashProgress { window: window.clone(), total: 0, current: 0 };

    let chunk_size = 1024 * 1024; // 1MB chunk size
    // let total_size = data.len();
    let mut offset = flash_offset;
    // let mut current_offset = 0;

    while !data.is_empty() {
      let (chunk, rest) = if data.len() > chunk_size {
          data.split_at(chunk_size)
      } else {
          (data.as_ref(), &[][..])
      };

      flasher.write_bin_to_flash(offset, chunk, Some(&mut progress)).map_err(|e| {
          let error = format!("Flash error: {:?}", e);
          emit_error(&window, &error);
          error
      })?;

      offset += chunk.len() as u32;
      data = rest.to_vec();
    }

    window.emit("flash-event", Some("Flash Done")).unwrap();

    Ok(())
}
