# themer
A non-complicated theme management library for Yew


# Examples
** TODO: This is in progress **
- Browser Preference: Example reading browser theme preference (Light/Dark)
- 2 Themes: Example of switching between two themes (e.g. Light theme, Dark theme)
- 3+ Themes: Example of switching between more than 2 themes (e.g. Winter theme, Summer theme, Autumn theme)
- Storage: Example of storing and reading theme preferences in the browser's local storage

Here's a minimal example:
```rs
use stylist::yew::styled_component;
use yew::prelude::*;

#[theme]
pub struct MyTheme {
    fg: &'static str,
    bg: &'static str,
}

pub const BLUE_THEME: MyTheme = MyTheme {
    fg: "blue",
    bg: "white",
};

#[function_component(Root)]
pub fn root() -> Html {
    html! {
        <ThemeProvider<MyTheme> theme={ BLUE_THEME } >
            <App />
        </ThemeProvider<MyTheme>>
    }
}

#[styled_component(App)]
pub fn app() -> Html {
    let theme = use_theme<MyTheme>();

    let style = css! {
        color: ${theme.fg};
        background-color: ${theme.bg};
    };

    html! {
        <label class={style} />
            {"I am a blue label"}
        </label>
    }
}

pub fn main() -> Html {
    yew::start_app::<App>();
}
```