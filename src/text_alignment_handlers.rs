use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew::events::MouseEvent;

// TODO: Add that it only applies to selected text or text that is abut to be written
// TODO: Make it apply only to selected text, work on details
pub fn align_left(text_alignment: UseStateHandle<String>) -> Callback<MouseEvent> {
    Callback::from(move |_| text_alignment.set("left".to_string()))
}

pub fn align_center(text_alignment: UseStateHandle<String>) -> Callback<MouseEvent> {
    Callback::from(move |_| text_alignment.set("center".to_string()))
}

pub fn align_right(text_alignment: UseStateHandle<String>) -> Callback<MouseEvent> {
    Callback::from(move |_| text_alignment.set("right".to_string()))
}

pub fn align_justify(text_alignment: UseStateHandle<String>) -> Callback<MouseEvent> {
    Callback::from(move |_| text_alignment.set("justify".to_string()))
}


#[derive(Properties, PartialEq)]
pub struct TextAlignmenteProps {
    pub text_alignment: UseStateHandle<String>,
}

#[function_component(TextAlignmentControls)]
pub fn font_size_controls(TextAlignmenteProps { text_alignment }: &TextAlignmenteProps) -> Html {
    let on_align_left = align_left(text_alignment.clone());
    let on_align_center = align_center(text_alignment.clone());
    let on_align_right = align_right(text_alignment.clone());
    let on_align_justify = align_justify(text_alignment.clone());

    html! {
        <div class="text-alignment-changer">
            <Icon icon_id={IconId::LucideAlignCenter} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon" title="Align Center" onclick={on_align_center}/>
            <Icon icon_id={IconId::LucideAlignJustify} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon" title="Align Justify" onclick={on_align_justify}/>
            <Icon icon_id={IconId::LucideAlignLeft} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon" title="Align Left" onclick={on_align_left}/>
            <Icon icon_id={IconId::LucideAlignRight} width={"2em".to_owned()} height={"2em".to_owned()} class="menubar-icon" title="Align Right" onclick={on_align_right}/>
        </div>
    }
}