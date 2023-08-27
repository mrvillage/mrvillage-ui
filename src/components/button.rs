use std::fmt::Display;

use leptos::{component, ev::MouseEvent, view, Children, IntoView, MaybeSignal, Scope};
use serde::{Deserialize, Serialize};

#[component]
pub fn Button<OC>(
    cx: Scope,
    children: Children,
    on_click: OC,
    color: MaybeSignal<ButtonColor>,
    class: MaybeSignal<String>,
) -> impl IntoView
where
    OC: Fn(MouseEvent) + 'static,
{
    let class = move || {
        format!(
            "font-semibold text-white px-2 py-1 font-semibold rounded {} {}",
            color(),
            class(),
        )
    };
    view! { cx,
        <button on:click=on_click class=class>
            {children(cx)}
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
}

impl Display for ButtonColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ButtonColor::*;

        match self {
            Black => write!(f, "bg-black hover:bg-stone-800 active:bg-stone-700"),
            White => write!(f, "bg-white hover:bg-stone-100 active:bg-stone-200"),
            Stone => write!(f, "bg-stone-700 hover:bg-stone-800 active:bg-stone-900"),
            Red => write!(f, "bg-red-700 hover:bg-red-800 active:bg-red-900"),
            Orange => write!(f, "bg-orange-700 hover:bg-orange-800 active:bg-orange-900"),
            Amber => write!(f, "bg-amber-700 hover:bg-amber-800 active:bg-amber-900"),
            Yellow => write!(f, "bg-yellow-700 hover:bg-yellow-800 active:bg-yellow-900"),
            Lime => write!(f, "bg-lime-700 hover:bg-lime-800 active:bg-lime-900"),
            Green => write!(f, "bg-green-700 hover:bg-green-800 active:bg-green-900"),
            Emerald => {
                write!(
                    f,
                    "bg-emerald-700 hover:bg-emerald-800 active:bg-emerald-900"
                )
            },
            Teal => write!(f, "bg-teal-700 hover:bg-teal-800 active:bg-teal-900"),
            Cyan => write!(f, "bg-cyan-700 hover:bg-cyan-800 active:bg-cyan-900"),
            Sky => write!(f, "bg-sky-700 hover:bg-sky-800 active:bg-sky-900"),
            Blue => write!(f, "bg-blue-700 hover:bg-blue-800 active:bg-blue-900"),
            Indigo => write!(f, "bg-indigo-700 hover:bg-indigo-800 active:bg-indigo-900"),
            Violet => write!(f, "bg-violet-700 hover:bg-violet-800 active:bg-violet-900"),
            Purple => write!(f, "bg-purple-700 hover:bg-purple-800 active:bg-purple-900"),
            Fuchsia => {
                write!(
                    f,
                    "bg-fuchsia-700 hover:bg-fuchsia-800 active:bg-fuchsia-900"
                )
            },
            Pink => write!(f, "bg-pink-700 hover:bg-pink-800 active:bg-pink-900"),
            Rose => write!(f, "bg-rose-700 hover:bg-rose-800 active:bg-rose-900"),
        }?;
        match self {
            White => write!(f, " text-black"),
            _ => write!(f, " text-white"),
        }
    }
}
