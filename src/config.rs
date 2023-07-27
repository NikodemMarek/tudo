use envpath::EnvPath;
use schematic::{Config, ConfigLoader};

#[derive(Config, Debug)]
#[config(rename_all = "snake_case")]
pub struct Cfg {
    #[setting(default = "client_secret.json")]
    pub client_secret: String,
}

pub fn get_config() -> anyhow::Result<Cfg> {
    let path = get_config_path();
    let result = ConfigLoader::<Cfg>::new().file(path)?.load()?;

    Ok(result.config)
}

fn get_config_path() -> std::path::PathBuf {
    EnvPath::from(["$dir: cfg", "tudo", "config.toml"])
        .de()
        .to_path_buf()
}
