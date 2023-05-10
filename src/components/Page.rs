use crate::components::Footer::*;
// use crate::components::Header::*;
use leptos::*;

#[component]
pub fn Page(cx: Scope, children: Children) -> impl IntoView {
    view! { cx, <div class="overflow-x-hidden bg-white dark:bg-black">{children(cx)} <Footer/></div> }
}
