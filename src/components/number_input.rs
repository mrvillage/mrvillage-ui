use std::str::FromStr;

use leptos::{component, view, IntoAttribute, IntoView, MaybeSignal};

#[component]
pub fn NumberInput<T>(
    #[prop(into, optional)] value: MaybeSignal<T>,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView
where
    T: FromStr + 'static + Default + Clone + IntoAttribute,
    wasm_bindgen::JsValue: From<T>,
{
    let class = move || format!("mu-input {}", class());
    view! {
        <input type="text" inputmode="numeric" class=class value=value() prop:value=value />
    }
}
