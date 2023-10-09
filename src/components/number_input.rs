use std::str::FromStr;

use leptos::*;

#[component]
pub fn NumberInput<T>(
    #[prop(into, optional)] value: MaybeSignal<T>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView
where
    T: FromStr + 'static + Default + Clone + IntoAttribute,
    wasm_bindgen::JsValue: From<T>,
{
    let class = move || format!("mu-input {}", class());
    view! {
        <input {..attrs} type="text" inputmode="numeric" class=class value=value.get_untracked() prop:value=value />
    }
}
