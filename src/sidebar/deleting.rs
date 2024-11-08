use serde_wasm_bindgen::to_value;
use shared::Project;
use wasm_bindgen_futures::spawn_local;
use yew::{Callback, MouseEvent, UseStateHandle};

use crate::app::{invoke, wizard::PathArgs};

use super::renaming::RenameKind;

pub fn get_delete_callback(
    project: UseStateHandle<Option<Project>>,
    title: String,
    chapter_index: Option<usize>,
    kind: RenameKind,
) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        e.stop_propagation();

        let mut temp_project = (*project.as_ref().unwrap()).clone();
        let mut delete_path = temp_project.path.clone();
        match kind {
            RenameKind::Book => {}
            RenameKind::Chapter => {
                delete_path.push("Chapters");
                delete_path.push(title.clone());

                temp_project
                    .chapters
                    .retain(|chapter| chapter.name != title);
            }
            RenameKind::Note => {
                delete_path.push("Chapters");

                temp_project.chapters[chapter_index.unwrap()]
                    .notes
                    .retain(|note| *note != title);
                let chapter_name = project.as_ref().unwrap().chapters[chapter_index.unwrap()]
                    .name
                    .clone();
                delete_path.push(chapter_name);
                delete_path.push("Notes");
                delete_path.push(title.clone());
                delete_path.set_extension("md");
            }
        }
        spawn_local(async move {
            invoke(
                "delete_path",
                to_value(&PathArgs {
                    path: delete_path.to_str().unwrap().to_string(),
                })
                .unwrap(),
            )
            .await;
        });
        project.set(Some(temp_project));
    })
}
