use stylist::yew::styled_component;
use yew::prelude::*;

use example::{MyTheme, MyThemeChoice, BLUE_THEME, LIGHT_THEME};
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

    let onclick = {
        let theme = theme.clone();
        Callback::from(move |_| {
            // Do nothing
            let next = {
                if *theme == *LIGHT_THEME {
                    MyThemeChoice::Blue
                } else if *theme == *BLUE_THEME {
                    MyThemeChoice::Dark
                } else {
                    MyThemeChoice::Light
                }
            };
            theme.set(next);
        })
    };

    let style = css! {
        color: ${theme.fg};
        background-color: ${theme.bg};
    };

    html! {
        <button class={ style } {onclick}>{"Switch themes"}</button>
    }
}

fn main() {
    yew::start_app::<Root>();
}
