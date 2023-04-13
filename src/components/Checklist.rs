use leptos::*;

#[component]
pub fn ChecklistItem(cx: Scope, text: String) -> impl IntoView {
    view! { cx,
        <div class="flex items-center pb-3">
            <img src="/images/check.svg" class="w-14 h-14"/>
            <p class="ml-2">{text}</p>
        </div>
    }
}

#[component]
pub fn Checklist(cx: Scope, items: Vec<String>) -> impl IntoView {
    let checklist_items: Vec<_> = items
        .iter()
        .map(|item_text| {
            ChecklistItem(
                cx,
                ChecklistItemProps {
                    text: item_text.clone(),
                },
            )
        })
        .collect();

    view! { cx,
        <div class="flex flex-col">
            {checklist_items}
        </div>
    }
}
