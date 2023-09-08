use leptos::{component, view, IntoAttribute, IntoView, MaybeSignal, SignalGetUntracked};

#[component]
pub fn TextInput(
    #[prop(into, optional)] value: MaybeSignal<String>,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView {
    let class = move || format!("mu-input {}", class());
    view! {
        <input type="text" class=class value=value.get_untracked() prop:value=value />
    }
}
