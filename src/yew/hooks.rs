use super::ThemeContext;
use themer_core::traits::ThemeKey;
use yew::use_context;

#[yew::hook]
pub fn use_theme<K>() -> ThemeContext<K>
where
    K: ThemeKey,
{
    use_context::<ThemeContext<K>>().expect("no theme provider")
}
