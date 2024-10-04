use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use wasm_bindgen_futures::spawn_local;

use wasm_bindgen::prelude::wasm_bindgen;

use serde_json::json;
use std::path::Path;
use wasm_bindgen::JsValue;

use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Properties, PartialEq)]
pub struct CharCountProps {
    pub pages_ref: NodeRef,
}

#[derive(Serialize, Deserialize)]
pub struct FileWriteData {
    pub path: String,
    pub content: String,
}

#[function_component]
pub fn CharCount(CharCountProps { pages_ref }: &CharCountProps) -> Html {
    let char_count = use_state(|| 0);
    let char_count_no_spaces = use_state(|| 0);
    {
        let pages_ref = pages_ref.clone();
        let char_count = char_count.clone();
        let char_count_no_spaces = char_count_no_spaces.clone();
        use_interval(
            {
                let pages_ref = pages_ref.clone();
                let char_count = char_count.clone();
                let char_count_no_spaces = char_count_no_spaces.clone();
                move || {
                    let pages_ref = pages_ref.clone();
                let char_count = char_count.clone();
                let char_count_no_spaces = char_count_no_spaces.clone();
                    spawn_local( async move {
                    if let Some(pages_element) = pages_ref.cast::<HtmlElement>() {
                        let text = pages_element.inner_text();
                        let count = text.len();
                        let count_no_spaces =
                            text.chars().filter(|c| !c.is_whitespace()).count();
                        char_count.set(count);
                        char_count_no_spaces.set(count_no_spaces);

                        let char_count_json = json!({
                            "char_count": *char_count.clone(),
                            "char_count_with_no_spaces": *char_count_no_spaces.clone()
                        }).to_string();

                        let path = Path::new("C:\\Users\\janni\\Desktop\\Schule\\Diplomarbeit\\Program\\statistic\\char_count.json");
                        let fileWriteData = FileWriteData {path: path.to_string_lossy().to_string(), content: char_count_json};

                        invoke("write_to_file", serde_wasm_bindgen::to_value(&fileWriteData).unwrap()).await;
                    }
                })

                }
            },
            1500,
        )
    }
    html! {
    <div>
        <p>{format!("Characters: {}, {} without spaces", *char_count, *char_count_no_spaces)}</p>
        </div>

    }
}