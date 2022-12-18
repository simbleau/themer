use crate::my_themes::{BLUE_THEME, DARK_THEME, LIGHT_THEME};
use themer::macros::*;
use themer::ThemeKey;

#[theme]
pub struct MyTheme {
    pub fg: &'static str,
    pub bg: &'static str,
}

#[theme_key]
pub enum MyThemeChoice {
    Light,
    Dark,
    Blue,
}

impl ThemeKey for MyThemeChoice {
    type Theme = MyTheme;
    fn theme(&self) -> &'static Self::Theme {
        match self {
            MyThemeChoice::Light => &LIGHT_THEME,
            MyThemeChoice::Dark => &DARK_THEME,
            MyThemeChoice::Blue => &BLUE_THEME,
        }
    }
}
