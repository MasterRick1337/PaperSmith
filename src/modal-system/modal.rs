use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub content: Html,
    pub button_configs: Vec<ButtonProps>,
}

#[function_component(Modal)]
pub fn modal(
    Props {
        content,
        button_configs,
    }: &Props,
) -> Html {
    let buttons: Vec<Html> = button_configs
        .iter()
        .map(|props| {
            html! {
                <Button
                    text={props.text.clone()}
                    text_color={props.text_color.clone()}
                    bg_color={props.bg_color.clone()}
                    callback={props.callback.clone()}
                />
            }
        })
        .collect();

    gloo_console::log!("Created Modal");
    html!(
        <>
            <div
                class="absolute top-0 left-0 z-50 bg-black/60 h-full w-full flex items-center justify-center text-text"
            >
                <div class="bg-base rounded-lg max-w-[60%] p-8">
                    { content.clone() }
                    <div id="footer" class="flex justify-end w-full pt-8">{ for buttons }</div>
                </div>
            </div>
        </>
    )
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub text: String,
    pub text_color: String,
    pub bg_color: String,
    pub callback: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(
    ButtonProps {
        text,
        text_color,
        bg_color,
        callback,
    }: &ButtonProps,
) -> Html {
    html!(
        <>
            <div class="bg-maroon text-crust" />
            <div class="bg-mauve text-crust" />
            <button
                onclick={callback}
                class={format!("rounded-lg px-2 py-1 ml-4 bg-{bg_color} text-{text_color}")}
            >
                { text }
            </button>
        </>
    )
}
