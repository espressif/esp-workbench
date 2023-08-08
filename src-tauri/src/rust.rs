use std::process::Command;

pub fn get_tool_version(command: &str, flags: &[&str], keyword: Option<&str>) -> Option<String> {
  let mut cmd = Command::new(command);
  for flag in flags {
      cmd.arg(flag);
  }

  let output = cmd.output().ok()?;

  if !output.status.success() {
      println!("command failed: {:?}", output);
      return None;
  }

  let stdout = String::from_utf8_lossy(&output.stdout);
  println!("stdout: {:?}", stdout);

  // Split by newline and take the first line.
  let binding = stdout.split('\n').collect::<Vec<&str>>();
  let line = binding.get(0)?;

  // If a keyword is provided, look for it in the line. If not found, return None.
  if let Some(keyword) = keyword {
      if !line.contains(keyword) {
          return None;
      }
  }

  // Extract just the version part for other cases.
  line.split_whitespace().nth(1).map(|s| s.to_string())
}


pub fn get_tool_version_xtensa(command: &str, flags: &[&str], keyword: Option<&str>) -> Option<String> {
  let mut cmd = Command::new(command);
  for flag in flags {
      cmd.arg(flag);
  }

  let output = cmd.output().ok()?;

  if !output.status.success() {
      println!("command failed: {:?}", output);
      return None;
  }

  let stdout = String::from_utf8_lossy(&output.stdout);
  println!("stdout: {:?}", stdout);

  // Split by newline and take the first line.
  let binding = stdout.split('\n').collect::<Vec<&str>>();
  let line = binding.get(0)?;

  // If a keyword is provided, look for it in the line. If not found, return None.
  if let Some(keyword) = keyword {
      if !line.contains(keyword) {
          return None;
      }
  }

  // Extract just the version part for other cases.
  line.split_whitespace().nth(4).map(|s| s.trim_matches(')').trim_matches('(').to_string())
}


#[derive(serde::Serialize)]
pub struct RustSupportResponse {
    xtensa: Option<String>,
    riscv: Option<String>,
    cargo: Option<String>,
}

#[tauri::command]
pub fn check_rust_support() -> Result<RustSupportResponse, String> {
  let cargo_version = get_tool_version("cargo", &["--version"], None);
  let riscv_version = get_tool_version("rustc", &["+nightly", "--version"], Some("rustc"));
  let xtensa_version = get_tool_version_xtensa("rustc", &["+esp", "--version"], Some("rustc"));


    println!("riscv: {:?}", riscv_version);
    Ok(RustSupportResponse {
      xtensa: xtensa_version,
      riscv: riscv_version,
      cargo: cargo_version,
  })
}
