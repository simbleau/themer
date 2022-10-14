pub mod macros {
    pub use themer_macros::*;
}

pub use themer_core::*;

pub fn use_theme<T>() -> T
where
    T: themer_core::Theme,
{
    yew::use_context::<T>().unwrap()
}

mod browser_preference;
pub use browser_preference::BrowserPreference;

mod theme_context;
pub use theme_context::ThemeContext;

mod theme_provider;
pub use theme_provider::ThemeProvider;
