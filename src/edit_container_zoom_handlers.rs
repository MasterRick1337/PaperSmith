use web_sys::HtmlElement;
use yew::prelude::*;
use yew_icons::{Icon, IconId};


pub fn zoom_increase_handler(font_size_edit: UseStateHandle<f64>, container_ref: NodeRef) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        // Get the current font size and increase it by 1
        let current_font_size = *font_size_edit;
        font_size_edit.set(current_font_size + 1.0);

        // Apply the new font size to the container using inline styles
        if let Some(container) = container_ref.cast::<HtmlElement>() {
            container.set_inner_html(&format!("font-size: {}px;", current_font_size + 1.0));
        }
    })
}

pub fn zoom_decrease_handler(font_size_edit: UseStateHandle<f64>, container_ref: NodeRef) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        // Get the current font size and decrease it by 1
        let current_font_size = *font_size_edit;
        font_size_edit.set(current_font_size - 1.0);

        // Apply the new font size to the container using inline styles
        if let Some(container) = container_ref.cast::<HtmlElement>() {
            container.set_inner_html(&format!("font-size: {}px;", current_font_size - 1.0));
        }
    })
}

#[derive(Properties, PartialEq)]
pub struct ZoomProps {
    pub font_size_edit: UseStateHandle<f64>,
}

// Component to render the zoome controls
#[function_component(ZoomControlsEdit)]
pub fn zoom_controls(ZoomProps { font_size_edit }: &ZoomProps) -> Html {
    let container_ref = use_node_ref();
    // Handlers for increasing and decreasing the toom
    let on_zoom_increase = zoom_increase_handler(font_size_edit.clone(), container_ref.clone());
    let on_zoom_decrease = zoom_decrease_handler(font_size_edit.clone(), container_ref.clone());

    // Render the controls with two buttons
    html! {
        <div class="subbar-edit-icon">
            // Button to decrease zoom
            <Icon
                class="zoom-out-button-edit"
                icon_id={IconId::LucideZoomOut}
                title="Zoom Out"
                onclick={on_zoom_decrease}
            />
            // Button to increase zoom
            <Icon
                class="zoom-in-button-edit"
                icon_id={IconId::LucideZoomIn}
                title="Zoom In"
                onclick={on_zoom_increase}
            />
        </div>
    }
}