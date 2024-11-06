use std::path::Path;

use deleting::get_delete_callback;
use serde_wasm_bindgen::to_value;
use shared::Chapter;
use shared::Project;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_icons::{Icon, IconId};

#[path = "renaming.rs"]
mod renaming;
use renaming::get_rename_callback;
use renaming::RenameKind;

#[path = "deleting.rs"]
mod deleting;

#[path = "dropdown.rs"]
mod dropdown;

use dropdown::Dropdown;
use dropdown::Type;

use crate::app::invoke;
use crate::app::wizard::PathArgs;

#[derive(Properties, PartialEq)]
pub struct SideBarProps {
    pub project: UseStateHandle<Option<Project>>,
}

fn get_file_name(path: &Path) -> String {
    path.to_str()
        .unwrap()
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or_else(|| path.to_str().unwrap())
        .to_string()
}

#[function_component(SideBar)]
pub fn sidebar(SideBarProps { project }: &SideBarProps) -> Html {
    let input_ref = use_node_ref();
    let title = use_state(|| get_file_name(&(*project).as_ref().unwrap().path));
    let name_display = use_state(|| html! { (*title).clone() });
    let chapters = use_state(Vec::<VNode>::new);

    {
        let title = title.clone();
        let project = project.clone();
        let name_display = name_display.clone();
        use_effect_with(project.clone(), move |_| {
            title.set(get_file_name(&(*project).as_ref().unwrap().path));
            name_display.set(html! { get_file_name(&(*project).as_ref().unwrap().path) });
        });
    }

    {
        let chapters = chapters.clone();
        let project = (*project).clone();

        use_effect_with((*project).clone(), move |_| {
            chapters.set(Vec::new());

            if let Some(project_data) = project.as_ref() {
                let new_chapters = project_data
                    .chapters
                    .iter()
                    .enumerate()
                    .map(|(index, chapter)| {
                        html! {
                            <ChapterComponent
                                key={chapter.name.clone()}
                                chapter={chapter.clone()}
                                index={index}
                                project={project.clone()}
                            />
                        }
                    })
                    .collect::<Vec<VNode>>();

                chapters.set(new_chapters);
            }
        });
    }

    let on_add_chapter = {
        let project = project.clone();
        Callback::from(move |_| {
            let project = project.clone();
            spawn_local(async move {
                let mut check_path = project.as_ref().unwrap().path.clone();
                check_path.push("Chapters");
                check_path.push("Untitled");
                let mut index = 1;
                while !invoke(
                    "can_create_path",
                    to_value(&PathArgs {
                        path: check_path.to_str().unwrap().to_string().clone(),
                    })
                    .unwrap(),
                )
                .await
                .as_string()
                .unwrap()
                .is_empty()
                {
                    check_path.pop();
                    check_path.push("Untitled".to_string() + &index.to_string());
                    index += 1;
                }
                invoke(
                    "add_chapter",
                    to_value(&PathArgs {
                        path: check_path.to_str().unwrap().to_string(),
                    })
                    .unwrap(),
                )
                .await;
                let mut temp_project = project.as_ref().unwrap().clone();
                temp_project.chapters.push(Chapter {
                    name: check_path.file_name().unwrap().to_string_lossy().into(),
                    notes: Vec::new(),
                    extras: Vec::new(),
                });
                project.set(Some(temp_project));
            });
        })
    };

    html! {
        <>
            <div id="file-explorer" class="select-none">
                <div
                    class="chapter-title text-ellipsis whitespace-nowrap overflow-hidden cursor-default"
                >
                    <div ref={input_ref.clone()} class="text-lg">{ (*name_display).clone() }</div>
                    <div class="sidebar-dropdown-icon-container hide-parent-hover">
                        <div
                            class="sidebar-dropdown-icon bg-mantle border-overlay0 hover: text-text mx-1"
                            onclick={get_rename_callback(name_display.clone(), title, input_ref, project.clone(),RenameKind::Book,None)}
                        >
                            <Icon icon_id={IconId::LucideEdit3} width="16px" height="16px" />
                        </div>
                        <div
                            class="sidebar-dropdown-icon bg-mantle border-overlay0 hover: text-text mx-1"
                            onclick={on_add_chapter}
                        >
                            <Icon icon_id={IconId::LucidePlus} width="16px" height="16px" />
                        </div>
                    </div>
                </div>
                <div class="chapter-list border-l-2 border-[#ccc] pl-2 ml-2">
                    { for (*chapters).clone() }
                </div>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct ChapterProps {
    pub chapter: Chapter,
    pub index: usize,
    pub project: UseStateHandle<Option<Project>>,
}

#[function_component(ChapterComponent)]
fn chapter(
    ChapterProps {
        chapter,
        index,
        project,
    }: &ChapterProps,
) -> Html {
    let note_elements: Vec<Html> = chapter
        .notes
        .iter()
        .map(|note| {
            html! {
                <Entry
                    key={note.clone()}
                    name={note.clone()}
                    project={project.clone()}
                    chapter_index={index}
                // Use the index here
                />
            }
        })
        .collect();
    let on_extras = {
        let project = project.clone();
        let index = *index;
        Callback::from(move |_| {
            let project = project.clone();
            spawn_local(async move {
                let project_clone = project.as_ref().unwrap().clone();
                let mut extras_path = project_clone.path.clone();
                extras_path.push("Chapters");
                extras_path.push(project_clone.chapters[index].name.clone());
                extras_path.push("Extras");
                invoke(
                    "open_explorer",
                    to_value(&PathArgs {
                        path: extras_path.to_str().unwrap().to_string(),
                    })
                    .unwrap(),
                )
                .await;
            });
        })
    };

    html! {
        <Dropdown
            title={chapter.name.clone()}
            open=false
            dropdown_type={Type::Chapter}
            project={project.clone()}
        >
            <div
                class="chapter-title rounded-md my-[1px] pl-5 hover:bg-sapphire hover:text-mantle"
            >
                { "Contents" }
            </div>
            <Dropdown
                title="Notes"
                open=false
                dropdown_type={Type::Notes}
                project={Some(project.clone())}
                chapter_index={index}
            >
                { for note_elements }
            </Dropdown>
            <div
                class="chapter-title rounded-md my-[1px] pl-5 hover:bg-sapphire hover:text-mantle"
                onclick={on_extras}
            >
                { "Extras" }
            </div>
        </Dropdown>
    }
}

#[derive(Properties, PartialEq)]
pub struct EntryProps {
    pub name: String,
    pub project: UseStateHandle<Option<Project>>,
    pub chapter_index: usize,
}

#[function_component(Entry)]
fn entry(
    EntryProps {
        name,
        project,
        chapter_index,
    }: &EntryProps,
) -> Html {
    let input_ref = use_node_ref();
    let title = use_state(|| name.clone());
    let name_display = use_state(|| html! { name.clone() });
    html! {
        <div class="chapter-title rounded-md hover:bg-green pl-2 hover:text-mantle">
            { (*name_display).clone() }
            <div class="sidebar-dropdown-icon-container hide-parent-hover">
                <div
                    class="sidebar-dropdown-icon bg-mantle border-overlay0 hover: text-text mx-1"
                    onclick={get_rename_callback(name_display, title.clone(), input_ref,project.clone(), RenameKind::Note, Some(*chapter_index) )}
                >
                    <Icon icon_id={IconId::LucideEdit3} width="16px" height="16px" />
                </div>
                <div
                    class="sidebar-dropdown-icon bg-mantle border-overlay0 hover: text-text mx-1"
                    onclick={get_delete_callback(project.clone(), name.clone(), Some(*chapter_index), RenameKind::Note)}
                >
                    <Icon icon_id={IconId::LucideTrash2} width="16px" height="16px" />
                </div>
            </div>
        </div>
    }
}
