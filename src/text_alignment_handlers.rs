
use web_sys::{window, Range};
use yew::prelude::*;
use yew_hooks::use_interval;
use yew_icons::{Icon, IconId};

fn apply_alignment_on_range(range: &Range, alignment: &str) {
    let window = window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    let notepad = document.get_element_by_id("notepad-textarea").unwrap();
    let container = document.create_element("div").unwrap();
    container
        .set_attribute("style", &format!("text-align: {alignment};"))
        .unwrap();
    
    gloo_console::log!("range", range);

    let content = range.extract_contents().unwrap();

    if content.text_content().is_none() || content.text_content().unwrap().trim().is_empty() {
        let placeholder = document.create_text_node("Placeholder Text");
        container.append_child(&placeholder).unwrap();
    } else {
        container.append_child(&content).unwrap();
    }

    range.delete_contents().unwrap();
    range.insert_node(&container).unwrap();
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

    html! {
        <Icon 
            icon_id={align_props.icon} 
            width={"2em".to_owned()} 
            height={"2em".to_owned()} 
            class="menubar-icon" 
            title={align_props.title.clone()} 
            onclick={onclick}
        />
    }
}

#[function_component(TextAlignmentControls)]
pub fn font_size_controls(TextAlignmentProps { text_alignment: _ }: &TextAlignmentProps) -> Html {

    let range_state = use_state(|| None);
    let inner_range_state = range_state.clone();
    use_interval(
        move || {
            let window = window().expect("should have a Window");
            let document = window.document().expect("should have a Document");

            if let Some(selection) = document.get_selection().expect("should have a Selection") {
                if let Some(range) = selection.get_range_at(0).ok() {
                    let common_ancestor = range.common_ancestor_container().unwrap();
                    let notepad = document.get_element_by_id("notepad-textarea").unwrap();

                    // web_sys::console::log_1(&format!("Common Ancestor: {:?}", common_ancestor).into());
                    // web_sys::console::log_1(
                    //     &format!("Notepad HTML: {:?}", notepad.inner_html()).into(),
                    // );

                    let mut is_within = false;
                    let mut node = common_ancestor.clone();
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
                        if let Some(range) = selection.get_range_at(0).ok() {
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
            <AlignmentButton range={range_state.clone()} icon={IconId::LucideAlignCenter} title="Align Center" align="center" />
            <AlignmentButton range={range_state.clone()} icon={IconId::LucideAlignJustify} title="Align Justify" align="justify" />
            <AlignmentButton range={range_state.clone()} icon={IconId::LucideAlignLeft} title="Align Left" align="left" />
            <AlignmentButton range={range_state.clone()} icon={IconId::LucideAlignRight} title="Align Right" align="right" />
        </div>
    }
}
