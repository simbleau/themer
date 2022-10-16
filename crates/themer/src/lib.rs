pub mod macros {
    pub use themer_macros::*;
}

pub use themer_core::*;

pub fn use_theme<K>() -> ThemeContext<K>
where
    K: ThemeKey,
{
    yew::use_context::<ThemeContext<K>>().expect("Missing theme context!")
}

mod browser_preference;
pub use browser_preference::BrowserPreference;

mod theme_context;
pub use theme_context::ThemeContext;

mod theme_provider;
pub use theme_provider::ThemeProvider;

pub mod prelude {
    pub use crate::macros::*;
    pub use crate::{
        use_theme, BrowserPreference, Theme, ThemeKey, ThemeProvider,
    };
}
