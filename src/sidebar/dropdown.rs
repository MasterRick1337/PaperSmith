use gloo_timers::callback::Timeout;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
#[path = "chevron.rs"]
mod chevron;

use chevron::Chevron;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub open: bool,
    pub children: Html,
    pub dropdown_type: Type,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Type {
    Chapter,
    Notes,
    Extras,
}

#[function_component(Dropdown)]
pub fn dropdown(
    Props {
        title,
        open,
        dropdown_type,
        children,
    }: &Props,
) -> Html {
    let transition_string = use_state(|| "max-height: 0px".to_string());
    let content_ref = use_node_ref();
    let chevron2_hidden = use_state(|| true);
    let chevron_rotated = use_state(|| *open);

    let onclick = {
        let transition_string = transition_string.clone();
        let content_ref = content_ref.clone();
        let chevron2_hidden = chevron2_hidden;
        let chevron1_rotated = chevron_rotated.clone();

        Callback::from(move |_: MouseEvent| {
            if transition_string.contains(" 0px") {
                let content = content_ref
                    .cast::<HtmlElement>()
                    .expect("div_ref not attached to div element");

                transition_string.set(format!(
                    "max-height: {}px; transition: max-height 100ms ease-in-out",
                    content.scroll_height()
                ));

                chevron2_hidden.set(false);
                chevron1_rotated.set(true);

                let _timeout = Timeout::new(100, {
                    let transition_string = transition_string.clone();
                    move || {
                        transition_string.set("max-height: none".to_string());
                    }
                })
                .forget();
            } else {
                let content = content_ref
                    .cast::<HtmlElement>()
                    .expect("div_ref not attached to div element");

                transition_string.set(format!("max-height: {}px", content.scroll_height()));

                let _timeout = Timeout::new(1, {
                    let transition_string = transition_string.clone();
                    move || {
                        transition_string.set(
                            "max-height: 0px; transition: max-height 100ms ease-in-out".to_string(),
                        );
                    }
                })
                .forget();

                chevron2_hidden.set(true);
                chevron1_rotated.set(false);
            }
        })
    };

    html! {
        <div class="chapter">
            <div
                class="chapter-title rounded-md my-[1px] hover:bg-mauve hover:text-mantle"
                onclick={onclick}
            >
                <Chevron rotated={*chevron_rotated} hidden=false />
                <div class="inline-block pl-5 text-ellipsis whitespace-nowrap overflow-hidden">
                    { title }
                </div>
                { get_buttons(*dropdown_type) }
            </div>
            <div
                class="chapter-contents pl-2 ml-2 border-l-2 border-[#ccc] text-[#AAA] overflow-hidden"
                style={<std::string::String as Clone>::clone(&*transition_string)}
                ref={content_ref}
            >
                { children.clone() }
            </div>
        </div>
    }
}

fn get_buttons(dropdown_type: Type) -> Html {
    match dropdown_type {
        Type::Chapter => {
            html! (
                <div class="sidebar-dropdown-icon-container hide-parent-hover">
                    <div class="sidebar-dropdown-icon bg-mantle border-overlay0 hover: text-text">
                        <Icon icon_id={IconId::LucideEdit3} width="16px" height="16px" />
                    </div>
                    <div
                        class="sidebar-dropdown-icon bg-mantle border-overlay0 hover: text-text mx-1"
                    >
                        <Icon icon_id={IconId::LucideTrash2} width="16px" height="16px" />
                    </div>
                </div>
            )
        }
        Type::Notes => {
            html! (
                <div class="sidebar-dropdown-icon-container hide-parent-hover">
                    <div
                        class="sidebar-dropdown-icon bg-mantle border-overlay0 hover: text-text mx-1"
                    >
                        <Icon icon_id={IconId::LucidePlus} width="16px" height="16px" />
                    </div>
                </div>
            )
        }
        Type::Extras => {
            html! (
                <div class="sidebar-dropdown-icon-container hide-parent-hover">
                    <div
                        class="sidebar-dropdown-icon bg-mantle border-overlay0 hover: text-text mx-1"
                    >
                        <Icon icon_id={IconId::LucidePlus} width="16px" height="16px" />
                    </div>
                </div>
            )
        }
    }
}
