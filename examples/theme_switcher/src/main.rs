use stylist::yew::styled_component;
use yew::prelude::*;

use example::MyThemeChoice;
use themer::{use_theme, ThemeProvider};

#[function_component(Root)]
pub fn root() -> Html {
    let initial_theme = MyThemeChoice::Light;

    html! {
        <ThemeProvider<MyThemeChoice> theme={initial_theme} >
            <App />
        </ThemeProvider<MyThemeChoice>>
    }
}

#[styled_component(App)]
fn app() -> Html {
    let theme = use_theme::<MyThemeChoice>();
    let style = css! {
        & > * {
            color: ${theme.fg};
            background-color: ${theme.bg};
        }
    };

    let onclick = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let next = match theme.kind() {
                MyThemeChoice::Light => MyThemeChoice::Dark,
                MyThemeChoice::Dark => MyThemeChoice::Blue,
                MyThemeChoice::Blue => MyThemeChoice::Light,
            };
            theme.set(next);
        })
    };

    let theme_text = match theme.kind() {
        MyThemeChoice::Light => "Light",
        MyThemeChoice::Dark => "Dark",
        MyThemeChoice::Blue => "Blue",
    };

    html! {
        <div class={style}>
            <button { onclick }>{"Switch themes"}</button>
            <p>{"Current theme: "}{ theme_text }</p>
        </div>
    }
}

fn main() {
    yew::start_app::<Root>();
}
