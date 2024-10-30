use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub content: Html,
}

#[function_component(Modal)]
pub fn modal(Props { content }: &Props) -> Html {
    html!(
        <>
            <div
                class="absolute top-0 left-0 z-50 bg-black/60 h-full w-full flex items-center justify-center text-text"
            >
                <div class="bg-base rounded-lg max-w-[80%] min-w-[80%] p-8">
                    { content.clone() }
                </div>
            </div>
        </>
    )
}
