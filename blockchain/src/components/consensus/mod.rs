//! 共识组件模块
//! 
//! 提供区块链共识机制，包括PoW、PoS、DPoS等

pub mod pow;
pub mod pos;
pub mod dpos;
pub mod pbft;

pub use pow::ProofOfWork;
pub use pos::ProofOfStake;
pub use dpos::DelegatedProofOfStake;
pub use pbft::PBFT;

use std::pin::Pin;
use std::future::Future;

use crate::core::{Result, BlockchainError, Block};

/// 共识组件结果类型
pub type ConsensusResult<T> = Result<T>;

/// 共识组件错误类型
#[derive(Debug, thiserror::Error)]
pub enum ConsensusError {
    #[error("共识验证失败: {0}")]
    ValidationFailed(String),
    #[error("挖矿失败: {0}")]
    MiningFailed(String),
    #[error("投票失败: {0}")]
    VotingFailed(String),
    #[error("提案失败: {0}")]
    ProposalFailed(String),
}

impl From<ConsensusError> for BlockchainError {
    fn from(err: ConsensusError) -> Self {
        BlockchainError::ConsensusError(err.to_string())
    }
}

/// 共识组件接口
pub trait ConsensusComponent {
    /// 初始化共识
    fn initialize(&mut self) -> Pin<Box<dyn Future<Output = ConsensusResult<()>> + Send + '_>>;
    
    /// 关闭共识
    fn shutdown(&mut self) -> Pin<Box<dyn Future<Output = ConsensusResult<()>> + Send + '_>>;
    
    /// 验证区块
    fn validate_block(&self, block: &Block) -> Pin<Box<dyn Future<Output = ConsensusResult<bool>> + Send + '_>>;
    
    /// 挖矿
    fn mine_block(&self, block: &mut Block) -> Pin<Box<dyn Future<Output = ConsensusResult<()>> + Send + '_>>;
    
    /// 获取共识统计信息
    fn get_stats(&self) -> Pin<Box<dyn Future<Output = ConsensusResult<ConsensusStats>> + Send + '_>>;
}

/// 共识统计信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConsensusStats {
    pub total_blocks_mined: u64,
    pub total_votes: u64,
    pub consensus_participants: u64,
    pub last_consensus_time: u64,
}
