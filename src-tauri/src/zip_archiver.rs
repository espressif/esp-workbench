use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::{Seek, Write};
use std::iter::Iterator;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use zip::result::ZipError;
use zip::write::FileOptions;

use std::sync::Mutex;
use tauri::{Manager, Window};

use log::info;

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
    _method: zip::CompressionMethod,
) -> Result<(), ZipError> {
    let method = zip::CompressionMethod::Deflated;
    info!(
        "Zipping {:?} to {:?} using method {:?} ...",
        src_dir, dst_file, method
    );
    let archive_file_path = Path::new(dst_file);
    let archive_file = File::create(archive_file_path).unwrap();

    let src_walkdir = WalkDir::new(src_dir);
    let src_it = src_walkdir.into_iter();

    zip_iter(
        window,
        app,
        &mut src_it.filter_map(|e| e.ok()),
        src_dir,
        archive_file,
        method,
    )?;
    Ok(())
}

fn is_abort_state(app: tauri::AppHandle) -> bool {
    let state_mutex = app.state::<Mutex<AppState>>();
    let state = state_mutex.lock().unwrap();
    matches!(state.builder, BuilderState::Abort)
}

fn zip_iter<T>(
    _window: Window,
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
            info!("Aborted");
            return Ok(());
        }
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            info!("adding file {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            info!("adding dir {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    info!("Complete");
    Result::Ok(())
}

pub fn unzip(
    _window: Window,
    app: tauri::AppHandle,
    file_path: String,
    output_directory: String,
) -> Result<(), ZipError> {
    let file_name = std::path::Path::new(&file_path);
    let file = fs::File::open(file_name).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        if is_abort_state(app.clone()) {
            info!("Aborted");
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
                info!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            // println!("* extracted: \"{}\"", outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            // info!("extracted {}", outpath.display());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
    info!("Complete");
    Ok(())
}
