use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use serde_json::json;
use std::path::Path;
use wasm_bindgen_futures::spawn_local;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Properties, PartialEq)]
    pub struct WordCountProps {
        pub pages_ref: NodeRef,
    }

#[derive(Serialize, Deserialize)]
pub struct FileWriteData {
    pub path: String,
    pub content: String,
}

#[function_component]
    pub fn WordCount(WordCountProps { pages_ref }: &WordCountProps) -> Html {
        let word_count = use_state(|| 0);
        {
            let pages_ref = pages_ref.clone();
            let word_count = word_count.clone();
            use_interval(
                move || {
                    let pages_ref = pages_ref.clone();
                    let word_count = word_count.clone();
                    spawn_local( async move {
                        if let Some(pages_element) = pages_ref.cast::<HtmlElement>() {
                            let text = pages_element.inner_text();
                            let count = text.split_whitespace().count();
                            word_count.set(count);

                            let word_count_json = json!({
                                "word_count": count
                            }).to_string();
                            
                            let path = Path::new("C:\\Users\\Jannis\\Schule\\Diplomarbeit\\statistic\\word_count.json");
                            let fileWriteData = FileWriteData {path: path.to_string_lossy().to_string(), content: word_count_json};
                            
                            invoke("write_to_file", serde_wasm_bindgen::to_value(&fileWriteData).unwrap()).await;
                        }
                    })
                },
                
                1500,
            )
        }

        html! {
        <div>{format!("{} Words", *word_count)}</div>
    }
}
