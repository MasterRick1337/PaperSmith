// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::de::value::BoolDeserializer;
use tauri::{CustomMenuItem, Menu, Submenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .menu(generate_menu())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn generate_menu() -> Menu {
    let new = CustomMenuItem::new("new".to_string(), "New");
    let open = CustomMenuItem::new("open".to_string(), "Open...");
    let recent_submenu = Submenu::new("Recent Projects", Menu::new());
    let save = CustomMenuItem::new("save".to_string(), "Save");
    let save_as = CustomMenuItem::new("save_as".to_string(), "Save As...");
    let save_copy_as = CustomMenuItem::new("save_copy_as".to_string(), "Save a Copy As...");
    let export = CustomMenuItem::new("export".to_string(), "Export");
    let project_settings = CustomMenuItem::new("project_settings".to_string(), "Project Settings");
    let close = CustomMenuItem::new("close".to_string(), "Close");

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

    let undo = CustomMenuItem::new("undo".to_string(), "Undo");
    let redo = CustomMenuItem::new("redo".to_string(), "Redo");
    let copy = CustomMenuItem::new("copy".to_string(), "Copy");
    let cut = CustomMenuItem::new("cut".to_string(), "Cut");
    let paste = CustomMenuItem::new("paste".to_string(), "Paste");
    let select_all = CustomMenuItem::new("select_all".to_string(), "Select All");
    let find = CustomMenuItem::new("find".to_string(), "Find");
    let find_replace = CustomMenuItem::new("find_replace".to_string(), "Find and Replace");
    let jump_wordcount = CustomMenuItem::new("jump_wordcount".to_string(), "Jump to Word Count");

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

    let italic = CustomMenuItem::new("italic".to_string(), "Italic");
    let bold = CustomMenuItem::new("bold".to_string(), "Bold");
    let heading1 = CustomMenuItem::new("heading1".to_string(), "Heading 1");
    let heading2 = CustomMenuItem::new("heading2".to_string(), "Heading 2");
    let heading3 = CustomMenuItem::new("heading3".to_string(), "Heading 3");
    let hyperlink = CustomMenuItem::new("hyperlink".to_string(), "Hyperlink");
    let quote = CustomMenuItem::new("quote".to_string(), "Quote");
    let list = CustomMenuItem::new("list".to_string(), "List");
    let list_numbered = CustomMenuItem::new("list_numbered".to_string(), "Numbered List");
    let separator = CustomMenuItem::new("separator".to_string(), "Separator");
    let inline_code = CustomMenuItem::new("inline_code".to_string(), "Inline Code");
    let code_block = CustomMenuItem::new("code_block".to_string(), "Code Block");

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
            .add_item(code_block),
    );

    let editor_settings = CustomMenuItem::new("editor_settings".to_string(), "Editor Settings");
    let help = CustomMenuItem::new("help".to_string(), "Help");
    let exit = CustomMenuItem::new("exit".to_string(), "Exit");

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
