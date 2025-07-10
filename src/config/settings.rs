use crate::utils::web_error::WebError;
use config::Config;
use migration::MigratorTrait;
use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;
use simple_log::LogConfig;
use std::path::Path;

pub static CONNECT_POOL: OnceCell<DatabaseConnection> = OnceCell::new();

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_config: DatabaseConfig,
    pub log_config: LogConfig,
    pub web_config: WebConfig,
}

impl Settings {
    pub fn load_config() -> Result<Settings, WebError> {
        let config_paths = vec![
            "config/config.toml".to_string(),
            format!(
                "{}/.dss/config.toml",
                std::env::var("HOME").unwrap_or_else(|_| String::from("/root"))
            ),
        ];

        for path in config_paths {
            if Path::new(&path).exists() {
                println!("使用配置文件的路径为: {}", path);
                return Config::builder()
                    .add_source(config::File::from(Path::new(&path)))
                    .build()
                    .map_err(|e| WebError::ActixError(format!("配置文件解析错误: {}", e)))?
                    .try_deserialize()
                    .map_err(|e| WebError::ActixError(format!("配置文件反序列化错误: {}", e)));
            }
        }

        // 处理没有找到配置文件的情况
        Err(WebError::NotFound("未找到有效的配置文件".to_string()))
    }
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl DatabaseConfig {
    /**
     * 初始化链接
     */
    pub async fn init_database(config: DatabaseConfig) -> Result<(), WebError> {
        println!("开始初始化数据库！！");
        let database_url = config.url;
        println!("准备数据库连接信息{:?}", database_url);

        let connect = if database_url.contains("sqlite:") {
            // 对于SQLite内存数据库
            Database::connect(&database_url).await.map_err(|e| {
                let err_msg = format!("数据库连接失败: {}", e);
                println!("{}", err_msg);
                WebError::DBError(e.to_string())
            })?
        } else {
            let mut connect_option = ConnectOptions::new(&database_url);
            connect_option
                .max_connections(config.max_connections)
                .min_connections(config.min_connections);
            Database::connect(connect_option).await.map_err(|e| {
                let err_msg = format!("数据库连接失败: {}", e);
                println!("{}", err_msg);
                WebError::DBError(e.to_string())
            })?
        };

        if database_url.contains("sqlite::memory:") {
            // 对于SQLite内存数据库，先重置迁移状态，然后重新运行所有迁移
            migration::Migrator::reset(&connect).await?;
        }
        migration::Migrator::up(&connect, None).await?;
        CONNECT_POOL.get_or_init(move || connect);

        Ok(())
    }

    /**
     * 获取数据库连接
     */
    pub async fn get_connect() -> Result<DatabaseConnection, WebError> {
        if let Some(connect) = CONNECT_POOL.get() {
            println!("拿到数据库连接！！");
            Ok(connect.clone())
        } else {
            Err(WebError::DBError(
                "Database connection pool not initialized".into(),
            ))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct WebConfig {
    pub host: String,
    pub port: u32,
}

impl WebConfig {
    pub fn format(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
