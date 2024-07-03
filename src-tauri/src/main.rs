// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rfd::FileDialog;

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
fn extract_div_contents(input: String) -> Vec<String> {
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
            if part.find("<br>") != None {
            } else {
                let content = &part[..end_index];
                result.push(content.to_string());
            }
        }
    }
    result
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            show_save_dialog,
            extract_div_contents
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
