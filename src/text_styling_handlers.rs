use yew::prelude::*;
use yew_icons::{Icon, IconId};

fn apply_style_textarea(textarea: &web_sys::HtmlTextAreaElement, style: &str) {
    let value = textarea.value();
    let selection_start = textarea.selection_start().unwrap().unwrap() as usize;
    let selection_end = textarea.selection_end().unwrap().unwrap() as usize;

    let selected_text = &value[selection_start..selection_end];
    let new_text = format!("{style}{selected_text}{style}");

    let new_value = format!(
        "{}{}{}",
        &value[..selection_start],
        new_text,
        &value[selection_end..]
    );

    textarea.set_value(&new_value);

    textarea.set_selection_range(selection_end as u32 + (style.len() * 2) as u32, selection_end as u32 + (style.len() * 2) as u32).unwrap();
}

#[derive(Properties, PartialEq)]
pub struct StyleButtonProps {
    pub textarea_ref: NodeRef,
    pub icon: IconId,
    pub title: String,
    pub style: String,
}

#[function_component(StyleButton)]
pub fn style_button(StyleButtonProps {textarea_ref, icon, title, style,}: &StyleButtonProps) -> Html {
    let onclick = {
        let textarea_ref = textarea_ref.clone();
        let style = style.clone();
        Callback::from(move |_| {
        if let Some(textarea) = textarea_ref.cast::<web_sys::HtmlTextAreaElement>() {
            apply_style_textarea(&textarea, &style);
        }
    })
    };

    html! {
        <Icon
            icon_id={*icon}
            width={"2em".to_owned()}
            height={"2em".to_owned()}
            class="menubar-icon"
            title={title.clone()}
            onclick={onclick}
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct TextStylingProps {
    pub text_styling_ref: NodeRef,
}

#[function_component(TextStylingControls)]
pub fn text_styling_controls(TextStylingProps { text_styling_ref }: &TextStylingProps) -> Html {

    html! {
        <>
            //<textarea id="notepad-textarea" ref={textarea_ref.clone()} class="notepad-textarea"></textarea>
            <div class="text-styling-changer">
                <StyleButton
                    textarea_ref={text_styling_ref.clone()}
                    icon={IconId::LucideBold}
                    title="Bold"
                    style="**"
                />
                <StyleButton
                    textarea_ref={text_styling_ref.clone()}
                    icon={IconId::LucideItalic}
                    title="Italic"
                    style="_"
                />
                <StyleButton
                    textarea_ref={text_styling_ref.clone()}
                    icon={IconId::LucideUnderline}
                    title="Underline"
                    style="__"
                />
            </div>
        </>
    }
}
