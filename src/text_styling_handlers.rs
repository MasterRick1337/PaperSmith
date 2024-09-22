use web_sys::{window, Range};
use yew::prelude::*;
use yew_hooks::use_interval;
use yew_icons::{Icon, IconId};
use wasm_bindgen::JsCast;

pub fn apply_styling_on_range(range: &Range, style: &str) {
    let window = window().expect("should have a Window");

    let container = range.start_container().unwrap();

    // Navigate to parent div if necessary
    let container = if container.node_type() != web_sys::Node::ELEMENT_NODE || container.node_name().to_lowercase() != "div" {
        container.parent_node().unwrap()
    } else {
        container
    };

    let container: web_sys::HtmlElement = container.unchecked_into();

    match style {
        "bold" => container.set_attribute("style", "font-weight: bold;").unwrap(),
        "italic" => container.set_attribute("style", "font-style: italic;").unwrap(),
        "underline" => container.set_attribute("style", "text-decoration: underline;").unwrap(),
        _ => (),
    }

    let selection = window.get_selection().unwrap().unwrap();
    selection.remove_all_ranges().unwrap();
    selection.add_range(range).unwrap();
    range.collapse();
}

#[derive(Properties, PartialEq)]
pub struct TextStylingProps {
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
        let range = (*range_state).clone();
        if let Some(range) = range.as_ref() {
            apply_styling_on_range(range, &style);
        }
    });

    html! {
        <Icon
            icon_id={style_props.icon}
            width={"2em".to_owned()}
            height={"2em".to_owned()}
            class="menubar-icon"
            title={style_props.title.clone()}
            onclick={onclick}
        />
    }
}

#[function_component(TextStylingControls)]
pub fn text_styling_controls(TextStylingProps { text_styling: _}: &TextStylingProps) -> Html {
    let range_state = use_state(|| None);
    let inner_range_state = range_state.clone();
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
                            inner_range_state.set(Some(range));
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
