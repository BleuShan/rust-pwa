use std::{
    env,
    path::Path,
};

use crate::prelude::*;
use figment::{
    providers::{
        Env,
        Format,
        Json,
        Toml,
        Yaml,
    },
    Figment,
};
use rocket::Config;
use tokio::fs;

pub async fn load() -> Result<Figment> {
    let mut config = Figment::from(Config::default());
    if cfg!(debug_assertions) {
        config = load_from_path(env!("CARGO_MANIFEST_DIR"), config).await?;
    }
    let exe_dir = env::current_exe()
        .ok()
        .map(|path| path.parent().map(|p| p.to_owned()))
        .flatten();
    if let Some(path) = exe_dir {
        config = load_from_path(path, config).await?;
    }
    config = load_from_path(env::current_dir()?, config).await?;
    Ok(config)
}

async fn load_from_path<PathRef>(path: PathRef, config: Figment) -> Result<Figment>
where
    PathRef: AsRef<Path>,
{
    dotenv::from_path(path.as_ref().join(".env")).ok();
    let mut result = config
        .merge(Env::prefixed("ROCKET_"))
        .merge(Env::prefixed("RUST_PWA_"));
    let mut dir = fs::read_dir(path.as_ref().clone()).await?;
    while let Some(entry) = dir.next_entry().await? {
        let file_type = entry.file_type().await?;
        let file_path = entry.path();
        let file_prefix = file_path.file_prefix().map(|s| s.to_string_lossy());
        if file_type.is_dir() || !file_prefix.is_some_with(|prefix| prefix == "application") {
            continue;
        }

        result = match file_path
            .extension()
            .map(|f| f.to_string_lossy().to_lowercase())
        {
            Some(ext) if ext == "json" => result.merge(Json::file(file_path)),
            Some(ext) if ext == "yaml" => result.merge(Yaml::file(file_path)),
            Some(ext) if ext == "toml" => result.merge(Toml::file(file_path)),
            _ => continue,
        };
    }

    Ok(result)
}
