use leptos::{component, view, IntoAttribute, IntoView, MaybeSignal, Scope, Signal};

#[component]
pub fn TextInput<V>(
    cx: Scope,
    #[prop(into, optional)] value: MaybeSignal<V>,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView
where
    V: Into<String> + 'static + Default + Clone,
{
    let value = match value {
        MaybeSignal::Static(value) => MaybeSignal::Static(value.into()),
        MaybeSignal::Dynamic(value) => {
            MaybeSignal::Dynamic(Signal::derive(cx, move || value().into()))
        },
    };
    let class = move || format!("mu-input {}", class(),);
    view! { cx,
        <input type="text" class=class prop:value=value value=value />
    }
}
