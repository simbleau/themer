use std::ops::Deref;
use yew::prelude::*;

use themer_core::ThemeKey;

#[derive(Debug, Clone)]
pub struct ThemeContext<K>
where
    K: ThemeKey,
{
    inner: UseStateHandle<K>,
}

impl<K> ThemeContext<K>
where
    K: ThemeKey,
{
    pub fn new(inner: UseStateHandle<K>) -> Self {
        Self { inner }
    }

    pub fn set(&self, key: K) {
        self.inner.set(key);
    }

    pub fn kind(&self) -> K {
        *self.inner
    }
}

impl<K> Deref for ThemeContext<K>
where
    K: ThemeKey,
{
    type Target = <K as ThemeKey>::Theme;

    fn deref(&self) -> &'static Self::Target {
        self.inner.theme()
    }
}

impl<K> PartialEq for ThemeContext<K>
where
    K: ThemeKey,
{
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}
