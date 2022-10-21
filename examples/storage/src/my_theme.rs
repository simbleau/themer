use serde::{Deserialize, Serialize};
use themer::macros::*;
use themer::ThemeKey;

#[theme]
pub struct MyTheme {
    pub fg: &'static str,
    pub bg: &'static str,
}

#[theme_key]
#[derive(Serialize, Deserialize)]
pub enum MyThemeChoice {
    Light,
    Dark,
}

impl Default for MyThemeChoice {
    fn default() -> Self {
        MyThemeChoice::Light
    }
}

impl ThemeKey for MyThemeChoice {
    type Theme = MyTheme;
    fn theme(&self) -> &'static Self::Theme {
        use crate::themes::{DARK_THEME, LIGHT_THEME};
        match self {
            MyThemeChoice::Light => &LIGHT_THEME,
            MyThemeChoice::Dark => &DARK_THEME,
        }
    }
}
