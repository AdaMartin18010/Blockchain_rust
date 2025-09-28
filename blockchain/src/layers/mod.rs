// 分层架构模块
// 实现区块链的分层架构设计

pub mod application;
pub mod business;
pub mod protocol;
pub mod infrastructure;

// 重新导出分层类型
pub use application::{ApplicationLayer, WalletService, DAppService, ApiService};
pub use business::{TransactionProcessor, StateManager, WalletManager};
pub use protocol::{NetworkProtocol, MessageProtocol, SyncProtocol, DiscoveryProtocol};
pub use infrastructure::{Database, Cache, Logging, Monitoring};
