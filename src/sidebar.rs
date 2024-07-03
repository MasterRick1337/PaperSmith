use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component(SideBar)]
pub fn sidebar() -> Html {
    html! {
        <>
            <div id="file-explorer">
                <div class="book-title">{"Book 1"}</div>
                <div class="chapter-list border-l-2 border-[#ccc] pl-2 ml-2">
                    <Chapter title={"Beginning".to_string()}/>
                    <Chapter title={"Middle".to_string()}/>
                    <Chapter title={"End".to_string()}/>
                </div>
            </div>
        </>

    }
}

#[derive(Properties, PartialEq)]
struct ChapterProps {
    pub title: String,
}

#[function_component(Chapter)]
fn chapter(ChapterProps { title }: &ChapterProps) -> Html {
    let content_ref = use_node_ref();
    let height = use_state(|| 0);
    let chevron2_hidden = use_state(|| true);
    let chevron1_rotated = use_state(|| false);

    let onclick = {
        let height = height.clone();
        let content_ref = content_ref.clone();
        let chevron2_hidden = chevron2_hidden.clone();
        let chevron1_rotated = chevron1_rotated.clone();

        Callback::from(move |_: MouseEvent| {
            if *height != 0 {
                height.set(0);
                chevron2_hidden.set(true);
                chevron1_rotated.set(false);
            } else {
                let content = content_ref
                    .cast::<HtmlElement>()
                    .expect("div_ref not attached to div element");
                height.set(content.scroll_height());
                chevron2_hidden.set(false);
                chevron1_rotated.set(true);
            }
        })
    };

    html! {
                    <div class="chapter">
                        <div class="chapter-title rounded-md my-[1px] hover:bg-blue-500" onclick={onclick}>
                            <Chevron rotated={*chevron1_rotated} hidden={false}/>
                            <div class="inline-block pl-5">{title}</div>
                            <div class="chapter-edit-button"></div>
                            <div class="chapter-delete-button"></div>
                        </div>
                        <div
                            class="chapter-contents pl-2 ml-2 border-l-2 border-[#ccc] text-[#AAA] overflow-hidden transition-max-height"
                            style={format!("max-height: {}px", *height)}
                            ref={content_ref}
                        >
                            <div class="notes">{"Notes"}</div>
                            <div class="extras">
                                <div class="extra-title ">
                                    <Chevron rotated={false} hidden={*chevron2_hidden}/>
                                    <div class="inline-block pl-5">{"Extras"}</div>
                                </div>
                                <div class="extras-list pl-2 ml-2 border-l-2 border-[#ccc]">
                                    <div class="extra-file ">{"extra.pdf"}</div>
                                </div>
                            </div>
                        </div>
                    </div>
    }
}

#[derive(Properties, PartialEq)]
struct ChevronProps {
    rotated: bool,
    hidden: bool,
}

#[function_component(Chevron)]
fn chevron(ChevronProps { rotated, hidden }: &ChevronProps) -> Html {
    let display = use_state(|| "none");
    let rotation = use_state(|| "0");
    let hidden = *hidden;
    let rotated = *rotated;

    {
        let display = display.clone();
        let rotation = rotation.clone();
        use_effect_with([hidden, rotated], move |_| {
            if rotated {
                rotation.set("0")
            } else {
                rotation.set("-90")
            }

            if hidden {
                display.set("none");
            } else {
                display.set("flex");
            }
        });
    }

    html! {
        <div class="chevron transition-transform"
            style={format!("display: {}; transform: rotate({}deg)", *display, *rotation)}
        >
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="svg-icon">
                <path d="M3 8L12 17L21 8"></path>
            </svg>
        </div>
    }
}
