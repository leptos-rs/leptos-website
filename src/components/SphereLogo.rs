use leptos::*;

#[component]
pub fn SphereLogo(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="stage  ">
            <div class="ball ">
                <div class="logo aspect-square"></div>
            </div>
        </div>
    }
}
