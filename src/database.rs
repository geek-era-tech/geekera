use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::OnceCell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub url: String,
    pub max_conn: Option<u32>,
    pub min_conn: Option<u32>,
    pub conn_timeout: Option<u64>,
    pub idle_timeout: Option<u64>,
    pub acquire_timeout: Option<u64>,
    pub max_lifetime: Option<u64>,
    pub use_log: Option<bool>,
    pub log_level: Option<String>,
    pub schema: Option<String>,
}

static INSTANCE: OnceCell<Arc<DatabaseConnection>> = OnceCell::const_new();

pub async fn init(config: &Config) -> anyhow::Result<()> {
    let mut opt = ConnectOptions::new(config.url.clone());

    opt.sqlx_logging(false);

    if let Some(min_conn) = config.min_conn {
        opt.min_connections(min_conn);
    }

    if let Some(max_conn) = config.max_conn {
        opt.max_connections(max_conn);
    }

    if let Some(conn_timeout) = config.conn_timeout {
        opt.connect_timeout(Duration::from_secs(conn_timeout));
    }

    if let Some(idle_timeout) = config.idle_timeout {
        opt.idle_timeout(Duration::from_secs(idle_timeout));
    }

    if let Some(acquire_timeout) = config.acquire_timeout {
        opt.acquire_timeout(Duration::from_secs(acquire_timeout));
    }

    if let Some(max_lifetime) = config.max_lifetime {
        opt.max_lifetime(Duration::from_secs(max_lifetime));
    }

    if let Some(use_log) = config.use_log {
        opt.sqlx_logging(use_log);
    }

    if let Some(log_level) = &config.log_level {
        opt.sqlx_logging_level(
            LevelFilter::from_str(&log_level.as_str()).map_err(|err| anyhow::anyhow!(err))?,
        );
    }

    if let Some(schema) = &config.schema {
        opt.set_schema_search_path(schema.into());
    }

    let db = Database::connect(opt).await?;
    let db = Arc::new(db);

    INSTANCE.set(db)?;

    Ok(())
}

pub fn get() -> anyhow::Result<&'static Arc<DatabaseConnection>> {
    INSTANCE.get().ok_or(anyhow::anyhow!("数据库未初始化"))
}
