use std::fmt::{Debug, Display};

use leptos::{component, view, Children, IntoAttribute, IntoView, MaybeSignal};
use serde::{Deserialize, Serialize};

#[component]
pub fn Button(
    children: Children,
    #[prop(into, optional)] color: MaybeSignal<Option<ButtonColor>>,
    #[prop(into, optional)] size: MaybeSignal<ButtonSize>,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView {
    let class = move || {
        format!(
            "mu-button {} {} {}",
            color().map(|s| s.to_string()).unwrap_or_else(String::new),
            size(),
            class(),
        )
    };
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
    Transparent,
}

impl Display for ButtonColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ButtonColor::*;

        match self {
            Black => write!(f, "mu-button-black"),
            White => write!(f, "mu-button-white"),
            Stone => write!(f, "mu-button-stone"),
            Red => write!(f, "mu-button-red"),
            Orange => write!(f, "mu-button-orange"),
            Amber => write!(f, "mu-button-amber"),
            Yellow => write!(f, "mu-button-yellow"),
            Lime => write!(f, "mu-button-lime"),
            Green => write!(f, "mu-button-green"),
            Emerald => write!(f, "mu-button-emerald"),
            Teal => write!(f, "mu-button-teal"),
            Cyan => write!(f, "mu-button-cyan"),
            Sky => write!(f, "mu-button-sky"),
            Blue => write!(f, "mu-button-blue"),
            Indigo => write!(f, "mu-button-indigo"),
            Violet => write!(f, "mu-button-violet"),
            Purple => write!(f, "mu-button-purple"),
            Fuchsia => write!(f, "mu-button-fuchsia"),
            Pink => write!(f, "mu-button-pink"),
            Rose => write!(f, "mu-button-rose"),
            Transparent => write!(f, "mu-button-transparent"),
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
