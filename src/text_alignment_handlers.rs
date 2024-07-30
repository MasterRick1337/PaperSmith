use yew::prelude::*;
use yew_icons::{Icon, IconId};
use web_sys::window;
use yew::events::MouseEvent;



fn apply_alignment(alignment: &str) {
    let window = window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    if let Some(selection) = document.get_selection().expect("should have a Selection") {
        if let Some(range) = selection.get_range_at(0).ok() {
            let common_ancestor = range.common_ancestor_container().unwrap();
            let notepad = document.get_element_by_id("notepad-textarea").unwrap();

            web_sys::console::log_1(&format!("Common Ancestor: {:?}", common_ancestor).into());
            web_sys::console::log_1(&format!("Notepad HTML: {:?}", notepad.inner_html()).into());

            let mut is_within = false;
            let mut node = common_ancestor.clone();
            while let Some(parent) = node.parent_node() {
                web_sys::console::log_1(&format!("Checking parent: {:?}", parent).into());
                if parent.is_same_node(Some(&notepad)) {
                    is_within = true;
                    break;
                }
                node = parent;
            }

            web_sys::console::log_1(&format!("Is within notepad: {}", is_within).into());

            if is_within {
                let container = document.create_element("div").unwrap();
                container.set_attribute("style", &format!("text-align: {};", alignment)).unwrap();

                let content = range.extract_contents().unwrap();
                container.append_child(&content).unwrap();
                range.insert_node(&container).unwrap();

                notepad.append_child(&container).unwrap();

                selection.remove_all_ranges().unwrap();
            } else {
                web_sys::console::log_1(&"Range is not within notepad".into());
            }
        } else {
            web_sys::console::log_1(&"Range not found".into());
        }
    } else {
        web_sys::console::log_1(&"Selection not found".into());
    }
}



// TODO: Add that it only applies to selected text or text that is abut to be written
pub fn align_left() -> Callback<MouseEvent> {
    Callback::from(move |_| apply_alignment("left"))
}

pub fn align_center() -> Callback<MouseEvent> {
    Callback::from(move |_| apply_alignment("center"))
}

pub fn align_right() -> Callback<MouseEvent> {
    Callback::from(move |_| apply_alignment("right"))
}

pub fn align_justify() -> Callback<MouseEvent> {
    Callback::from(move |_| apply_alignment("justify"))
}



#[derive(Properties, PartialEq)]
pub struct TextAlignmentProps {
    pub text_alignment: UseStateHandle<String>,
}

#[function_component(TextAlignmentControls)]
pub fn font_size_controls(TextAlignmentProps { text_alignment: _ }: &TextAlignmentProps) -> Html {
    let on_align_left = align_left();
    let on_align_center = align_center();
    let on_align_right = align_right();
    let on_align_justify = align_justify();

    html! {
        <div class="text-alignment-changer">
            <Icon icon_id={IconId::LucideAlignCenter} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon" title="Align Center" onclick={on_align_center}/>
            <Icon icon_id={IconId::LucideAlignJustify} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon" title="Align Justify" onclick={on_align_justify}/>
            <Icon icon_id={IconId::LucideAlignLeft} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon" title="Align Left" onclick={on_align_left}/>
            <Icon icon_id={IconId::LucideAlignRight} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon" title="Align Right" onclick={on_align_right}/>
        </div>
    }
}