//! # P2P 网络模块
//! 
//! 基于 WebSocket 的点对点网络通信
//! P2P network module based on WebSocket communication

use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::{accept_async, connect_async, tungstenite::Message};
use thiserror::Error;
use uuid::Uuid;

/// 网络错误类型
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum NetworkError {
    #[error("Connection failed")]
    ConnectionFailed,
    #[error("Message serialization failed")]
    SerializationFailed,
    #[error("Message deserialization failed")]
    DeserializationFailed,
    #[error("Peer not found")]
    PeerNotFound,
    #[error("Invalid message format")]
    InvalidMessageFormat,
    #[error("Network timeout")]
    Timeout,
    #[error("Authentication failed")]
    AuthenticationFailed,
}

/// 消息类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// 握手消息
    Handshake(HandshakeMessage),
    /// 区块消息
    Block(BlockMessage),
    /// 交易消息
    Transaction(TransactionMessage),
    /// 请求消息
    Request(RequestMessage),
    /// 响应消息
    Response(ResponseMessage),
    /// 心跳消息
    Ping,
    /// 心跳响应
    Pong,
}

/// 握手消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeMessage {
    pub version: String,
    pub node_id: String,
    pub capabilities: Vec<String>,
    pub timestamp: u64,
}

/// 区块消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMessage {
    pub block_hash: String,
    pub block_data: Vec<u8>,
    pub height: u64,
}

/// 交易消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMessage {
    pub tx_hash: String,
    pub tx_data: Vec<u8>,
    pub fee: u64,
}

/// 请求消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMessage {
    pub request_id: String,
    pub request_type: String,
    pub data: Vec<u8>,
}

/// 响应消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMessage {
    pub request_id: String,
    pub success: bool,
    pub data: Vec<u8>,
    pub error: Option<String>,
}

/// 网络消息包装器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub id: String,
    pub message_type: MessageType,
    pub timestamp: u64,
    pub ttl: u32, // 生存时间
}

impl NetworkMessage {
    /// 创建新的网络消息
    /// Create new network message
    pub fn new(message_type: MessageType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            message_type,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl: 10, // 默认 TTL
        }
    }

    /// 序列化消息
    /// Serialize message
    pub fn serialize(&self) -> Result<Vec<u8>, NetworkError> {
        bincode::serialize(self).map_err(|_| NetworkError::SerializationFailed)
    }

    /// 反序列化消息
    /// Deserialize message
    pub fn deserialize(data: &[u8]) -> Result<Self, NetworkError> {
        bincode::deserialize(data).map_err(|_| NetworkError::DeserializationFailed)
    }

    /// 检查消息是否过期
    /// Check if message is expired
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        now - self.timestamp > self.ttl as u64
    }
}

/// 对等节点信息
#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub id: String,
    pub address: SocketAddr,
    pub version: String,
    pub capabilities: Vec<String>,
    pub last_seen: u64,
    pub is_connected: bool,
}

/// 消息处理器 trait
#[async_trait]
pub trait MessageHandler: Send + Sync {
    async fn handle_message(&self, message: NetworkMessage, peer_id: String) -> Result<(), NetworkError>;
}

/// P2P 网络节点
pub struct P2PNode {
    pub node_id: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub peers: Arc<RwLock<HashMap<String, PeerInfo>>>,
    pub message_handler: Arc<dyn MessageHandler>,
    pub incoming_tx: mpsc::UnboundedReceiver<NetworkMessage>,
    pub outgoing_tx: mpsc::UnboundedSender<NetworkMessage>,
    pub listen_address: Option<SocketAddr>,
}

impl P2PNode {
    /// 创建新的 P2P 节点
    /// Create new P2P node
    pub fn new(
        node_id: String,
        version: String,
        capabilities: Vec<String>,
        message_handler: Arc<dyn MessageHandler>,
    ) -> Self {
        let (incoming_tx, incoming_rx) = mpsc::unbounded_channel();
        let (outgoing_tx, outgoing_rx) = mpsc::unbounded_channel();

        Self {
            node_id,
            version,
            capabilities,
            peers: Arc::new(RwLock::new(HashMap::new())),
            message_handler,
            incoming_tx: incoming_rx,
            outgoing_tx,
            listen_address: None,
        }
    }

