use gloo::utils::document;
use pulldown_cmark::{html, Options, Parser};
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use sidebar::buttons::Button;
use statistic::_CharCountProps::closing_callback;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlDocument;
use web_sys::HtmlElement;
use web_sys::Node;
use yew::events::InputEvent;
use yew::events::MouseEvent;
use yew::prelude::*;
use yew_icons::IconId;

use serde::{Deserialize, Serialize};

#[path = "menubar/zoom/zoom_handlers.rs"]
mod zoom_edit_container_handlers;
use zoom_edit_container_handlers::ZoomControls;

#[path = "toolbar/toolbar.rs"]
mod toolbar;
use toolbar::Toolbar;
use yew_hooks::use_interval;
use std::path::PathBuf;

#[path = "theme-switcher/switcher.rs"]
mod switcher;
use switcher::ThemeSwitcher;

//#[path = "text_alignment_handlers.rs"]
//mod text_alignment_handlers;
//use text_alignment_handlers::TextAlignmentControls;

#[path = "menubar/text/text_styling_handlers.rs"]
mod text_styling_handlers;
use text_styling_handlers::TextStylingControls;

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

#[derive(Properties, PartialEq)]
pub struct WordCountProps {
    pub pages_ref: NodeRef,
}

#[derive(Serialize, Deserialize)]
pub struct FileWriteData {
    pub path: String,
    pub content: String
}

// #[derive(Properties, PartialEq)]
// pub struct StatisticProps {
//     pub statistics: StatisticProp,
// }

