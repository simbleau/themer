use std::ops::Deref;
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

    pub fn set(&self, choice: T) {
        self.inner.set(choice);
    }

    pub fn kind(&self) -> T {
        (*self.inner).clone()
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
