// 对等节点管理实现
use crate::components::{ComponentResult, ComponentError};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{RwLock};
use std::time::{Duration, Instant};

/// 对等节点管理器
#[derive(Debug)]
pub struct PeerManager {
    /// 对等节点列表
    peers: Arc<RwLock<HashMap<String, Peer>>>,
    /// 是否已初始化
    initialized: bool,
}

/// 对等节点信息
#[derive(Debug, Clone)]
pub struct Peer {
    pub id: String,
    pub address: SocketAddr,
    pub connected_at: Instant,
    pub last_seen: Instant,
    pub is_active: bool,
    pub connection_count: u32,
    pub message_count: u64,
    pub latency: Option<Duration>,
}

impl PeerManager {
    /// 创建新的对等节点管理器
    pub fn new() -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            initialized: false,
        }
    }
    
    /// 初始化对等节点管理器
    pub async fn initialize(&mut self) -> ComponentResult<()> {
        self.initialized = true;
        Ok(())
    }
    
    /// 添加对等节点
    pub async fn add_peer(&mut self, address: String) -> ComponentResult<()> {
        if !self.initialized {
            return Err(ComponentError::NetworkError("Peer manager not initialized".to_string()));
        }
        
        let addr: SocketAddr = address.parse()
            .map_err(|e| ComponentError::NetworkError(format!("Invalid address format: {}", e)))?;
        
        let peer_id = format!("peer_{}", addr);
        let peer = Peer {
            id: peer_id.clone(),
            address: addr,
            connected_at: Instant::now(),
            last_seen: Instant::now(),
            is_active: true,
            connection_count: 1,
            message_count: 0,
            latency: None,
        };
        
        self.peers.write().await.insert(peer_id, peer);
        Ok(())
    }
    
    /// 移除对等节点
    pub async fn remove_peer(&mut self, peer_id: String) -> ComponentResult<()> {
        if !self.initialized {
            return Err(ComponentError::NetworkError("Peer manager not initialized".to_string()));
        }
        
        self.peers.write().await.remove(&peer_id);
        Ok(())
    }
    
    /// 获取对等节点
    pub async fn get_peer(&self, peer_id: &str) -> Option<Peer> {
        self.peers.read().await.get(peer_id).cloned()
    }
    
    /// 获取所有对等节点
    pub async fn get_all_peers(&self) -> Vec<Peer> {
        self.peers.read().await.values().cloned().collect()
    }
    
    /// 获取活跃的对等节点
    pub async fn get_active_peers(&self) -> Vec<String> {
        self.peers.read().await
            .iter()
            .filter(|(_, peer)| peer.is_active)
            .map(|(id, _)| id.clone())
            .collect()
    }
    
    /// 获取对等节点数量
    pub async fn get_peer_count(&self) -> usize {
        self.peers.read().await.len()
    }
    
    /// 获取活跃对等节点数量
    pub async fn get_active_peer_count(&self) -> usize {
        self.peers.read().await
            .values()
            .filter(|peer| peer.is_active)
            .count()
    }
    
    /// 更新对等节点状态
    pub async fn update_peer_status(&mut self, peer_id: &str, is_active: bool) -> ComponentResult<()> {
        if !self.initialized {
            return Err(ComponentError::NetworkError("Peer manager not initialized".to_string()));
        }
        
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(peer_id) {
            peer.is_active = is_active;
            peer.last_seen = Instant::now();
        }
        
        Ok(())
    }
    
    /// 更新对等节点延迟
    pub async fn update_peer_latency(&mut self, peer_id: &str, latency: Duration) -> ComponentResult<()> {
        if !self.initialized {
            return Err(ComponentError::NetworkError("Peer manager not initialized".to_string()));
        }
        
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(peer_id) {
            peer.latency = Some(latency);
            peer.last_seen = Instant::now();
        }
        
        Ok(())
    }
    
    /// 增加对等节点消息计数
    pub async fn increment_message_count(&mut self, peer_id: &str) -> ComponentResult<()> {
        if !self.initialized {
            return Err(ComponentError::NetworkError("Peer manager not initialized".to_string()));
        }
        
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(peer_id) {
            peer.message_count += 1;
            peer.last_seen = Instant::now();
        }
        
        Ok(())
    }
    
    /// 清理不活跃的对等节点
    pub async fn cleanup_inactive_peers(&mut self, timeout: Duration) -> ComponentResult<()> {
        if !self.initialized {
            return Err(ComponentError::NetworkError("Peer manager not initialized".to_string()));
        }
        
        let now = Instant::now();
        let mut peers = self.peers.write().await;
        peers.retain(|_, peer| {
            now.duration_since(peer.last_seen) < timeout
        });
        
        Ok(())
    }
    
    /// 获取对等节点统计信息
    pub async fn get_peer_statistics(&self) -> PeerStatistics {
        let peers = self.peers.read().await;
        let total_peers = peers.len();
        let active_peers = peers.values().filter(|peer| peer.is_active).count();
        let total_messages: u64 = peers.values().map(|peer| peer.message_count).sum();
        let total_connections: u32 = peers.values().map(|peer| peer.connection_count).sum();
        
        let latencies: Vec<Duration> = peers.values()
            .filter_map(|peer| peer.latency)
            .collect();
        
        let avg_latency = if !latencies.is_empty() {
            let total_latency: Duration = latencies.iter().sum();
            total_latency / latencies.len() as u32
        } else {
            Duration::from_millis(0)
        };
        
        PeerStatistics {
            total_peers,
            active_peers,
            total_messages,
            total_connections,
            average_latency: avg_latency,
        }
    }
    
    /// 检查是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// 获取对等节点连接时间
    pub async fn get_peer_connection_time(&self, peer_id: &str) -> Option<Duration> {
        self.peers.read().await
            .get(peer_id)
            .map(|peer| peer.connected_at.elapsed())
    }
    
    /// 获取对等节点最后活跃时间
    pub async fn get_peer_last_seen(&self, peer_id: &str) -> Option<Instant> {
        self.peers.read().await
            .get(peer_id)
            .map(|peer| peer.last_seen)
    }
    
    /// 重置对等节点统计信息
    pub async fn reset_peer_statistics(&mut self, peer_id: &str) -> ComponentResult<()> {
        if !self.initialized {
            return Err(ComponentError::NetworkError("Peer manager not initialized".to_string()));
        }
        
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(peer_id) {
            peer.message_count = 0;
            peer.connection_count = 0;
            peer.latency = None;
        }
        
        Ok(())
    }
}

