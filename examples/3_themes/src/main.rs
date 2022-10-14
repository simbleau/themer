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
    let theme = use_theme::<MyTheme>();

    let style = css! {
        r#"
            color: ${fg};
            background-color: ${bg};
        "#,
        fg = theme.fg,
        bg = theme.bg
    };

    html! {
        <>
        <h1 class={style}>{"I am a themed label"}</h1>
        </>
    }
}

fn main() {
    yew::start_app::<Root>();
}
