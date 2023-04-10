use leptos::*;
use leptos_router::*;

pub enum AlertType {
    Info,
    Success,
    Warning,
    Error,
}

pub const EXAMPLE_COMPONENT_CODE: &str = r#"```tsx
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
            <Button/>
            <div class="pink-bg">
                <Button/>
            </div>
    }
}

#[component]
pub fn Button(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {move || count()}
        </button>
    }
}
```"#;

#[component]
pub fn Button(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <button
            class="border border-black px-4 py-2 max-w-fit  block rounded-md bg-white"
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {move || count()}
        </button>
    }
}

pub struct VideoData {
    pub url: String,
    pub title: String,
    pub description: String,
}

#[component]
pub fn ComponentExample(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="w-full flex flex-col gap-2 m-4 max-w-fit">
            <Button/>
            <div class=" bg-pink  p-4 rounded-md max-w-fit ">
                <Button/>
            </div>
        </div>
    }
}
