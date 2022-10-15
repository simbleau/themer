use once_cell::sync::Lazy;

use crate::MyTheme;

pub static LIGHT_THEME: Lazy<MyTheme> = Lazy::new(|| MyTheme {
    fg: "#000",
    bg: "#fff",
});

pub static DARK_THEME: Lazy<MyTheme> = Lazy::new(|| MyTheme {
    fg: "#fff",
    bg: "#000",
});
