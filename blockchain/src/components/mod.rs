// 区块链组件模块
// 包含区块链的核心组件实现

pub mod cryptography;
pub mod network;
pub mod storage;
pub mod consensus;

// 重新导出组件类型
pub use cryptography::{CryptographyComponent, HashEngine, SignatureEngine, EncryptionEngine};
pub use network::{NetworkComponent, P2PNetwork, MessageRouter, PeerManager};
pub use storage::{StorageComponent, BlockStorage, StateStorage, TransactionStorage};
pub use consensus::{ConsensusComponent, ProofOfWork, ProofOfStake, DelegatedProofOfStake, PBFT};

// 组件错误类型
#[derive(Debug, thiserror::Error)]
pub enum ComponentError {
    #[error("Cryptography error: {0}")]
    CryptographyError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Consensus error: {0}")]
    ConsensusError(String),
    
    #[error("Component initialization error: {0}")]
    InitializationError(String),
    
    #[error("Component configuration error: {0}")]
    ConfigurationError(String),
}

pub type ComponentResult<T> = std::result::Result<T, ComponentError>;
