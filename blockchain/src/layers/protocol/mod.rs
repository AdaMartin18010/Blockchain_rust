//! 协议层模块
//! 
//! 提供区块链协议实现，包括网络协议、消息协议、同步协议等

pub mod network_protocol;
pub mod message_protocol;
pub mod sync_protocol;
pub mod discovery_protocol;

pub use network_protocol::NetworkProtocol;
pub use message_protocol::MessageProtocol;
pub use sync_protocol::SyncProtocol;
pub use discovery_protocol::DiscoveryProtocol;

use crate::core::{Result, BlockchainError};

/// 协议层结果类型
pub type ProtocolResult<T> = Result<T>;

/// 协议层错误类型
#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("协议错误: {0}")]
    ProtocolError(String),
    #[error("消息解析失败: {0}")]
    MessageParsingFailed(String),
    #[error("同步失败: {0}")]
    SyncFailed(String),
    #[error("发现失败: {0}")]
    DiscoveryFailed(String),
}

impl From<ProtocolError> for BlockchainError {
    fn from(err: ProtocolError) -> Self {
        BlockchainError::ProtocolError(err.to_string())
    }
}
