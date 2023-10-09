use leptos::*;

#[component]
pub fn TextInput(
    #[prop(into, optional)] value: MaybeSignal<String>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = move || format!("mu-input {}", class());
    view! {
        <input {..attrs} type="text" class=class value=value.get_untracked() prop:value=value />
    }
}
