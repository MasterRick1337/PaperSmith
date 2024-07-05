use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[path = "dropdown.rs"]
mod dropdown;

use dropdown::Dropdown;
use dropdown::DropdownType;

#[function_component(SideBar)]
pub fn sidebar() -> Html {
    html! {
        <>
            <div id="file-explorer">

                <div class="chapter-title text-ellipsis whitespace-nowrap overflow-hidden">
                    {"Book 1"}
                    <div class="sidebar-dropdown-icon-container hide-parent-hover">
                        <div class="sidebar-dropdown-icon bg-mantle border-overlay0 hover: text-text mx-1">
                            <Icon icon_id={IconId::LucideEdit3} width={"16px"} height={"16px"}/>
                        </div>
                    </div>
                </div>
                <div class="chapter-list border-l-2 border-[#ccc] pl-2 ml-2">
                    <Chapter title={"Beginning".to_string()}/>
                    <Chapter title={"Middle".to_string()}/>
                    <Chapter title={"End".to_string()}/>
                </div>
            </div>
        </>

    }
}

#[derive(Properties, PartialEq)]
struct ChapterProps {
    pub title: String,
}

#[function_component(Chapter)]
fn chapter(ChapterProps { title }: &ChapterProps) -> Html {
    html! {
        <Dropdown title={title.clone()} open=false dropdown_type={DropdownType::Chapter}>
            <Dropdown title="Notes" open=false dropdown_type={DropdownType::Notes}>{"Test"}</Dropdown>
            <Dropdown title="Extras" open=false dropdown_type={DropdownType::Extras}>{"Test"}</Dropdown>
        </Dropdown>
    }
}
