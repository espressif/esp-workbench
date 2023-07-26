use std::io::{Seek, Write};
use std::io::Read;
use std::iter::Iterator;
use std::fs::File;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use zip::result::ZipError;
use zip::write::FileOptions;

use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    pct: String,
}

const PROGRESS_EVENT: &str = "progress";

pub fn zip_dir(
    window: Window,
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

    zip_iter(window, &mut src_it.filter_map(|e| e.ok()), src_dir, archive_file, method)?;
    Ok(())
}

fn zip_iter<T>(
    window: Window,
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
    Result::Ok(())
}

