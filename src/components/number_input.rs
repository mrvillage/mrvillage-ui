use leptos::{component, view, IntoAttribute, IntoView, MaybeSignal, Scope};

#[component]
pub fn NumberInput<T>(
    cx: Scope,
    #[prop(into, optional)] value: MaybeSignal<T>,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView
where
    T: IntoAttribute + Default + Clone + 'static,
{
    let class = move || format!("mu-input {}", class());
    view! { cx,
        <input type="text" class=class prop:value=value value=value />
    }
}
