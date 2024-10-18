use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::events::InputEvent;
use yew::prelude::*;
use yew_icons::{Icon, IconId};


pub fn font_size_change_handler(font_size: UseStateHandle<f64>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        // Check if event target is an HtmlInputElement
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            // Get value from input field
            let new_font_size = input.value_as_number();
            font_size.set(new_font_size);

            // Get the window and document object
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                // Get the HTML element
                if let Some(_style) = document
                    .get_element_by_id("dynamic-style")
                    .and_then(|el| el.dyn_into::<HtmlElement>().ok())
                {
                    // Update the CSS variable for font size
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
        // Get the current font size and increase it by 1
        let current_font_size = *font_size;
        font_size.set(current_font_size + 1.0);

        // Update the CSS variable for the new font size
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
        // Get the current font size and decrease it by 1
        let current_font_size = *font_size;
        font_size.set(current_font_size - 1.0);

        // Update the CSS variable for the new font size
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

// Component to render the font size controls
#[function_component(FontSizeControls)]
pub fn font_size_controls(FontSizeProps { font_size }: &FontSizeProps) -> Html {
    // Handlers for changing, increasing, and decreasing the font size
    let on_font_size_change = font_size_change_handler(font_size.clone());
    let on_font_size_increase = font_size_increase_handler(font_size.clone());
    let on_font_size_decrease = font_size_decrease_handler(font_size.clone());

    // Render the controls with two buttons and an input field for font size
    html! {
        <div class="font-size-changer">
            // Button to decrease font size
            <Icon
                icon_id={IconId::LucideMinus}
                width={"32px".to_owned()}
                height={"32px".to_owned()}
                class="font-size-button"
                title="Decrease font size"
                onclick={on_font_size_decrease}
            />
            // Input field to set font size
            <input
                type="number"
                value={format!("{}", **font_size)}
                class="font-size-input"
                title="Font Size"
                oninput={on_font_size_change}
            />
            // Button to increase font size
            <Icon
                icon_id={IconId::LucidePlus}
                width={"32px".to_owned()}
                height={"32px".to_owned()}
                class="font-size-button"
                title="Increase font size"
                onclick={on_font_size_increase}
            />
        </div>
    }
}