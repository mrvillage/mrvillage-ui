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
    Transparent,
}

impl Display for ButtonColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ButtonColor::*;

        match self {
            Black => {
                write!(
                    f,
                    "tw-bg-black hover:tw-bg-stone-700 active:tw-bg-stone-600"
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
                    "tw-bg-stone-600 hover:tw-bg-stone-700 active:tw-bg-stone-800"
                )
            },
            Red => write!(f, "tw-bg-red-600 hover:tw-bg-red-700 active:tw-bg-red-800"),
            Orange => {
                write!(
                    f,
                    "tw-bg-orange-600 hover:tw-bg-orange-700 active:tw-bg-orange-800"
                )
            },
            Amber => {
                write!(
                    f,
                    "tw-bg-amber-600 hover:tw-bg-amber-700 active:tw-bg-amber-800"
                )
            },
            Yellow => {
                write!(
                    f,
                    "tw-bg-yellow-600 hover:tw-bg-yellow-700 active:tw-bg-yellow-800"
                )
            },
            Lime => {
                write!(
                    f,
                    "tw-bg-lime-600 hover:tw-bg-lime-700 active:tw-bg-lime-800"
                )
            },
            Green => {
                write!(
                    f,
                    "tw-bg-green-600 hover:tw-bg-green-700 active:tw-bg-green-800"
                )
            },
            Emerald => {
                write!(
                    f,
                    "tw-bg-emerald-600 hover:tw-bg-emerald-700 active:tw-bg-emerald-800"
                )
            },
            Teal => {
                write!(
                    f,
                    "tw-bg-teal-600 hover:tw-bg-teal-700 active:tw-bg-teal-800"
                )
            },
            Cyan => {
                write!(
                    f,
                    "tw-bg-cyan-600 hover:tw-bg-cyan-700 active:tw-bg-cyan-800"
                )
            },
            Sky => write!(f, "tw-bg-sky-600 hover:tw-bg-sky-700 active:tw-bg-sky-800"),
            Blue => {
                write!(
                    f,
                    "tw-bg-blue-600 hover:tw-bg-blue-700 active:tw-bg-blue-800"
                )
            },
            Indigo => {
                write!(
                    f,
                    "tw-bg-indigo-600 hover:tw-bg-indigo-700 active:tw-bg-indigo-800"
                )
            },
            Violet => {
                write!(
                    f,
                    "tw-bg-violet-600 hover:tw-bg-violet-700 active:tw-bg-violet-800"
                )
            },
            Purple => {
                write!(
                    f,
                    "tw-bg-purple-600 hover:tw-bg-purple-700 active:tw-bg-purple-800"
                )
            },
            Fuchsia => {
                write!(
                    f,
                    "tw-bg-fuchsia-600 hover:tw-bg-fuchsia-700 active:tw-bg-fuchsia-800"
                )
            },
            Pink => {
                write!(
                    f,
                    "tw-bg-pink-600 hover:tw-bg-pink-700 active:tw-bg-pink-800"
                )
            },
            Rose => {
                write!(
                    f,
                    "tw-bg-rose-600 hover:tw-bg-rose-700 active:tw-bg-rose-800"
                )
            },
            None => write!(f, ""),
            Transparent => {
                write!(
                    f,
                    "tw-bg-black tw-bg-opacity-0 hover:tw-bg-opacity-10 active:bg-opacity-20"
                )
            },
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
    Lg,
    Xl,
    Xxl,
    Xxxl,
}

impl Display for ButtonSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ButtonSize::*;

        match self {
            Base => write!(f, "mu-button"),
            Lg => write!(f, "mu-button-lg"),
            Xl => write!(f, "mu-button-xl"),
            Xxl => write!(f, "mu-button-xxl"),
            Xxxl => write!(f, "mu-button-xxxl"),
        }
    }
}