    /// 启动节点服务器
    /// Start node server
    pub async fn start_server(&mut self, address: SocketAddr) -> Result<(), NetworkError> {
        let listener = TcpListener::bind(address).await
            .map_err(|_| NetworkError::ConnectionFailed)?;

        self.listen_address = Some(address);

        println!("🚀 P2P 节点启动在地址: {}", address);

        // 启动消息处理循环
        let peers = Arc::clone(&self.peers);
        let message_handler = Arc::clone(&self.message_handler);
        let node_id = self.node_id.clone();

        tokio::spawn(async move {
            while let Some(message) = self.incoming_tx.recv().await {
                // 处理接收到的消息
                if let Err(e) = message_handler.handle_message(message, node_id.clone()).await {
                    eprintln!("消息处理错误: {}", e);
                }
            }
        });

        // 接受连接
        while let Ok((stream, addr)) = listener.accept().await {
            let peers = Arc::clone(&self.peers);
            let node_id = self.node_id.clone();
            let version = self.version.clone();
            let capabilities = self.capabilities.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, addr, peers, node_id, version, capabilities).await {
                    eprintln!("连接处理错误: {}", e);
                }
            });
        }

        Ok(())
    }

    /// 连接到对等节点
    /// Connect to peer node
    pub async fn connect_to_peer(&self, address: SocketAddr) -> Result<(), NetworkError> {
        let stream = TcpStream::connect(address).await
            .map_err(|_| NetworkError::ConnectionFailed)?;

        let peers = Arc::clone(&self.peers);
        let node_id = self.node_id.clone();
        let version = self.version.clone();
        let capabilities = self.capabilities.clone();

        tokio::spawn(async move {
            if let Err(e) = Self::handle_connection(stream, address, peers, node_id, version, capabilities).await {
                eprintln!("连接处理错误: {}", e);
            }
        });

        Ok(())
    }

    /// 处理连接
    /// Handle connection
    async fn handle_connection(
        stream: TcpStream,
        addr: SocketAddr,
        peers: Arc<RwLock<HashMap<String, PeerInfo>>>,
        node_id: String,
        version: String,
        capabilities: Vec<String>,
    ) -> Result<(), NetworkError> {
        let ws_stream = accept_async(stream).await
            .map_err(|_| NetworkError::ConnectionFailed)?;

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // 发送握手消息
        let handshake = NetworkMessage::new(MessageType::Handshake(HandshakeMessage {
            version: version.clone(),
            node_id: node_id.clone(),
            capabilities: capabilities.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }));

        let handshake_data = handshake.serialize()?;
        ws_sender.send(Message::Binary(handshake_data)).await
            .map_err(|_| NetworkError::ConnectionFailed)?;

        // 接收握手响应
        if let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Binary(data)) => {
                    let response = NetworkMessage::deserialize(&data)?;
                    
                    if let MessageType::Handshake(handshake_msg) = response.message_type {
                        // 添加对等节点
                        let peer_info = PeerInfo {
                            id: handshake_msg.node_id.clone(),
                            address: addr,
                            version: handshake_msg.version,
                            capabilities: handshake_msg.capabilities,
                            last_seen: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                            is_connected: true,
                        };

                        peers.write().await.insert(handshake_msg.node_id, peer_info);
                        println!("✅ 成功连接到对等节点: {}", addr);
                    }
                }
                _ => return Err(NetworkError::InvalidMessageFormat),
            }
        }

        // 处理消息循环
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Binary(data)) => {
                    if let Ok(network_message) = NetworkMessage::deserialize(&data) {
                        if !network_message.is_expired() {
                            // 处理消息
                            println!("📨 收到消息: {:?}", network_message.message_type);
                        }
                    }
                }
                Ok(Message::Ping(data)) => {
                    ws_sender.send(Message::Pong(data)).await
                        .map_err(|_| NetworkError::ConnectionFailed)?;
                }
                Ok(Message::Close(_)) => {
                    println!("🔌 连接关闭: {}", addr);
                    break;
                }
                Err(e) => {
                    eprintln!("WebSocket 错误: {}", e);
                    break;
                }
                _ => {}
            }
        }

        // 清理对等节点
        if let Some(peer_info) = peers.write().await.values_mut().find(|p| p.address == addr) {
            peer_info.is_connected = false;
        }

        Ok(())
    }

    /// 广播消息
    /// Broadcast message
    pub async fn broadcast_message(&self, message: NetworkMessage) -> Result<(), NetworkError> {
        let peers = self.peers.read().await;
        let connected_peers: Vec<_> = peers
            .values()
            .filter(|peer| peer.is_connected)
            .collect();

        for peer in connected_peers {
            // 这里应该发送消息到对等节点
            // 简化实现，只打印消息
            println!("📤 广播消息到 {}: {:?}", peer.address, message.message_type);
        }

        Ok(())
    }

    /// 发送消息到特定对等节点
    /// Send message to specific peer
    pub async fn send_message_to_peer(
        &self,
        peer_id: &str,
        message: NetworkMessage,
    ) -> Result<(), NetworkError> {
        let peers = self.peers.read().await;
        if let Some(peer) = peers.get(peer_id) {
            if peer.is_connected {
                println!("📤 发送消息到 {}: {:?}", peer.address, message.message_type);
                // 这里应该实际发送消息
                Ok(())
            } else {
                Err(NetworkError::ConnectionFailed)
            }
        } else {
            Err(NetworkError::PeerNotFound)
        }
    }

    /// 获取对等节点列表
    /// Get peer list
    pub async fn get_peers(&self) -> Vec<PeerInfo> {
        self.peers.read().await.values().cloned().collect()
    }

    /// 获取连接的对等节点数量
    /// Get number of connected peers
    pub async fn get_connected_peer_count(&self) -> usize {
        self.peers.read().await
            .values()
            .filter(|peer| peer.is_connected)
            .count()
    }

    /// 启动心跳机制
    /// Start heartbeat mechanism
    pub async fn start_heartbeat(&self) {
        let peers = Arc::clone(&self.peers);
        let node_id = self.node_id.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                let ping_message = NetworkMessage::new(MessageType::Ping);
                // 这里应该发送心跳到所有连接的对等节点
                
                // 清理过期连接
                let mut peers_guard = peers.write().await;
                peers_guard.retain(|_, peer| {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    
                    peer.is_connected && (now - peer.last_seen) < 120 // 2分钟超时
                });
            }
        });
    }
}

