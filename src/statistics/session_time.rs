use chrono::prelude::*;
use chrono::TimeDelta;
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

#[derive(Serialize, Deserialize)]
pub struct FileWriteData {
    pub path: String,
    pub content: String,
}

#[function_component]
pub fn SessionTime() -> Html {
    let start_time = use_state(Local::now);
    let session_time = use_state(|| TimeDelta::new(0, 0).unwrap());
    let formatted_time = use_state(|| String::from(""));
    
    {
        let start_time = start_time.clone();
        let session_time = session_time.clone();
        let formatted_time = formatted_time.clone();
        use_interval(
        move || {
                let start_time = start_time.clone();
                let session_time = session_time.clone();
                let formatted_time = formatted_time.clone();
                spawn_local( async move {
                    let current_time = Local::now();
                    session_time.set(current_time - *start_time);
                    
                    let total_seconds = session_time.num_seconds();
                    let hours = total_seconds / 3600;
                    let minutes = (total_seconds % 3600) / 60;
                    let seconds = total_seconds % 60;

                    formatted_time.set( format!("{:02}:{:02}:{:02}", hours, minutes, seconds));

                    let session_time_json = json!({
                        "session_time": <std::string::String as Clone>::clone(&*formatted_time.clone())
                    }).to_string();

                    let path = Path::new("C:\\Users\\Jannis\\Schule\\Diplomarbeit\\statistic\\session_time.json");
                    let fileWriteData = FileWriteData {path: path.to_string_lossy().to_string(), content: session_time_json};

                    invoke("write_to_file", serde_wasm_bindgen::to_value(&fileWriteData).unwrap()).await;
                })
        },
        1000,
    );

    
    }

    html! {

        <p>{<std::string::String as Clone>::clone(&*formatted_time.clone())}</p>
    }
}