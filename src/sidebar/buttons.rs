use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub callback: Callback<MouseEvent>,
    pub icon: IconId,
    #[prop_or_default]
    pub title: String,
    #[prop_or(1.)]
    pub size: f64,
}

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    pub button_props: Vec<Props>,
}

#[function_component(Button)]
pub fn button(
    Props {
        callback,
        icon,
        title,
        size,
    }: &Props,
) -> Html {
    html! {
        <div
            class="group/button flex bg-base text-text mx-1 my-auto rounded-md p-[2px] cursor-pointer items-center content-center"
            onclick={callback}
        >
            <Icon
                icon_id={*icon}
                width={format!("{size}em")}
                height={format!("{size}em")}
                title={title.clone()}
                class="group-hover/button:scale-90"
            />
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
