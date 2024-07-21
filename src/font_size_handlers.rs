use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::events::InputEvent;
use yew::prelude::*;
use yew_icons::{Icon, IconId};


// TODO: Add that it only applies to selected text or text that is abut to be written
pub fn font_size_change_handler(font_size: UseStateHandle<f64>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            let new_font_size = input.value_as_number();
            font_size.set(new_font_size);

            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if let Some(_style) = document
                    .get_element_by_id("dynamic-style")
                    .and_then(|el| el.dyn_into::<HtmlElement>().ok())
                {
                    if let Some(style) = document
                        .get_element_by_id("dynamic-style")
                        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
                    {
                        style.set_inner_html(&format!(
                            ":root {{ --font-size: {}px; }}",
                            new_font_size
                        ));
                    }
                }
            }
        }
    })
}

pub fn font_size_increase_handler(font_size: UseStateHandle<f64>) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let current_font_size = *font_size;
        font_size.set(current_font_size + 1.0);

        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(style) = document
                .get_element_by_id("dynamic-style")
                .and_then(|el| el.dyn_into::<HtmlElement>().ok())
            {
                style.set_inner_html(&format!(
                    ":root {{ --font-size: {}px; }}",
                    current_font_size + 1.0
                ));
            }
        }
    })
}

pub fn font_size_decrease_handler(font_size: UseStateHandle<f64>) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let current_font_size = *font_size;
        font_size.set(current_font_size - 1.0);

        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(style) = document
                .get_element_by_id("dynamic-style")
                .and_then(|el| el.dyn_into::<HtmlElement>().ok())
            {
                style.set_inner_html(&format!(
                    ":root {{ --font-size: {}px; }}",
                    current_font_size - 1.0
                ));
            }
        }
    })
}



#[derive(Properties, PartialEq)]
pub struct FontSizeProps {
    pub font_size: UseStateHandle<f64>,
}



#[function_component(FontSizeControls)]
pub fn font_size_controls(FontSizeProps { font_size }: &FontSizeProps) -> Html {
    let on_font_size_change = font_size_change_handler(font_size.clone());
    let on_font_size_increase = font_size_increase_handler(font_size.clone());
    let on_font_size_decrease = font_size_decrease_handler(font_size.clone());

    html! {
        <div class="font-size-changer">
            <Icon icon_id={IconId::LucideMinus} width={"2em".to_owned()} height={"2em".to_owned()} class="font-size-button" title="Decrease font size" onclick={on_font_size_decrease}/>
            <input type="number" value={format!("{}", **font_size)} class="font-size-input" title="Font Size" oninput={on_font_size_change} />
            <Icon icon_id={IconId::LucidePlus} width={"2em".to_owned()} height={"2em".to_owned()} class = "font-size-button" title="Increase font size" onclick={on_font_size_increase}/>
        </div>
    }
}
