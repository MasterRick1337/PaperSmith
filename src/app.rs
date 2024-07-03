use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::events::InputEvent;
use yew_icons::{Icon, IconId};
use yew::prelude::*;

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


    let on_font_increase = {
        let font_size = font_size.clone();
        Callback::from(move |_| {
            font_size.set(*font_size + 1.0);
        })
    };

    let on_font_decrease = {
        let font_size = font_size.clone();
        Callback::from(move |_| {
            font_size.set(*font_size - 1.0);
        })
    };

    html! {
        <>
            <style id="dynamic-style"></style>
            <div class="menubar">
                <Icon icon_id={IconId::LucideUndo} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon"/>
                <Icon icon_id={IconId::LucideRedo} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon"/>
                <div class="separator"></div>

                <div class="font-size-changer">
                    <Icon icon_id={IconId::LucideMinus} width={"2em".to_owned()} height={"2em".to_owned()} class="font-size-button" title="Decrease font size" onclick={on_font_decrease}/>
                    <input type="number" value={format!("{}", *font_size)} class="font-size-input" oninput={on_font_size_change} />
                    <Icon icon_id={IconId::LucidePlus} width={"2em".to_owned()} height={"2em".to_owned()} class = "font-size-button" title="Increase font size" onclick={on_font_increase}/>
                </div>

                //<Icon icon_id={IconId::}/>
                <div class="separator"></div>
                <Icon icon_id={IconId::LucideBold} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon"/>
                <Icon icon_id={IconId::LucideItalic} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon"/>
                <Icon icon_id={IconId::LucideUnderline} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon"/>
                <Icon icon_id={IconId::LucideBaseline} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon"/>
                <Icon icon_id={IconId::LucideHighlighter} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon"/>
                <div class="separator"></div>
                <Icon icon_id={IconId::LucideAlignCenter} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon"/>
                <Icon icon_id={IconId::LucideAlignJustify} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon"/>
                <Icon icon_id={IconId::LucideAlignLeft} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon"/>
                <Icon icon_id={IconId::LucideAlignRight} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon"/>
                <Icon icon_id={IconId::LucideList} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon"/>
                <Icon icon_id={IconId::LucideListChecks} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon"/>

                //<Icon icon_id={IconId::LucideSpellCheck}/>
            </div>

            <div class="sidebar">
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
                    <Icon icon_id={IconId::LucideMinus} class="zoom-button" title="Zoom Out" onclick={on_zoom_decrease}/>
                    <input type="range" min="0" max="200" class="zoom-slider" id="zoom-slider" title="Zoom" value={format!("{}", *zoom_level)} oninput={on_zoom_change} />
                    <Icon icon_id={IconId::LucidePlus} class = "zoom-button" title="Zoom In" onclick={on_zoom_increase}/>
                    <span class="zoom-text" id="zoom-value">{format!("{}%", *zoom_level)}</span>
                </div>
            </div>
        </>
    }
}



fn text_input_handler(text_input_ref: NodeRef, lines: UseStateHandle<Vec<String>>,) -> Callback<InputEvent> {
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
                if let Some(style) = document
                    .get_element_by_id("dynamic-style")
                    .and_then(|el| el.dyn_into::<HtmlElement>().ok())
                {
                    style.set_inner_html(&format!(":root {{ --font-size: {}px; }}", new_font_size));
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
