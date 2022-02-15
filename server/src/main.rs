#![forbid(future_incompatible)]
#![warn(
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    unreachable_pub
)]
#![feature(
    never_type,
    trait_alias,
    backtrace,
    path_file_prefix,
    is_some_with,
    total_cmp
)]
#![recursion_limit = "1024"]

pub mod config;
pub mod handlers;
mod prelude;
use prelude::*;

use rocket::fs::{
    relative,
    FileServer,
};

#[tokio::main]
async fn main() -> Result<Unit> {
    color_eyre::install()?;
    let config = config::load().await?;
    rocket::custom(config)
        .mount("/", FileServer::from(relative!("static")))
        .launch()
        .await
        .map_err(|err| format_err!("{}", err))
}
