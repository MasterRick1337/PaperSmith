use serde::Serialize;
use shared::Chapter;
use shared::Project;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[path = "renaming.rs"]
mod renaming;

#[path = "dropdown.rs"]
mod dropdown;

use dropdown::Dropdown;
use dropdown::Type;

#[derive(Properties, PartialEq)]
pub struct SideBarProps {
    pub project: UseStateHandle<Option<Project>>,
}

#[derive(Serialize)]
struct AddChapterArgs {
    path: String,
    name: String,
}

#[function_component(SideBar)]
pub fn sidebar(SideBarProps { project }: &SideBarProps) -> Html {
    let chapter_elements: Vec<Html> = project
        .as_ref()
        .unwrap()
        .chapters
        .iter()
        .map(|chapter| {
            html! { <ChapterComponent chapter={chapter.clone()} project={project.clone()} /> }
        })
        .collect();

    html! {
        <>
            <div id="file-explorer">
                <div class="chapter-title text-ellipsis whitespace-nowrap overflow-hidden">
                    { project.as_ref().unwrap().path.file_name().unwrap().to_string_lossy().into_owned() }
                    <div class="sidebar-dropdown-icon-container hide-parent-hover">
                        <div
                            class="sidebar-dropdown-icon bg-mantle border-overlay0 hover: text-text mx-1"
                        >
                            <Icon icon_id={IconId::LucideEdit3} width="16px" height="16px" />
                        </div>
                    </div>
                </div>
                <div class="chapter-list border-l-2 border-[#ccc] pl-2 ml-2">
                    { for chapter_elements }
                </div>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct ChapterProps {
    pub chapter: Chapter,
    pub project: UseStateHandle<Option<Project>>,
}

#[function_component(ChapterComponent)]
fn chapter(ChapterProps { chapter, project }: &ChapterProps) -> Html {
    let note_elements: Vec<Html> = chapter
        .notes
        .iter()
        .map(|note| html! { <Entry name={note.clone()} /> })
        .collect();

    let extra_elements: Vec<Html> = chapter
        .extras
        .iter()
        .map(|extra_file| html! { <Entry name={extra_file.clone()} /> })
        .collect();

    html! {
        <Dropdown
            title={chapter.name.clone()}
            open=false
            dropdown_type={Type::Chapter}
            project={project.clone()}
        >
            <Dropdown title="Notes" open=false dropdown_type={Type::Notes} project={None}>
                { for note_elements }
            </Dropdown>
            <Dropdown title="Extras" open=false dropdown_type={Type::Extras} project={None}>
                { for extra_elements }
            </Dropdown>
        </Dropdown>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct EntryProps {
    pub name: String,
}

#[function_component(Entry)]
fn entry(EntryProps { name }: &EntryProps) -> Html {
    html! {
        <div class="chapter-title rounded-md hover:bg-sapphire pl-2 hover:text-mantle">
            { name }
        </div>
    }
}
