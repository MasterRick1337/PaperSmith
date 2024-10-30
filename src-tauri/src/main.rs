// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::Write;

use rfd::FileDialog;

mod loader;
use loader::parse_project;

mod checking;
use checking::can_create_path;
use checking::check_if_folder_exists;
use checking::choose_folder;

mod menu;
use menu::generate as generate_menu;

mod saving;
use saving::create_project;

use shared::Project;

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            show_save_dialog,
            extract_div_contents,
            get_project,
            write_to_file,
            choose_folder,
            check_if_folder_exists,
            can_create_path,
            create_project
        ])
        .menu(generate_menu())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn show_save_dialog() -> Result<String, String> {
    let path = FileDialog::new()
        .set_title("Save File")
        .add_filter("Text", &["txt"])
        .add_filter("MarkDown", &["md"])
        .save_file()
        .ok_or_else(|| "No file selected".to_string())?;

    Ok(path.to_str().unwrap_or_default().to_string())
}

#[tauri::command]
fn get_project() -> Option<Project> {
    let project_path = FileDialog::new().pick_folder().unwrap();
    parse_project(project_path)
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
fn extract_div_contents(input: &str) -> Vec<String> {
    // Initialize an empty vector to store the extracted contents
    let mut result = Vec::new();

    // Define the start and end tag strings
    let start_tag = "<div>";
    let end_tag = "</div>";

    // Split the input string by the start tag
    let parts: Vec<&str> = input.split(start_tag).collect();

    // Iterate over the parts and extract the contents between the start and end tags
    for part in parts {
        if let Some(end_index) = part.find(end_tag) {
            if part.contains("<br>") {
            } else {
                let content = &part[..end_index];
                result.push(content.to_string());
            }
        }
    }
    result
}

#[tauri::command]
fn write_to_file(path: &str, content: &str) {
    //let path = Path::new(&path);
    // if !path.exists() {
    //     match fs::create_dir_all("C:/Users/janni/Desktop/Schule/Diplomarbeit/PaperSmith/statistic") {
    //         Ok(_) => println!("Directory created: {:?}", path),
    //         Err(e) => eprintln!("Failed to create directory: {:?}", e),
    //     }
    // }

    let mut file = fs::File::create(path).unwrap();
    match write!(file, "{}", content) {
        Ok(_) => println!("Directory created: {:?}", path),
        Err(e) => eprintln!("Failed to create directory: {:?}", e),
    }

    // match fs::write(path, content) {
    //     Ok(_) => println!("JSON file created successfully."),
    //     Err(e) => eprintln!("Failed to write to file: {:?}", e),
    // }
}
