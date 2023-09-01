use leptos::{component, view, IntoAttribute, IntoProperty, IntoView, MaybeSignal, Scope};

#[component]
pub fn NumberInput<T>(
    cx: Scope,
    #[prop(into, default = MaybeSignal::Static((0 as usize).into()))] value: MaybeSignal<T>,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView
where
    T: IntoAttribute + Clone + From<usize> + 'static,
    (leptos::Scope, MaybeSignal<T>): IntoProperty,
{
    let class = move || format!("mu-input {}", class());
    view! { cx,
        <input type="text" inputmode="numeric" class=class prop:value=value />
    }
}