/// 对等节点统计信息
#[derive(Debug, Clone)]
pub struct PeerStatistics {
    pub total_peers: usize,
    pub active_peers: usize,
    pub total_messages: u64,
    pub total_connections: u32,
    pub average_latency: Duration,
}

impl PeerStatistics {
    /// 获取对等节点活跃率
    pub fn get_active_rate(&self) -> f64 {
        if self.total_peers == 0 {
            0.0
        } else {
            self.active_peers as f64 / self.total_peers as f64
        }
    }
    
    /// 获取平均消息数
    pub fn get_average_messages(&self) -> f64 {
        if self.total_peers == 0 {
            0.0
        } else {
            self.total_messages as f64 / self.total_peers as f64
        }
    }
    
    /// 获取平均连接数
    pub fn get_average_connections(&self) -> f64 {
        if self.total_peers == 0 {
            0.0
        } else {
            self.total_connections as f64 / self.total_peers as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_peer_manager_initialization() {
        let mut manager = PeerManager::new();
        assert!(!manager.is_initialized());
        
        assert!(manager.initialize().await.is_ok());
        assert!(manager.is_initialized());
    }

    #[tokio::test]
    async fn test_add_remove_peer() {
        let mut manager = PeerManager::new();
        manager.initialize().await.unwrap();
        
        assert_eq!(manager.get_peer_count().await, 0);
        
        assert!(manager.add_peer("127.0.0.1:8080".to_string()).await.is_ok());
        assert_eq!(manager.get_peer_count().await, 1);
        
        let peers = manager.get_all_peers().await;
        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].address.to_string(), "127.0.0.1:8080");
        
        assert!(manager.remove_peer("peer_127.0.0.1:8080".to_string()).await.is_ok());
        assert_eq!(manager.get_peer_count().await, 0);
    }

