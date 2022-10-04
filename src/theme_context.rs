use gloo_storage::Storage;
use std::ops::Deref;
use yew::prelude::*;

use super::ThemeChoice;

#[derive(Debug, Clone)]
pub struct ThemeContext<T> {
    inner: UseStateHandle<ThemeChoice>,
}

impl<T> ThemeContext<T> {
    pub fn new(inner: UseStateHandle<ThemeChoice>) -> Self {
        Self { inner }
    }

    pub fn set(&self, choice: ThemeChoice) {
        self.inner.set(choice);
        // Try to save in local storage
        gloo_storage::LocalStorage::set("theme", choice)
            .expect("Theme preference could not be saved");
    }

    pub fn kind(&self) -> ThemeChoice {
        *self.inner
    }
}

impl<T> Deref for ThemeContext<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.theme()
    }
}

impl<T> PartialEq for ThemeContext<T> {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}
