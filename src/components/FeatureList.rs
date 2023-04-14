use leptos::*;

#[component]
pub fn FeatureListItem(cx: Scope, text: String) -> impl IntoView {
    view! { cx,
        <div class="flex items-center pb-3">
            <div class="w-14 h-14">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" stroke="currentColor" stroke-width="1" class="w-14 h-14 stroke-purple dark:stroke-eggshell" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0z"/></svg>
            </div>
            <p class="ml-2 text-purple dark:text-eggshell">{text}</p>
        </div>
    }
}

#[component]
pub fn FeatureList(cx: Scope, items: Vec<String>) -> impl IntoView {
    let feature_list_items: Vec<_> = items
        .iter()
        .map(|item_text| {
            FeatureListItem(
                cx,
                FeatureListItemProps {
                    text: item_text.clone(),
                },
            )
        })
        .collect();

    view! { cx,
        <div class="flex flex-col">
            {feature_list_items}
        </div>
    }
}
