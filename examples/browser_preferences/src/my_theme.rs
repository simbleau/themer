use themer::macros::*;
use themer::{BrowserPreference, ThemeKey};

#[theme]
pub struct MyTheme {
    pub fg: &'static str,
    pub bg: &'static str,
}

impl ThemeKey<MyTheme> for BrowserPreference {
    fn theme(&self) -> &'static MyTheme {
        use crate::{DARK_THEME, LIGHT_THEME};
        match self {
            BrowserPreference::Light => &LIGHT_THEME,
            BrowserPreference::Dark => &DARK_THEME,
        }
    }
}
