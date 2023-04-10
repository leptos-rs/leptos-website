use leptos::*;

pub enum AlertType {
    Info,
    Success,
    Warning,
    Error,
}

pub const EXAMPLE_COMPONENT_CODE: &str = r#"```tsx
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
            class="text-lg py-2 px-4 text-purple dark:text-eggshell rounded-md border border-purple dark:border-eggshell"
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
pub fn ExampleComponent(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="px-2 py-6 h-full w-full flex flex-col justify-center items-center ">
            <Button/>
        </div>
    }
}
