# themer
Non-complicated theme management in Rust for WebAssembly


# Example
```rs
pub static MY_THEME: Lazy<Theme> = Lazy::new(|| Theme {
    fg: Color::opaque(0x44, 0x44, 0x44),
    bg: Color::opaque(0xff, 0xff, 0xff),
});

#[function_component(App)]
pub fn app() -> Html {
    let theme = use_theme();
    let fg = theme.fg;
    let bg = theme.bg;

    let style = css! {
        color: ${fg};
        background-color: ${bg};
    };

    html! {
        <ThemeProvider>
            <label class={style} />{"Hello"}</label>
        </ThemeProvider>
    }
}

pub fn main() -> Html {
    themer::set_theme(MY_THEME);
    yew::start_app::<App>();
}
```