use std::path::Path;

use deleting::get_delete_callback;
use serde_wasm_bindgen::to_value;
use shared::Chapter;
use shared::Project;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_icons::IconId;

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

#[path = "buttons.rs"]
mod buttons;
use buttons::{ButtonContainer, Props as ButtonProps};

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

    let button_props = vec![
        ButtonProps {
            callback: get_rename_callback(
                name_display.clone(),
                title,
                input_ref.clone(),
                project.clone(),
                RenameKind::Book,
                None,
            ),
            icon: IconId::LucideEdit3,
        },
        ButtonProps {
            callback: on_add_chapter,
            icon: IconId::LucidePlus,
        },
    ];

    html! {
        <>
            <div id="file-explorer" class="select-none outline-none">
                <div
                    class="group items-center flex relative transition text-ellipsis whitespace-nowrap overflow-hidden cursor-default text-xl"
                >
                    <div ref={input_ref.clone()} class="pl-2 mb-1">{ (*name_display).clone() }</div>
                    <ButtonContainer button_props={button_props} />
                </div>
                <div
                    class="text-lg border-l-2 border-r-0 border-y-0 border-solid border-overlay2 pl-2 ml-2"
                >
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
            <Title>
                <div class="pl-5">{ "Contents" }</div>
            </Title>
            <Dropdown
                title="Notes"
                open=false
                dropdown_type={Type::Notes}
                project={Some(project.clone())}
                chapter_index={index}
            >
                { for note_elements }
            </Dropdown>
            <Title onclick={on_extras}>
                <div class="pl-5">{ "Extras" }</div>
            </Title>
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

    let button_props = vec![
        ButtonProps {
            callback: get_rename_callback(
                name_display.clone(),
                title,
                input_ref,
                project.clone(),
                RenameKind::Note,
                Some(*chapter_index),
            ),
            icon: IconId::LucideEdit3,
        },
        ButtonProps {
            callback: get_delete_callback(
                project.clone(),
                name.clone(),
                Some(*chapter_index),
                RenameKind::Note,
            ),
            icon: IconId::LucideTrash2,
        },
    ];
    html! {
        <Title button_props={button_props}>
            <div class="pl-2">{ (*name_display).clone() }</div>
        </Title>
    }
}

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub children: Html,
    #[prop_or_default]
    pub button_props: Vec<ButtonProps>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Title)]
fn title(
    TitleProps {
        children,
        button_props,
        onclick,
    }: &TitleProps,
) -> Html {
    html! {
        <div
            class="group items-center flex relative transition rounded-md my-[1px] hover:bg-sapphire hover:text-mantle"
            onclick={onclick}
        >
            { children.clone() }
            <ButtonContainer button_props={(*button_props).clone()} />
        </div>
    }
}
//rounded-md hover:bg-green pl-2 hover:text-mantle">
//text-text text-ellipsis whitespace-nowrap overflow-hidden cursor-default text-xl"
//rounded-md my-[1px] pl-5 hover:bg-sapphire hover:text-mantle"
//rounded-md my-[1px] content-center transition text-subtext1 hover:bg-mauve hover:text-mantle"
//rounded-md my-[1px] pl-5 hover:bg-sapphire hover:text-mantle"
