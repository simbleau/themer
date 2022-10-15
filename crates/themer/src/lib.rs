pub mod macros {
    pub use themer_macros::*;
}

pub use themer_core::*;

pub fn use_theme<T>() -> ThemeContext<T>
where
    T: crate::Theme,
{
    yew::use_context::<ThemeContext<T>>().expect("Missing theme context!")
}

mod browser_preference;
pub use browser_preference::BrowserPreference;

mod theme_context;
pub use theme_context::ThemeContext;

mod theme_provider;
pub use theme_provider::ThemeProvider;
