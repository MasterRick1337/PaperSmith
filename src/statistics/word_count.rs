use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Properties, PartialEq)]
    pub struct WordCountProps {
        pub pages_ref: NodeRef,
    }

#[function_component]
    pub fn WordCount(WordCountProps { pages_ref }: &WordCountProps) -> Html {
        let word_count = use_state(|| 0);
        {
            let pages_ref = pages_ref.clone();
            let word_count = word_count.clone();
            use_interval(
                {
                    let pages_ref = pages_ref.clone();
                    let word_count = word_count.clone();
                    move || {
                        if let Some(pages_element) = pages_ref.cast::<HtmlElement>() {
                            let text = pages_element.inner_text();
                            let count = text.split_whitespace().count();
                            word_count.set(count);
                        }
                    }
                },
                1500,
            )
        }

        html! {
        <div>{format!("{} Words", *word_count)}</div>
    }
    }
