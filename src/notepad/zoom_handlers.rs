use web_sys::HtmlElement;
use yew::prelude::*;
use yew_icons::IconId;

use crate::app::sidebar::buttons::Button;

pub fn zoom_increase_handler(
    font_size: UseStateHandle<f64>,
    container_ref: NodeRef,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        // Get the current font size and increase it by 1
        let current_font_size = *font_size;
        font_size.set(current_font_size + 1.0);

        // Apply the new font size to the container using inline styles
        if let Some(container) = container_ref.cast::<HtmlElement>() {
            container.set_inner_html(&format!("font-size: {}px;", current_font_size + 1.0));
        }
    })
}

pub fn zoom_decrease_handler(
    font_size: UseStateHandle<f64>,
    container_ref: NodeRef,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        // Get the current font size and decrease it by 1
        let current_font_size = *font_size;
        font_size.set(current_font_size - 1.0);

        // Apply the new font size to the container using inline styles
        if let Some(container) = container_ref.cast::<HtmlElement>() {
            container.set_inner_html(&format!("font-size: {}px;", current_font_size - 1.0));
        }
    })
}

#[derive(Properties, PartialEq)]
pub struct ZoomProps {
    pub font_size: UseStateHandle<f64>,
    pub container: NodeRef,
}

// Component to render the zoome controls
#[function_component(ZoomControls)]
pub fn zoom_controls(
    ZoomProps {
        font_size,
        container,
    }: &ZoomProps,
) -> Html {
    // Handlers for increasing and decreasing the toom
    let on_zoom_increase = zoom_increase_handler(font_size.clone(), container.clone());
    let on_zoom_decrease = zoom_decrease_handler(font_size.clone(), container.clone());

    // Render the controls with two buttons
    html! {
        <div class="subbar-icon flex items-center m-1">
            <Button callback={on_zoom_decrease} icon={IconId::LucideZoomOut} title="Zoom Out" size=2.5 />
            <Button callback={on_zoom_increase} icon={IconId::LucideZoomIn}  title="Zoom In" size=2.5 />
        </div>
    }
}
