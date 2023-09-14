mod components;

pub use components::*;
pub use leptos_tabler_icons::*;

pub const CSS: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/css/output.css"));
