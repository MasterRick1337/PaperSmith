use yew::prelude::*;

#[path = "dropdown.rs"]
mod dropdown;

use dropdown::Dropdown;

#[function_component(SideBar)]
pub fn sidebar() -> Html {
    html! {
        <>
            <div id="file-explorer">
                <div class="book-title">{"Book 1"}</div>
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
        <Dropdown title={title.clone()} open=false>
            <Dropdown title="Notes" open=false>{"Test"}</Dropdown>
            <Dropdown title="Extras" open=false>{"Test"}</Dropdown>
        </Dropdown>
    }
}
