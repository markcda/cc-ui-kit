#![feature(let_chains)]
#![allow(non_snake_case)]

pub mod utils;

pub mod components;
pub mod hvalues;
pub mod router;

pub mod prelude;

// re-exports
pub use dioxus_v04_optional_hooks as optional_hooks;
