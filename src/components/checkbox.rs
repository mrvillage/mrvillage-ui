use std::fmt::Display;

use leptos::{component, view, IntoAttribute, IntoView, MaybeSignal, Scope};
use serde::{Deserialize, Serialize};

#[component]
pub fn Checkbox(
    cx: Scope,
    #[prop(into, optional)] value: MaybeSignal<bool>,
    #[prop(into)] color: MaybeSignal<CheckboxColor>,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView {
    let class = move || format!("mu-checkbox {} {}", color(), class(),);
    view! { cx,
        <input type="checkbox" class=class prop:value=value value=value />
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum CheckboxColor {
    Black,
    White,
    Stone,
    Red,
    Orange,
    Amber,
    Yellow,
    Lime,
    Green,
    Emerald,
    Teal,
    Cyan,
    Sky,
    Blue,
    Indigo,
    Violet,
    Purple,
    Fuchsia,
    Pink,
    Rose,
}

impl Display for CheckboxColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CheckboxColor::*;

        match self {
            Black => {
                write!(
                    f,
                    "checked:border-black checked:bg-black dark:checked:border-black \
                     dark:checked:bg-black"
                )
            },
            White => {
                write!(
                    f,
                    "checked:border-white checked:bg-white dark:checked:border-white \
                     dark:checked:bg-white"
                )
            },
            Stone => {
                write!(
                    f,
                    "checked:border-stone-700 checked:bg-stone-700 dark:checked:border-stone-700 \
                     dark:checked:bg-stone-700"
                )
            },
            Red => {
                write!(
                    f,
                    "checked:border-red-700 checked:bg-red-700 dark:checked:border-red-700 \
                     dark:checked:bg-red-700"
                )
            },
            Orange => {
                write!(
                    f,
                    "checked:border-orange-700 checked:bg-orange-700 \
                     dark:checked:border-orange-700 dark:checked:bg-orange-700"
                )
            },
            Amber => {
                write!(
                    f,
                    "checked:border-amber-700 checked:bg-amber-700 dark:checked:border-amber-700 \
                     dark:checked:bg-amber-700"
                )
            },
            Yellow => {
                write!(
                    f,
                    "checked:border-yellow-700 checked:bg-yellow-700 \
                     dark:checked:border-yellow-700 dark:checked:bg-yellow-700"
                )
            },
            Lime => {
                write!(
                    f,
                    "checked:border-lime-700 checked:bg-lime-700 dark:checked:border-lime-700 \
                     dark:checked:bg-lime-700"
                )
            },
            Green => {
                write!(
                    f,
                    "checked:border-green-700 checked:bg-green-700 dark:checked:border-green-700 \
                     dark:checked:bg-green-700"
                )
            },
            Emerald => {
                write!(
                    f,
                    "checked:border-emerald-700 checked:bg-emerald-700 \
                     dark:checked:border-emerald-700 dark:checked:bg-emerald-700"
                )
            },
            Teal => {
                write!(
                    f,
                    "checked:border-teal-700 checked:bg-teal-700 dark:checked:border-teal-700 \
                     dark:checked:bg-teal-700"
                )
            },
            Cyan => {
                write!(
                    f,
                    "checked:border-cyan-700 checked:bg-cyan-700 dark:checked:border-cyan-700 \
                     dark:checked:bg-cyan-700"
                )
            },
            Sky => {
                write!(
                    f,
                    "checked:border-sky-700 checked:bg-sky-700 dark:checked:border-sky-700 \
                     dark:checked:bg-sky-700"
                )
            },
            Blue => {
                write!(
                    f,
                    "checked:border-blue-700 checked:bg-blue-700 dark:checked:border-blue-700 \
                     dark:checked:bg-blue-700"
                )
            },
            Indigo => {
                write!(
                    f,
                    "checked:border-indigo-700 checked:bg-indigo-700 \
                     dark:checked:border-indigo-700 dark:checked:bg-indigo-700"
                )
            },
            Violet => {
                write!(
                    f,
                    "checked:border-violet-700 checked:bg-violet-700 \
                     dark:checked:border-violet-700 dark:checked:bg-violet-700"
                )
            },
            Purple => {
                write!(
                    f,
                    "checked:border-purple-700 checked:bg-purple-700 \
                     dark:checked:border-purple-700 dark:checked:bg-purple-700"
                )
            },
            Fuchsia => {
                write!(
                    f,
                    "checked:border-fuchsia-700 checked:bg-fuchsia-700 \
                     dark:checked:border-fuchsia-700 dark:checked:bg-fuchsia-700"
                )
            },
            Pink => {
                write!(
                    f,
                    "checked:border-pink-700 checked:bg-pink-700 dark:checked:border-pink-700 \
                     dark:checked:bg-pink-700"
                )
            },
            Rose => {
                write!(
                    f,
                    "checked:border-rose-700 checked:bg-rose-700 dark:checked:border-rose-700 \
                     dark:checked:bg-rose-700"
                )
            },
        }
    }
}
