use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub service: ServiceConfig,
    pub storage: StorageConfig,
    pub rabbitmq: RabbitMqConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            service: ServiceConfig::default(),
            storage: StorageConfig::default(),
            rabbitmq: RabbitMqConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub port: u16,
    pub bind_address: String,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            port: 13591,
            bind_address: "127.0.0.1".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub redis_url: String,
    pub root_dir: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        Self {
            redis_url: "redis://localhost:6379".to_string(),
            root_dir: format!("{}/.local/share/logjangler", home),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RabbitMqConfig {
    pub url: String,
    pub exchange: String,
}

impl Default for RabbitMqConfig {
    fn default() -> Self {
        Self {
            url: "amqp://logjangler:logjangler@localhost:5672".to_string(),
            exchange: "logjangler.events".to_string(),
        }
    }
}
