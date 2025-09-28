// 消息路由实现
use crate::core::{Transaction, Block};
use crate::components::{ComponentResult, ComponentError};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// 消息路由器
pub struct MessageRouter {
    /// 消息处理器
    handlers: Arc<RwLock<HashMap<String, Box<dyn MessageHandler + Send + Sync>>>>,
    /// 消息队列
    message_queue: Arc<Mutex<Vec<NetworkMessage>>>,
    /// 是否已初始化
    initialized: bool,
}

/// 网络消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    Transaction(Transaction),
    Block(Block),
    PeerDiscovery(PeerDiscoveryMessage),
    SyncRequest(SyncRequestMessage),
    SyncResponse(SyncResponseMessage),
    Ping(PingMessage),
    Pong(PongMessage),
}

/// 对等节点发现消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerDiscoveryMessage {
    pub peer_id: String,
    pub address: String,
    pub timestamp: u64,
}

/// 同步请求消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequestMessage {
    pub from_height: u64,
    pub to_height: u64,
    pub peer_id: String,
}

/// 同步响应消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResponseMessage {
    pub blocks: Vec<Block>,
    pub peer_id: String,
}

/// Ping消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingMessage {
    pub timestamp: u64,
    pub peer_id: String,
}

/// Pong消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PongMessage {
    pub timestamp: u64,
    pub peer_id: String,
}

/// 消息处理器 trait
pub trait MessageHandler: Send + Sync {
    fn handle_message(&self, message: &NetworkMessage) -> ComponentResult<()>;
    fn message_type(&self) -> &str;
}

impl MessageRouter {
    /// 创建新的消息路由器
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            message_queue: Arc::new(Mutex::new(Vec::new())),
            initialized: false,
        }
    }
    
    /// 初始化消息路由器
    pub async fn initialize(&mut self) -> ComponentResult<()> {
        // 注册默认消息处理器
        self.register_handler("transaction", Box::new(TransactionHandler)).await;
        self.register_handler("block", Box::new(BlockHandler)).await;
        self.register_handler("peer_discovery", Box::new(PeerDiscoveryHandler)).await;
        self.register_handler("sync_request", Box::new(SyncRequestHandler)).await;
        self.register_handler("sync_response", Box::new(SyncResponseHandler)).await;
        self.register_handler("ping", Box::new(PingHandler)).await;
        self.register_handler("pong", Box::new(PongHandler)).await;
        
        self.initialized = true;
        Ok(())
    }
    
    /// 注册消息处理器
    pub async fn register_handler(&mut self, message_type: &str, handler: Box<dyn MessageHandler + Send + Sync>) {
        self.handlers.write().await.insert(message_type.to_string(), handler);
    }
    
    /// 广播消息
    pub async fn broadcast(&mut self, message: NetworkMessage) -> ComponentResult<()> {
        if !self.initialized {
            return Err(ComponentError::NetworkError("Message router not initialized".to_string()));
        }
        
        // 将消息添加到队列
        self.message_queue.lock().await.push(message.clone());
        
        // 处理消息
        self.process_message(&message).await?;
        
        Ok(())
    }
    
    /// 发送消息到特定对等节点
    pub async fn send_to_peer(&mut self, _peer_id: &str, message: NetworkMessage) -> ComponentResult<()> {
        if !self.initialized {
            return Err(ComponentError::NetworkError("Message router not initialized".to_string()));
        }
        
        // 这里应该实现发送到特定对等节点的逻辑
        // 暂时只是处理消息
        self.process_message(&message).await?;
        
        Ok(())
    }
    
    /// 处理消息
    async fn process_message(&self, message: &NetworkMessage) -> ComponentResult<()> {
        let message_type = match message {
            NetworkMessage::Transaction(_) => "transaction",
            NetworkMessage::Block(_) => "block",
            NetworkMessage::PeerDiscovery(_) => "peer_discovery",
            NetworkMessage::SyncRequest(_) => "sync_request",
            NetworkMessage::SyncResponse(_) => "sync_response",
            NetworkMessage::Ping(_) => "ping",
            NetworkMessage::Pong(_) => "pong",
        };
        
        let handlers = self.handlers.read().await;
        if let Some(handler) = handlers.get(message_type) {
            handler.handle_message(message)?;
        }
        
        Ok(())
    }
    
    /// 获取消息队列大小
    pub async fn get_queue_size(&self) -> usize {
        self.message_queue.lock().await.len()
    }
    
    /// 清空消息队列
    pub async fn clear_queue(&mut self) {
        self.message_queue.lock().await.clear();
    }
    
    /// 获取支持的消息类型
    pub async fn get_supported_message_types(&self) -> Vec<String> {
        self.handlers.read().await.keys().cloned().collect()
    }
    
    /// 检查是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/// 交易消息处理器
struct TransactionHandler;

impl MessageHandler for TransactionHandler {
    fn handle_message(&self, message: &NetworkMessage) -> ComponentResult<()> {
        if let NetworkMessage::Transaction(tx) = message {
            // 处理交易消息
            println!("Received transaction: {:?}", tx.hash());
        }
        Ok(())
    }
    
    fn message_type(&self) -> &str {
        "transaction"
    }
}

/// 区块消息处理器
struct BlockHandler;

impl MessageHandler for BlockHandler {
    fn handle_message(&self, message: &NetworkMessage) -> ComponentResult<()> {
        if let NetworkMessage::Block(block) = message {
            // 处理区块消息
            println!("Received block: height {}", block.height());
        }
        Ok(())
    }
    
