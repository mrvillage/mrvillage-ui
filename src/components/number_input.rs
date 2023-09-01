use leptos::{component, view, IntoAttribute, IntoProperty, IntoView, MaybeSignal, Scope};

#[component]
pub fn NumberInput<T>(
    cx: Scope,
    #[prop(into, optional)] value: MaybeSignal<T>,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView
where
    T: IntoAttribute + Default + Clone + 'static,
    (leptos::Scope, MaybeSignal<T>): IntoProperty,
{
    let class = move || format!("mu-input {}", class());
    view! { cx,
        <input type="text" class=class prop:value=value />
    }
}
