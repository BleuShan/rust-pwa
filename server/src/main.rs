#![forbid(future_incompatible)]
#![warn(
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    unreachable_pub
)]
#![feature(never_type, trait_alias, backtrace)]
#![recursion_limit = "1024"]

use rocket::{
    figment::{
        providers::Env,
        Figment,
    },
    fs::{
        relative,
        FileServer,
    },
    Config,
};
use rust_pwa_core::prelude::*;
use tokio::{
    fs::File,
    io::{
        AsyncReadExt,
        AsyncWriteExt,
    },
};

#[tokio::main]
async fn main() -> Result<Unit> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    let setting_path = relative!("settings.json");
    let config = match File::open(setting_path).await {
        Err(err) if err.kind() == IOErrorKind::NotFound => {
            let config = Config::default();
            let mut file = File::create(setting_path).await?;
            let content = json::to_vec_pretty(&config)?;
            file.write_all(content.as_slice()).await?;
            config
        }
        Err(err) => {
            return Err(format_err!("{}", err));
        }
        Ok(mut file) => {
            let mut buf: Vec<u8> = Vec::new();
            file.read_to_end(&mut buf).await?;
            json::from_slice(buf.as_slice())?
        }
    };
    rocket::custom(
        Figment::from(config)
            .merge(Env::prefixed("ROCKET_"))
            .merge(Env::prefixed("RUST_PWA_")),
    )
    .mount("/", FileServer::from(relative!("static")))
    .launch()
    .await
    .map_err(|err| format_err!("{}", err))
}
