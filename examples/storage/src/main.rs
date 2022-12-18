use example_storage::my_theme_spec::MyThemeChoice;
use stylist::yew::styled_component;
use themer::prelude::*;
use web_sys::HtmlLabelElement;
use yew::prelude::*;

#[function_component(Root)]
pub fn root() -> Html {
    let stored_theme = match BrowserPreference::load::<MyThemeChoice>() {
        Some(pref) => pref,
        None => MyThemeChoice::default(),
    };
    html! {
        <ThemeProvider<MyThemeChoice> theme={stored_theme} >
            <App />
        </ThemeProvider<MyThemeChoice>>
    }
}

#[styled_component(App)]
fn app() -> Html {
    let theme = use_theme::<MyThemeChoice>();
    let style = css! {
        & > * {
            color: ${theme.fg};
            background-color: ${theme.bg};
        }
    };

    // References for labels
    let saved_ref = use_node_ref();
    let current_ref = use_node_ref();

    // Button callbacks
    let switch = {
        let theme = theme.clone();
        let (saved_ref, current_ref) = (saved_ref.clone(), current_ref.clone());
        Callback::from(move |_| {
            let next = match theme.kind() {
                MyThemeChoice::Light => MyThemeChoice::Dark,
                MyThemeChoice::Dark => MyThemeChoice::Light,
            };
            theme.set(next);
            update_labels(next, &saved_ref, &current_ref);
        })
    };
    let save = {
        let theme = theme.clone();
        let (saved_ref, current_ref) = (saved_ref.clone(), current_ref.clone());
        Callback::from(move |_| {
            BrowserPreference::save(theme.kind()).unwrap();
            update_labels(theme.kind(), &saved_ref, &current_ref);
        })
    };
    let clear = {
        let theme = theme.clone();
        let (saved_ref, current_ref) = (saved_ref.clone(), current_ref.clone());
        Callback::from(move |_| {
            BrowserPreference::clear();
            update_labels(theme.kind(), &saved_ref, &current_ref);
        })
    };

    // Effect to update labels
    use_effect({
        let theme = theme.clone();
        let (saved_ref, current_ref) = (saved_ref.clone(), current_ref.clone());
        move || {
            update_labels(theme.kind(), &saved_ref, &current_ref);
            move || {}
        }
    });

    html! {
        <div class={style}>
            <button onclick={ save }>{"Save"}</button>
            <button onclick={ clear }>{"Clear"}</button>
            <label ref={saved_ref} />
            <br />
            <button onclick={ switch }>{"Switch themes"}</button>
            <label ref={current_ref} />
        </div>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}

// Helper method to update the labels
fn update_labels(
    theme: MyThemeChoice,
    saved_ref: &NodeRef,
    current_ref: &NodeRef,
) {
    // Label texts
    let saved = format!(
        "Storage: {}",
        match BrowserPreference::load::<MyThemeChoice>() {
            Some(pref) => format!("{:?}", pref),
            None => "No saved preference".into(),
        }
    );
    let current = format!(
        "Current theme: {}",
        match theme {
            MyThemeChoice::Light => "Light",
            MyThemeChoice::Dark => "Dark",
        }
    );

    let saved_label = saved_ref.cast::<HtmlLabelElement>().unwrap();
    saved_label.set_inner_text(&saved);
    let current_label = current_ref.cast::<HtmlLabelElement>().unwrap();
    current_label.set_inner_text(&current);
}
