use chrono::prelude::*;
use chrono::TimeDelta;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;
use yew::events::InputEvent;
use yew::events::MouseEvent;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_icons::{Icon, IconId};

#[path = "font_size_handlers.rs"]
mod font_size_handlers;
use font_size_handlers::FontSizeControls;

#[path = "zoom_level_handlers.rs"]
mod zoom_level_handlers;
use zoom_level_handlers::ZoomControls;

#[path = "text_alignment_handlers.rs"]
mod text_alignment_handlers;

//TODO Toast System
//TODO File Opening

#[path = "sidebar/sidebar.rs"]
mod sidebar;
use shared::Project;
use sidebar::SideBar;

#[path = "modal-system/modal.rs"]
mod modal;
use modal::ButtonProps as ModalButtonProps;
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

#[derive(Properties, PartialEq)]
pub struct WordCountProps {
    pub pages_ref: NodeRef,
}

#[function_component(App)]
pub fn app() -> Html {
    let pages_ref: NodeRef = use_node_ref();
    let text_input_ref = use_node_ref();
    let lines = use_state(Vec::new);
    let font_size = use_state(|| 16.0);
    let zoom_level = use_state(|| 100.0);
    let project: UseStateHandle<Option<Project>> = use_state(|| None);
    let sidebar = use_state(|| {
        html! { <>{ "No Project Loaded" }</> }
    });
    let modal = use_state(|| html!());

    let start_time = use_state(|| None);
    let word_count = use_state(|| 0);
    let wpm = use_state(|| 0.0);

    let on_text_input = text_input_handler(
        text_input_ref.clone(),
        lines,
        start_time,
        word_count,
        wpm.clone(),
    );

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
        Callback::from(move |_| {
            modal.set(html! {
                <Modal
                    content={html!({"This is some test text for me to know how this looks with more text in it bla bla bla bla"})}
                    button_configs={vec![
                        ModalButtonProps {text:"Cancel".to_string(), text_color:"crust".to_string(), bg_color:"maroon".to_string(), callback: {
                        let modal = modal.clone();
                        Callback::from(move |_| modal.set(html!()))
                        }},
                        ModalButtonProps {text:"Apply".to_string(), text_color:"crust".to_string(), bg_color:"mauve".to_string(), callback: {
                        let modal = modal.clone();
                        Callback::from(move |_| modal.set(html!()))
                        }}]}
                />
            });
        })
    };

    {
        let sidebar = sidebar.clone();
        let project = project.clone();
        use_effect_with(project.clone(), move |_| {
            if (*project).is_none() {
                sidebar.set(html! { { "No Project Loaded" } });
            } else {
                sidebar.set(html! {
                    <SideBar
                        project={<std::option::Option<shared::Project> as Clone>::clone(&(project)).unwrap()}
                    />
                });
            }
        });
    };

    let on_load = {
        let project = project;
        Callback::from(move |_: MouseEvent| {
            let project = project.clone();
            {
                spawn_local(async move {
                    let project_jsvalue = invoke("get_project", JsValue::null()).await;
                    let project_or_none: Option<Project> =
                        serde_wasm_bindgen::from_value(project_jsvalue).unwrap();
                    if project_or_none.is_some() {
                        project.set(project_or_none);
                    } else {
                        gloo_console::log!("bruh");
                    }
                });
            }
        })
    };

    #[function_component]
    fn WordCount(WordCountProps { pages_ref }: &WordCountProps) -> Html {
        let word_count = use_state(|| 0);
        {
            let pages_ref = pages_ref.clone();
            let word_count = word_count.clone();
            use_interval(
                {
                    let pages_ref = pages_ref;
                    let word_count = word_count;
                    move || {
                        if let Some(pages_element) = pages_ref.cast::<HtmlElement>() {
                            let text = pages_element.inner_text();
                            let count = text.split_whitespace().count();
                            word_count.set(count);
                        }
                    }
                },
                1500,
            );
        }

        html! { <div>{ format!("{} Words", *word_count) }</div> }
    }

    #[derive(Properties, PartialEq)]
    pub struct CharCountProps {
        pub pages_ref: NodeRef,
    }

    #[function_component]
    fn CharCount(CharCountProps { pages_ref }: &CharCountProps) -> Html {
        let char_count = use_state(|| 0);
        let char_count_no_spaces = use_state(|| 0);
        {
            let pages_ref = pages_ref.clone();
            let char_count = char_count.clone();
            let char_count_no_spaces = char_count_no_spaces.clone();
            use_interval(
                {
                    move || {
                        if let Some(pages_element) = pages_ref.cast::<HtmlElement>() {
                            let text = pages_element.inner_text();
                            let count = text.len();
                            let count_no_spaces =
                                text.chars().filter(|c| !c.is_whitespace()).count();
                            // gloo_console::log!("Text: {}", text);
                            // gloo_console::log!("Character count: {}", count);
                            // gloo_console::log!("Character count (no spaces): {}", count_no_spaces);
                            char_count.set(count);
                            char_count_no_spaces.set(count_no_spaces);
                        }
                    }
                },
                1500,
            );
        }
        html! {
            <div>
                <p>
                    { format!("Characters: {}, {} without spaces", *char_count, *char_count_no_spaces) }
                </p>
            </div>
        }
    }

    html! {
        <>
            <div class="modal-wrapper">{ (*modal).clone() }</div>
            <style id="dynamic-style" />
            <div class="menubar">
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
                <Icon
                    icon_id={IconId::LucideAlignCenter}
                    width={"2em".to_owned()}
                    height={"2em".to_owned()}
                    class="menubar-icon"
                />
                <Icon
                    icon_id={IconId::LucideAlignJustify}
                    width={"2em".to_owned()}
                    height={"2em".to_owned()}
                    class="menubar-icon"
                />
                <Icon
                    icon_id={IconId::LucideAlignLeft}
                    width={"2em".to_owned()}
                    height={"2em".to_owned()}
                    class="menubar-icon"
                />
                <Icon
                    icon_id={IconId::LucideAlignRight}
                    width={"2em".to_owned()}
                    height={"2em".to_owned()}
                    class="menubar-icon"
                />
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
                            contenteditable="true"
                            oninput={on_text_input}
                        />
                    </div>
                </div>
            </div>
            <div class="bottombar">
                <div class="bottombar-left">
                    <SessionTime />
                    <WordCount pages_ref={pages_ref.clone()} />
                    <CharCount pages_ref={pages_ref.clone()} />
                    <p>{ format!("WPM: {:.2}", *wpm) }</p>
                </div>
                <div class="bottombar-right">
                    <ZoomControls zoom_level={zoom_level.clone()} />
                </div>
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
    let start_time = use_state(Local::now);
    let session_time = use_state(|| TimeDelta::new(0, 0).unwrap());

    use_interval(
        {
            let session_time = session_time.clone();
            move || {
                let current_time = Local::now();
                session_time.set(current_time - *start_time);
            }
        },
        1000,
    );

    let total_seconds = session_time.num_seconds();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    let formatted_time = format!("{hours:02}:{minutes:02}:{seconds:02}");

    html! { <p>{ formatted_time }</p> }
}

fn calculate_wpm(word_count: usize, start_time: Option<DateTime<Local>>) -> f64 {
    if let Some(start) = start_time {
        let elapsed = Local::now() - start;
        let elapsed_seconds = elapsed.num_seconds() as f64;
        if elapsed_seconds > 0.0 {
            return (word_count as f64 / elapsed_seconds) * 60.0;
        }
    }
    0.0
}

fn text_input_handler(
    text_input_ref: NodeRef,
    lines: UseStateHandle<Vec<String>>,
    start_time: UseStateHandle<Option<DateTime<Local>>>,
    word_count: UseStateHandle<usize>,
    wpm: UseStateHandle<f64>,
) -> Callback<InputEvent> {
    Callback::from(move |_| {
        if let Some(input) = text_input_ref.cast::<HtmlElement>() {
            let inner_text = input.inner_text();
            let new_lines: Vec<String> = inner_text.lines().map(String::from).collect();
            lines.set(new_lines);

            let words = inner_text.split_whitespace().count();
            word_count.set(words);

            if start_time.is_none() {
                start_time.set(Some(Local::now()));
            }

            let calculated_wpm = calculate_wpm(*word_count, *start_time);
            wpm.set(calculated_wpm);
        }
    })
}
