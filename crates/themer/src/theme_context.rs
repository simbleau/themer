use std::ops::Deref;
use themer_core::ThemeKey;
use yew::prelude::*;

use crate::Theme;

#[derive(Debug, Clone)]
pub struct ThemeContext<T>
where
    T: Theme,
{
    inner: UseStateHandle<T>,
}

impl<T> ThemeContext<T>
where
    T: Theme,
{
    pub fn new(inner: UseStateHandle<T>) -> Self {
        Self { inner }
    }

    pub fn set<K>(&self, key: K)
    where
        K: ThemeKey<T>,
    {
        self.inner.set(key.theme().clone());
    }
}

impl<T> Deref for ThemeContext<T>
where
    T: Theme,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl<T> PartialEq for ThemeContext<T>
where
    T: Theme,
{
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}
