use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrowserPreference {
    Dark,
    Light,
}

impl BrowserPreference {
    pub fn get() -> Option<Self> {
        // query the browser for preference
        match web_sys::window()
            .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok())
            .flatten()
            .map(|m| m.matches())
        {
            // Browser prefers dark theme
            Some(true) => Some(BrowserPreference::Dark),
            // Browser prefers light theme
            Some(false) => Some(BrowserPreference::Light),
            // Browser was not queryable
            None => None,
        }
    }
}
