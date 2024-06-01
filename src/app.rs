/*use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with(
            name2,
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let args = to_value(&GreetArgs { name: &*name }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke("greet", args).await.as_string().unwrap();
                    greet_msg.set(new_msg);
                });

                || {}
            },
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
            </div>

            <p>{"Click on the Tauri and Yew logos to learn more."}</p>

            <p>
                {"Recommended IDE setup: "}
                <a href="https://code.visualstudio.com/" target="_blank">{"VS Code"}</a>
                {" + "}
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{"Tauri"}</a>
                {" + "}
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{"rust-analyzer"}</a>
            </p>

            <form class="row" onsubmit={greet}>
                <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                <button type="submit">{"Greet"}</button>
            </form>

            <p><b>{ &*greet_msg }</b></p>
        </main>
    }
}
*/

use yew::prelude::*;
use web_sys::HtmlElement;
use yew::events::InputEvent;

#[function_component(App)]
pub fn app() -> Html {
    let text_input_ref = use_node_ref();
    let lines = use_state(Vec::new);

    let on_input = {
        let lines = lines.clone();
        let text_input_ref = text_input_ref.clone();
        Callback::from(move |_: InputEvent| {
            if let Some(input) = text_input_ref.cast::<HtmlElement>() {
                let inner_text = input.inner_text();
                let new_lines: Vec<String> = inner_text.lines().map(String::from).collect();
                lines.set(new_lines);
            }
        })
    };

    html! {
        <>
            <style>
                {"
                body {
                    margin: 0;
                    padding: 0;
                    background-color: #1e1e1e;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    font-family: Arial, sans-serif;
                    padding-top: 20px;
                    padding-bottom: 20px;
                }
                
                .top-bar {
                    height: 50px;
                    background-color: #333333;
                    color: #ffffff;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    width: 100%;
                    position: fixed;
                    top: 0;
                    left: 0;
                    right: 0;
                    padding: 0 20px;
                    /*box-shadow: 0 -2px 5px rgba(0, 0, 0, 0.2);*/
                    border-bottom: 1px solid #444444;
                }
                
                .sidebar {
                    width: 300px;
                    background-color: #2d2d2d;
                    color: #e0e0e0;
                    overflow-y: auto;
                    position: fixed;
                    left: 0;
                    top: 50px;
                    bottom: 20px;
                    padding: 20px;
                    box-shadow: 2px 0 5px rgba(0, 0, 0, 0.2);
                }
                
                .bottom-bar {
                    height: 20px;
                    background-color: #333333;
                    color:  #e0e0e0;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    width: 100%;
                    position: fixed;
                    bottom: 0;
                    left: 0;
                    right: 0;
                    padding: 0 20px;
                    /*box-shadow: 0 -2px 5px rgba(0, 0, 0, 0.2);*/
                    border-top: 1px solid #444444;
                }

                .notepad-container {
                    margin-top: 80px;
                    margin-bottom: 40px;
                    margin-left: 330px;
                    width: 420px;
                    height: calc(100vh - 160px);
                    width: calc((100vh - 160px) / 1.414);
                    background-color: #2d2d2d;
                    border: 1px solid #444444;
                    padding: 20px;
                    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
                    overflow: hidden;
                }
                
                .notepad-textarea {
                    width: 100%;
                    height: 100%;
                    padding: 10px;
                    font-size: 16px;
                    border: none;
                    outline: none;
                    resize: none;
                    background-color: #2d2d2d;
                    color: #ffffff;
                    overflow-y: auto;
                }
                "}
            </style>
            <div class="top-bar">
                <p>{"Placeholder"}</p>
            </div>

            <div class="sidebar">
            </div>

            <div class="notepad-container">
                <div
                    class="notepad-textarea"
                    ref={text_input_ref}
                    contenteditable = "true"
                    oninput={on_input}
                ></div>
            </div>

            <div class="bottom-bar">
                <p>{"Placeholder"}</p>
            </div>
        </>
    }
}
