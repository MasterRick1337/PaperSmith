use std::path::PathBuf;

use gloo_timers::callback::Timeout;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use shared::Project;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{virtual_dom::VNode, Callback, MouseEvent, NodeRef, UseStateHandle};

use crate::app::invoke;

#[derive(Serialize)]
struct RenameArgs {
    path: PathBuf,
    old: String,
    new: String,
}

#[derive(Serialize)]
struct PathArgs {
    path: String,
}

#[derive(Clone)]
pub enum RenameKind {
    Book,
    Chapter,
    Note,
}

pub fn get_rename_callback(
    display: UseStateHandle<VNode>,
    title: UseStateHandle<String>,
    input_ref: NodeRef,
    project: UseStateHandle<Option<Project>>,
    kind: RenameKind,
    chapter_index: Option<usize>,
) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        e.stop_propagation();
        let kind = kind.clone();
        let input_ref = input_ref.clone();
        let display = display.clone();
        let onblur = {
            let name_display = display.clone();
            let title = title.clone();
            Callback::from(move |_| {
                let name_display = name_display.clone();
                let title = title.clone();
                name_display.set(html!(<>{ (*title).clone() }</>));
            })
        };
        let onenter = {
            let name_display = display.clone();
            let input_ref = input_ref.clone();
            let title = title.clone();
            let project = project.clone();
            Callback::from(move |e: KeyboardEvent| {
                let kind = kind.clone();
                let input_ref = input_ref.clone();
                let name_display = name_display.clone();
                let title = title.clone();
                let project = project.clone();
                if e.key() == "Enter" {
                    if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                        let value = input.value();
                        let mut note_value = value.clone();
                        note_value.push_str(".md");

                        spawn_local(async move {
                            let mut path = project.as_ref().unwrap().clone().path;
                            match kind {
                                RenameKind::Book => {
                                    path.pop();
                                }
                                RenameKind::Chapter => {
                                    path.push("Chapters");
                                }
                                RenameKind::Note => {
                                    path.push("Chapters");
                                    let chapter_name = project.as_ref().unwrap().chapters
                                        [chapter_index.unwrap()]
                                    .name
                                    .clone();
                                    path.push(chapter_name);
                                    path.push("Notes");
                                }
                            }
                            let mut check_path = path.clone().join(value.clone());

                            if matches!(kind, RenameKind::Note) {
                                check_path.set_extension("md");
                            };

                            let result = invoke(
                                "can_create_path",
                                to_value(&PathArgs {
                                    path: check_path.to_str().unwrap().into(),
                                })
                                .unwrap(),
                            )
                            .await
                            .as_string()
                            .unwrap()
                            .is_empty();

                            if result {
                                let args = RenameArgs {
                                    path,
                                    old: (*title).clone() + ".md",
                                    new: note_value.clone(),
                                };
                                let args = to_value(&args).unwrap();
                                invoke("rename_path", args).await;
                                title.set(value.clone());
                                name_display.set(html!(<>{ value.clone() }</>));
                                let mut temp_project = project.clone().as_ref().unwrap().clone();
                                match kind {
                                    RenameKind::Book => temp_project.path = check_path,
                                    RenameKind::Chapter => {
                                        for chapter in &mut temp_project.chapters {
                                            if chapter.name == *title {
                                                chapter.name.clone_from(&value);
                                            }
                                        }
                                    }
                                    RenameKind::Note => {
                                        for note in &mut temp_project.chapters
                                            [chapter_index.unwrap()]
                                        .notes
                                        .iter_mut()
                                        {
                                            if *note == *title {
                                                (*note).clone_from(&value);
                                            }
                                        }
                                    }
                                }
                                project.set(Some(temp_project.clone()));
                            }
                        });
                    }
                }
            })
        };
        display.set(html!(
            <input
                onblur={onblur}
                onkeypress={onenter}
                ref={input_ref.clone()}
                class="bg-inherit text-inherit"
            />
        ));

        let _timeout = Timeout::new(1, {
            let title = title.clone();
            move || {
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    input.set_value(&title);
                    let _ = input.focus();
                    input.select();
                }
            }
        })
        .forget();
    })
}
