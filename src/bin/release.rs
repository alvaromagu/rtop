use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
  let version = get_cargo_version();
  println!("Version detected: {}", version);
  // if version exists, exit program with error code and message
  if Path::new("releases/rtop.exe").exists() {    
    println!("Error: rtop.exe already exists in releases directory. Please delete it before building a new version.");
    std::process::exit(1);
  }
  println!("Building version: {}...", version);
  let output = Command::new("cargo")
    .arg("build")
    .arg("--release")
    .output()
    .expect("Failed to build");
  println!("status: {}", output.status);
  // move the .exe file to /releases
  fs::create_dir_all("releases").expect("Failed to create releases directory");
  // copy the .exe file to /releases/rtop.exe and /releases/rtop-<version>.exe
  fs::copy("target/release/rtop.exe", "releases/rtop.exe").expect("Failed to copy file");
  fs::copy("target/release/rtop.exe", format!("releases/rtop-{}.exe", version)).expect("Failed to copy file");
}

fn get_cargo_version () -> String {
  return env!("CARGO_PKG_VERSION").to_string();
}
