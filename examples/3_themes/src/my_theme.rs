use themer::macros::*;
use themer::ThemeKey;

#[theme]
pub struct MyTheme {
    pub fg: &'static str,
    pub bg: &'static str,
}

#[theme_key(MyTheme)]
pub enum MyThemeChoice {
    Light,
    Dark,
    Blue,
}

impl ThemeKey<MyTheme> for MyThemeChoice {
    fn theme(&self) -> &'static MyTheme {
        use crate::{BLUE_THEME, DARK_THEME, LIGHT_THEME};
        match self {
            MyThemeChoice::Light => &LIGHT_THEME,
            MyThemeChoice::Dark => &DARK_THEME,
            MyThemeChoice::Blue => &BLUE_THEME,
        }
    }
}
