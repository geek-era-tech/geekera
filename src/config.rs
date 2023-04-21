use std::sync::Arc;

pub async fn init<T>() -> anyhow::Result<T> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .add_source(
            config::Environment::with_prefix("APP")
                .try_parsing(true)
                .separator("_")
                .list_separator(" "),
        )
        .build()?;

    let config: T = settings.try_deserialize()?;

    Ok(config)
}