#[function_component(App)]
pub fn app() -> Html {
    let pages_ref: NodeRef = use_node_ref();
    let zoom_compile_ref = use_node_ref();
    let zoom_edit_ref = use_node_ref();
    let text_input_ref = use_node_ref();
    let font_size_edit = use_state(|| 16.0);
    let font_size_compile = use_state(|| 16.0);
    let zoom_level = use_state(|| 100.0);
    let project: UseStateHandle<Option<Project>> = use_state(|| None);
    let text_alignment = use_state(|| "left".to_string());
    let sidebar = use_state(|| {
        html! {
            <>
                <div class="text-lg">{ "No Project Loaded" }</div>
            </>
        }
    });
    let modal = use_state(|| html!());

    let render_ref = use_node_ref();

    let on_text_input = text_input_handler(text_input_ref.clone(), render_ref.clone());


    let project_path = project.as_ref().and_then(|proj| Some(proj.path.clone()));

    let save = {
        let text_input_ref = text_input_ref.clone();
        let project_path = project_path.clone();
        let modal = modal.clone();
    
        Callback::from(move |_| {
            let text_input_ref = text_input_ref.clone();
            let project_path = project_path.clone();
            let modal = modal.clone();
    
            spawn_local(async move {
                if let Some(input_element) = text_input_ref.cast::<HtmlElement>() {
                    let text = input_element.inner_text();
    
                    if let Some(mut path) = project_path.clone() {
                        path.push("Chapters");
                        path.push("Beginning");
                        path.push("Content.md");
    
                        let write_data = FileWriteData {
                            path: path.to_string_lossy().to_string(),
                            content: text
                        };
    
                        invoke("write_to_file", serde_wasm_bindgen::to_value(&write_data).unwrap()).await;
    
                        modal.set(html! {
                            // <Modal
                            //     content={html! {
                            //         <div>{ "Successfully saved" }</div>
                            //     }}
                            //     /*
                            //     button_configs={
                            //         vec![
                            //             ModalButtonProps {
                            //                 text: "Close".to_string(),
                            //                 text_color: "white".to_string(),
                            //                 bg_color: "green".to_string(),
                            //                 callback: {
                            //                     let modal = modal.clone();
                            //                     Callback::from(move |_| modal.set(html!()))
                            //                 }
                            //             }
                            //         ]
                            //     }*/
                            // />
                        });
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

    //let statistic_window = {
    //    let modal = modal.clone();
    //    let pages_ref = pages_ref.clone();
    //    Callback::from(move |_| {
    //        modal.set(html! {
    //            <Modal
    //            content={html!{<Statistics pages_ref={pages_ref.clone()}/>}}
    //                button_configs={vec![
    //                    ModalButtonProps {text:"Close".to_string(), text_color:"crust".to_string(), bg_color:"maroon".to_string(), callback: {
    //                    let modal = modal.clone();
    //                    Callback::from(move |_| modal.set(html!()))
    //                    }}]}
    //            />
    //        });
    //    })
    //};

    {
        let sidebar = sidebar.clone();
        let project = project.clone();
        let text_input_ref = text_input_ref.clone();
        use_effect_with(project.clone(), move |_| {
            if (*project).is_none() {
                sidebar.set(html! {
                    <div class="cursor-default select-none text-lg">{ "No Project Loaded" }</div>
                });
            } else {
                sidebar.set(
                    html! { <SideBar project={project.clone()} input_ref={text_input_ref} /> },
                );
            }
        });
    };

    let on_load = {
        //let project = project.clone();
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

    let on_undo = Callback::from(move |_: MouseEvent| {
        let html_doc: HtmlDocument = document().dyn_into().unwrap();
        html_doc.exec_command("undo").unwrap();
    });

    let on_redo = Callback::from(move |_: MouseEvent| {
        let html_doc: HtmlDocument = document().dyn_into().unwrap();
        html_doc.exec_command("redo").unwrap();
    });

    //let print_project = {
    //    Callback::from(move |_| {
    //        let project = project.clone();
    //        gloo_console::log!(format!("{}", project.as_ref().unwrap()));
    //    })
    //};

    html! {
        <div>
            <div class="light lightdark medium dark verydark" />
            <div class="modal-wrapper">{ (*modal).clone() }</div>
            <style id="dynamic-style" />
            <Toolbar />
            <div
                class="menubar bg-crust border-solid border-t-[2px] border-x-0 border-b-0 border-text"
            >
                <Button callback={open_modal} icon={IconId::LucideFilePlus} size=1.5 />
                <Button callback={on_load} icon={IconId::LucideFolderOpen} size=1.5 />
                <Button callback={save} icon={IconId::LucideSave} size=1.5 />
                <div class="w-[1px] h-[20px] bg-subtext my-0 mx-1 " />
                <Button callback={on_undo} icon={IconId::LucideUndo} size=1.5 />
                <Button callback={on_redo} icon={IconId::LucideRedo} size=1.5 />
                <div class="w-[1px] h-[20px] bg-subtext my-0 mx-1 " />
                <TextStylingControls />
            </div>
            <div class="sidebar bg-crust">
                { (*sidebar).clone() }
                <div class="absolute bottom-5 left-2 right-2">
                    <ThemeSwitcher />
                </div>
            </div>
            <div class="notepad-outer-container bg-crust" ref={pages_ref.clone()}>
                <div class="notepad-container-container bg-base">
                    <div
                        class="subbar border-b-[2px] border-t-0 border-x-0 border-solid flex items-center"
                    >
                        <ZoomControls font_size={font_size_edit.clone()} container={zoom_edit_ref} />
                    </div>
                    <div class="notepad-wrapper-edit">
                        <div
                            class="notepad-textarea-edit"
                            id="notepad-textarea-edit"
                            ref={text_input_ref}
                            style={format!("font-size: {}px; text-align: {}; transform: scale({});", *font_size_edit, *text_alignment, *zoom_level / 100.0)}
                            contenteditable="true"
                            oninput={on_text_input}
                        />
                    </div>
                </div>
                <div
                    class="notepad-container-container bg-base"
                    style={format!("font-size: {}px;", *font_size_compile)}
                >
                    <div class="subbar border-b-[2px] border-t-0 border-x-0 border-solid">
                        <ZoomControls
                            font_size={font_size_compile.clone()}
                            container={zoom_compile_ref}
                        />
                    </div>
                    <div
                        class="notepad-textarea-compile"
                        id="notepad-textarea-compile"
                        style={format!("font-size: {}px;", *font_size_compile)}
                        ref={render_ref}
                    />
                </div>
            </div>
            <div
                class="bottombar bg-crust border-solid border-t-[2px] border-x-0 border-b-0 border-text"
            >
                <div class="bottombar-left">
                    //<Statistics
                    //closing_callback={
                    //    let modal = modal.clone();
                    //    Callback::from(move |_| modal.set(html!()))
                    //}
                    //pages_ref={pages_ref.clone()}
                    // />
                    //<div>{format!("{}, {} Words; Characters: {}, {} without spaces, {:.2} wpm", *session_time, *word_count, *char_count,*char_count_no_spaces, calculated_wpm)}</div>
                    //{format!("{}", props.statistics.char_count)}
                    <Statistics
                        closing_callback={
                            let modal = modal.clone();
                            Callback::from(move |_| modal.set(html!()))
                    }
                    pages_ref={pages_ref.clone()}
                    />
                </div>
                <div class="bottombar-right" />
            </div>
        </>
    }
}

fn text_input_handler(text_input_ref: NodeRef, render_ref: NodeRef) -> Callback<InputEvent> {
    Callback::from(move |_| {
        if let Some(input) = text_input_ref.cast::<HtmlElement>() {
            let inner_text = input.inner_text();
            gloo_console::log!(&inner_text);
            let new_lines: Vec<String> = inner_text.lines().map(String::from).collect();
            //lines.set(new_lines);
            rendering_handler(&render_ref, &new_lines);
        }
    })
}

// ad br tag after end of each line (make it one string)
fn rendering_handler(render_ref: &NodeRef, new_lines: &[String]) {
    let html_strings: Vec<String> = new_lines
        .iter()
        .map(|line| {
            gloo_console::log!(line);
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            options.insert(Options::ENABLE_TABLES);

            if line.trim().is_empty() {
                "<br>".to_string()
            } else {
                let parser = Parser::new_ext(line.as_str(), options);
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);
                html_output
            }
        })
        .collect();

    let html_string: String = html_strings.join("\n");

    if let Some(rendered) = render_ref.cast::<HtmlElement>() {
        rendered.set_inner_html(html_string.as_str());
    }
}