    #[tokio::test]
    async fn test_peer_status_update() {
        let mut manager = PeerManager::new();
        manager.initialize().await.unwrap();
        
        manager.add_peer("127.0.0.1:8080".to_string()).await.unwrap();
        
        assert!(manager.update_peer_status("peer_127.0.0.1:8080", false).await.is_ok());
        assert_eq!(manager.get_active_peer_count().await, 0);
        
        assert!(manager.update_peer_status("peer_127.0.0.1:8080", true).await.is_ok());
        assert_eq!(manager.get_active_peer_count().await, 1);
    }

    #[tokio::test]
    async fn test_peer_latency_update() {
        let mut manager = PeerManager::new();
        manager.initialize().await.unwrap();
        
        manager.add_peer("127.0.0.1:8080".to_string()).await.unwrap();
        
        let latency = Duration::from_millis(100);
        assert!(manager.update_peer_latency("peer_127.0.0.1:8080", latency).await.is_ok());
        
        let peer = manager.get_peer("peer_127.0.0.1:8080").await.unwrap();
        assert_eq!(peer.latency, Some(latency));
    }

    #[tokio::test]
    async fn test_message_count_increment() {
        let mut manager = PeerManager::new();
        manager.initialize().await.unwrap();
        
        manager.add_peer("127.0.0.1:8080".to_string()).await.unwrap();
        
        assert!(manager.increment_message_count("peer_127.0.0.1:8080").await.is_ok());
        assert!(manager.increment_message_count("peer_127.0.0.1:8080").await.is_ok());
        
        let peer = manager.get_peer("peer_127.0.0.1:8080").await.unwrap();
        assert_eq!(peer.message_count, 2);
    }

    #[tokio::test]
    async fn test_peer_statistics() {
        let mut manager = PeerManager::new();
        manager.initialize().await.unwrap();
        
        manager.add_peer("127.0.0.1:8080".to_string()).await.unwrap();
        manager.add_peer("127.0.0.1:8081".to_string()).await.unwrap();
        
        manager.update_peer_status("peer_127.0.0.1:8081", false).await.unwrap();
        manager.increment_message_count("peer_127.0.0.1:8080").await.unwrap();
        manager.update_peer_latency("peer_127.0.0.1:8080", Duration::from_millis(100)).await.unwrap();
        
        let stats = manager.get_peer_statistics().await;
        assert_eq!(stats.total_peers, 2);
        assert_eq!(stats.active_peers, 1);
        assert_eq!(stats.total_messages, 1);
        assert_eq!(stats.average_latency, Duration::from_millis(100));
        assert_eq!(stats.get_active_rate(), 0.5);
    }

    #[tokio::test]
    async fn test_cleanup_inactive_peers() {
        let mut manager = PeerManager::new();
        manager.initialize().await.unwrap();
        
        manager.add_peer("127.0.0.1:8080".to_string()).await.unwrap();
        manager.add_peer("127.0.0.1:8081".to_string()).await.unwrap();
        
        // 模拟一个对等节点不活跃
        manager.update_peer_status("peer_127.0.0.1:8081", false).await.unwrap();
        
        // 清理不活跃的对等节点
        assert!(manager.cleanup_inactive_peers(Duration::from_millis(1)).await.is_ok());
        
        // 等待一段时间
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // 再次清理
        assert!(manager.cleanup_inactive_peers(Duration::from_millis(1)).await.is_ok());
        
        // 应该只剩下活跃的对等节点
        assert_eq!(manager.get_peer_count().await, 1);
    }

    #[tokio::test]
    async fn test_peer_connection_time() {
        let mut manager = PeerManager::new();
        manager.initialize().await.unwrap();
        
        manager.add_peer("127.0.0.1:8080".to_string()).await.unwrap();
        
        // 等待一段时间
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let connection_time = manager.get_peer_connection_time("peer_127.0.0.1:8080").await.unwrap();
        assert!(connection_time >= Duration::from_millis(100));
    }
}
