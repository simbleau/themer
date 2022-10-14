use stylist::yew::styled_component;
use yew::prelude::*;

use example::{MyTheme, MyThemeChoice};
use themer::{use_theme, ThemeProvider};

#[function_component(Root)]
pub fn root() -> Html {
    html! {
        <ThemeProvider<MyTheme, MyThemeChoice> theme={ MyThemeChoice::Dark } >
            <App />
        </ThemeProvider<MyTheme, MyThemeChoice>>
    }
}

#[styled_component(App)]
fn app() -> Html {
    let theme = use_theme::<MyTheme>().expect("No theme!");

    let style = css! {
        color: ${theme.fg};
        background-color: ${theme.bg};
    };

    html! {
        <>
        <h1 class={style}>{"I am a themed label"}</h1>
        </>
    }
}

fn main() {
    #[cfg(debug_assertions)]
    {
        // Initialize log and panics to forward to browser log if debugging
        console_log::init_with_level(log::Level::Trace)
            .expect("Failed to initialise Log!");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
    yew::start_app::<Root>();
}
