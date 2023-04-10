use crate::{app::DarkModeContext, components::Footer::*};
// use crate::components::Header::*;
use leptos::*;

#[component]
pub fn Page(cx: Scope, children: Children) -> impl IntoView {
    let dark_mode = use_context::<DarkModeContext>(cx).unwrap().dark_mode;

    view! { cx,
        <div class=move || match dark_mode() {
            true => "bg-black dark overflow-x-hidden",
            false => "bg-white light overflow-x-hidden",
        }>{children(cx)} <Footer/></div>
    }
}
