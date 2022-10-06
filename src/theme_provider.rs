use crate::{Theme, ThemeKey};

use super::ThemeContext;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct ThemeProviderProps<T: Theme> {
    pub theme: T,
    pub children: Children,
}

#[function_component(ThemeProvider)]
pub fn theme_provider<T, K>(props: &ThemeProviderProps<T>) -> Html
where
    T: Theme,
    K: ThemeKey<T>,
{
    let theme_state = use_state(move || props.theme);
    let theme_ctx = ThemeContext::new(theme_state);

    html! {
        <ContextProvider<ThemeContext<T>> context={theme_ctx}>
            {props.children.clone()}
        </ContextProvider<ThemeContext<T>>>
    }
}
