use web_sys::HtmlInputElement;
use yew::events::InputEvent;
use yew::prelude::*;
use yew_icons::{Icon, IconId};


fn zoom_change_handler(zoom_level: UseStateHandle<f64>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        // Check if event target is an HtmlInputElement
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            // Get the new zoom level from the input field
            let new_zoom_level = input.value_as_number();
            zoom_level.set(new_zoom_level);
        }
    })
}


fn zoom_increase_handler(zoom_level: UseStateHandle<f64>) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let current_zoom = *zoom_level;
        // Determine the new zoom level by increasing by 10, ensuring it's rounded to the nearest multiple of 10
        let new_zoom_level = if current_zoom % 10.0 == 0.0 {
            (current_zoom + 10.0).min(200.0)
        } else {
            ((current_zoom / 10.0).ceil() * 10.0).min(200.0)
        };
        zoom_level.set(new_zoom_level);
    })
}


// TODO: Add better minimum zoom level value
fn zoom_decrease_handler(zoom_level: UseStateHandle<f64>) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let current_zoom = *zoom_level;
        // Determine the new zoom level by decreasing by 10, ensuring it's rounded to the nearest multiple of 10
        // Prevent the zoom level from going below 0
        let new_zoom_level = if current_zoom % 10.0 == 0.0 {
            (current_zoom - 10.0).max(50.0)
        } else {
            ((current_zoom / 10.0).floor() * 10.0).max(50.0)
        };
        zoom_level.set(new_zoom_level);
    })
}

// Properties for the ZoomControls component, holding the current zoom level state
#[derive(Properties, PartialEq)]
pub struct ZoomProps {
    pub zoom_level: UseStateHandle<f64>,
}

// TODO: Fix issue where, when zoomed, the sidebar overlaps the textarea

// Component to render the zoom controls
#[function_component(ZoomControls)]
pub fn zoom_controls(ZoomProps { zoom_level }: &ZoomProps) -> Html {
    // Handlers for changing, increasing, and decreasing the zoom level
    let on_zoom_change = zoom_change_handler(zoom_level.clone());
    let on_zoom_increase = zoom_increase_handler(zoom_level.clone());
    let on_zoom_decrease = zoom_decrease_handler(zoom_level.clone());

    // Render the zoom controls with two buttons and a slider for zoom level
    html! {
        <div class="zoom_level_changer" id="zoom">

            <Icon icon_id={IconId::LucideMinus} class="zoom-button" title="Zoom Out" onclick={on_zoom_decrease}/>
            <input type="range" min="50" max="200" class="zoom-slider" id="zoom-slider" title="Zoom" value={format!("{}", **zoom_level)} oninput={on_zoom_change} />
            <Icon icon_id={IconId::LucidePlus} class = "zoom-button" title="Zoom In" onclick={on_zoom_increase}/>
            <span class="zoom-text" id="zoom-value">{format!("{}%", **zoom_level)}</span>
        </div>
    }
}