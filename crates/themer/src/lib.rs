// Re-export core
pub use themer_core::*;

// Re-export macros
pub mod macros {
    pub use themer_macros::*;
}

// Export Yew integration
pub mod yew;

// Provide prelude
pub mod prelude {
    pub use crate::browser::BrowserPreference;
    pub use crate::macros::{theme, theme_key};
    pub use crate::traits::{Theme, ThemeKey};
    pub use crate::yew::{use_theme, ThemeProvider};
}
