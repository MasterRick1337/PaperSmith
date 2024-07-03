use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::events::InputEvent;
use yew::prelude::*;
#[path = "sidebar.rs"]
mod sidebar;

use sidebar::SideBar;

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
    let zoom_level = use_state(|| 100.0);

    let on_text_input = text_input_handler(text_input_ref.clone(), lines.clone());
    let on_font_size_change = font_size_change_handler(font_size.clone());
    let on_zoom_change = zoom_change_handler(zoom_level.clone());
    let on_zoom_increase = zoom_increase_handler(zoom_level.clone());
    let on_zoom_decrease = zoom_decrease_handler(zoom_level.clone());
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
                <div class="menubar-left" id="font-size">
                    <input type="number" value={format!("{}", *font_size)} oninput={on_font_size_change} />
                </div>
                <button onclick={save}>{"Save"}</button>
                <p>{"Placeholder"}</p>
            </div>

            <div class="sidebar">
                <SideBar/>
            </div>


            <div class="notepad-outer-container">
                <div class="notepad-container" style={format!("transform: scale({});", *zoom_level / 100.0)}>
                    <a class="anchor"></a>
                    <div class="notepad-wrapper">
                        <div
                            class="notepad-textarea"
                            ref={text_input_ref}
                            contenteditable = "true"
                            oninput={on_text_input}
                        ></div>
                    </div>
                </div>
            </div>

            <div class="bottombar">
                <div class="bottombar-right" id="zoom">
                    <button class="zoom-button" title="Zoom Out" onclick={on_zoom_decrease}>{"-"}</button>
                    <input type="range" min="0" max="200" class="zoom-slider" id="zoom-slider" title="Zoom" value={format!("{}", *zoom_level)} oninput={on_zoom_change} />
                    <button class = "zoom-button" title="Zoom In" onclick={on_zoom_increase}>{"+"}</button>
                    <span class="zoom-text" id="zoom-value">{format!("{}%", *zoom_level)}</span>
                </div>
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
    let time_string = use_state(|| "Time Placeholder".to_string());

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
                if let Some(_style) = document
                    .get_element_by_id("dynamic-style")
                    .and_then(|el| el.dyn_into::<HtmlElement>().ok())
                {
                    if let Some(style) = document
                        .get_element_by_id("dynamic-style")
                        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
                    {
                        style.set_inner_html(&format!(
                            ":root {{ --font-size: {}px; }}",
                            new_font_size
                        ));
                    }
                }
            }
        }
    })
}

fn zoom_change_handler(zoom_level: UseStateHandle<f64>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            let new_zoom_level = input.value_as_number();
            zoom_level.set(new_zoom_level);
        }
    })
}

fn zoom_increase_handler(zoom_level: UseStateHandle<f64>) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let current_zoom = *zoom_level;
        let new_zoom_level = if current_zoom % 10.0 == 0.0 {
            (current_zoom + 10.0).min(200.0)
        } else {
            ((current_zoom / 10.0).ceil() * 10.0).min(200.0)
        };
        zoom_level.set(new_zoom_level);
    })
}

fn zoom_decrease_handler(zoom_level: UseStateHandle<f64>) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let current_zoom = *zoom_level;
        let new_zoom_level = if current_zoom % 10.0 == 0.0 {
            (current_zoom - 10.0).max(0.0)
        } else {
            ((current_zoom / 10.0).floor() * 10.0).max(0.0)
        };
        zoom_level.set(new_zoom_level);
    })
}
