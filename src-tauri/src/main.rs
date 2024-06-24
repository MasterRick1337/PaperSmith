// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use serde::Deserialize;
use tauri::Manager;
use std::fs::File;
use std::io::Write;
use rfd::{AsyncFileDialog, FileDialog};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[derive(Deserialize)]
struct SaveFileArgs {
    content: String,
    filename: String,
}

#[tauri::command]
async fn show_save_dialog() -> Result<String, String> {
    let path = FileDialog::new()
        .set_title("Save File")
        .add_filter("Text", &["txt"])
        .save_file()
        .ok_or_else(|| "No file selected".to_string())?;



    Ok(path.to_str().unwrap_or_default().to_string())
}

/*this one worked--------------------------------------------------------------
#[tauri::command]
async fn show_save_dialog() {
    let test: &str = "Test";
    println!("{}", test);
    dialog::FileDialogBuilder::default()
        .add_filter("Markdown", &["md"])
        .pick_file(|path_buf| match path_buf {
            Some(p) => {}
            _ => {}
        });
}*/

#[tauri::command]
fn save_file(args: SaveFileArgs) -> Result<(), String> {
    let mut file = File::create(&args.filename)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    file.write_all(args.content.as_bytes())
        .map_err(|e| format!("Failed to write to file: {}", e))?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![show_save_dialog, save_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
