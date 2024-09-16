use web_sys::{window, range};
use yew::prelude::*;
use yew_hooks::use_interval;
use yew_icons::{icon, iconid};
use wasm_bindgen::jscast;

/*
todo: first line is no div!
todo: revise cursor handeling when applying text alignment?
todo: text alignment not working properly when selecting two or more lines  
todo: fixing that when you press alignment change multiple times without selecting something or beeing in an empty line, it does noting.
*/
fn apply_alignment_on_range(range: &range, alignment: &str) {
    let window = window().expect("should have a window");
    let document = window.document().expect("should have a document");
    let container = range.start_container().unwrap();

    // navigate to parent div if necessary
    let container = if container.node_type() != web_sys::node::element_node || container.node_name().to_lowercase() != "div" {
        container.parent_node().unwrap()
    } else {
        container
    };

    let container: web_sys::htmlelement = container.unchecked_into();

    let new_container = document.create_element("div").unwrap();
    new_container.set_attribute("style", &format!("text-align: {alignment};")).unwrap();

    // put inner content of old div into new div
    let content = container.inner_html();

    if content.trim().is_empty() {
        let placeholder = document.create_text_node("\u{00a0}");
        new_container.append_child(&placeholder).unwrap();
    } else {
        new_container.set_inner_html(&content);
    }

    // replace old container with new one
    container.replace_with_with_node_1(&new_container).unwrap();

    range.delete_contents().unwrap();

    range.insert_node(&new_container).unwrap();

    let selection = window.get_selection().unwrap().unwrap();
    selection.remove_all_ranges().unwrap();
    selection.add_range(range).unwrap();
    range.collapse();
    range.insert_node(&container).unwrap();
}


#[derive(properties, partialeq)]
pub struct textalignmentprops {
    pub text_alignment: usestatehandle<string>,
}

#[derive(properties, partialeq)]
pub struct alignmentbuttonprops {
    pub range: usestatehandle<option<range>>,
    pub icon: iconid,
    pub title: string,
    pub align: string,
}

#[function_component(alignmentbutton)]
pub fn alignment_button(align_props: &alignmentbuttonprops) -> html {
    let align = align_props.align.clone();
    let range_state = align_props.range.clone();

    let onclick = callback::from(move |_| {
        let range = range_state.clone();
        if let some(range) = range.as_ref() {
            apply_alignment_on_range(range, &align);
        }
    });

    html! {
        <icon
            icon_id={align_props.icon}
            width={"2em".to_owned()}
            height={"2em".to_owned()}
            class="menubar-icon"
            title={align_props.title.clone()}
            onclick={onclick}
        />
    }
}

#[function_component(textalignmentcontrols)]
pub fn font_size_controls(textalignmentprops { text_alignment: _ }: &textalignmentprops) -> html {
    let range_state = use_state(|| none);
    let inner_range_state = range_state.clone();
    use_interval(
        move || {
            let window = window().expect("should have a window");
            let document = window.document().expect("should have a document");

            if let some(selection) = document.get_selection().expect("should have a selection") {
                if let ok(range) = selection.get_range_at(0) {
                    let common_ancestor = range.common_ancestor_container().unwrap();
                    let notepad = document.get_element_by_id("notepad-textarea").unwrap();

                    // web_sys::console::log_1(&format!("common ancestor: {:?}", common_ancestor).into());
                    // web_sys::console::log_1(
                    //     &format!("notepad html: {:?}", notepad.inner_html()).into(),
                    // );

                    let mut is_within = false;
                    let mut node = common_ancestor;
                    while let some(parent) = node.parent_node() {
                        // web_sys::console::log_1(&format!("checking parent: {:?}", parent).into());
                        if parent.is_same_node(some(&notepad)) {
                            is_within = true;
                            break;
                        }
                        node = parent;
                    }

                    // web_sys::console::log_1(&format!("is within notepad: {}", is_within).into());
                    if is_within {
                        if let ok(range) = selection.get_range_at(0) {
                            inner_range_state.set(some(range));
                        }
                    }
                }
            }
        },
        10,
    );

    html! {
        <div class="text-alignment-changer">
            <alignmentbutton
                range={range_state.clone()}
                icon={iconid::lucidealigncenter}
                title="align center"
                align="center"
            />
            <alignmentbutton
                range={range_state.clone()}
                icon={iconid::lucidealignjustify}
                title="align justify"
                align="justify"
            />
            <alignmentbutton
                range={range_state.clone()}
                icon={iconid::lucidealignleft}
                title="align left"
                align="left"
            />
            <alignmentbutton
                range={range_state.clone()}
                icon={iconid::lucidealignright}
                title="align right"
                align="right"
            />
        </div>
    }
}

