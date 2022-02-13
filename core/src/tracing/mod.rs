//! Tracing reexport and utilities

use ::tracing::{
    Dispatch,
    Subscriber,
};
use tracing_subscriber::Registry;

use crate::prelude::*;

pub mod prelude;

/// Initialize tracing
pub fn init<S, F>(configure: F) -> Unit
where
    S: Subscriber + Into<Dispatch>,
    F: FnOnce(Registry) -> S,
{
    let reg = tracing_subscriber::registry();
    configure(reg).init();
}
