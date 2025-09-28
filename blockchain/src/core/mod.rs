// 区块链核心模块
// 包含区块链的基本数据结构和核心逻辑

pub mod blockchain;
pub mod block;
pub mod transaction;
pub mod state;
pub mod merkle;

// 重新导出核心类型
pub use blockchain::Blockchain;
pub use block::{Block, BlockHeader};
pub use transaction::{Transaction, TxInput, TxOutput, Witness};
pub use state::{State, StateChange, StateKey, StateValue};
pub use merkle::{MerkleTree, MerkleProof};

// 核心错误类型
#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
    #[error("Invalid block: {0}")]
    InvalidBlock(String),
    
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("Consensus failed: {0}")]
    ConsensusFailed(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Cryptographic error: {0}")]
    CryptographicError(String),

    // Extended error variants used by other layers/modules
    #[error("Consensus error: {0}")]
    ConsensusError(String),
    #[error("Business error: {0}")]
    BusinessError(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),
    #[error("Algorithm error: {0}")]
    AlgorithmError(String),
    #[error("Smart contract error: {0}")]
    SmartContractError(String),
    #[error("Utils error: {0}")]
    UtilsError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

pub type Result<T> = std::result::Result<T, BlockchainError>;
