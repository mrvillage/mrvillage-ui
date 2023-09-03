use leptos::{component, view, IntoAttribute, IntoView, MaybeSignal, Signal};

pub trait NumMarker {}

impl NumMarker for i8 {}
impl NumMarker for i16 {}
impl NumMarker for i32 {}
impl NumMarker for i64 {}
impl NumMarker for i128 {}
impl NumMarker for isize {}
impl NumMarker for u8 {}
impl NumMarker for u16 {}
impl NumMarker for u32 {}
impl NumMarker for u64 {}
impl NumMarker for u128 {}
impl NumMarker for usize {}
impl NumMarker for f32 {}
impl NumMarker for f64 {}

#[component]
pub fn NumberInput(
    #[prop(into, optional)] value: MaybeSignal<i128>,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView
// where
    // Signal<T>: Into<MaybeSignal<T>> + SignalGet<T>,
    // T: NumMarker + 'static + Default,
    // S: Into<String> + 'static + std::fmt::Display + std::default::Default + Clone,
{
    let class = move || format!("mu-input {}", class());
    view! { cx,
        <input type="text" inputmode="numeric" class=class prop:value=value />
    }
}

#[component]
pub fn Test() -> impl IntoView {
    let value: i128 = 0;
    let signal = Signal::derive(move || value);
    view! {
        <NumberInput value=signal class="" />
    }
}
