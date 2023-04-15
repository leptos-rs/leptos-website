use leptos::*;

#[component]
pub fn GettingStartedSquare(
    cx: Scope,
    tag: String,
    title: String,
    body: String,
    link: String,
    background_class: String,
) -> impl IntoView {
    view! { cx,
        <div class=format!("h-52 w-64 bg-black rounded p-4 flex flex-col gap-4 text-eggshell {}", background_class)>
            <span class="text-xs font-thin">{tag}</span>
            <h1 class="text-2xl font-bold">{title}</h1>
            <p>{body}</p>
        </div>
    }
}
