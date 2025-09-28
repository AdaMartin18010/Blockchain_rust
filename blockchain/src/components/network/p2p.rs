// P2P网络实现
use crate::components::{ComponentResult, ComponentError};
use std::collections::HashMap;
use std::net::{SocketAddr};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::net::{TcpListener as TokioTcpListener, TcpStream as TokioTcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// P2P网络
pub struct P2PNetwork {
    /// 监听地址
    listen_addr: Option<SocketAddr>,
    /// 连接的对等节点
    connections: Arc<RwLock<HashMap<String, Connection>>>,
    /// 是否正在运行
    running: Arc<Mutex<bool>>,
    /// 消息处理器
    message_handlers: Arc<RwLock<HashMap<String, Box<dyn MessageHandler + Send + Sync>>>>,
}

/// 网络连接
#[derive(Debug)]
pub struct Connection {
    pub peer_id: String,
    pub address: SocketAddr,
    pub stream: Arc<Mutex<TokioTcpStream>>,
    pub connected_at: std::time::Instant,
    pub last_seen: std::time::Instant,
}

/// 消息处理器 trait
pub trait MessageHandler: Send + Sync {
    fn handle_message(&self, message: &[u8], peer_id: &str) -> ComponentResult<Vec<u8>>;
    fn message_type(&self) -> &str;
}

/// 网络消息类型
#[derive(Debug, Clone)]
pub enum NetworkMessageType {
    Transaction,
    Block,
    PeerDiscovery,
    SyncRequest,
    SyncResponse,
    Ping,
    Pong,
}

impl P2PNetwork {
    /// 创建新的P2P网络
    pub fn new() -> Self {
        Self {
            listen_addr: None,
            connections: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(Mutex::new(false)),
            message_handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// 初始化P2P网络
    pub async fn initialize(&mut self) -> ComponentResult<()> {
        // 注册默认消息处理器
        self.register_message_handler("ping", Box::new(PingHandler)).await;
        self.register_message_handler("pong", Box::new(PongHandler)).await;
        
        Ok(())
    }
    
    /// 启动P2P网络
    pub async fn start(&mut self, port: u16) -> ComponentResult<()> {
        let addr = format!("0.0.0.0:{}", port);
        let listener = TokioTcpListener::bind(&addr).await
            .map_err(|e| ComponentError::NetworkError(format!("Failed to bind to {}: {}", addr, e)))?;
        
        self.listen_addr = Some(listener.local_addr()
            .map_err(|e| ComponentError::NetworkError(format!("Failed to get local address: {}", e)))?);
        
        let connections = Arc::clone(&self.connections);
        let running = Arc::clone(&self.running);
        let message_handlers = Arc::clone(&self.message_handlers);
        
        // 启动监听任务
        tokio::spawn(async move {
            let mut running_guard = running.lock().await;
            *running_guard = true;
            drop(running_guard);
            
            while *running.lock().await {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        let peer_id = format!("peer_{}", addr);
                        let connection = Connection {
                            peer_id: peer_id.clone(),
                            address: addr,
                            stream: Arc::new(Mutex::new(stream)),
                            connected_at: std::time::Instant::now(),
                            last_seen: std::time::Instant::now(),
                        };
                        
                        connections.write().await.insert(peer_id.clone(), connection);
                        
                        // 启动连接处理任务
                        let connections_clone = Arc::clone(&connections);
                        let message_handlers_clone = Arc::clone(&message_handlers);
                        tokio::spawn(async move {
                            Self::handle_connection(peer_id, connections_clone, message_handlers_clone).await;
                        });
                    }
                    Err(e) => {
                        eprintln!("Failed to accept connection: {}", e);
                    }
                }
            }
        });
        
        Ok(())
    }
    
    /// 停止P2P网络
    pub async fn stop(&mut self) -> ComponentResult<()> {
        let mut running_guard = self.running.lock().await;
        *running_guard = false;
        drop(running_guard);
        
        // 关闭所有连接
        let mut connections = self.connections.write().await;
        connections.clear();
        
        Ok(())
    }
    
    /// 连接到对等节点
    pub async fn connect_to_peer(&mut self, address: &str) -> ComponentResult<()> {
        let stream = TokioTcpStream::connect(address).await
            .map_err(|e| ComponentError::NetworkError(format!("Failed to connect to {}: {}", address, e)))?;
        
        let addr = stream.peer_addr()
            .map_err(|e| ComponentError::NetworkError(format!("Failed to get peer address: {}", e)))?;
        
        let peer_id = format!("peer_{}", addr);
        let connection = Connection {
            peer_id: peer_id.clone(),
            address: addr,
            stream: Arc::new(Mutex::new(stream)),
            connected_at: std::time::Instant::now(),
            last_seen: std::time::Instant::now(),
        };
        
        self.connections.write().await.insert(peer_id.clone(), connection);
        
        Ok(())
    }
    
    /// 断开与对等节点的连接
    pub async fn disconnect_peer(&mut self, peer_id: &str) -> ComponentResult<()> {
        self.connections.write().await.remove(peer_id);
        Ok(())
    }
    
    /// 发送消息到对等节点
    pub async fn send_message(&mut self, peer_id: &str, message: &[u8]) -> ComponentResult<()> {
        let connections = self.connections.read().await;
        if let Some(connection) = connections.get(peer_id) {
            let mut stream = connection.stream.lock().await;
            stream.write_all(message).await
                .map_err(|e| ComponentError::NetworkError(format!("Failed to send message: {}", e)))?;
        } else {
            return Err(ComponentError::NetworkError(format!("Peer {} not found", peer_id)));
        }
        
        Ok(())
    }
    
    /// 广播消息到所有对等节点
    pub async fn broadcast_message(&mut self, message: &[u8]) -> ComponentResult<()> {
        let connections = self.connections.read().await;
        let peer_ids: Vec<String> = connections.keys().cloned().collect();
        drop(connections);
        
        for peer_id in peer_ids {
            if let Err(e) = self.send_message(&peer_id, message).await {
                eprintln!("Failed to send message to {}: {}", peer_id, e);
            }
        }
        
        Ok(())
    }
    
    /// 注册消息处理器
    pub async fn register_message_handler(&mut self, message_type: &str, handler: Box<dyn MessageHandler + Send + Sync>) {
        self.message_handlers.write().await.insert(message_type.to_string(), handler);
    }
    
    /// 处理连接
    async fn handle_connection(
        peer_id: String,
        connections: Arc<RwLock<HashMap<String, Connection>>>,
        message_handlers: Arc<RwLock<HashMap<String, Box<dyn MessageHandler + Send + Sync>>>>,
    ) {
        let mut buffer = [0u8; 1024];
        
        loop {
            let stream = {
                let connections = connections.read().await;
                if let Some(connection) = connections.get(&peer_id) {
                    Arc::clone(&connection.stream)
                } else {
                    break;
                }
            };
            
            let mut stream_guard = stream.lock().await;
            match stream_guard.read(&mut buffer).await {
                Ok(0) => {
                    // 连接关闭
                    break;
                }
                Ok(n) => {
                    let message = &buffer[..n];
                    
                    // 处理消息
                    let handlers = message_handlers.read().await;
                    for (_, handler) in handlers.iter() {
                        if let Ok(response) = handler.handle_message(message, &peer_id) {
                            if let Err(e) = stream_guard.write_all(&response).await {
                                eprintln!("Failed to send response: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from connection: {}", e);
                    break;
                }
            }
        }
        
        // 清理连接
        connections.write().await.remove(&peer_id);
    }
    
    /// 获取连接数量
    pub async fn get_connection_count(&self) -> usize {
        self.connections.read().await.len()
    }
    
    /// 获取所有对等节点ID
    pub async fn get_peer_ids(&self) -> Vec<String> {
        self.connections.read().await.keys().cloned().collect()
    }
    
    /// 检查是否正在运行
    pub async fn is_running(&self) -> bool {
        *self.running.lock().await
    }
    
    /// 获取监听地址
    pub fn get_listen_addr(&self) -> Option<SocketAddr> {
        self.listen_addr
    }
}

/// Ping消息处理器
struct PingHandler;

impl MessageHandler for PingHandler {
    fn handle_message(&self, _message: &[u8], _peer_id: &str) -> ComponentResult<Vec<u8>> {
        Ok(b"pong".to_vec())
    }
    
    fn message_type(&self) -> &str {
        "ping"
    }
}

/// Pong消息处理器
struct PongHandler;

impl MessageHandler for PongHandler {
    fn handle_message(&self, _message: &[u8], _peer_id: &str) -> ComponentResult<Vec<u8>> {
        Ok(b"ping".to_vec())
    }
    
    fn message_type(&self) -> &str {
        "pong"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_p2p_network_initialization() {
        let mut network = P2PNetwork::new();
        assert!(network.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_p2p_network_start_stop() {
        let mut network = P2PNetwork::new();
        network.initialize().await.unwrap();
        
        assert!(network.start(8080).await.is_ok());
        assert!(network.is_running().await);
        
        assert!(network.stop().await.is_ok());
        assert!(!network.is_running().await);
    }

    #[tokio::test]
    async fn test_message_handler_registration() {
        let mut network = P2PNetwork::new();
        network.initialize().await.unwrap();
        
        network.register_message_handler("test", Box::new(TestHandler)).await;
        
        let handlers = network.message_handlers.read().await;
        assert!(handlers.contains_key("test"));
    }

    #[tokio::test]
    async fn test_connection_count() {
        let mut network = P2PNetwork::new();
        network.initialize().await.unwrap();
        
        assert_eq!(network.get_connection_count().await, 0);
    }

    struct TestHandler;

    impl MessageHandler for TestHandler {
        fn handle_message(&self, _message: &[u8], _peer_id: &str) -> ComponentResult<Vec<u8>> {
            Ok(b"test_response".to_vec())
        }
        
        fn message_type(&self) -> &str {
            "test"
        }
    }
}
