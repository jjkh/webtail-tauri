#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use anyhow::Result;
use std::{fs::File, io::Read};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![test_interop])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn test_interop(filename: String) -> Result<Vec<String>, String> {
  println!("invoked with filename {filename}");
  let mut file = match File::open(filename) {
    Ok(f) => f,
    Err(e) => return Err(format!("failed to open file: [{:?}] {e}", e.kind())),
  };

  let mut buf = String::new();
  match file.read_to_string(&mut buf) {
    Ok(_) => Ok(buf.lines().map(str::to_string).collect()),
    Err(e) => return Err(format!("failed to read file: [{:?}] {e}", e.kind())),
  }
}
