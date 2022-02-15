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
pub mod prelude;
pub mod typed_header;
use log::LevelFilter;
use prelude::*;

use handlers::FileServer;
use rocket::{
    fs::relative,
    Build,
    Rocket,
};
use tracing_appender::{
    non_blocking,
    non_blocking::WorkerGuard,
};
use tracing_log::LogTracer;

fn rocket<P>(provider: P) -> Rocket<Build>
where
    P: figment::Provider,
{
    rocket::custom(provider).mount("/", FileServer::from(relative!("/static")))
}

fn tracing() -> Result<WorkerGuard> {
    color_eyre::install()?;
    let (stderr, guard) = non_blocking(std::io::stderr());
    let fmt_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_level(true)
        .with_target(true)
        .with_ansi(true)
        .with_writer(stderr)
        .with_file(true);
    tracing_subscriber::registry().with(fmt_layer).init();
    Ok(guard)
}

#[instrument]
#[tokio::main]
async fn main() -> Result<Unit> {
    let _guard = tracing()?;
    let config = config::load().await?;
    rocket(config)
        .launch()
        .await
        .map_err(|err| format_err!("{}", err))
}
