use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::events::InputEvent;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize)]
struct SaveFileArgs {
    content: String,
    filename: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let text_input_ref = use_node_ref();
    let lines = use_state(Vec::new);
    let font_size = use_state(|| 16.0);

    let on_text_input = text_input_handler(text_input_ref.clone(), lines.clone());
    let on_font_size_change = font_size_change_handler(font_size.clone());

    let save = {
        let text_input_ref = text_input_ref.clone();
        Callback::from(move |_| {
            let text_input_ref = text_input_ref.clone();
            spawn_local(async move {
                if let Some(input_element) = text_input_ref.cast::<HtmlElement>() {
                    let text = input_element.inner_text();
                    let result = invoke("show_save_dialog", JsValue::NULL).await.as_string();
                    if let Some(path) = result {
                        let save_args = SaveFileArgs {
                            content: text,
                            filename: path.clone(),
                        };

                        let args = to_value(&save_args).unwrap();
                        invoke("save_file", args).await;
                    }
                }
            });
        })
    };

    html! {
        <>
            <style id="dynamic-style"></style>
            <div class="menubar">
                <p>{"Placeholder"}</p>
                <input type="number" value={format!("{}", *font_size)} oninput={on_font_size_change} />
            </div>
            <div class="toolbar">
                <button onclick={save}>{"Save"}</button>
            </div>

            <div class="sidebar">
            </div>

            <div class="notepad-container">
                <div class="notepad-wrapper">
                    <div
                        class="notepad-textarea"
                        ref={text_input_ref}
                        contenteditable = "true"
                        oninput={on_text_input}
                    ></div>
                </div>
            </div>

            <div class="bottombar">
                <SessionTime/>
            </div>
        </>
    }
}
/*let save = Callback::from(move |_: MouseEvent| {
    let args = to_value(&()).unwrap();
    let ahhh = invoke("show_save_dialog", args).await;
});*/

/*This one worked----------------------------------------------------------
let save = {
    Callback::from(move |_| {
        spawn_local(async move {
            let args = to_value(&()).unwrap();
            let ahhh = invoke("show_save_dialog", args).await;
        });
    })
};*/

/*let save = {
    Callback::from(move |_| {
        spawn_local(async move {
            let args = to_value(&()).unwrap();
            invoke("saveTest", args).await.as_string();
        });
    })
};*/

#[function_component]
fn SessionTime() -> Html {
    let time_string = use_state(|| "31515235".to_string());

    html! {
        <p>{ <std::string::String as Clone>::clone(&*time_string)}</p>
    }
}

fn text_input_handler(
    text_input_ref: NodeRef,
    lines: UseStateHandle<Vec<String>>,
) -> Callback<InputEvent> {
    Callback::from(move |_| {
        if let Some(input) = text_input_ref.cast::<HtmlElement>() {
            let inner_text = input.inner_text();
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
