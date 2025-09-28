// 网络组件模块
pub mod p2p;
pub mod message;
pub mod peer;

pub use p2p::P2PNetwork;
pub use message::{MessageRouter, NetworkMessage};
pub use peer::PeerManager;

use crate::core::{Transaction, Block, Result, BlockchainError};
use crate::components::{ComponentResult};

/// 网络组件
pub struct NetworkComponent {
    pub p2p_network: P2PNetwork,
    pub message_router: MessageRouter,
    pub peer_manager: PeerManager,
}

impl NetworkComponent {
    /// 创建新的网络组件
    pub fn new() -> Self {
        Self {
            p2p_network: P2PNetwork::new(),
            message_router: MessageRouter::new(),
            peer_manager: PeerManager::new(),
        }
    }
    
    /// 初始化网络组件
    pub async fn initialize(&mut self) -> ComponentResult<()> {
        self.p2p_network.initialize().await?;
        self.message_router.initialize().await?;
        self.peer_manager.initialize().await?;
        Ok(())
    }
    
    /// 广播交易
    pub async fn broadcast_transaction(&mut self, tx: &Transaction) -> Result<()> {
        let message = NetworkMessage::Transaction(tx.clone());
        self.message_router.broadcast(message).await
            .map_err(|e| BlockchainError::NetworkError(format!("Failed to broadcast transaction: {}", e)))?;
        Ok(())
    }
    
    /// 广播区块
    pub async fn broadcast_block(&mut self, block: &Block) -> Result<()> {
        let message = NetworkMessage::Block(block.clone());
        self.message_router.broadcast(message).await
            .map_err(|e| BlockchainError::NetworkError(format!("Failed to broadcast block: {}", e)))?;
        Ok(())
    }
    
    /// 与对等节点同步
    pub async fn sync_with_peers(&mut self) -> Result<()> {
        let peers = self.peer_manager.get_active_peers().await;
        for peer in peers {
            self.sync_with_peer(peer).await?;
        }
        Ok(())
    }
    
    /// 与特定对等节点同步
    #[allow(dead_code)]
    #[allow(unused_variables)]
    async fn sync_with_peer(&mut self, peer: String) -> Result<()> {
        // 实现与特定对等节点的同步逻辑
        // 这里应该包含区块同步、交易同步等
        Ok(())
    }
    
    /// 获取对等节点数量
    #[allow(dead_code)]
    pub async fn get_peer_count(&self) -> usize {
        self.peer_manager.get_peer_count().await
    }
    
    /// 添加对等节点
    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub async fn add_peer(&mut self, peer: String) -> Result<()> {
        self.peer_manager.add_peer(peer).await
            .map_err(|e| BlockchainError::NetworkError(format!("Failed to add peer: {}", e)))?;
        Ok(())
    }
    
    /// 移除对等节点
    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub async fn remove_peer(&mut self, peer: String) -> Result<()> {
        self.peer_manager.remove_peer(peer).await
            .map_err(|e| BlockchainError::NetworkError(format!("Failed to remove peer: {}", e)))?;
        Ok(())
    }
    
    /// 启动网络服务
    pub async fn start(&mut self, port: u16) -> Result<()> {
        self.p2p_network.start(port).await
            .map_err(|e| BlockchainError::NetworkError(format!("Failed to start network: {}", e)))?;
        Ok(())
    }
    
    /// 停止网络服务
    pub async fn stop(&mut self) -> Result<()> {
        self.p2p_network.stop().await
            .map_err(|e| BlockchainError::NetworkError(format!("Failed to stop network: {}", e)))?;
        Ok(())
    }
}
