use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_wasm_bindgen::to_value;
use shared::Project;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlButtonElement, HtmlElement, HtmlInputElement};
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
    pub project_ref: UseStateHandle<Option<Project>>,
}

#[derive(Serialize)]
struct ChooseFolderArgs {
    title: String,
}

#[derive(Serialize)]
struct CheckFolderArgs {
    path: String,
}
// TODO:
// implement project creation tauri function
// *including error checking*

#[function_component(ProjectWizard)]
pub fn project_wizard(
    Props {
        default_location,
        closing_callback: on_close,
        project_ref,
    }: &Props,
) -> Html {
    let title = use_state(String::new);
    let location = use_state(|| default_location.clone());
    let title_ref = use_node_ref();
    let location_ref = use_node_ref();
    let confirm_button_ref = use_node_ref();
    let is_hovered = use_state(|| false);
    let is_data_valid = use_state(|| true);
    let error_message = use_state(String::new);

    let on_title_input = text_input_handler(title.clone());
    let on_location_input = text_input_handler(location.clone());

    {
        let default_location = default_location.clone();
        let location_ref = location_ref.clone();
        use_effect_with((), move |()| {
            if let Some(input) = location_ref.cast::<HtmlInputElement>() {
                input.set_value(&default_location);
            }
        });
    }

    let on_mouse_over = {
        let is_hovered = is_hovered.clone();
        Callback::from(move |_| is_hovered.set(true))
    };

    let on_mouse_out = {
        let is_hovered = is_hovered.clone();
        Callback::from(move |_| is_hovered.set(false))
    };

    {
        let is_data_valid = is_data_valid.clone();
        let location = location.clone();
        let title = title.clone();
        let error_message = error_message.clone();
        use_effect_with((location.clone(), title.clone()), move |_| {
            let is_data_valid = is_data_valid.clone();
            let location = location.clone();
            let error_message = error_message.clone();
            if title.is_empty() {
                is_data_valid.set(false);
                error_message.set("Please enter a title".to_string());
                return;
            }
            if location.is_empty() {
                is_data_valid.set(false);
                error_message.set("Please enter a location.".to_string());
                return;
            }
            spawn_local(async move {
                let is_data_valid = is_data_valid.clone();
                let location = location.clone();
                let error_message = error_message.clone();

                let complete_path = PathBuf::from(&*location).join(&*title);

                gloo_console::log!(format!(
                    "Complete Path: {}",
                    &complete_path
                        .clone()
                        .into_os_string()
                        .into_string()
                        .unwrap()
                ));

                let result = invoke(
                    "can_create_path",
                    serde_wasm_bindgen::to_value(&CheckFolderArgs {
                        path: complete_path.into_os_string().into_string().unwrap(),
                    })
                    .unwrap(),
                )
                .await
                .as_string()
                .expect("Something went horribly wrong in validation");
                //gloo_console::log!(format!("{}{}", &*location, &*is_data_valid));
                let (is_valid, message) = match result.as_str() {
                    "" => (true, ""),
                    e => (false, e),
                };

                is_data_valid.set(is_valid);
                error_message.set(message.to_string());
            });
        });
    }

    {
        let confirm_button_ref = confirm_button_ref.clone();
        let is_data_valid = is_data_valid.clone();
        use_effect_with(is_data_valid.clone(), move |_| {
            if let Some(button) = confirm_button_ref.cast::<HtmlButtonElement>() {
                gloo_console::log!(format!("Is valid: {}", *is_data_valid));
                if *is_data_valid {
                    let _ = button.style().set_property("opacity", "1");
                    button.set_disabled(false);
                } else {
                    let _ = button.style().set_property("opacity", "0.5");
                    button.set_disabled(true);
                }
            }
        });
    }

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

    let on_choose_folder = {
        let location = location.clone();
        let location_ref = location_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let location = location.clone();
            let location_ref = location_ref.clone();
            spawn_local(async move {
                let save_args = ChooseFolderArgs {
                    title: "Choose location".to_string(),
                };

                let args = to_value(&save_args).unwrap();
                let location_jsvalue = invoke("choose_folder", args).await;
                let location_string = location_jsvalue.as_string();
                match location_string {
                    None => (),
                    Some(e) => {
                        if let Some(input) = location_ref.cast::<HtmlInputElement>() {
                            input.set_value(&e);
                        }
                        location.set(e);
                    }
                }
            });
        })
    };

    let on_confirm = {
        let is_data_valid = is_data_valid.clone();
        let location = location.clone();
        let title = title.clone();
        let on_close = on_close.clone();
        let project_ref = project_ref.clone();
        Callback::from(move |_| {
            let location = location.clone();
            let title = title.clone();
            let project_ref = project_ref.clone();
            gloo_console::log!("Help!");
            if !*is_data_valid {
                return;
            }
            spawn_local(async move {
                let complete_path = PathBuf::from(&*location).join(&*title);
                let project_jsvalue = invoke(
                    "create_project",
                    serde_wasm_bindgen::to_value(&CheckFolderArgs {
                        path: complete_path.into_os_string().into_string().unwrap(),
                    })
                    .unwrap(),
                )
                .await;
                let project_or_none: Option<Project> =
                    serde_wasm_bindgen::from_value(project_jsvalue).unwrap();
                if project_or_none.is_some() {
                    project_ref.set(project_or_none);
                } else {
                    gloo_console::log!("bruh");
                }
            });
            on_close.emit(MouseEvent::new("ahhhh").unwrap());
        })
    };

    html!(
        <>
            <div class="text-xl font-bold">{ "Create Project" }</div>
            <br />
            <div class="font-semibold">{ "Name:" }</div>
            <div class="flex rounded-lg border-2 my-2 border-transparent hover:border-mauve">
                <input
                    oninput={on_title_input}
                    ref={title_ref}
                    class="outline-none w-full bg-crust text-text p-2 rounded-lg"
                />
            </div>
            <br />
            <div class="font-semibold">{ "Location:" }</div>
            <div class="flex rounded-lg border-2 my-2 border-transparent hover:border-mauve">
                <input
                    oninput={on_location_input}
                    ref={location_ref}
                    class="outline-none w-full bg-crust text-text p-2 rounded-tl-lg rounded-bl-lg"
                />
                <div
                    onmouseover={on_mouse_over}
                    onmouseout={on_mouse_out}
                    onclick={on_choose_folder}
                    class="content-center hover:text-mauve rounded-tr-lg border-l-2 border-overlay0 rounded-br-lg bg-crust p-2"
                >
                    { icon }
                </div>
            </div>
            <div id="footer" class="flex justify-end w-full pt-8">
                <div class="text-text underline decoration-red break-words mr-auto">
                    { (*error_message).clone() }
                </div>
                <button
                    ref={confirm_button_ref}
                    onclick={on_confirm}
                    class={format!("rounded-lg text-lg px-2 py-1 ml-4 bg-mauve text-crust hover:scale-105")}
                >
                    { "Confirm" }
                </button>
                <button
                    onclick={on_close}
                    class={format!("rounded-lg text-lg px-2 py-1 ml-4 bg-red text-crust hover:scale-105")}
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
fn text_input_handler(value: UseStateHandle<String>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            let text = input.value();
            value.set(text.clone());
        }
    })
}
