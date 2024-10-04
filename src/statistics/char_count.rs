use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CharCountProps {
    pub pages_ref: NodeRef,
}

#[function_component]
pub fn CharCount(CharCountProps { pages_ref }: &CharCountProps) -> Html {
    let char_count = use_state(|| 0);
    let char_count_no_spaces = use_state(|| 0);
    {
        let pages_ref = pages_ref.clone();
        let char_count = char_count.clone();
        let char_count_no_spaces = char_count_no_spaces.clone();
        use_interval(
            {
                let pages_ref = pages_ref.clone();
                let char_count = char_count.clone();
                let char_count_no_spaces = char_count_no_spaces.clone();
                move || {
                    if let Some(pages_element) = pages_ref.cast::<HtmlElement>() {
                        let text = pages_element.inner_text();
                        let count = text.len();
                        let count_no_spaces =
                            text.chars().filter(|c| !c.is_whitespace()).count();
                        gloo_console::log!("Text: {}", text.to_string());
                        gloo_console::log!("Character count: {}", count);
                        gloo_console::log!("Character count (no spaces): {}", count_no_spaces);
                        char_count.set(count);
                        char_count_no_spaces.set(count_no_spaces);
                    }
                }
            },
            1500,
        )
    }
    html! {
    <div>
        <p>{format!("Characters: {}, {} without spaces", *char_count, *char_count_no_spaces)}</p>
        </div>

    }
}