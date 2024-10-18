// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{io::Write, path::Path};
use std::fs::OpenOptions;
use std::fs::{self, File};
use chrono::{Utc, DateTime};
use rfd::FileDialog;
use tauri::{CustomMenuItem, Menu, Submenu};
use lazy_static::lazy_static;
use std::sync::Mutex;
use dirs_next;
mod loader;

use loader::parse_project;

use shared::Project;

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

// Definiere eine globale Variable für die Startzeit
lazy_static! {
    static ref START_TIME: Mutex<DateTime<Utc>> = Mutex::new(Utc::now());
}

#[tauri::command]
fn write_to_json(path: &str, content: &str) {
    let start_time = START_TIME.lock().unwrap().clone();
    let formatted_time = start_time.format("%Y-%m-%dT%H-%M-%S").to_string();

    let file_name = format!("{}.json", formatted_time);
    let file_path = format!("{}/{}", path, file_name);

    let mut file = match File::create(&file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error when creating: {:?}", e);
            return;
        }
    };
    // match write!(file, "{}", content) {
    //     Ok(_) => println!("Wrote in file: {:?}", file_path),
    //     Err(e) => eprintln!("Error when writing in file: {:?}", e),
    // }
}

#[tauri::command]
fn get_data_dir() -> String {
    if let Some(config_dir) = dirs_next::data_dir() {
         return config_dir.to_string_lossy().to_string();
    } else {
        return "No path".to_string();
    }
}

