//! 日志实现

use super::InfrastructureResult;
// use crate::core::{Result, BlockchainError};

/// 日志
#[derive(Debug)]
pub struct Logging {
    // 日志相关状态
}

impl Logging {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn log(&self, level: LogLevel, message: &str) -> InfrastructureResult<()> {
        // 简化的日志记录
        println!("[{}] {}", level, message);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}
