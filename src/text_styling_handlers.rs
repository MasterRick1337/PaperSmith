use web_sys::{window, Range};
use yew::prelude::*;
use yew_hooks::use_interval;
use yew_icons::{Icon, IconId};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub fn apply_styling_on_range(range: &Range, style: &str) {
    let document = window().expect("should have a Document").document().expect("should have a Document");

    let mut selected_text = range.to_string();
    if style == "bold" {
        selected_text = format!("**{}**", selected_text);
    } else if style == "italic" {
        selected_text = format!("*{}*", selected_text);
    } else if style == "underline" {
        selected_text = format!("_{}_", selected_text);
    }

    // Replace the selected text with the formatted text
    let range_start = range.start_container().unwrap();
    let range_end = range.end_container().unwrap();

    let container = if range_start.node_type() != web_sys::Node::ELEMENT_NODE || range_start.node_name().to_lowercase() != "div" {
        range_start.parent_node().unwrap()
    } else {
        range_start
    };

    let container: web_sys::HtmlElement = container.unchecked_into();
    let new_html = container.inner_html().replace(&range.to_string(), &selected_text);

    container.set_inner_html(&new_html);
    
    let selection = document.get_selection().expect("should have a Selection");
    selection.remove_all_ranges().unwrap();
    selection.add_range(range).unwrap();
    range.collapse();
}

#[derive(Properties, PartialEq)]
pub struct StyleAlignmentProps {
    pub text_styling: UseStateHandle<String>,
}

#[derive(Properties, PartialEq)]
pub struct StylingButtonProps {
    pub range: UseStateHandle<Option<Range>>,
    pub icon: IconId,
    pub title: String,
    pub style: String,
}

#[function_component(StylingButton)]
pub fn styling_button(style_props: &StylingButtonProps) -> Html {
    let style = style_props.style.clone();
    let range_state = style_props.range.clone();

    let onclick = Callback::from(move |_| {
        let range = range_state.clone();
        if let Some(range) = range.as_ref() {
            apply_styling_on_range(range, &style);
        }
    });

    html! {
        <Icon
            icon_id={style_props.icon}
            width={"2em".to_owned()}
            height={"2em".to_owned()}
            class="menubar_icon"
            title={style_props.title.clone()}
            onclick={onclick}
        />
    }
}

#[function_component(TextStylingControls)]
pub fn text_styling_controls() -> Html {
    let range_state = use_state(|| None);
    use_interval(
        move || {
            let window = window().expect("should have a Window");
            let document = window.document().expect("should have a Document");

            if let Some(selection) = document.get_selection().expect("should have a Selection") {
                if let Ok(range) = selection.get_range_at(0) {
                    let common_ancestor = range.common_ancestor_container().unwrap();
                    let notepad = document.get_element_by_id("notepad-textarea").unwrap();

                    let mut is_within = false;
                    let mut node = common_ancestor;
                    while let Some(parent) = node.parent_node() {
                        if parent.is_same_node(Some(&notepad)) {
                            is_within = true;
                            break;
                        }
                        node = parent;
                    }

                    if is_within {
                        if let Ok(range) = selection.get_range_at(0) {
                            range_state.set(Some(range));
                        }
                    }
                }
            }
        },
        10,
    );

    html! {
        <div class="text-styling-changer">
            <StylingButton
                range={range_state.clone()}
                icon={IconId::LucideBold}
                title="Bold"
                style="bold"
            />
            <StylingButton
                range={range_state.clone()}
                icon={IconId::LucideItalic}
                title="Italic"
                style="italic"
            />
            <StylingButton
                range={range_state.clone()}
                icon={IconId::LucideUnderline}
                title="Underline"
                style="underline"
            />
        </div>
    }
}
