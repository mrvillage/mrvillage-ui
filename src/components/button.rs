use std::fmt::Display;

use leptos::{component, view, Children, IntoAttribute, IntoView, MaybeSignal};
use serde::{Deserialize, Serialize};

#[component]
pub fn Button(
    children: Children,
    #[prop(into)] color: MaybeSignal<ButtonColor>,
    #[prop(into, optional)] size: MaybeSignal<ButtonSize>,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView {
    let class = move || format!("mu-button {} {} {}", color(), size(), class(),);
    view! {
        <button class=class>
            {children()}
        </button>
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ButtonColor {
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
    None,
}

impl Display for ButtonColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ButtonColor::*;

        match self {
            Black => {
                write!(
                    f,
                    "tw-bg-black hover:tw-bg-stone-800 active:tw-bg-stone-700"
                )
            },
            White => {
                write!(
                    f,
                    "tw-bg-white hover:tw-bg-stone-100 active:tw-bg-stone-200"
                )
            },
            Stone => {
                write!(
                    f,
                    "tw-bg-stone-700 hover:tw-bg-stone-800 active:tw-bg-stone-900"
                )
            },
            Red => write!(f, "tw-bg-red-700 hover:tw-bg-red-800 active:tw-bg-red-900"),
            Orange => {
                write!(
                    f,
                    "tw-bg-orange-700 hover:tw-bg-orange-800 active:tw-bg-orange-900"
                )
            },
            Amber => {
                write!(
                    f,
                    "tw-bg-amber-700 hover:tw-bg-amber-800 active:tw-bg-amber-900"
                )
            },
            Yellow => {
                write!(
                    f,
                    "tw-bg-yellow-700 hover:tw-bg-yellow-800 active:tw-bg-yellow-900"
                )
            },
            Lime => {
                write!(
                    f,
                    "tw-bg-lime-700 hover:tw-bg-lime-800 active:tw-bg-lime-900"
                )
            },
            Green => {
                write!(
                    f,
                    "tw-bg-green-700 hover:tw-bg-green-800 active:tw-bg-green-900"
                )
            },
            Emerald => {
                write!(
                    f,
                    "tw-bg-emerald-700 hover:tw-bg-emerald-800 active:tw-bg-emerald-900"
                )
            },
            Teal => {
                write!(
                    f,
                    "tw-bg-teal-700 hover:tw-bg-teal-800 active:tw-bg-teal-900"
                )
            },
            Cyan => {
                write!(
                    f,
                    "tw-bg-cyan-700 hover:tw-bg-cyan-800 active:tw-bg-cyan-900"
                )
            },
            Sky => write!(f, "tw-bg-sky-700 hover:tw-bg-sky-800 active:tw-bg-sky-900"),
            Blue => {
                write!(
                    f,
                    "tw-bg-blue-700 hover:tw-bg-blue-800 active:tw-bg-blue-900"
                )
            },
            Indigo => {
                write!(
                    f,
                    "tw-bg-indigo-700 hover:tw-bg-indigo-800 active:tw-bg-indigo-900"
                )
            },
            Violet => {
                write!(
                    f,
                    "tw-bg-violet-700 hover:tw-bg-violet-800 active:tw-bg-violet-900"
                )
            },
            Purple => {
                write!(
                    f,
                    "tw-bg-purple-700 hover:tw-bg-purple-800 active:tw-bg-purple-900"
                )
            },
            Fuchsia => {
                write!(
                    f,
                    "tw-bg-fuchsia-700 hover:tw-bg-fuchsia-800 active:tw-bg-fuchsia-900"
                )
            },
            Pink => {
                write!(
                    f,
                    "tw-bg-pink-700 hover:tw-bg-pink-800 active:tw-bg-pink-900"
                )
            },
            Rose => {
                write!(
                    f,
                    "tw-bg-rose-700 hover:tw-bg-rose-800 active:tw-bg-rose-900"
                )
            },
            None => write!(f, ""),
        }?;
        match self {
            White => write!(f, " tw-text-black"),
            None => write!(f, ""),
            _ => write!(f, " tw-text-white"),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub enum ButtonSize {
    #[default]
    Base,
}

impl Display for ButtonSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ButtonSize::*;

        match self {
            Base => write!(f, "mu-button"),
        }
    }
}
