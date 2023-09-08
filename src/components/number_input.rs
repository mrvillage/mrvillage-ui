use std::str::FromStr;

use leptos::{component, view, IntoAttribute, IntoView, MaybeSignal, Signal, SignalGet};

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
pub fn NumberInput<T>(
    #[prop(into, optional)] value: MaybeSignal<T>,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView
where
    Signal<T>: Into<MaybeSignal<T>> + SignalGet<Value = T>,
    T: FromStr + 'static + Default,
{
    let class = move || format!("mu-input {}", class());
    view! {
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
