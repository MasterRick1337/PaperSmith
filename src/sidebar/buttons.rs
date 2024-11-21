use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq, Clone)]
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
            class="flex bg-mantle border-overlay0 text-text mx-1 my-auto border-solid border-[1px] rounded-md p-[1px] cursor-pointer items-center content-center"
            onclick={callback}
        >
            <Icon icon_id={*icon} width="1em" height="1em" />
        </div>
    }
}
#[function_component(ButtonContainer)]
pub fn button_container(ContainerProps { button_props }: &ContainerProps) -> Html {
    html!(
        <div class="hidden group-hover:flex items-center ml-auto my-auto">
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
