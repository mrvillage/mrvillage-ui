use leptos::{component, view, IntoAttribute, IntoView, MaybeSignal};

#[component]
pub fn TextInput(
    #[prop(into, optional)] value: MaybeSignal<String>,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView {
    let class = move || format!("mu-input {}", class());
    view! {
        <input type="text" class=class value=value() prop:value=value />
    }
}
