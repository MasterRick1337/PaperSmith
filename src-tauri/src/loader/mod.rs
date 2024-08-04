use std::fs;
use std::path::PathBuf;

use shared::{Chapter, Project};

pub fn parse_project(path: PathBuf) -> Option<Project> {
    let mut file_path = path.clone();
    file_path.push(".papersmith.json");

    if !(file_path.exists() && file_path.is_file()) {
        return None;
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
                notes.push(note.file_stem().unwrap().to_string_lossy().into_owned());
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
        });
    }

    Some(Project { path, chapters })
}
