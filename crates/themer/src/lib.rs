// Re-export core
pub use themer_core::*;

// Re-export macros
#[cfg(feature = "macros")]
pub mod macros {
    pub use themer_macros::*;
}

// Export Yew integration
#[cfg(feature = "yew_integration")]
pub mod yew;

// Provide prelude
pub mod prelude {
    pub use crate::browser::BrowserPreference;
    pub use crate::traits::{Theme, ThemeKey};

    #[cfg(feature = "macros")]
    pub use crate::macros::{theme, theme_key};
    #[cfg(feature = "yew")]
    pub use crate::yew::{use_theme, ThemeProvider};
}
