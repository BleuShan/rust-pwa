#![forbid(future_incompatible)]
#![warn(
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    unreachable_pub
)]
#![feature(never_type, trait_alias, backtrace)]
#![recursion_limit = "1024"]
pub mod prelude;
use js_sys::JsString;
use web_sys::console;

use crate::prelude::*;

#[wasm_bindgen]
pub fn start() {
    console::log_1(&JsString::from_str("Hello World!").unwrap());
}
