mod components;

pub use components::*;

pub const CSS: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/css/output.css"));
