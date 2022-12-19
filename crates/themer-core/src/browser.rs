use gloo_storage::errors::StorageError;
use gloo_storage::Storage;
use serde::{Deserialize, Serialize};
use crate::traits::ThemeKey;

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

    pub fn load<T>() -> Option<T>
    where
        T: ThemeKey + for<'de> serde::Deserialize<'de>,
    {
        gloo_storage::LocalStorage::get("theme").ok()
    }

    pub fn save<T>(key: T) -> Result<(), StorageError>
    where
        T: ThemeKey + serde::Serialize,
    {
        gloo_storage::LocalStorage::set("theme", key)
    }

    pub fn clear() {
        gloo_storage::LocalStorage::delete("theme")
    }
}
