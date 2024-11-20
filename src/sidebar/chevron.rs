use yew::prelude::*;
use yew_icons::{Icon, IconId};

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
            class="absolute w-5 h-4 content-center items-center flex transition-transform"
            style={format!("display: {}; transform: rotate({}deg)", *display, *rotation)}
        >
            <Icon
                icon_id={IconId::LucideChevronDown}
                width={"2em".to_owned()}
                height={"2em".to_owned()}
                class=""
            />
        </div>
    }
}
