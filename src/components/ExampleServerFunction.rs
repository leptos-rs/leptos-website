use leptos::*;

pub const EXAMPLE_SERVER_FUNCTION_CODE: &str = r#"```rust
#[server(SaveFavorites, "/api")]
pub async fn save_favorites(
    cx: Scope,
    favorite_cookie_type: String,
    favorite_color: String,
) -> Result<(), ServerFnError> {
    let pool = get_pool(cx)?;

    let query = "
        INSERT INTO USERS 
        (favorite_cookie_type, favorite_color)
        VALUES ($1, $2)
    ";

    sqlx::query(query)
        .bind(favorite_cookie_type)
        .bind(favorite_color)
        .execute(&pool)
        .await
        .map_err(|e| 
            ServerFnError::ServerError(e.to_string())?;

    Ok(())
}
```"#;

#[component]
pub fn ExampleServerFunction(cx: Scope) -> impl IntoView {
    let (favorite_cookie_type, set_favorite_cookie_type) = create_signal(cx, String::new());
    let (favorite_color, set_favorite_color) = create_signal(cx, String::new());

    let (loading, set_loading) = create_signal(cx, false);

    #[derive(Clone)]
    enum ServerResult {
        NoResult,
        Success,
        Error(String),
    }

    let (result_of_save, set_result_of_save) = create_signal(cx, ServerResult::NoResult);

    view! { cx,
        <div class="p-4 sm:p-8">
            <h2 class="text-2xl font-bold text-black dark:text-eggshell">"Save to database"</h2>
            <div class="my-4">
                <div class="flex flex-col gap-4">
                    <div>
                        <label
                            htmlfor="favorite_cookie_type"
                            class="block text-sm font-bold text-black dark:text-eggshell "
                        >
                            "Favorite type of cookie"
                        </label>
                        <div class="mt-1">
                            <input
                                type="text"
                                on:input=move |ev| { set_favorite_cookie_type(event_target_value(&ev)) }
                                favorite_cookie_type="favorite_cookie_type"
                                id="favorite_cookie_type"
                                class="block w-full p-2 max-w-[250px] rounded-md border border-black text-black"
                            />
                        </div>
                    </div>
                    <div>
                        <label
                            htmlfor="favorite_color"
                            class="block text-sm font-bold text-black dark:text-eggshell"
                        >
                            "Favorite color"
                        </label>
                        <div class="mt-1">
                            <input
                                id="favorite_color"
                                on:input=move |ev| { set_favorite_color(event_target_value(&ev)) }
                                favorite_cookie_type="favorite_color"
                                type="favorite_color"
                                class="block w-full p-2 max-w-[250px] rounded-md border border-black text-black"
                            />
                        </div>
                    </div>
                    <button
                        on:click=move |_| {
                            set_loading(true);
                            set_loading(false);
                            if !favorite_cookie_type().is_empty() && !favorite_color().is_empty() {
                                set_result_of_save.update(|sr: &mut ServerResult| *sr = ServerResult::Success)
                            }
                            if favorite_cookie_type().is_empty() || favorite_color().is_empty() {
                                set_result_of_save
                                    .update(|sr: &mut ServerResult| {
                                        *sr = ServerResult::Error("Please fill out all fields".to_string());
                                    })
                            }
                        }
                        class="block max-w-fit mt-1 text-lg py-2 px-4 text-purple dark:text-eggshell rounded-md border border-purple dark:border-eggshell"
                    >
                        "Submit"
                    </button>
                    {move || {
                        if loading() {
                            view! { cx,
                                <>
                                    <div class="text-black dark:text-eggshell">"Loading!"</div>
                                </>
                            }
                        } else {
                            view! { cx, <></> }
                        }
                    }}
                    {move || match result_of_save() {
                        ServerResult::NoResult => {
                            view! { cx, <></> }
                        }
                        ServerResult::Success => {
                            view! { cx,
                                <>
                                    <div class="text-black dark:text-eggshell">"Success!"</div>
                                </>
                            }
                        }
                        ServerResult::Error(e) => {
                            view! { cx,
                                <>
                                    <div class="text-black dark:text-eggshell">{e}</div>
                                </>
                            }
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
