use crate::pages::Home::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path, SsrMode, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta
                    name="description"
                    content="Leptos is a cutting-edge Rust web framework designed for building fast, reliable, web applications."
                />
                <link rel="stylesheet" id="leptos" href="/pkg/leptos_website.css"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let formatter = |text| format!("{text} - Leptos");
    provide_meta_context();

    view! {
        <Title formatter/>
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("") view=Home ssr=SsrMode::Async/>
            </Routes>
        </Router>
        <script defer data-domain="leptos.dev" src="https://plausible.io/js/script.js"></script>
    }
}
