use web_sys::{window, Range};
use yew::prelude::*;
use yew_hooks::use_interval;
use yew_icons::{Icon, IconId};

fn apply_style(range: &Range, opening_style: &str, closing_style: &str) {
    let document = window().unwrap().document().unwrap();

    let selected_text = range.to_string();

    let new_text = format!("{opening_style}{selected_text}{closing_style}");

    let text_node = document.create_text_node(&new_text);
    range.delete_contents().unwrap();
    range.insert_node(&text_node).unwrap();

    // Clear the current selection
    let window = window().unwrap();
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
pub struct StyleButtonProps {
    pub range: UseStateHandle<Option<Range>>,
    pub icon: IconId,
    pub title: String,
    pub opening_style: String,
    pub closing_style: String,
    pub class_name: String,
}

#[function_component(StyleButton)]
pub fn style_button(style_props: &StyleButtonProps) -> Html {
    let range_state = style_props.range.clone();
    let opening_style = style_props.opening_style.clone();
    let closing_style = style_props.closing_style.clone();

    let onclick = Callback::from(move |_| {
        if let Some(range) = range_state.as_ref() {
            apply_style(range, &opening_style, &closing_style);
        }
    });

    let combined_class = classes!("menubar-icon", style_props.class_name.clone());

    html! {
        <Icon
            icon_id={style_props.icon}
            width={"2em".to_owned()}
            height={"2em".to_owned()}
            class={combined_class}
            title={style_props.title.clone()}
            onclick={onclick}
        />
    }
}

#[function_component(TextStylingControls)]
pub fn text_styling_controls() -> Html {
    let range_state = use_state(|| None);
    let inner_range_state = range_state.clone();
    use_interval(
        move || {
            let window = window().unwrap();
            let document = window.document().unwrap();

            if let Some(selection) = document.get_selection().unwrap() {
                if let Ok(range) = selection.get_range_at(0) {
                    let common_ancestor = range.common_ancestor_container().unwrap();
                    let notepad = document.get_element_by_id("notepad-textarea-edit").unwrap();

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
            <StyleButton
                class_name={"bold-button".to_string()}
                range={range_state.clone()}
                icon={IconId::LucideBold}
                title="Bold"
                opening_style="**"
                closing_style="**"
            />
            <StyleButton
                class_name={"italic-button".to_string()}
                range={range_state.clone()}
                icon={IconId::LucideItalic}
                title="Italic"
                opening_style="_"
                closing_style="_"
            />
            <StyleButton
                class_name={"underline-button".to_string()}
                range={range_state.clone()}
                icon={IconId::LucideUnderline}
                title="Underline"
                opening_style="<u>"
                    closing_style="</u>"
            />
            <StyleButton
                class_name={"highlight-button".to_string()}
                range={range_state.clone()}
                icon={IconId::LucideHighlighter}
                title="Highlighter"
                opening_style="<mark>"
                closing_style="</mark>"
            />
        </div>
    }
}