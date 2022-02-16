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

pub mod application;
pub mod database;
pub mod handlers;
pub mod prelude;
pub mod typed_header;

use application::App;
use handlers::FileServer;
use prelude::*;
use rocket::{
    fs::relative,
    Build,
    Rocket,
};
use tracing_appender::{
    non_blocking,
    non_blocking::WorkerGuard,
};
use tracing_subscriber::filter::LevelFilter;

fn tracing() -> Result<WorkerGuard> {
    let (stderr, guard) = non_blocking(std::io::stderr());
    let fmt_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_level(true)
        .with_ansi(true)
        .with_writer(stderr)
        .with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(fmt_layer).init();
    Ok(guard)
}

#[instrument]
#[tokio::main]
async fn main() -> Result<Unit> {
    let app = App::init()?;
    let _guard = tracing()?;
    rocket::build()
        .mount("/", FileServer::from(relative!("/static")))
        .launch()
        .await
        .map_err(|err| format_err!("{}", err))
}
