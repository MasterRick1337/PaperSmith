use web_sys::{HtmlButtonElement, HtmlInputElement};
use yew::prelude::*;
use scan_fonts::scan_fonts;

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
    let input_font_size_ref = use_node_ref();
    let on_font_size_input = number_input_handler(font_size.clone());

    let _ = load_fonts();

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
            <div id="font_size_change" class="flex w-full pt-8 justify-between">
                <div class="font-semibold self-center">{ "Font Size" }</div>
                <div class="rounded-lg border-transparent hover:border-mauve">
                    <input
                        oninput={on_font_size_input}
                        class="outline-none bg-crust p-2 rounded-lg border-2 border-transparent"
                        ref={input_font_size_ref}
                    />
                </div>
            </div>
            <div id="font_change" class="flex w-full pt-8 justify-between">
                <div class="font-semibold self-center">{"Font"}</div>
                <div class="rounded-lg border-transparent hover:border-mauve">
                    <select id="fonts">
                        <option value="Arial">{ "Arial" }</option>
                    </select>
                </div>
            </div>
            <div class="flex justify-end w-full pt-8">
                <button
                ref={confirm_button_ref}
                onclick={on_confirm}
                class="rounded-lg text-lg px-2 py-1 ml-4 bg-mauve text-crust hover:scale-105"
                >
                    { "Confirm" }
                </button>
            </div>
        </>
    )
}


fn number_input_handler(value: UseStateHandle<String>) -> Callback<InputEvent> {
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
                    gloo_console::log!(format!("could not convert '{}' to number: {:?}", text, err));

                    let _ = input.style().set_property("border-color", "red");
                }
            }
        }
    })
}

fn load_fonts() -> Html {
    let Some(fonts) = scan_fonts(".");
    println!(fonts);
}
