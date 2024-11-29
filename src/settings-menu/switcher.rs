use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument, HtmlSelectElement};
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(ThemeSwitcher)]
pub fn switcher() -> Html {
    let dropdown_content = use_state(|| html!());
    let is_open = use_state(|| false);

    let switch_ref = use_node_ref();

    let themes = [
        "Light".to_string(),
        "Light Dark".to_string(),
        "Medium".to_string(),
        "Dark".to_string(),
        "Very Dark".to_string(),
    ];

    let onchange = {
        let select_ref = switch_ref.clone();

        Callback::from(move |_| {
            let select = select_ref.cast::<HtmlSelectElement>();

            if let Some(select) = select {
                let theme = select.value();
                switch_theme(theme.to_string().to_lowercase().replace(' ', ""));
            }
        })
    };


    let themes_vec = themes_to_html(Vec::from(themes.clone()));



    // let on_click_open = {
    //     let dropdown_content = dropdown_content.clone();

    //     Callback::from(move |_| {
    //         if *is_open {
    //             dropdown_content.set(html!());
    //             is_open.set(false);
    //             return;
    //         }

    //         dropdown_content.set(html! {
    //             <div class="text-lg">
    //                 { themes.iter().map(|theme| {
    //                     let theme_clone = theme.clone();
    //                 let switch_theme_callback = Callback::from(move |_| {
    //                         switch_theme(theme_clone.to_string().to_lowercase().replace(' ', ""));
    //                     });

    //                     html! {
    //                         <div
    //                         class="cursor-pointer p-2 hover:bg-mantle hover:text-subtext rounded-md"
    //                         onclick={switch_theme_callback}>{theme}
    //                         </div>
    //                     }
    //                 }).collect::<Html>() }
    //             </div>
    //         });
    //         is_open.set(true);
    //     })
    // };

    html! {
        // <div>
        //     { (*dropdown_content).clone() }
        //     <div
        //         class="group/buttoncontainer text-xl p-2 bg-base rounded-md select-none cursor-pointer hover:text-subtext hover:bg-mantle flex items-center flex-row"
        //         onclick={on_click_open.clone()}
        //     >
        //         <div class="flex-grow pl-2">{ "Theme switcher" }</div>
        //         <Icon
        //             icon_id={IconId::FontAwesomeSolidSwatchbook}
        //             width="1em"
        //             height="1em"
        //             class="pr-2"
        //         />
        //     </div>
        // </div>

        <div>
            <div>
                <select ref={switch_ref}
                        onchange={onchange}
                        class="bg-base rounded-lg text-text focus:ring-secondary border-1 border-primary"
                >
                    { themes_vec }
                </select>
            </div>
        </div>
    }
}

fn switch_theme(theme: String) {
    let html_doc: HtmlDocument = document().dyn_into().unwrap();
    let body = html_doc.body().unwrap();
    body.set_class_name(format!("{theme} bg-crust text-text").as_str());
}

fn themes_to_html(themes: Vec<String>) -> Html {
    themes
        .iter()
        .map(|theme| {
            html! {
                <option value={theme.clone()}
                >
                { theme }
                </option>
            }
        })
        .collect()
}
