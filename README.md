# Themer
A non-complicated theme management library for Yew.

Note: Themer is **NOT** a CSS solution. You may use any CSS styling solution you wish (e.g. [Stylist](https://crates.io/crates/stylist)).

# Status
**Ongoing development. No/little documentation, rapidly changing, etc.**

# Examples
Examples can be served with [Trunk](https://trunkrs.dev/) (e.g. `trunk serve`)

- [Minimal](examples/minimal/): Example using the browser's preferred theme.
- [Read browser preference](examples/read-browser-preference/): Display your browser's theme preference (Light/Dark) on a label (no styling applied).
- [Theme Switcher](examples/theme-switcher/): A button which switches between 3 themes.
- [Storage](examples/storage/): Store user-saved theme preferences with a button and load those preferences automatically on refresh.

## API Foreward
### Step 1: Setup a Theme and Key
The API asks you to create a Theme template struct with the `#[theme]` macro, and a key for access with the `#[theme_key]` macro. The theme key is used to reduce memory footprint and provides ergonomics.
```rs
#[theme]
pub struct MyTheme {
    // Foreground color
    fg: &'static str,
    // Background color
    bg: &'static str,
    // Font size
    fs: &'static str,
}

#[theme_key]
pub enum MyThemeChoice {
    Light,
    Dark,
}

impl ThemeKey for MyThemeChoice {
    type Theme = MyTheme;
    fn theme(&self) -> &'static Self::Theme {
        match self {
            MyThemeChoice::Light => &MyTheme {
                fg: "black",
                bg: "white",
                fs: "1.2em",
            },
            MyThemeChoice::Dark => &MyTheme {
                fg: "white",
                bg: "black",
                fs: "1.2em",
            },
        }
    }
}
```

### Step 2: Provide your theme
To pass your theme to your app, you must register a `ThemeProvider` with your theme choice. This works as a [Context](https://yew.rs/docs/concepts/contexts) in Yew.

In this example, `<App />` is a function component containing your app.

```rs
#[function_component(Root)]
pub fn root() -> Html {
    let initial_theme = MyThemeChoice::Light;
    html! {
        <ThemeProvider<MyThemeChoice> theme={ initial_theme } >
            <App />
        </ThemeProvider<MyThemeChoice>>
    }
}
```
*You can match your browser's theme preference (Light/Dark) to help decide on an initial theme, via [`themer::BrowserPreference::get()`](#TODO)*

### Step 3: Use your theme
`use_theme()` will dereference into your theme. This works as a [Hook](https://yew.rs/docs/concepts/function-components/pre-defined-hooks) in Yew.

```rs
#[function_component(App)]
pub fn app() -> Html {
    let theme = use_theme::<MyThemeChoice>();
    let style = format!("color:{}; background-color:{};", theme.fg, theme.bg);

    html! {
        <label {style}>
            { "I am a themed label!" }
        </label>
    }
}
```