use chrono::prelude::*;
use chrono::TimeDelta;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component]
pub fn SessionTime() -> Html {
    let start_time = use_state(Local::now);
    let session_time = use_state(|| TimeDelta::new(0, 0).unwrap());

    use_interval(
        {
            let session_time = session_time.clone();
            move || {
                let current_time = Local::now();
                session_time.set(current_time - *start_time);
            }
        },
        1000,
    );

    let total_seconds = session_time.num_seconds();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    let formatted_time = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    html! {

        <p>{formatted_time}</p>
    }
}