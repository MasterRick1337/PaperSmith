use std::fs::{self, File};
use std::path::PathBuf;

use shared::Project;

use crate::loader::parse_project;

#[tauri::command]
pub fn create_project(path: String) -> Option<Project> {
    let mut path = PathBuf::from(path);

    let _ = fs::create_dir(&path);

    path.push("Chapters");
    let _ = fs::create_dir(&path);
    path.pop();

    path.push("Backups");
    let _ = fs::create_dir(&path);
    path.pop();

    path.push("Exports");
    let _ = fs::create_dir(&path);
    path.pop();

    path.push(".papersmith");
    path.set_extension("json");
    let _ = File::create(&path);
    path.pop();

    parse_project(path)
}
