use stylist::yew::styled_component;
use yew::prelude::*;

use themer::BrowserPreference;

#[styled_component(App)]
fn app() -> Html {
    let text = match BrowserPreference::get() {
        Some(BrowserPreference::Light) => "Your browser prefers a Light theme.",
        Some(BrowserPreference::Dark) => "Your browser prefers a Dark theme.",
        None => "Your browser doesn't have a theme preference.",
    };

    html! {
        <h1>
            { text }
        </h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
