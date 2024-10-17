use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub default_location: String,
    pub closing_callback: Callback<MouseEvent>,
}

#[derive(Serialize)]
struct ChooseFolderArgs {
    title: String,
}

// TODO:
// validate before doing anything
// implement project creation tauri function

#[function_component(ProjectWizard)]
pub fn project_wizard(
    Props {
        default_location,
        closing_callback,
    }: &Props,
) -> Html {
    let location = use_state(|| default_location.clone());
    let is_hovered = use_state(|| false);

    let on_mouse_over = {
        let is_hovered = is_hovered.clone();
        Callback::from(move |_| is_hovered.set(true))
    };

    let on_mouse_out = {
        let is_hovered = is_hovered.clone();
        Callback::from(move |_| is_hovered.set(false))
    };

    let icon = if *is_hovered {
        html! {
            <Icon
                icon_id={IconId::LucideFolderOpen}
                width={"1.5em".to_owned()}
                height={"1.5em".to_owned()}
            />
        }
    } else {
        html! {
            <Icon
                icon_id={IconId::LucideFolder}
                width={"1.5em".to_owned()}
                height={"1.5em".to_owned()}
            />
        }
    };

    let on_load = {
        let location = location.clone();
        Callback::from(move |_: MouseEvent| {
            let location = location.clone();
            spawn_local(async move {
                let save_args = ChooseFolderArgs {
                    title: "Choose location".to_string(),
                };

                let args = to_value(&save_args).unwrap();
                let location_jsvalue = invoke("choose_folder", args).await;
                let location_string = location_jsvalue.as_string();
                match location_string {
                    None => (),
                    Some(e) => location.set(e),
                }
            });
        })
    };

    html!(
        <>
            <div class="text-xl font-bold">{ "Create Project" }</div>
            <br />
            <div class="font-semibold">{ "Name:" }</div>
            <div class="flex rounded-lg border-2 my-2 border-transparent hover:border-mauve">
                <input class="outline-none w-full bg-crust text-text p-2 rounded-lg" />
            </div>
            <br />
            <div class="font-semibold">{ "Location:" }</div>
            <div class="flex rounded-lg border-2 my-2 border-transparent hover:border-mauve">
                <input
                    value={(*location).clone()}
                    class="outline-none w-full bg-crust text-text p-2 rounded-tl-lg rounded-bl-lg"
                />
                <div
                    onmouseover={on_mouse_over}
                    onmouseout={on_mouse_out}
                    onclick={on_load}
                    class="content-center hover:text-mauve rounded-tr-lg border-l-2 border-overlay0 rounded-br-lg bg-crust p-2"
                >
                    { icon }
                </div>
            </div>
            <div id="footer" class="flex justify-end w-full pt-8">
                <button
                    onclick={closing_callback}
                    class={format!("rounded-lg px-2 py-1 ml-4 bg-red text-text")}
                >
                    { "Close" }
                </button>
            </div>
        </>
    )
}

//html!(
//    <>
//        <div class="bg-maroon text-crust" />
//        <div class="bg-mauve text-crust" />
//        <button
//            onclick={callback}
//            class={format!("rounded-lg px-2 py-1 ml-4 bg-{bg_color} text-{text_color}")}
//        >
//            { text }
//        </button>
//    </>
//)
