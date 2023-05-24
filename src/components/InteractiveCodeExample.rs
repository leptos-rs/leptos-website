use std::time::Duration;

use super::CodeExample::{CodeExampleLayout, CodeExampleMode};
use leptos::{leptos_dom::helpers::TimeoutHandle, *};
use leptos_router::use_query_map;

#[component]
pub fn InteractiveCodeExample(
    cx: Scope,
    shadow: bool,
    border: bool,
    background: String,
) -> impl IntoView {
    let (is_active, set_is_active) = create_signal(cx, false);

    view! { cx,
        <CodeExampleLayout
            shadow
            border
            background
            code=CodeExampleMode::View(
                view! { cx, <CodeView is_active/> }
                    .into_view(cx),
            )
        >
            <ExampleComponent is_active set_is_active/>
        </CodeExampleLayout>
    }
}

#[component]
fn CodeView(cx: Scope, is_active: ReadSignal<bool>) -> impl IntoView {
    let callback_class = move || {
        if is_active() {
            "code-example-ping"
        } else {
            ""
        }
    };
    let setter_class = move || {
        if is_active() {
            "code-example-ping-1"
        } else {
            ""
        }
    };
    let getter_class = move || {
        if is_active() {
            "code-example-ping-2"
        } else {
            ""
        }
    };

    view! { cx,
        <pre class="code-block-inner" data-lang="tsx">
            "#"
            <i class="hh8">"["</i>
            <i class="hh15">"component"</i>
            <i class="hh8">"]"</i>
            "\n"
            <i class="hh15">"pub"</i>
            " "
            <i class="hh15">"fn"</i>
            " "
            <i class="hh13">"Button"</i>
            <i class="hh8">"("</i>
            <i class="hh15">"cx"</i>
            ": "
            <i class="hh13">"Scope"</i>
            <i class="hh8">")"</i>
            " "
            <i class="hh5">"-"</i>
            <i class="hh5">">"</i>
            " "
            <i class="hh15">"impl"</i>
            " "
            <i class="hh13">"IntoView"</i>
            " "
            <i class="hh8">"{"</i>
            "\n  "
            <i class="hh6">"let"</i>
            " "
            <i class="hh8">"("</i>
            <span class=getter_class>
                <i class="hh17">"count"</i>
            </span>
            <i class="hh9">","</i>
            " "
            <span class=setter_class>
                <i class="hh17">"set_count"</i>
            </span>
            <i class="hh8">")"</i>
            " "
            <i class="hh5">"="</i>
            " "
            <i class="hh6">"create_signal"</i>
            <i class="hh8">"("</i>
            <i class="hh17">"cx"</i>
            <i class="hh9">","</i>
            " "
            "0"
            <i class="hh8">")"</i>
            <i class="hh9">";"</i>
            "\n  "
            <i class="hh15">"view"</i>
            <i class="hh5">"!"</i>
            " "
            <i class="hh8">"{"</i>
            " "
            <i class="hh17">"cx"</i>
            <i class="hh9">","</i>
            " \n    "
            <i class="hh5">"<"</i>
            <i class="hh12">"button"</i>
            " "
            <i class="hh15">"on"</i>
            ":"
            <span class=callback_class>
                <i class="hh15">"click"</i>
            </span>
            <i class="hh5">"="</i>
            <i class="hh15">"move"</i>
            " "
            <i class="hh5">"|"</i>
            <i class="hh15">"_"</i>
            <i class="hh5">"|"</i>
            " "
            <i class="hh8">"{"</i>
            " \n        "
            <i class="hh17">"set_count"</i>
            <i class="hh9">"."</i>
            <span class=setter_class>
                <i class="hh3">"update"</i>
            </span>
            <i class="hh8">"("</i>
            <i class="hh5">"|"</i>
            <i class="hh15">"n"</i>
            <i class="hh5">"|"</i>
            " "
            <i class="hh5">"*"</i>
            <i class="hh15">"n"</i>
            " "
            <i class="hh5">"+="</i>
            " "
            "1"
            <i class="hh8">")"</i>
            <i class="hh9">";"</i>
            "\n      "
            <i class="hh8">"}"</i>
            "\n    "
            <i class="hh5">">"</i>
            "\n      "
            "\"Click me: \""
            "\n      "
            <i class="hh8">"{"</i>
            <span class=getter_class>
                <i class="hh17">"count"</i>
            </span>
            <i class="hh8">"}"</i>
            "\n    "
            <i class="hh5">"<"</i>
            <i class="hh5">"/"</i>
            <i class="hh12">"button"</i>
            <i class="hh5">">"</i>
            "\n  "
            <i class="hh8">"}"</i>
            "\n"
            <i class="hh8">"}"</i>
        </pre>
    }
}

#[component]
fn ExampleComponent(cx: Scope, is_active: ReadSignal<bool>, set_is_active: WriteSignal<bool>) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let timeout_handle = store_value(cx, None::<TimeoutHandle>);
    let query = use_query_map(cx);
    let interactive = move || query().get("interactive").map(|s| s.as_str()) == Some("tell");

    view! { cx,
        <div class="px-2 py-6 h-full w-full flex flex-col justify-center items-center ">
            <div class="flex justify-around w-full mb-8">
                <a
                    class=move || {
                        if !interactive() {
                            "border-b-2 dark:text-white dark:border-white"
                        } else {
                            "dark:text-white dark:border-white"
                        }
                    }
                    href="?interactive=show"
                >
                    "Counter Button"
                </a>
                <a
                    class=move || {
                        if interactive() {
                            "border-b-2 dark:text-white dark:border-white"
                        } else {
                            "dark:text-white dark:border-white"
                        }
                    }
                    href="?interactive=tell"
                >
                    "How does it work?"
                </a>
            </div>
            <button
                class="text-lg py-2 px-4 text-purple dark:text-eggshell rounded-md \
                border border-purple dark:border-eggshell disabled:opacity-50"
                disabled=is_active
                on:click=move |_| {
                    set_count.update(|n| *n += 1);
                    if interactive() {
                        set_is_active(true);
                        if let Some(handle) = timeout_handle.get_value() {
                            handle.clear();
                        }
                        timeout_handle
                            .set_value(
                                set_timeout_with_handle(
                                        move || {
                                            set_is_active(false);
                                        },
                                        Duration::from_millis(750),
                                    )
                                    .ok(),
                            );
                    }
                }
            >
                "Click me: "
                {count}
            </button>
            {move || {
                interactive()
                    .then(|| {
                        view! { cx,
                            <ol class="text-sm w-3/4 mt-8 dark:text-white list-decimal">
                                <li>
                                    "The component function runs once, creating the DOM elements and setting up the reactive system."
                                </li>
                                <li>"Clicking the button fires the " <code>"on:click"</code> " handler. "</li>
                                <li>
                                    "The click handler calls " <code>"set_count.update"</code>
                                    " to increment the count."
                                </li>
                                <li>
                                    "The change to the " <code>"count"</code>
                                    " signal makes a targeted update to the DOM, changing the value of a single text node without re-rendering anything else."
                                </li>
                            </ol>
                        }
                    })
            }}
        </div>
    }
}