/// 默认消息处理器实现
pub struct DefaultMessageHandler;

#[async_trait]
impl MessageHandler for DefaultMessageHandler {
    async fn handle_message(&self, message: NetworkMessage, _peer_id: String) -> Result<(), NetworkError> {
        match message.message_type {
            MessageType::Handshake(handshake) => {
                println!("🤝 收到握手消息: {}", handshake.node_id);
            }
            MessageType::Block(block) => {
                println!("📦 收到区块消息: {}", block.block_hash);
            }
            MessageType::Transaction(tx) => {
                println!("💰 收到交易消息: {}", tx.tx_hash);
            }
            MessageType::Request(request) => {
                println!("❓ 收到请求消息: {}", request.request_type);
            }
            MessageType::Response(response) => {
                println!("✅ 收到响应消息: {}", response.request_id);
            }
            MessageType::Ping => {
                println!("💓 收到心跳消息");
            }
            MessageType::Pong => {
                println!("💓 收到心跳响应");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_message() {
        let message = NetworkMessage::new(MessageType::Ping);
        assert!(!message.is_expired());
        
        let serialized = message.serialize().unwrap();
        let deserialized = NetworkMessage::deserialize(&serialized).unwrap();
        assert_eq!(message.id, deserialized.id);
    }

    #[test]
    fn test_handshake_message() {
        let handshake = HandshakeMessage {
            version: "1.0.0".to_string(),
            node_id: "test_node".to_string(),
            capabilities: vec!["blockchain".to_string(), "smart_contracts".to_string()],
            timestamp: 1234567890,
        };
        
        let message = NetworkMessage::new(MessageType::Handshake(handshake));
        assert_eq!(message.ttl, 10);
    }

    #[test]
    fn test_peer_info() {
        let peer = PeerInfo {
            id: "peer_1".to_string(),
            address: "127.0.0.1:8080".parse().unwrap(),
            version: "1.0.0".to_string(),
            capabilities: vec!["blockchain".to_string()],
            last_seen: 1234567890,
            is_connected: true,
        };
        
        assert_eq!(peer.id, "peer_1");
        assert!(peer.is_connected);
    }
}
