use std::marker::PhantomData;

use crate::{Theme, ThemeKey};

use super::ThemeContext;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct ThemeProviderProps<T, K>
where
    T: Theme,
    K: ThemeKey<T>,
{
    pub theme: K,
    pub children: Children,
    #[prop_or_default]
    _pd: PhantomData<T>,
}

#[function_component(ThemeProvider)]
pub fn theme_provider<T, K>(props: &ThemeProviderProps<T, K>) -> Html
where
    T: Theme,
    K: ThemeKey<T>,
{
    let theme_state = use_state(move || props.theme.theme().clone());
    let theme_ctx = ThemeContext::new(theme_state);

    html! {
        <ContextProvider<ThemeContext<T>> context={theme_ctx}>
            {props.children.clone()}
        </ContextProvider<ThemeContext<T>>>
    }
}