    fn message_type(&self) -> &str {
        "block"
    }
}

/// 对等节点发现消息处理器
struct PeerDiscoveryHandler;

impl MessageHandler for PeerDiscoveryHandler {
    fn handle_message(&self, message: &NetworkMessage) -> ComponentResult<()> {
        if let NetworkMessage::PeerDiscovery(peer_msg) = message {
            // 处理对等节点发现消息
            println!("Peer discovery: {} at {}", peer_msg.peer_id, peer_msg.address);
        }
        Ok(())
    }
    
    fn message_type(&self) -> &str {
        "peer_discovery"
    }
}

/// 同步请求消息处理器
struct SyncRequestHandler;

impl MessageHandler for SyncRequestHandler {
    fn handle_message(&self, message: &NetworkMessage) -> ComponentResult<()> {
        if let NetworkMessage::SyncRequest(sync_msg) = message {
            // 处理同步请求消息
            println!("Sync request from {}: {} to {}", sync_msg.peer_id, sync_msg.from_height, sync_msg.to_height);
        }
        Ok(())
    }
    
    fn message_type(&self) -> &str {
        "sync_request"
    }
}

/// 同步响应消息处理器
struct SyncResponseHandler;

impl MessageHandler for SyncResponseHandler {
    fn handle_message(&self, message: &NetworkMessage) -> ComponentResult<()> {
        if let NetworkMessage::SyncResponse(sync_msg) = message {
            // 处理同步响应消息
            println!("Sync response from {}: {} blocks", sync_msg.peer_id, sync_msg.blocks.len());
        }
        Ok(())
    }
    
    fn message_type(&self) -> &str {
        "sync_response"
    }
}

/// Ping消息处理器
struct PingHandler;

impl MessageHandler for PingHandler {
    fn handle_message(&self, message: &NetworkMessage) -> ComponentResult<()> {
        if let NetworkMessage::Ping(ping_msg) = message {
            // 处理Ping消息
            println!("Ping from {} at {}", ping_msg.peer_id, ping_msg.timestamp);
        }
        Ok(())
    }
    
    fn message_type(&self) -> &str {
        "ping"
    }
}

/// Pong消息处理器
struct PongHandler;

impl MessageHandler for PongHandler {
    fn handle_message(&self, message: &NetworkMessage) -> ComponentResult<()> {
        if let NetworkMessage::Pong(pong_msg) = message {
            // 处理Pong消息
            println!("Pong from {} at {}", pong_msg.peer_id, pong_msg.timestamp);
        }
        Ok(())
    }
    
    fn message_type(&self) -> &str {
        "pong"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Block, Transaction, TxInput, TxOutput};
    use crate::core::transaction::OutPoint;

    #[tokio::test]
    async fn test_message_router_initialization() {
        let mut router = MessageRouter::new();
        assert!(!router.is_initialized());
        
        assert!(router.initialize().await.is_ok());
        assert!(router.is_initialized());
    }

    #[tokio::test]
    async fn test_transaction_broadcast() {
        let mut router = MessageRouter::new();
        router.initialize().await.unwrap();
        
        let tx = create_test_transaction();
        let message = NetworkMessage::Transaction(tx);
        
        assert!(router.broadcast(message).await.is_ok());
    }

    #[tokio::test]
    async fn test_block_broadcast() {
        let mut router = MessageRouter::new();
        router.initialize().await.unwrap();
        
        let block = create_test_block();
        let message = NetworkMessage::Block(block);
        
        assert!(router.broadcast(message).await.is_ok());
    }

    #[tokio::test]
    async fn test_ping_pong_messages() {
        let mut router = MessageRouter::new();
        router.initialize().await.unwrap();
        
        let ping_msg = PingMessage {
            timestamp: 1234567890,
            peer_id: "test_peer".to_string(),
        };
        let ping_message = NetworkMessage::Ping(ping_msg);
        
        let pong_msg = PongMessage {
            timestamp: 1234567890,
            peer_id: "test_peer".to_string(),
        };
        let pong_message = NetworkMessage::Pong(pong_msg);
        
        assert!(router.broadcast(ping_message).await.is_ok());
        assert!(router.broadcast(pong_message).await.is_ok());
    }

    #[tokio::test]
    async fn test_message_queue() {
        let mut router = MessageRouter::new();
        router.initialize().await.unwrap();
        
        assert_eq!(router.get_queue_size().await, 0);
        
        let tx = create_test_transaction();
        let message = NetworkMessage::Transaction(tx);
        router.broadcast(message).await.unwrap();
        
        assert_eq!(router.get_queue_size().await, 1);
        
        router.clear_queue().await;
        assert_eq!(router.get_queue_size().await, 0);
    }

    #[tokio::test]
    async fn test_supported_message_types() {
        let mut router = MessageRouter::new();
        router.initialize().await.unwrap();
        
        let types = router.get_supported_message_types().await;
        assert!(types.contains(&"transaction".to_string()));
        assert!(types.contains(&"block".to_string()));
        assert!(types.contains(&"ping".to_string()));
        assert!(types.contains(&"pong".to_string()));
    }

    fn create_test_transaction() -> Transaction {
        let input = TxInput::new(
            OutPoint::new([1u8; 32], 0),
            1000,
            "test_address_1".to_string(),
        );
        let output = TxOutput::new(900, "test_address_2".to_string());
        Transaction::new(vec![input], vec![output])
    }

    fn create_test_block() -> Block {
        Block::create_genesis_block().unwrap()
    }
}
