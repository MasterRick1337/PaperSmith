use std::fs;
use std::path::PathBuf;

use shared::{Chapter, PaperSmithError, Project};

pub fn parse_project(path: PathBuf) -> Result<Project, PaperSmithError> {
    let file_path = path.join(".papersmith.json");

    // Check if the file exists
    if file_path.exists() && file_path.is_file() {
        return Err(PaperSmithError::new_only_code(2));
    }
    let mut chapters_path = path.clone();
    chapters_path.push("Chapters");
    if !chapters_path.exists() {
        fs::create_dir_all(&chapters_path).unwrap();
    }
    let mut chapters: Vec<Chapter> = vec![];

    for chapter in chapters_path
        .read_dir()
        .unwrap()
        .filter(|x| x.as_ref().unwrap().file_type().unwrap().is_dir())
    {
        let chapter_path = chapter.unwrap().path();

        let mut notes_path = chapter_path.clone();
        notes_path.push("Notes");
        if !notes_path.exists() {
            fs::create_dir_all(&notes_path).unwrap();
        }
        let mut notes: Vec<String> = vec![];

        for note in notes_path
            .read_dir()
            .unwrap()
            .filter(|x| x.as_ref().unwrap().file_type().unwrap().is_file())
        {
            let note = note.unwrap().path();
            if note.extension().unwrap() == "md" {
                notes.push(note.file_name().unwrap().to_string_lossy().into_owned())
            }
        }

        let mut extras_path = chapter_path.clone();
        extras_path.push("Extras");
        if !extras_path.exists() {
            fs::create_dir_all(&extras_path).unwrap();
        }
        let mut extras: Vec<String> = vec![];

        for extra_file in extras_path
            .read_dir()
            .unwrap()
            .filter(|x| x.as_ref().unwrap().file_type().unwrap().is_file())
        {
            let extra_file = extra_file.unwrap().path();
            extras.push(
                extra_file
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            );
        }

        chapters.push(Chapter {
            name: chapter_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            notes,
            extras,
        })
    }

    Ok(Project { path, chapters })
}
