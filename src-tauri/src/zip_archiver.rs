use std::io;
use std::io::{Seek, Write};
use std::io::Read;
use std::iter::Iterator;
use std::fs;
use std::fs::File;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use zip::result::ZipError;
use zip::write::FileOptions;

use tauri::{Window, Manager};
use std::sync::{Mutex};

use crate::app_state::{AppState, BuilderState};

#[derive(Clone, serde::Serialize)]
struct Payload {
    pct: String,
}

const PROGRESS_EVENT: &str = "progress";

pub fn zip_dir(
    window: Window,
    app: tauri::AppHandle,
    src_dir: &str,
    dst_file: &str,
    method: zip::CompressionMethod,
) -> Result<(), ZipError>
{

    let method = zip::CompressionMethod::Deflated;
    println!("Zipping {:?} to {:?} using method {:?} ...", src_dir, dst_file, method);
    let archive_file_path = Path::new(dst_file);
    let archive_file = File::create(archive_file_path).unwrap();

    let src_walkdir = WalkDir::new(src_dir);
    let src_it = src_walkdir.into_iter();

    zip_iter(window, app, &mut src_it.filter_map(|e| e.ok()), src_dir, archive_file, method)?;
    Ok(())
}

fn is_abort_state(app: tauri::AppHandle) -> bool {
    let state_mutex = app.state::<Mutex<AppState>>();
    let mut state = state_mutex.lock().unwrap();
    match state.builder {
        BuilderState::Abort => true,
        _ => false
    }
}

fn zip_iter<T>(
    window: Window,
    app: tauri::AppHandle,
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        if is_abort_state(app.clone()) {
            let payload = Payload {
                pct: "Aborted".to_string(),
            };
            window.emit(PROGRESS_EVENT, payload).unwrap();
            return Ok(());
        }
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            let message = format!("adding file {path:?} as {name:?} ...");
            let payload = Payload {
                pct: message.to_string(),
            };
            window.emit(PROGRESS_EVENT, payload).unwrap();
            println!("{message:?}");
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            let message = format!("adding dir {path:?} as {name:?} ...");
            let payload = Payload {
                pct: message.to_string(),
            };
            window.emit(PROGRESS_EVENT, payload).unwrap();
            println!("{message:?}");
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    let payload = Payload {
        pct: "Complete".to_string(),
    };
    window.emit(PROGRESS_EVENT, payload).unwrap();
    Result::Ok(())
}


pub fn unzip(
    window: Window,
    app: tauri::AppHandle,
    file_path: String,
    output_directory: String
) -> Result<(), ZipError> {
    let file_name = std::path::Path::new(&file_path);
    let file = fs::File::open(&file_name).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        if is_abort_state(app.clone()) {
            let payload = Payload {
                pct: "Aborted".to_string(),
            };
            window.emit(PROGRESS_EVENT, payload).unwrap();
            return Ok(());
        }

        let mut file = archive.by_index(i).unwrap();
        let file_outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        let mut components = file_outpath.components();

        // Skip the first component (the top level directory)
        components.next();

        // Add path prefix to extract the file
        let mut outpath = std::path::PathBuf::new();
        outpath.push(&output_directory);

        // Append the rest of the components
        for component in components {
            outpath.push(component);
        }

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("* extracted: \"{}\"", outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            let message = format!("extracted {} as {} ...", outpath.display(), file.size());
            let payload = Payload {
                pct: message.to_string(),
            };
            window.emit(PROGRESS_EVENT, payload).unwrap();
            println!(
                "{}",
                message
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
    let payload = Payload {
        pct: "Complete".to_string(),
    };
    window.emit(PROGRESS_EVENT, payload).unwrap();
    Ok(())
}
