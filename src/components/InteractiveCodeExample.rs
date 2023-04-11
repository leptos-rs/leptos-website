use std::time::Duration;

use super::CodeExample::{CodeExampleLayout, CodeExampleMode};
use leptos::{*, leptos_dom::helpers::TimeoutHandle};

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
            <ExampleComponent set_is_active/>
        </CodeExampleLayout>
    }
}

#[component]
fn CodeView(cx: Scope, is_active: ReadSignal<bool>) -> impl IntoView {
	let callback_class = move || if is_active() {
		"bg-red"
	} else {
		""
	};
	let setter_class = move || if is_active() {
		"bg-orange delay-100"
	} else {
		""
	};
	let getter_class = move || if is_active() {
		"bg-yellow delay-200"
	} else {
		""
	};

    view! { cx,
        <pre class="code-block-inner" data-lang="tsx">
            "#"
            <i class="hh8">"["</i>
            <i class="hh15">"component"</i>
            <i class="hh8">"]"</i>"\n"
            <i class="hh15">"pub"</i>" "
            <i class="hh15">"fn"</i>" "
            <i class="hh13">"Button"</i>
            <i class="hh8">"("</i>
            <i class="hh15">"cx"</i>
            ": "
            <i class="hh13">"Scope"</i>
            <i class="hh8">")"</i>" "
            <i class="hh5">"-"</i>
            <i class="hh5">">"</i>" "
            <i class="hh15">"impl"</i>" "
            <i class="hh13">"IntoView"</i>" "
            <i class="hh8">"{{"</i>"\n  "
            <i class="hh6">"let"</i>" "
            <i class="hh8">"("</i>
            <span class=getter_class><i class="hh17">"count"</i></span>
            <i class="hh9">","</i>" "
            <span class=setter_class><i class="hh17">"set_count"</i></span>
            <i class="hh8">")"</i>" "
            <i class="hh5">"="</i>" "
            <i class="hh6">"create_signal"</i>
            <i class="hh8">"("</i>
            <i class="hh17">"cx"</i>
            <i class="hh9">","</i>" "
            "0"
            <i class="hh8">")"</i>
            <i class="hh9">";"</i>"\n  "
            <i class="hh15">"view"</i>
            <i class="hh5">"!"</i>" "
            <i class="hh8">"{{"</i>" "
            <i class="hh17">"cx"</i>
            <i class="hh9">","</i>" \n    "
            <i class="hh5">"<"</i>
            <i class="hh12">"button"</i>" "
            <i class="hh15">"on"</i>
            ":"
            <span class=callback_class><i class="hh15">"click"</i></span>
            <i class="hh5">"="</i>
            <i class="hh15">"move"</i>" "
            <i class="hh5">"|"</i>
            <i class="hh15">"_"</i>
            <i class="hh5">"|"</i>" "
            <i class="hh8">"{{"</i>" \n        "
            <i class="hh17">"set_count"</i>
            <i class="hh9">"."</i>
            <span class=setter_class><i class="hh3">"update"</i></span>
            <i class="hh8">"("</i>
            <i class="hh5">"|"</i>
            <i class="hh15">"n"</i>
            <i class="hh5">"|"</i>" "
            <i class="hh5">"*"</i>
            <i class="hh15">"n"</i>" "
            <i class="hh5">"+="</i>" "
            "1"
            <i class="hh8">")"</i>
            <i class="hh9">";"</i>"\n      "
            <i class="hh8">"}}"</i>"\n    "
            <i class="hh5">">"</i>"\n      "
            "\"Click me: \"""\n      "
            <i class="hh8">"{{"</i>
            <span class=getter_class><i class="hh17">"count"</i></span>
            <i class="hh8">"}}"</i>"\n    "
            <i class="hh5">"<"</i>
            <i class="hh5">"/"</i>
            <i class="hh12">"button"</i>
            <i class="hh5">">"</i>"\n  "
            <i class="hh8">"}}"</i>"\n"
            <i class="hh8">"}}"</i>
        </pre>
    }
}

#[component]
fn ExampleComponent(cx: Scope, set_is_active: WriteSignal<bool>) -> impl IntoView {
	let (count, set_count) = create_signal(cx, 0);
	let timeout_handle = store_value(cx, None::<TimeoutHandle>);

    view! { cx,
        <div class="px-2 py-6 h-full w-full flex flex-col justify-center items-center ">
            <button
				class="text-lg py-2 px-4 text-purple dark:text-eggshell rounded-md border border-purple dark:border-eggshell"
				on:click=move |_| {
					set_count.update(|n| *n += 1);
					set_is_active(true);

					if let Some(handle) = timeout_handle.get_value() {
						handle.clear();
					}
					timeout_handle.set_value(
						set_timeout_with_handle(
							move || {
								set_is_active(false);
							},
							Duration::from_millis(500),
						)
						.ok(),
					);
				}
			>
				"Click me: "
				{move || count()}
			</button>
        </div>
    }
}
