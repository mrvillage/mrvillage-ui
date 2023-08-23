use leptos::{component, Scope, MaybeSignal, IntoView, view, ev::MouseEvent, tracing};

#[component]
pub fn Button<OC>(cx: Scope, text: MaybeSignal<String>, on_click: OC) -> impl IntoView 
where OC: Fn(MouseEvent) + 'static {
    view! { cx,
        <button on:click=on_click>
            {text}
        </button>
    }
}
