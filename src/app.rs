use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;
use yew::events::InputEvent;
use yew::events::MouseEvent;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[path = "font_size_handlers.rs"]
mod font_size_handlers;
use font_size_handlers::FontSizeControls;

#[path = "zoom_level_handlers.rs"]
mod zoom_level_handlers;
use zoom_level_handlers::ZoomControls;

#[path = "statistics/statistic.rs"]
mod statistic;
use statistic::Statistics;

#[path = "text_alignment_handlers.rs"]
mod text_alignment_handlers;
use text_alignment_handlers::TextAlignmentControls;

#[path = "sidebar/sidebar.rs"]
mod sidebar;
use shared::Project;
use sidebar::SideBar;

#[path = "project-wizard/wizard.rs"]
mod wizard;
use wizard::ProjectWizard;

#[path = "modal-system/modal.rs"]
mod modal;
use modal::Modal;

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
    let pages_ref: NodeRef = use_node_ref();
    let text_input_ref = use_node_ref();
    let lines = use_state(Vec::new);
    let font_size = use_state(|| 16.0);
    let zoom_level = use_state(|| 100.0);
    let project: UseStateHandle<Option<Project>> = use_state(|| None);
    let text_alignment = use_state(|| "left".to_string());
    let sidebar = use_state(|| {
        html! { <>{ "No Project Loaded" }</> }
    });
    let modal = use_state(|| html!());

    let on_text_input = text_input_handler(text_input_ref.clone(), lines);

    let save = {
        let text_input_ref = text_input_ref.clone();
        Callback::from(move |_| {
            let text_input_ref = text_input_ref.clone();
            spawn_local(async move {
                if let Some(input_element) = text_input_ref.cast::<HtmlElement>() {
                    let text = input_element.inner_text();
                    let result: Option<String> =
                        invoke("show_save_dialog", JsValue::NULL).await.as_string();
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

    let open_modal = {
        let modal = modal.clone();
        let project = project.clone();
        Callback::from(move |_| {
            modal.set(html! {
                <Modal
                    content={html! {
                    <ProjectWizard

                        closing_callback={
                            let modal = modal.clone();
                            Callback::from(move |_| modal.set(html!()))
                        }
                        project_ref={project.clone()}
                    />
                    }}
                />
            });
        })
    };

    {
        let sidebar = sidebar.clone();
        let project = project.clone();
        use_effect_with(project.clone(), move |_| {
            if (*project).is_none() {
                sidebar.set(
                    html! { <div class="cursor-default select-none">{ "No Project Loaded" }</div> },
                );
            } else {
                sidebar.set(html! { <SideBar project={project.clone()} /> });
            }
        });
    };

    let on_load = {
        let project = project.clone();
        Callback::from(move |_: MouseEvent| {
            let project = project.clone();
            spawn_local(async move {
                let project_jsvalue = invoke("get_project", JsValue::null()).await;
                let project_or_none: Option<Project> =
                    serde_wasm_bindgen::from_value(project_jsvalue).unwrap();
                if project_or_none.is_some() {
                    project.set(project_or_none);
                }
            });
        })
    };

    let print_project = {
        let project = project.clone();
        Callback::from(move |_| {
            let project = project.clone();
            gloo_console::log!(format!("{}", project.as_ref().unwrap()));
        })
    };

    html! {
        <div>
            <div class="modal-wrapper">{ (*modal).clone() }</div>
            <style id="dynamic-style" />
            <div class="menubar">
                <Icon
                    onclick={print_project}
                    icon_id={IconId::LucideUndo}
                    width={"2em".to_owned()}
                    height={"2em".to_owned()}
                    class="menubar-icon"
                />
                <Icon
                    icon_id={IconId::LucideUndo}
                    width={"2em".to_owned()}
                    height={"2em".to_owned()}
                    class="menubar-icon"
                />
                <Icon
                    icon_id={IconId::LucideRedo}
                    width={"2em".to_owned()}
                    height={"2em".to_owned()}
                    class="menubar-icon"
                />
                <div class="separator" />
                <FontSizeControls font_size={font_size.clone()} />
                //<Icon icon_id={IconId::}/>
                <div class="separator" />
                <Icon
                    icon_id={IconId::LucideBold}
                    width={"2em".to_owned()}
                    height={"2em".to_owned()}
                    class="menubar-icon"
                />
                <Icon
                    icon_id={IconId::LucideItalic}
                    width={"2em".to_owned()}
                    height={"2em".to_owned()}
                    class="menubar-icon"
                />
                <Icon
                    icon_id={IconId::LucideUnderline}
                    width={"2em".to_owned()}
                    height={"2em".to_owned()}
                    class="menubar-icon"
                />
                <Icon
                    icon_id={IconId::LucideBaseline}
                    width={"2em".to_owned()}
                    height={"2em".to_owned()}
                    class="menubar-icon"
                />
                <Icon
                    icon_id={IconId::LucideHighlighter}
                    width={"2em".to_owned()}
                    height={"2em".to_owned()}
                    class="menubar-icon"
                />
                <div class="separator" />
                <TextAlignmentControls text_alignment={text_alignment.clone()} />
                <Icon
                    icon_id={IconId::LucideList}
                    width={"2em".to_owned()}
                    height={"2em".to_owned()}
                    class="menubar-icon"
                />
                <Icon
                    icon_id={IconId::LucideListChecks}
                    width={"2em".to_owned()}
                    height={"2em".to_owned()}
                    class="menubar-icon"
                />
                //<Icon icon_id={IconId::LucideSpellCheck}/>
                <button onclick={save}>{ "Save" }</button>
                <button onclick={on_load}>{ "Load" }</button>
                <button onclick={open_modal}>{ "Modal" }</button>
            </div>
            <div class="sidebar">{ (*sidebar).clone() }</div>
            <div class="notepad-outer-container" ref={pages_ref.clone()}>
                <div
                    class="notepad-container"
                    style={format!("transform: scale({});", *zoom_level / 100.0)}
                >
                    <a class="anchor" />
                    <div class="notepad-wrapper">
                        <div
                            class="notepad-textarea"
                            id="notepad-textarea"
                            ref={text_input_ref}
                            style={format!("text-align: {};", *text_alignment)}
                            contenteditable="true"
                            oninput={on_text_input}
                        />
                    </div>
                </div>
            </div>
            <div class="bottombar">
                <div class="bottombar-left">
                    <Statistics pages_ref={pages_ref.clone()} />
                </div>
                <div class="bottombar-right">
                    <ZoomControls zoom_level={zoom_level.clone()} />
                </div>
            </div>
        </div>
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
