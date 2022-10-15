use stylist::yew::styled_component;
use yew::prelude::*;

use example::MyTheme;
use themer::{use_theme, BrowserPreference, ThemeProvider};

#[function_component(Root)]
pub fn root() -> Html {
    let preference =
        BrowserPreference::get().unwrap_or(BrowserPreference::Light);

    html! {
        <ThemeProvider<MyTheme, BrowserPreference> theme={ preference } >
            <App />
        </ThemeProvider<MyTheme, BrowserPreference>>
    }
}

#[styled_component(App)]
fn app() -> Html {
    let theme = use_theme::<MyTheme>();

    let style = css! {
        color: ${theme.fg};
        background-color: ${theme.bg};
    };

    let text = match BrowserPreference::get() {
        Some(BrowserPreference::Light) => "Your browser prefers a Light theme.",
        Some(BrowserPreference::Dark) => "Your browser prefers a Dark theme.",
        None => "Your browser doesn't have a theme preference, so we defaulted to Light theme.",
    };

    html! {
        <h1 class={style}>
            { text }
        </h1>
    }
}

fn main() {
    yew::start_app::<Root>();
}
