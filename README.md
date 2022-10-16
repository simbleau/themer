# Themer
A non-complicated theme management library for Yew.

Note: Themer is **NOT** a CSS solution. You may use any CSS styling solution you wish (e.g. [Stylist](https://crates.io/crates/stylist)).

# Status
**Ongoing development. No/little documentation, rapidly changing, etc.**

# Examples
Examples can be served with [Trunk](https://trunkrs.dev/) (e.g. `trunk serve`)

**TODO: This is in progress**
- [Browser Preference](examples/browser_preferences/): Example of showing your browser's theme preference (Light/Dark)
- [Minimal](examples/minimal/): Example of applying the browser's preferred theme
- [Theme Switcher](examples/theme_switcher/): Example of switching between themes
- **TODO** *Storage: Example of storing and retrieving changeable theme preferences via local storage*

## API Foreward
### Step 1: Setup a Theme and Key
The API asks you to create a Theme template struct with the `#[theme]` macro, and a key for access with the `#[theme_key]` macro. The theme key is used to reduce memory footprint and provides ergonomics.
```rs
#[theme]
pub struct MyTheme {
    fg: &'static str,
    bg: &'static str,
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
            },
            MyThemeChoice::Dark => &MyTheme {
                fg: "white",
                bg: "black",
            },
        }
    }
}
```

### Step 2: Provide your theme
To pass your theme to your app, you must fist register a `ThemeProvider` with an initial theme choice. This follows the same pattern as [Contexts](https://yew.rs/docs/concepts/contexts) in Yew.

In this example, we embed `<App />` is a function component containing your app.

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
*Hint: You can query your browser's theme preference (Light/Dark) to help decide on an initial theme, via [`themer::BrowserPreference::get()`](#)*

### Step 3: Use your theme
Use your theme with `use_theme()` (which dereferences into your theme). This follows the same pattern as [Hooks](https://yew.rs/docs/concepts/function-components/pre-defined-hooks) in Yew.

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