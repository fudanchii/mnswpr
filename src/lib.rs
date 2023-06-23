use std::ops::Div;

pub mod components;
pub mod errors;
pub mod exec;
pub mod external_binding;
pub mod store;

pub(crate) fn current_seconds() -> u64 {
    external_binding::now().div(1000f64).floor() as u64
}
