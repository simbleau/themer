use serde::{Deserialize, Serialize};
use themer::{
    macros::{theme, theme_key},
    traits::ThemeKey,
};

#[theme]
pub struct MyTheme {
    pub fg: &'static str,
    pub bg: &'static str,
}

#[theme_key]
#[derive(Serialize, Deserialize, Default)]
pub enum MyThemeChoice {
    #[default]
    Light,
    Dark,
}

impl ThemeKey for MyThemeChoice {
    type Theme = MyTheme;
    fn theme(&self) -> &'static Self::Theme {
        use crate::my_themes::{DARK_THEME, LIGHT_THEME};
        match self {
            MyThemeChoice::Light => &LIGHT_THEME,
            MyThemeChoice::Dark => &DARK_THEME,
        }
    }
}