#[tauri::command]
fn write_to_file(path: &str, content: &str) {

    use std::fs::{self, OpenOptions};
    use std::io::Write;

    // Ensure the directory exists
    let path = std::path::Path::new(path);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            match fs::create_dir_all(parent) {
                Ok(_) => println!("Directory created: {:?}", parent),
                Err(e) => eprintln!("Failed to create directory: {:?}", e),
            }
        }
    }

    // Open the file in append mode or create it if it doesn't exist
    let mut file = match OpenOptions::new().append(true).create(true).open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open or create the file: {:?}", e);
            return;
        }
    };

    // Write the content to the file
    match write!(file, "{}", content) {
        Ok(_) => println!("Content appended to file: {:?}", path),
        Err(e) => eprintln!("Failed to write to file: {:?}", e),
    }
}

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            show_save_dialog,
            extract_div_contents,
            get_project,
            write_to_file,
            write_to_json,
            get_data_dir,
        ])
        .menu(generate_menu())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn generate_menu() -> Menu {
    let new = CustomMenuItem::new("new".to_string(), "New").accelerator("CTRL+N");
    let open = CustomMenuItem::new("open".to_string(), "Open...").accelerator("CTRL+O");
    let recent_submenu = Submenu::new(
        "Recent Projects",
        Menu::new().add_item(CustomMenuItem::new("nothing".to_string(), "Nothing").disabled()),
    );
    let save = CustomMenuItem::new("save".to_string(), "Save").accelerator("CTRL+S");
    let save_as =
        CustomMenuItem::new("save_as".to_string(), "Save As...").accelerator("CTRL+SHIFT+S");
    let save_copy_as = CustomMenuItem::new("save_copy_as".to_string(), "Save a Copy As...");
    let export = CustomMenuItem::new("export".to_string(), "Export").accelerator("CTRL+E");
    let project_settings = CustomMenuItem::new("project_settings".to_string(), "Project Settings")
        .accelerator("ALT+CTRL+S");
    let close = CustomMenuItem::new("close".to_string(), "Close").accelerator("ALT+CTRL+X");

    let project_submenu = Submenu::new(
        "Project",
        Menu::new()
            .add_item(new)
            .add_item(open)
            .add_submenu(recent_submenu)
            .add_item(save)
            .add_item(save_as)
            .add_item(save_copy_as)
            .add_item(export)
            .add_item(project_settings)
            .add_item(close),
    );

    let undo = CustomMenuItem::new("undo".to_string(), "Undo").accelerator("CTRL+Z");
    let redo = CustomMenuItem::new("redo".to_string(), "Redo").accelerator("CTRL+Y");
    let copy = CustomMenuItem::new("copy".to_string(), "Copy").accelerator("CTRL+C");
    let cut = CustomMenuItem::new("cut".to_string(), "Cut").accelerator("CTRL+X");
    let paste = CustomMenuItem::new("paste".to_string(), "Paste").accelerator("CTRL+V");
    let select_all =
        CustomMenuItem::new("select_all".to_string(), "Select All").accelerator("CTRL+A");
    let find = CustomMenuItem::new("find".to_string(), "Find").accelerator("CTRL+F");
    let find_replace =
        CustomMenuItem::new("find_replace".to_string(), "Find and Replace").accelerator("CTRL+H");
    let jump_wordcount = CustomMenuItem::new("jump_wordcount".to_string(), "Jump to Word Count")
        .accelerator("CTRL+J");

    let edit_submenu = Submenu::new(
        "Edit",
        Menu::new()
            .add_item(undo)
            .add_item(redo)
            .add_item(copy)
            .add_item(cut)
            .add_item(paste)
            .add_item(select_all)
            .add_item(find)
            .add_item(find_replace)
            .add_item(jump_wordcount),
    );

    let italic = CustomMenuItem::new("italic".to_string(), "Italic").accelerator("CTRL+I");
    let bold = CustomMenuItem::new("bold".to_string(), "Bold").accelerator("CTRL+B");
    let heading1 = CustomMenuItem::new("heading1".to_string(), "Heading 1").accelerator("CTRL+1");
    let heading2 = CustomMenuItem::new("heading2".to_string(), "Heading 2").accelerator("CTRL+2");
    let heading3 = CustomMenuItem::new("heading3".to_string(), "Heading 3").accelerator("CTRL+3");
    let hyperlink = CustomMenuItem::new("hyperlink".to_string(), "Hyperlink").accelerator("CTRL+K");
    let quote = CustomMenuItem::new("quote".to_string(), "Quote").accelerator("CTRL+Q");
    let list = CustomMenuItem::new("list".to_string(), "List").accelerator("CTRL+L");
    let list_numbered = CustomMenuItem::new("list_numbered".to_string(), "Numbered List")
        .accelerator("CTRL+SHIFT+L");
    let separator = CustomMenuItem::new("separator".to_string(), "Separator");
    let inline_code =
        CustomMenuItem::new("inline_code".to_string(), "Inline Code").accelerator("CTRL+C");
    let code_block =
        CustomMenuItem::new("code_block".to_string(), "Code Block").accelerator("CTRL+SHIFT+C");
    let increase_size =
        CustomMenuItem::new("increase_size".to_string(), "Increase Size").accelerator("ALT+CTRL+I");
    let decrease_size =
        CustomMenuItem::new("decrease_size".to_string(), "Decrease Size").accelerator("ALT+CTRL+D");

    let format_submenu = Submenu::new(
        "Format",
        Menu::new()
            .add_item(italic)
            .add_item(bold)
            .add_item(heading1)
            .add_item(heading2)
            .add_item(heading3)
            .add_item(hyperlink)
            .add_item(quote)
            .add_item(list)
            .add_item(list_numbered)
            .add_item(separator)
            .add_item(inline_code)
            .add_item(code_block)
            .add_item(increase_size)
            .add_item(decrease_size),
    );

    let editor_settings = CustomMenuItem::new("editor_settings".to_string(), "Editor Settings")
        .accelerator("ALT+SHIFT+S");
    let help = CustomMenuItem::new("help".to_string(), "Help").accelerator("F11");
    let exit = CustomMenuItem::new("exit".to_string(), "Exit").accelerator("ALT+F4");

    let misc_submenu = Submenu::new(
        "Misc",
        Menu::new()
            .add_item(editor_settings)
            .add_item(help)
            .add_item(exit),
    );
    Menu::new()
        .add_submenu(project_submenu)
        .add_submenu(edit_submenu)
        .add_submenu(format_submenu)
        .add_submenu(misc_submenu)
}
