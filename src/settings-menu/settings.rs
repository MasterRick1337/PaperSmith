use web_sys::{HtmlButtonElement, HtmlInputElement};
use yew::prelude::*;

#[path = "switcher.rs"]
mod switcher;
use switcher::ThemeSwitcher;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub closing_callback: Callback<MouseEvent>,
}

#[function_component(Settings)]
pub fn settings_menu(
    Props {
        closing_callback: on_close,
    }: &Props,
) -> Html {
    let font_size = use_state(String::new);
    let confirm_button_ref = use_node_ref();

    let on_confirm = {
        let on_close = on_close.clone();

        Callback::from(move |_| {
            gloo_console::log!("button pressed");

            on_close.emit(MouseEvent::new("Dummy").unwrap())
        })
    };

    html!(
        <>
            <div class="text-xl font-bold">{ "Settings" }</div>
            <br />
            // <div id="font_size_change" class="flex w-full pt-8 justify-between">
            //     <div class="font-semibold self-center">{ "Font Size" }</div>
            //     <div class="rounded-lg border-transparent hover:border-mauve">
            //         <input
            //             oninput={on_font_size_input}
            //             class="outline-none bg-crust p-2 rounded-lg border-2 border-transparent"
            //             ref={input_font_size_ref}
            //         />
            //     </div>
            // </div>
            <div id="theme_change" class="flex w-full pt-8 justify-between">
                <div class="font-bold self-center">{"Theme"}</div>
                <ThemeSwitcher />
            </div>
            <div class="flex justify-end w-full pt-8">
                <button
                ref={confirm_button_ref}
                onclick={on_confirm}
                class="rounded-lg text-lg px-2 py-1 ml-4 bg-primary text-crust hover:scale-105 border-0"
                >
                    { "Confirm" }
                </button>
                <button
                onclick={on_close}
                class="rounded-lg text-lg px-2 py-1 ml-4 bg-secondary text-crust hover:scale-105 border-0"
                >
                { "Close" }
            </button>
            </div>
        </>
    )
}

fn field_input_handler(value: UseStateHandle<String>) -> Callback<InputEvent> {
    Callback::from(move |ev: InputEvent| {
        if let Some(input) = ev.target_dyn_into::<HtmlInputElement>() {
            let text = input.value();

            match text.clone().parse::<u32>() {
                Ok(n) => {
                    gloo_console::log!(format!("text is {:?}", n));
                    let _ = input.style().set_property("border-color", "transparent");
                    value.set(text)
                }
                Err(err) => {
                    gloo_console::log!(format!(
                        "could not convert '{}' to number: {:?}",
                        text, err
                    ));

                    let _ = input.style().set_property("border-color", "red");
                }
            }
        }
    })
}
