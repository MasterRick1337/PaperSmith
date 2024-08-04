use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub rotated: bool,
    pub hidden: bool,
}

#[function_component(Chevron)]
pub fn chevron(Props { rotated, hidden }: &Props) -> Html {
    let display = use_state(|| "none");
    let rotation = use_state(|| "0");
    let hidden = *hidden;
    let rotated = *rotated;

    {
        let display = display.clone();
        let rotation = rotation.clone();
        use_effect_with([hidden, rotated], move |_| {
            if rotated {
                rotation.set("0");
            } else {
                rotation.set("-90");
            }

            if hidden {
                display.set("none");
            } else {
                display.set("flex");
            }
        });
    }

    html! {
        <div
            class="chevron transition-transform"
            style={format!("display: {}; transform: rotate({}deg)", *display, *rotation)}
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="svg-icon"
            >
                <path d="M3 8L12 17L21 8" />
            </svg>
        </div>
    }
}
