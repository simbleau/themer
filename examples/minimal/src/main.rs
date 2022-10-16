use themer::prelude::*;
use yew::prelude::*;

#[theme]
pub struct MyTheme {
    fg: &'static str,
    bg: &'static str,
}

#[theme_key]
pub enum MyThemeChoice {
    Light,
    Dark,
}

impl ThemeKey for MyThemeChoice {
    type Theme = MyTheme;
    fn theme(&self) -> &'static Self::Theme {
        match self {
            MyThemeChoice::Light => &MyTheme {
                fg: "black",
                bg: "white",
            },
            MyThemeChoice::Dark => &MyTheme {
                fg: "white",
                bg: "black",
            },
        }
    }
}

#[function_component(Root)]
pub fn root() -> Html {
    // Get browser's preference
    let initial_theme =
        match BrowserPreference::get().unwrap_or(BrowserPreference::Light) {
            BrowserPreference::Light => MyThemeChoice::Light,
            BrowserPreference::Dark => MyThemeChoice::Dark,
        };
    html! {
        <ThemeProvider<MyThemeChoice> theme={ initial_theme } >
            <App />
        </ThemeProvider<MyThemeChoice>>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let theme = use_theme::<MyThemeChoice>();
    let style = format!("color:{}; background-color:{};", theme.fg, theme.bg);

    html! {
        <div {style}>
            {
                match theme.kind() {
                    MyThemeChoice::Light => "Light theme",
                    MyThemeChoice::Dark => "Dark theme",
                }
            }
        </div>
    }
}

pub fn main() {
    yew::start_app::<Root>();
}
