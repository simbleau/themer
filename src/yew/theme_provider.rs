use super::ThemeContext;
use themer_core::traits::ThemeKey;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct ThemeProviderProps<K>
where
    K: ThemeKey,
{
    pub theme: K,
    pub children: Children,
}

#[function_component(ThemeProvider)]
pub fn theme_provider<K>(props: &ThemeProviderProps<K>) -> Html
where
    K: ThemeKey,
{
    let themekey = props.theme;
    let theme_state = use_state(move || themekey);
    let theme_ctx = ThemeContext::new(theme_state);

    html! {
        <ContextProvider<ThemeContext<K>> context={ theme_ctx }>
            {props.children.clone()}
        </ContextProvider<ThemeContext<K>>>
    }
}
