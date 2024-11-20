use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub callback: Callback<MouseEvent>,
    pub icon: IconId,
}

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    pub button_props: Vec<Props>,
}

#[function_component(Button)]
pub fn button(Props { callback, icon }: &Props) -> Html {
    html! {
        <div
            class="sidebar-dropdown-icon bg-mantle border-overlay0 text-text mx-1 items-center content-center flex"
            onclick={callback}
        >
            <Icon icon_id={*icon} width="1em" height="1em" />
        </div>
    }
}
#[function_component(ButtonContainer)]
pub fn button_container(ContainerProps { button_props }: &ContainerProps) -> Html {
    html!(
        <div class="items-center ml-auto my-auto hide-parent-hover">
            { button_props
            .iter()
            .map(|props| {
                html! { <>
                    <Button callback={props.callback.clone()} icon={props.icon}/>
                    </>
                }
            })
            .collect::<Html>() }
        </div>
    )
}
