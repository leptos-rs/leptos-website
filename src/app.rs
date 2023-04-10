use crate::pages::About::*;
use crate::pages::Home::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Clone)]
pub struct DarkModeContext {
    pub dark_mode: ReadSignal<bool>,
    pub set_dark_mode: WriteSignal<bool>,
}

pub fn provide_dark_mode_context(cx: Scope) {
    let (dark_mode, set_dark_mode) = create_signal(cx, false);

    provide_context(
        cx,
        DarkModeContext {
            dark_mode: dark_mode.into(),
            set_dark_mode: set_dark_mode.into(),
        },
    );
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let formatter = |text| format!("{text} - Leptos");
    provide_meta_context(cx);

    provide_dark_mode_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/leptos-website.css"/>
        <Title formatter/>
        <Meta name="description" content="Leptos is a cutting-edge Rust web framework designed for building fast, reliable, web applications."/>
        <Router>
            <Routes>
                <Route
                    path=""
                    view=|cx| {
                        view! { cx, <Home/> }
                    }
                />
                <Route
                    path="/about"
                    view=|cx| {
                        view! { cx, <About/> }
                    }
                />
            </Routes>
        </Router>
    }
}
