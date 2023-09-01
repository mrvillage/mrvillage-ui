use leptos::{component, view, IntoAttribute, IntoView, MaybeSignal, Scope};

#[component]
pub fn TextInput(
    cx: Scope,
    #[prop(into, optional)] value: MaybeSignal<String>,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView {
    let class = move || format!("mu-input {}", class());
    view! { cx,
        <input type="text" class=class prop:value=value />
    }
}
