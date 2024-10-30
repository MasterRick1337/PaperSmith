use rfd::FileDialog;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

#[tauri::command]
pub fn check_if_folder_exists(path: &str) -> bool {
    Path::new(&path).exists()
}

#[tauri::command]
pub fn choose_folder(title: String) -> String {
    let path = FileDialog::new().set_title(title).pick_folder();

    println!("choosing folder tauri");
    path.map_or_else(
        || "No folder selected".to_string(),
        |path| path.to_string_lossy().to_string(),
    )
}

#[tauri::command]
pub fn can_create_path(path: &str) -> String {
    let parsed_path = Path::new(path);

    if parsed_path.exists() {
        return "Path already exists.".into();
    }

    // Check if the path is empty or just whitespace.
    if path.trim().is_empty() {
        return "Path cannot be empty.".into();
    }

    // Check if the path is too long (some OSes have limitations, e.g., 255 characters).
    //if parsed_path
    //    .file_name()
    //    .expect("yo")
    //    .to_str()
    //    .expect("yo")
    //    .chars()
    //    .count()
    //    > 255
    //{
    //    return "Path is too long.".into();
    //}

    // Check if the parent directory exists.
    if let Some(parent) = parsed_path.parent() {
        if !parent.exists() {
            return format!("Directory '{}' does not exist.", parent.display());
        }
    } else {
        return "Path does not have a parent directory.".into();
    }

    // Check if the path is a reserved name (e.g., on Windows).
    #[cfg(target_os = "windows")]
    {
        let reserved_names = [
            "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
            "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
        ];
        if reserved_names.contains(&path.file_name().unwrap_or_default().to_str().unwrap_or("")) {
            return "The path uses a reserved name.".into();
        }
    }

    // Check if we can write to the target directory.
    let temp_file_path = parsed_path
        .parent()
        .expect("fuck")
        .join(".can_create_check.tmp");
    match fs::File::create(&temp_file_path) {
        Ok(_) => {
            // Clean up the temporary file.
            let _ = fs::remove_file(&temp_file_path);
            String::new()
        }
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => {
                "Cannot create the file at this path: Permission denied.".to_string()
            }
            _ => {
                format!("An error occurred while creating the file: {e}")
            }
        },
    }
}
