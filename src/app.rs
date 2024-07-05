use markdown::{self, to_html};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    console::{info_1, log_1},
    window, Element, HtmlElement, HtmlInputElement, Node,
};
use yew::events::InputEvent;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let text_input_ref = use_node_ref();
    let lines = use_state(Vec::new);
    let font_size = use_state(|| 16.0);

    let on_text_input = text_input_handler(text_input_ref.clone(), lines.clone());
    let on_font_size_change = font_size_change_handler(font_size.clone());

    html! {
        <>
            <style id="dynamic-style"></style>
            <div class="menubar">
                <p>{"Placeholder"}</p>
                <input type="number" value={format!("{}", *font_size)} oninput={on_font_size_change} />
            </div>
            <div class="toolbar">
                <p>{"Placeholder"}</p>
            </div>

            <div class="sidebar">
            </div>

            <div class="notepad-container">
                <div class="notepad-wrapper">
                    <div
                        class="notepad-textarea"
                        id="notepad-textarea"
                        ref={text_input_ref}
                        contenteditable = "true"
                        oninput={on_text_input}
                    ></div>
                </div>
            </div>

            <div class="bottombar">
                <p>{"Placeholder"}</p>
            </div>
        </>
    }
}

fn text_input_handler(
    text_input_ref: NodeRef,
    lines: UseStateHandle<Vec<String>>,
) -> Callback<InputEvent> {
    Callback::from(move |_| {
        if let Some(input) = text_input_ref.cast::<HtmlElement>() {
            let mut inner_text = input.inner_text();

            inner_text = to_html(&inner_text);
            log_1(&JsValue::from_str(&inner_text));

            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(textarea) = document
                        .get_element_by_id("notepad-textarea")
                        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
                    {
                        let selection = window.get_selection().unwrap().unwrap();
                        if selection.range_count() != 0 {
                            let range = selection.get_range_at(0).unwrap();
                            range.collapse();
                            range.insert_node(node);
                        }

                        textarea.set_inner_html(&inner_text);
                    }
                }
            }
            let new_lines: Vec<String> = inner_text.lines().map(String::from).collect();
            lines.set(new_lines);
        }
    })
}

fn font_size_change_handler(font_size: UseStateHandle<f64>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            let new_font_size = input.value_as_number();
            font_size.set(new_font_size);

            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if let Some(style) = document
                    .get_element_by_id("dynamic-style")
                    .and_then(|el| el.dyn_into::<HtmlElement>().ok())
                {
                    style.set_inner_html(&format!(":root {{ --font-size: {}px; }}", new_font_size));
                }
            }
        }
    })
}
