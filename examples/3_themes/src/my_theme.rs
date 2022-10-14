use themer::macros::*;
use themer::ThemeKey;

use crate::{BLUE_THEME, DARK_THEME, LIGHT_THEME};

#[theme_key(MyTheme)]
pub enum MyThemeChoice {
    Light,
    Dark,
    Blue,
}

impl ThemeKey<MyTheme> for MyThemeChoice {
    fn theme(&self) -> &'static MyTheme {
        match self {
            MyThemeChoice::Light => &LIGHT_THEME,
            MyThemeChoice::Dark => &DARK_THEME,
            MyThemeChoice::Blue => &BLUE_THEME,
        }
    }
}

#[theme]
pub struct MyTheme {
    pub fg: &'static str,
    pub bg: &'static str,
}
