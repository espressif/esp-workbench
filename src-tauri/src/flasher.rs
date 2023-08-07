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

struct FlashProgress {
    window: Window,
}

impl ProgressCallbacks for FlashProgress {

    fn init(&mut self, addr: u32, total: usize) {
        todo!()
    }

    fn update(&mut self, current: usize) {
        todo!()
    }

    fn finish(&mut self) {
        todo!()
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

pub async fn flash_file(window: Window, app: AppHandle, port: String, file_path: String, flash_offset: u32)  -> Result<(), String> {
    // Read the binary file
    let binary_file = PathBuf::from(file_path);
    let data = read(&binary_file).unwrap();

    let dtr = Some(1);
    let rts = Some(0);

    // let port_info = get_serial_port_info(port.as_str()).unwrap();

    let serial_port_info = get_serial_port_info(port.as_str()).unwrap();
    let port_info = match &serial_port_info.port_type {
        serialport::SerialPortType::UsbPort(info) => Some(info.clone()),
        _ => return Err(("Port is not a USB port".to_string() ))
    };
    let mut serial = Interface::new(&serial_port_info, dtr, rts).unwrap();
    let mut flasher = Flasher::connect(serial, port_info.unwrap(), None, false).unwrap();



    // Emit the line to the frontend
    let payload = Payload {
        pct: "Start flashing...".to_string(),
    };
    window.emit("flash-event", payload).unwrap();

    let mut progress = FlashProgress { window: window.clone() };
    flasher.write_bin_to_flash(flash_offset, &data, Some(&mut progress)).unwrap();

    window.emit("flash-event", Some("Flash Done")).unwrap();

    Ok(())
}
