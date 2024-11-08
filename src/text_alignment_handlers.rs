use wasm_bindgen::JsCast;
use web_sys::{window, Range};
use yew::prelude::*;
use yew_hooks::use_interval;
use yew_icons::{Icon, IconId};

/*
TODO: First line is no div!
TODO: Revise cursor handeling when applying text alignment?
TODO: Text alignment not working properly when selecting two or more lines
TODO: Fixing that when you press alignment change multiple times without selecting something or beeing in an empty line, it does noting.
*/

fn apply_alignment_on_range(range: &Range, alignment: &str) {
    let window = window().expect("should have a Window");

    let container = range.start_container().unwrap();

    // Navigate to parent div if necessary
    let container = if container.node_type() != web_sys::Node::ELEMENT_NODE
        || container.node_name().to_lowercase() != "div"
    {
        container.parent_node().unwrap()
    } else {
        container
    };

    let container: web_sys::HtmlElement = container.unchecked_into();

    let new_container = document.create_element("div").unwrap();
    new_container
        .set_attribute("style", &format!("text-align: {alignment};"))
        .unwrap();

    // Put inner content of old div into new div
    let content = container.inner_html();

    if content.trim().is_empty() {
        let placeholder = document.create_text_node("\u{00A0}");
        new_container.append_child(&placeholder).unwrap();
    } else {
        new_container.set_inner_html(&content);
    }

    // Replace old container with new one
    container.replace_with_with_node_1(&new_container).unwrap();

    range.delete_contents().unwrap();
    range.insert_node(&new_container).unwrap();

    let selection = window.get_selection().unwrap().unwrap();
    selection.remove_all_ranges().unwrap();
    selection.add_range(range).unwrap();
    range.collapse();
}

#[derive(Properties, PartialEq)]
pub struct TextAlignmentProps {
    pub text_alignment: UseStateHandle<String>,
}

#[derive(Properties, PartialEq)]
pub struct AlignmentButtonProps {
    pub range: UseStateHandle<Option<Range>>,
    pub icon: IconId,
    pub title: String,
    pub align: String,
    pub class_name: String,
}

#[function_component(AlignmentButton)]
pub fn alignment_button(align_props: &AlignmentButtonProps) -> Html {
    let align = align_props.align.clone();
    let range_state = align_props.range.clone();

    let onclick = Callback::from(move |_| {
        let range = range_state.clone();
        if let Some(range) = range.as_ref() {
            apply_alignment_on_range(range, &align);
        }
    });

    let combined_class = classes!("menubar-icon", align_props.class_name.clone());

    html! {
        <Icon
            icon_id={align_props.icon}
            width={"2em".to_owned()}
            height={"2em".to_owned()}
            class={combined_class}
            title={align_props.title.clone()}
            onclick={onclick}
        />
    }
}

#[function_component(TextAlignmentControls)]
pub fn text_alignment_controls(
    TextAlignmentProps { text_alignment: _ }: &TextAlignmentProps,
) -> Html {
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

                    // web_sys::console::log_1(&format!("Common Ancestor: {:?}", common_ancestor).into());
                    // web_sys::console::log_1(
                    //     &format!("Notepad HTML: {:?}", notepad.inner_html()).into(),
                    // );

                    let mut is_within = false;
                    let mut node = common_ancestor;
                    while let Some(parent) = node.parent_node() {
                        // web_sys::console::log_1(&format!("Checking parent: {:?}", parent).into());
                        if parent.is_same_node(Some(&notepad)) {
                            is_within = true;
                            break;
                        }
                        node = parent;
                    }

                    // web_sys::console::log_1(&format!("Is within notepad: {}", is_within).into());
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
        <div class="text-alignment-changer">
            <AlignmentButton
                class_name={"align-center-button".to_string()}
                range={range_state.clone()}
                icon={IconId::LucideAlignCenter}
                title="Align Center"
                align="center"
            />
            <AlignmentButton
                class_name={"align-justify-button".to_string()}
                range={range_state.clone()}
                icon={IconId::LucideAlignJustify}
                title="Align Justify"
                align="justify"
            />
            <AlignmentButton
                class_name={"align-left-button".to_string()}
                range={range_state.clone()}
                icon={IconId::LucideAlignLeft}
                title="Align Left"
                align="left"
            />
            <AlignmentButton
                class_name={"align-right-button".to_string()}
                range={range_state.clone()}
                icon={IconId::LucideAlignRight}
                title="Align Right"
                align="right"
            />
        </div>
    }
}

