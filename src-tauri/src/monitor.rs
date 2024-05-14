use tauri::{Manager, Window};

use crate::app_state::{AppState, BuilderState};
use espflash::interface::Interface;
use serialport::available_ports;
use serialport::SerialPortInfo;
use std::io;
use std::sync::Mutex;
use std::{io::ErrorKind, time::Duration};

fn normalized<I>(iter: I) -> impl Iterator<Item = u8>
where
    I: Iterator<Item = u8>,
{
    iter.map(|byte| match byte {
        0..=31 | 127 => b'?', // replace control characters with '?'
        _ => byte,
    })
}

fn handle_serial(buff: &[u8], window: &Window) {
    // println!("Serial: {:?}", buff);
    // Normalize the buffer content
    let text: Vec<u8> = normalized(buff.iter().copied()).collect();
    let text = String::from_utf8_lossy(&text).to_string();

    // Emit the line to the frontend
    let payload = Payload {
        pct: format!("{}\r\n", text),
    };
    // println!("payload: {:?}", payload);
    window.emit("monitor-event", payload).unwrap();
}

fn is_abort_state(app: tauri::AppHandle) -> bool {
    let state_mutex = app.state::<Mutex<AppState>>();
    let state = state_mutex.lock().unwrap();
    matches!(state.builder, BuilderState::Abort)
}

pub fn get_serial_port_info(port_name: &str) -> io::Result<SerialPortInfo> {
    let ports = available_ports()?;
    for p in ports {
        if p.port_name == port_name {
            return Ok(p);
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "Port not found"))
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    pct: String,
}

pub async fn monitor_port(window: Window, app: tauri::AppHandle, port: String) -> Result<(), ()> {
    // let state_mutex = app.get_state::<Mutex<AppState>>().unwrap();

    // create necessary ConnectArgs and Config
    // let connect_args = ConnectArgs {
    //   port: Some(port.co),
    //   baud: Some(115200),
    //   no_stub: false,
    // };

    let dtr = Some(1);
    let rts = Some(0);

    let port_info = get_serial_port_info(port.as_str()).unwrap();

    let mut serial = Interface::new(&port_info, dtr, rts).unwrap();
    serial.serial_port_mut().set_baud_rate(115200).unwrap();
    serial
        .serial_port_mut()
        .set_timeout(Duration::from_millis(5))
        .unwrap();
    //  let port_x = UsbPortInfo {
    //   vid: 0,
    //   pid: 0,
    //   serial_number: None,
    //   manufacturer: None,
    //   product: None,
    // };
    // create Flasher object
    // let flasher = Flasher::connect(
    //   interface,
    //   port_x,
    //   Some(115200),
    //   false,
    // ).unwrap();

    let mut buff = [0; 1024];
    // let mut serial = flasher.into_interface();

    let payload = Payload {
        pct: format!("{}\r\n", "Starting monitoring"),
    };
    window.emit("monitor-event", payload).unwrap();
    loop {
        let read_count = match serial.serial_port_mut().read(&mut buff) {
            Ok(count) => Ok(count),
            Err(e) if e.kind() == ErrorKind::TimedOut => Ok(0),
            Err(e) if e.kind() == ErrorKind::Interrupted => continue,
            err => err,
        }
        .unwrap();

        if read_count > 0 {
            handle_serial(&buff[0..read_count], &window);
        }

        if is_abort_state(app.clone()) {
            let payload = Payload {
                pct: format!("{}\r\n","Monitoring stopped"),
            };
            window.emit("monitor-event", payload).unwrap();
            break;
        }
    }

    Ok(())
}
