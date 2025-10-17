# 网络层实现

## 📋 目录

- [网络层实现](#网络层实现)
  - [📋 目录](#-目录)
  - [1. P2P网络架构](#1-p2p网络架构)
    - [1.1 网络拓扑](#11-网络拓扑)
    - [1.2 节点发现](#12-节点发现)
    - [1.3 连接管理](#13-连接管理)
  - [2. 传输层协议](#2-传输层协议)
    - [2.1 TCP/IP实现](#21-tcpip实现)
    - [2.2 QUIC支持](#22-quic支持)
    - [2.3 WebSocket](#23-websocket)
  - [3. 消息协议](#3-消息协议)
    - [3.1 协议格式](#31-协议格式)
    - [3.2 消息类型](#32-消息类型)
    - [3.3 序列化](#33-序列化)
  - [4. 同步机制](#4-同步机制)
    - [4.1 区块同步](#41-区块同步)
    - [4.2 状态同步](#42-状态同步)
    - [4.3 快照同步](#43-快照同步)
  - [5. 广播与传播](#5-广播与传播)
    - [5.1 Gossip协议](#51-gossip协议)
    - [5.2 交易传播](#52-交易传播)
    - [5.3 区块传播](#53-区块传播)
  - [6. NAT穿透](#6-nat穿透)
    - [6.1 UPnP](#61-upnp)
    - [6.2 STUN/TURN](#62-stunturn)
    - [6.3 Hole Punching](#63-hole-punching)
  - [7. 网络安全](#7-网络安全)
    - [7.1 TLS加密](#71-tls加密)
    - [7.2 节点认证](#72-节点认证)
    - [7.3 DDoS防护](#73-ddos防护)
  - [8. 性能优化](#8-性能优化)
    - [8.1 连接池](#81-连接池)
    - [8.2 消息压缩](#82-消息压缩)
    - [8.3 流量控制](#83-流量控制)
  - [9. 监控与诊断](#9-监控与诊断)
    - [9.1 网络指标](#91-网络指标)
    - [9.2 连接状态](#92-连接状态)
    - [9.3 故障诊断](#93-故障诊断)
  - [10. 总结](#10-总结)

## 1. P2P网络架构

### 1.1 网络拓扑

```rust
use tokio::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// P2P网络节点
pub struct P2PNode {
    /// 节点ID
    node_id: NodeId,
    /// 监听地址
    listen_addr: SocketAddr,
    /// 对等节点连接
    peers: Arc<RwLock<HashMap<NodeId, PeerConnection>>>,
    /// 网络配置
    config: NetworkConfig,
    /// 路由表
    routing_table: Arc<RwLock<RoutingTable>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub [u8; 32]);

#[derive(Debug)]
pub struct PeerConnection {
    /// 对等节点ID
    peer_id: NodeId,
    /// TCP流
    stream: TcpStream,
    /// 连接状态
    status: ConnectionStatus,
    /// 最后活跃时间
    last_active: SystemTime,
    /// 信誉分数
    reputation: i32,
}

#[derive(Debug, Clone)]
pub enum ConnectionStatus {
    Connecting,
    Connected,
    Disconnected,
    Banned,
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// 最大连接数
    pub max_peers: usize,
    /// 最大入站连接
    pub max_inbound: usize,
    /// 最大出站连接
    pub max_outbound: usize,
    /// 引导节点
    pub bootstrap_nodes: Vec<String>,
    /// 监听端口
    pub listen_port: u16,
}

impl P2PNode {
    /// 创建新的P2P节点
    pub async fn new(config: NetworkConfig) -> Result<Self, NetworkError> {
        let node_id = Self::generate_node_id();
        let listen_addr = format!("0.0.0.0:{}", config.listen_port)
            .parse()
            .unwrap();
        
        Ok(Self {
            node_id,
            listen_addr,
            peers: Arc::new(RwLock::new(HashMap::new())),
            config,
            routing_table: Arc::new(RwLock::new(RoutingTable::new())),
        })
    }
    
    /// 启动P2P网络
    pub async fn start(&self) -> Result<(), NetworkError> {
        // 启动监听器
        let listener = TcpListener::bind(self.listen_addr).await?;
        println!("P2P节点监听在: {}", self.listen_addr);
        
        // 连接到引导节点
        self.connect_to_bootstrap_nodes().await?;
        
        // 启动接受连接的循环
        let peers = Arc::clone(&self.peers);
        let config = self.config.clone();
        
        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        println!("接受来自 {} 的连接", addr);
                        
                        let mut peers_guard = peers.write().await;
                        if peers_guard.len() >= config.max_peers {
                            println!("达到最大连接数，拒绝连接");
                            continue;
                        }
                        
                        // 处理新连接
                        let peer_id = NodeId::random();
                        let connection = PeerConnection {
                            peer_id: peer_id.clone(),
                            stream,
                            status: ConnectionStatus::Connected,
                            last_active: SystemTime::now(),
                            reputation: 0,
                        };
                        
                        peers_guard.insert(peer_id, connection);
                    }
                    Err(e) => {
                        eprintln!("接受连接失败: {}", e);
                    }
                }
            }
        });
        
        Ok(())
    }
    
    /// 连接到引导节点
    async fn connect_to_bootstrap_nodes(&self) -> Result<(), NetworkError> {
        for node_addr in &self.config.bootstrap_nodes {
            match self.connect_to_peer(node_addr).await {
                Ok(_) => println!("成功连接到引导节点: {}", node_addr),
                Err(e) => eprintln!("连接引导节点失败 {}: {}", node_addr, e),
            }
        }
        Ok(())
    }
    
    /// 连接到指定对等节点
    pub async fn connect_to_peer(&self, addr: &str) -> Result<NodeId, NetworkError> {
        let stream = TcpStream::connect(addr).await?;
        let peer_id = NodeId::random();
        
        let connection = PeerConnection {
            peer_id: peer_id.clone(),
            stream,
            status: ConnectionStatus::Connected,
            last_active: SystemTime::now(),
            reputation: 0,
        };
        
        let mut peers = self.peers.write().await;
        peers.insert(peer_id.clone(), connection);
        
        Ok(peer_id)
    }
    
    fn generate_node_id() -> NodeId {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut id = [0u8; 32];
        rng.fill(&mut id);
        NodeId(id)
    }
    
    /// 获取已连接的对等节点数量
    pub async fn peer_count(&self) -> usize {
        self.peers.read().await.len()
    }
}

impl NodeId {
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut id = [0u8; 32];
        rng.fill(&mut id);
        NodeId(id)
    }
}
```

### 1.2 节点发现

```rust
/// Kademlia DHT节点发现
pub struct KademliaDiscovery {
    /// 路由表
    routing_table: RoutingTable,
    /// 本地节点ID
    local_id: NodeId,
}

#[derive(Debug)]
pub struct RoutingTable {
    /// K桶（K-buckets）
    buckets: Vec<KBucket>,
    /// K值（每个桶的大小）
    k: usize,
}

#[derive(Debug)]
pub struct KBucket {
    /// 桶中的节点
    nodes: Vec<KadNode>,
    /// 最大容量
    capacity: usize,
}

#[derive(Debug, Clone)]
pub struct KadNode {
    id: NodeId,
    addr: SocketAddr,
    last_seen: SystemTime,
}

impl KademliaDiscovery {
    pub fn new(local_id: NodeId, k: usize) -> Self {
        Self {
            routing_table: RoutingTable::new_with_k(k),
            local_id,
        }
    }
    
    /// 查找最近的K个节点
    pub fn find_closest_nodes(&self, target: &NodeId, k: usize) -> Vec<KadNode> {
        let mut nodes = Vec::new();
        
        for bucket in &self.routing_table.buckets {
            for node in &bucket.nodes {
                nodes.push((self.distance(&node.id, target), node.clone()));
            }
        }
        
        // 按距离排序
        nodes.sort_by_key(|(distance, _)| *distance);
        
        nodes.into_iter()
            .take(k)
            .map(|(_, node)| node)
            .collect()
    }
    
    /// 计算节点距离（XOR距离）
    fn distance(&self, a: &NodeId, b: &NodeId) -> u256 {
        let mut result = [0u8; 32];
        for i in 0..32 {
            result[i] = a.0[i] ^ b.0[i];
        }
        U256::from_big_endian(&result)
    }
    
    /// 添加节点到路由表
    pub fn add_node(&mut self, node: KadNode) {
        let bucket_index = self.bucket_index(&node.id);
        if let Some(bucket) = self.routing_table.buckets.get_mut(bucket_index) {
            bucket.add_node(node);
        }
    }
    
    /// 确定节点应该放入哪个桶
    fn bucket_index(&self, node_id: &NodeId) -> usize {
        let distance = self.distance(&self.local_id, node_id);
        // 计算前导零的数量
        256 - distance.leading_zeros() as usize
    }
}

impl RoutingTable {
    pub fn new() -> Self {
        Self::new_with_k(20) // 默认K=20
    }
    
    pub fn new_with_k(k: usize) -> Self {
        let mut buckets = Vec::new();
        for _ in 0..256 {
            buckets.push(KBucket {
                nodes: Vec::new(),
                capacity: k,
            });
        }
        
        Self { buckets, k }
    }
}

impl KBucket {
    fn add_node(&mut self, node: KadNode) {
        // 检查节点是否已存在
        if let Some(existing) = self.nodes.iter_mut().find(|n| n.id == node.id) {
            existing.last_seen = node.last_seen;
            return;
        }
        
        // 如果桶未满，直接添加
        if self.nodes.len() < self.capacity {
            self.nodes.push(node);
        } else {
            // 桶已满，替换最老的节点（如果该节点无响应）
            if let Some(oldest) = self.nodes.first() {
                if SystemTime::now().duration_since(oldest.last_seen).unwrap() 
                    > Duration::from_secs(3600) {
                    self.nodes.remove(0);
                    self.nodes.push(node);
                }
            }
        }
    }
}
```

### 1.3 连接管理

```rust
/// 连接管理器
pub struct ConnectionManager {
    /// 活跃连接
    connections: Arc<RwLock<HashMap<NodeId, Connection>>>,
    /// 连接池配置
    config: PoolConfig,
}

#[derive(Debug)]
pub struct Connection {
    id: NodeId,
    stream: TcpStream,
    state: ConnectionState,
    metrics: ConnectionMetrics,
}

#[derive(Debug)]
pub enum ConnectionState {
    Idle,
    Active,
    Closing,
}

#[derive(Debug)]
pub struct ConnectionMetrics {
    bytes_sent: u64,
    bytes_received: u64,
    messages_sent: u64,
    messages_received: u64,
    last_activity: SystemTime,
}

#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub min_connections: usize,
    pub max_connections: usize,
    pub idle_timeout: Duration,
    pub connection_timeout: Duration,
}

impl ConnectionManager {
    pub fn new(config: PoolConfig) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }
    
    /// 获取或创建连接
    pub async fn get_connection(&self, peer_id: &NodeId, addr: &str) 
        -> Result<Arc<Connection>, NetworkError> {
        let connections = self.connections.read().await;
        
        if let Some(conn) = connections.get(peer_id) {
            // 检查连接是否仍然有效
            if Self::is_connection_valid(conn) {
                return Ok(Arc::new(conn.clone()));
            }
        }
        
        drop(connections);
        
        // 创建新连接
        self.create_connection(peer_id.clone(), addr).await
    }
    
    async fn create_connection(&self, peer_id: NodeId, addr: &str) 
        -> Result<Arc<Connection>, NetworkError> {
        let stream = tokio::time::timeout(
            self.config.connection_timeout,
            TcpStream::connect(addr)
        ).await??;
        
        let connection = Connection {
            id: peer_id.clone(),
            stream,
            state: ConnectionState::Active,
            metrics: ConnectionMetrics {
                bytes_sent: 0,
                bytes_received: 0,
                messages_sent: 0,
                messages_received: 0,
                last_activity: SystemTime::now(),
            },
        };
        
        let mut connections = self.connections.write().await;
        connections.insert(peer_id, connection.clone());
        
        Ok(Arc::new(connection))
    }
    
    fn is_connection_valid(conn: &Connection) -> bool {
        matches!(conn.state, ConnectionState::Active | ConnectionState::Idle)
    }
    
    /// 清理空闲连接
    pub async fn cleanup_idle_connections(&self) {
        let mut connections = self.connections.write().await;
        let now = SystemTime::now();
        
        connections.retain(|_, conn| {
            now.duration_since(conn.metrics.last_activity)
                .unwrap_or(Duration::from_secs(0))
                < self.config.idle_timeout
        });
    }
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        // 注意：这里简化了实现，实际中不应该直接克隆TcpStream
        panic!("Connection cannot be cloned with TcpStream");
    }
}
```

## 2. 传输层协议

### 2.1 TCP/IP实现

```rust
/// TCP传输层
pub struct TcpTransport {
    listener: Option<TcpListener>,
    local_addr: SocketAddr,
}

impl TcpTransport {
    pub async fn new(port: u16) -> Result<Self, NetworkError> {
        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
        let listener = TcpListener::bind(addr).await?;
        
        Ok(Self {
            listener: Some(listener),
            local_addr: addr,
        })
    }
    
    /// 接受入站连接
    pub async fn accept(&mut self) -> Result<(TcpStream, SocketAddr), NetworkError> {
        if let Some(ref listener) = self.listener {
            let (stream, addr) = listener.accept().await?;
            
            // 配置TCP选项
            Self::configure_tcp_stream(&stream)?;
            
            Ok((stream, addr))
        } else {
            Err(NetworkError::NotListening)
        }
    }
    
    /// 建立出站连接
    pub async fn connect(&self, addr: SocketAddr) -> Result<TcpStream, NetworkError> {
        let stream = TcpStream::connect(addr).await?;
        Self::configure_tcp_stream(&stream)?;
        Ok(stream)
    }
    
    fn configure_tcp_stream(stream: &TcpStream) -> Result<(), NetworkError> {
        // 启用TCP_NODELAY（禁用Nagle算法）
        stream.set_nodelay(true)?;
        
        // 设置keepalive
        let keepalive = socket2::TcpKeepalive::new()
            .with_time(Duration::from_secs(60))
            .with_interval(Duration::from_secs(10));
        
        let socket = socket2::Socket::from(stream.as_ref());
        socket.set_tcp_keepalive(&keepalive)?;
        
        Ok(())
    }
}
```

### 2.2 QUIC支持

```rust
/// QUIC传输层（基于Quinn）
pub struct QuicTransport {
    endpoint: quinn::Endpoint,
}

impl QuicTransport {
    pub async fn new(port: u16) -> Result<Self, NetworkError> {
        let server_config = Self::configure_server()?;
        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
        
        let endpoint = quinn::Endpoint::server(server_config, addr)?;
        
        Ok(Self { endpoint })
    }
    
    fn configure_server() -> Result<quinn::ServerConfig, NetworkError> {
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
        let cert_der = cert.serialize_der()?;
        let priv_key = cert.serialize_private_key_der();
        
        let cert_chain = vec![rustls::Certificate(cert_der)];
        let key = rustls::PrivateKey(priv_key);
        
        let mut server_config = quinn::ServerConfig::with_single_cert(cert_chain, key)?;
        
        // 配置传输参数
        let transport_config = Arc::get_mut(&mut server_config.transport)
            .unwrap();
        transport_config.max_concurrent_uni_streams(100u32.into());
        transport_config.max_concurrent_bidi_streams(100u32.into());
        
        Ok(server_config)
    }
    
    /// 接受连接
    pub async fn accept(&self) -> Result<quinn::Connection, NetworkError> {
        let connecting = self.endpoint.accept().await
            .ok_or(NetworkError::EndpointClosed)?;
        
        let connection = connecting.await?;
        Ok(connection)
    }
    
    /// 建立连接
    pub async fn connect(&self, addr: SocketAddr) -> Result<quinn::Connection, NetworkError> {
        let connection = self.endpoint.connect(addr, "localhost")?.await?;
        Ok(connection)
    }
}
```

### 2.3 WebSocket

```rust
use tokio_tungstenite::{accept_async, connect_async, WebSocketStream};
use tungstenite::Message;

/// WebSocket传输层
pub struct WebSocketTransport;

impl WebSocketTransport {
    /// 接受WebSocket连接
    pub async fn accept(stream: TcpStream) -> Result<WebSocketStream<TcpStream>, NetworkError> {
        let ws_stream = accept_async(stream).await?;
        Ok(ws_stream)
    }
    
    /// 建立WebSocket连接
    pub async fn connect(url: &str) -> Result<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, NetworkError> {
        let (ws_stream, _) = connect_async(url).await?;
        Ok(ws_stream)
    }
    
    /// 发送消息
    pub async fn send<S>(
        ws: &mut WebSocketStream<S>,
        data: Vec<u8>
    ) -> Result<(), NetworkError>
    where
        S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    {
        ws.send(Message::Binary(data)).await?;
        Ok(())
    }
    
    /// 接收消息
    pub async fn receive<S>(
        ws: &mut WebSocketStream<S>
    ) -> Result<Option<Vec<u8>>, NetworkError>
    where
        S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    {
        use futures::StreamExt;
        
        match ws.next().await {
            Some(Ok(Message::Binary(data))) => Ok(Some(data)),
            Some(Ok(Message::Close(_))) => Ok(None),
            Some(Err(e)) => Err(e.into()),
            _ => Ok(None),
        }
    }
}
```

## 3. 消息协议

### 3.1 协议格式

```rust
/// 消息协议
#[derive(Debug, Clone)]
pub struct Message {
    /// 消息头
    pub header: MessageHeader,
    /// 消息体
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct MessageHeader {
    /// 协议版本
    pub version: u8,
    /// 消息类型
    pub message_type: MessageType,
    /// 消息长度
    pub length: u32,
    /// 校验和
    pub checksum: u32,
    /// 时间戳
    pub timestamp: u64,
}

impl Message {
    /// 编码消息
    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        
        // 编码消息头
        buffer.push(self.header.version);
        buffer.push(self.header.message_type as u8);
        buffer.extend_from_slice(&self.header.length.to_be_bytes());
        buffer.extend_from_slice(&self.header.checksum.to_be_bytes());
        buffer.extend_from_slice(&self.header.timestamp.to_be_bytes());
        
        // 编码消息体
        buffer.extend_from_slice(&self.payload);
        
        buffer
    }
    
    /// 解码消息
    pub fn decode(data: &[u8]) -> Result<Self, NetworkError> {
        if data.len() < 18 {
            return Err(NetworkError::InvalidMessage);
        }
        
        let version = data[0];
        let message_type = MessageType::from_u8(data[1])?;
        let length = u32::from_be_bytes([data[2], data[3], data[4], data[5]]);
        let checksum = u32::from_be_bytes([data[6], data[7], data[8], data[9]]);
        let timestamp = u64::from_be_bytes([
            data[10], data[11], data[12], data[13],
            data[14], data[15], data[16], data[17],
        ]);
        
        let payload = data[18..].to_vec();
        
        // 验证校验和
        let calculated_checksum = Self::calculate_checksum(&payload);
        if checksum != calculated_checksum {
            return Err(NetworkError::ChecksumMismatch);
        }
        
        Ok(Self {
            header: MessageHeader {
                version,
                message_type,
                length,
                checksum,
                timestamp,
            },
            payload,
        })
    }
    
    fn calculate_checksum(data: &[u8]) -> u32 {
        use crc32fast::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize()
    }
}
```

### 3.2 消息类型

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    // 连接相关
    Handshake = 0x01,
    HandshakeAck = 0x02,
    Ping = 0x03,
    Pong = 0x04,
    Disconnect = 0x05,
    
    // 节点发现
    FindNode = 0x10,
    Neighbors = 0x11,
    
    // 区块链数据
    GetBlockHeaders = 0x20,
    BlockHeaders = 0x21,
    GetBlockBodies = 0x22,
    BlockBodies = 0x23,
    NewBlock = 0x24,
    NewBlockHashes = 0x25,
    
    // 交易
    Transactions = 0x30,
    GetPooledTransactions = 0x31,
    PooledTransactions = 0x32,
    
    // 状态同步
    GetNodeData = 0x40,
    NodeData = 0x41,
    GetReceipts = 0x42,
    Receipts = 0x43,
}

impl MessageType {
    pub fn from_u8(value: u8) -> Result<Self, NetworkError> {
        match value {
            0x01 => Ok(MessageType::Handshake),
            0x02 => Ok(MessageType::HandshakeAck),
            0x03 => Ok(MessageType::Ping),
            0x04 => Ok(MessageType::Pong),
            0x05 => Ok(MessageType::Disconnect),
            0x10 => Ok(MessageType::FindNode),
            0x11 => Ok(MessageType::Neighbors),
            0x20 => Ok(MessageType::GetBlockHeaders),
            0x21 => Ok(MessageType::BlockHeaders),
            0x22 => Ok(MessageType::GetBlockBodies),
            0x23 => Ok(MessageType::BlockBodies),
            0x24 => Ok(MessageType::NewBlock),
            0x25 => Ok(MessageType::NewBlockHashes),
            0x30 => Ok(MessageType::Transactions),
            0x31 => Ok(MessageType::GetPooledTransactions),
            0x32 => Ok(MessageType::PooledTransactions),
            0x40 => Ok(MessageType::GetNodeData),
            0x41 => Ok(MessageType::NodeData),
            0x42 => Ok(MessageType::GetReceipts),
            0x43 => Ok(MessageType::Receipts),
            _ => Err(NetworkError::UnknownMessageType(value)),
        }
    }
}
```

### 3.3 序列化

```rust
use serde::{Serialize, Deserialize};

/// 消息序列化trait
pub trait MessageCodec: Sized {
    fn encode(&self) -> Result<Vec<u8>, NetworkError>;
    fn decode(data: &[u8]) -> Result<Self, NetworkError>;
}

/// RLP编码器
pub struct RlpCodec;

impl RlpCodec {
    pub fn encode<T: rlp::Encodable>(value: &T) -> Vec<u8> {
        rlp::encode(value).to_vec()
    }
    
    pub fn decode<T: rlp::Decodable>(data: &[u8]) -> Result<T, NetworkError> {
        rlp::decode(data).map_err(|e| NetworkError::DecodingError(e.to_string()))
    }
}

/// Protocol Buffers编码器
pub struct ProtobufCodec;

impl ProtobufCodec {
    pub fn encode<T: prost::Message>(value: &T) -> Vec<u8> {
        let mut buf = Vec::new();
        value.encode(&mut buf).unwrap();
        buf
    }
    
    pub fn decode<T: prost::Message + Default>(data: &[u8]) -> Result<T, NetworkError> {
        T::decode(data).map_err(|e| NetworkError::DecodingError(e.to_string()))
    }
}

/// JSON编码器
pub struct JsonCodec;

impl JsonCodec {
    pub fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, NetworkError> {
        serde_json::to_vec(value)
            .map_err(|e| NetworkError::EncodingError(e.to_string()))
    }
    
    pub fn decode<T: for<'de> Deserialize<'de>>(data: &[u8]) -> Result<T, NetworkError> {
        serde_json::from_slice(data)
            .map_err(|e| NetworkError::DecodingError(e.to_string()))
    }
}
```

## 4. 同步机制

### 4.1 区块同步

```rust
/// 区块同步器
pub struct BlockSynchronizer {
    /// 当前高度
    current_height: Arc<RwLock<u64>>,
    /// 目标高度
    target_height: Arc<RwLock<u64>>,
    /// 同步状态
    sync_state: Arc<RwLock<SyncState>>,
}

#[derive(Debug, Clone)]
pub enum SyncState {
    Idle,
    Syncing {
        from: u64,
        to: u64,
        progress: f64,
    },
    Completed,
    Failed(String),
}

impl BlockSynchronizer {
    pub fn new(current_height: u64) -> Self {
        Self {
            current_height: Arc::new(RwLock::new(current_height)),
            target_height: Arc::new(RwLock::new(current_height)),
            sync_state: Arc::new(RwLock::new(SyncState::Idle)),
        }
    }
    
    /// 开始同步
    pub async fn start_sync(&self, peer: &P2PNode) -> Result<(), NetworkError> {
        // 1. 获取对等节点的最新高度
        let peer_height = self.get_peer_height(peer).await?;
        
        let mut current = self.current_height.write().await;
        let mut target = self.target_height.write().await;
        *target = peer_height;
        
        if peer_height <= *current {
            println!("已是最新状态");
            return Ok(());
        }
        
        // 2. 更新同步状态
        let mut state = self.sync_state.write().await;
        *state = SyncState::Syncing {
            from: *current,
            to: peer_height,
            progress: 0.0,
        };
        
        drop(state);
        drop(target);
        
        // 3. 分批同步区块
        const BATCH_SIZE: u64 = 100;
        while *current < peer_height {
            let end = (*current + BATCH_SIZE).min(peer_height);
            
            match self.sync_block_range(peer, *current, end).await {
                Ok(blocks) => {
                    // 处理区块
                    for block in blocks {
                        // 验证和导入区块
                        println!("导入区块 #{}", block.number);
                    }
                    *current = end;
                    
                    // 更新进度
                    let progress = (*current - *self.current_height.read().await) as f64 
                        / (peer_height - *self.current_height.read().await) as f64;
                    
                    let mut state = self.sync_state.write().await;
                    if let SyncState::Syncing { from, to, .. } = *state {
                        *state = SyncState::Syncing { from, to, progress };
                    }
                }
                Err(e) => {
                    let mut state = self.sync_state.write().await;
                    *state = SyncState::Failed(e.to_string());
                    return Err(e);
                }
            }
        }
        
        // 4. 完成同步
        let mut state = self.sync_state.write().await;
        *state = SyncState::Completed;
        
        Ok(())
    }
    
    async fn get_peer_height(&self, peer: &P2PNode) -> Result<u64, NetworkError> {
        // 向对等节点请求最新高度
        Ok(10000) // 示例值
    }
    
    async fn sync_block_range(
        &self,
        peer: &P2PNode,
        from: u64,
        to: u64
    ) -> Result<Vec<Block>, NetworkError> {
        // 请求指定范围的区块
        Ok(vec![]) // 示例
    }
}
```

### 4.2 状态同步

```rust
/// 状态同步器
pub struct StateSynchronizer {
    /// Merkle Patricia Trie根
    root: Hash,
    /// 已同步的节点
    synced_nodes: Arc<RwLock<HashMap<Hash, Vec<u8>>>>,
}

impl StateSynchronizer {
    pub fn new(root: Hash) -> Self {
        Self {
            root,
            synced_nodes: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// 同步状态树
    pub async fn sync_state(&self, peer: &P2PNode) -> Result<(), NetworkError> {
        // 使用广度优先搜索遍历状态树
        let mut queue = vec![self.root];
        
        while let Some(node_hash) = queue.pop() {
            // 检查是否已同步
            if self.synced_nodes.read().await.contains_key(&node_hash) {
                continue;
            }
            
            // 从对等节点获取节点数据
            let node_data = self.request_node_data(peer, &node_hash).await?;
            
            // 解析节点
            let node = self.parse_node(&node_data)?;
            
            // 添加子节点到队列
            queue.extend(node.children());
            
            // 保存节点
            self.synced_nodes.write().await.insert(node_hash, node_data);
        }
        
        Ok(())
    }
    
    async fn request_node_data(
        &self,
        peer: &P2PNode,
        hash: &Hash
    ) -> Result<Vec<u8>, NetworkError> {
        // 发送GetNodeData请求
        Ok(vec![]) // 示例
    }
    
    fn parse_node(&self, data: &[u8]) -> Result<TrieNode, NetworkError> {
        // 解析MPT节点
        Ok(TrieNode::default())
    }
}

#[derive(Debug, Default)]
struct TrieNode {
    // 简化实现
}

impl TrieNode {
    fn children(&self) -> Vec<Hash> {
        vec![] // 示例
    }
}
```

### 4.3 快照同步

```rust
/// 快照同步
pub struct SnapshotSync {
    /// 快照元数据
    metadata: SnapshotMetadata,
}

#[derive(Debug)]
pub struct SnapshotMetadata {
    /// 快照高度
    pub block_number: u64,
    /// 状态根
    pub state_root: Hash,
    /// 快照大小
    pub size: u64,
    /// 快照分片
    pub chunks: Vec<ChunkInfo>,
}

#[derive(Debug)]
pub struct ChunkInfo {
    pub index: u32,
    pub hash: Hash,
    pub size: u32,
}

impl SnapshotSync {
    /// 下载快照
    pub async fn download_snapshot(
        &self,
        peer: &P2PNode
    ) -> Result<(), NetworkError> {
        println!("开始下载快照...");
        
        for chunk in &self.metadata.chunks {
            match self.download_chunk(peer, chunk).await {
                Ok(data) => {
                    // 验证分片
                    if !self.verify_chunk(chunk, &data) {
                        return Err(NetworkError::InvalidChunk);
                    }
                    
                    // 保存分片
                    self.save_chunk(chunk.index, data)?;
                }
                Err(e) => {
                    eprintln!("下载分片 {} 失败: {}", chunk.index, e);
                    return Err(e);
                }
            }
        }
        
        // 重建状态
        self.rebuild_state()?;
        
        Ok(())
    }
    
    async fn download_chunk(
        &self,
        peer: &P2PNode,
        chunk: &ChunkInfo
    ) -> Result<Vec<u8>, NetworkError> {
        // 下载分片
        Ok(vec![]) // 示例
    }
    
    fn verify_chunk(&self, chunk: &ChunkInfo, data: &[u8]) -> bool {
        use sha3::{Digest, Keccak256};
        let hash = Keccak256::digest(data);
        hash.as_slice() == chunk.hash.as_bytes()
    }
    
    fn save_chunk(&self, index: u32, data: Vec<u8>) -> Result<(), NetworkError> {
        // 保存到磁盘
        Ok(())
    }
    
    fn rebuild_state(&self) -> Result<(), NetworkError> {
        // 从快照重建状态
        Ok(())
    }
}
```

## 5. 广播与传播

### 5.1 Gossip协议

```rust
/// Gossip协议实现
pub struct GossipProtocol {
    /// 已知消息缓存
    known_messages: Arc<RwLock<LruCache<Hash, ()>>>,
    /// 传播配置
    config: GossipConfig,
}

#[derive(Debug, Clone)]
pub struct GossipConfig {
    /// 传播到多少个对等节点
    pub fanout: usize,
    /// 消息TTL
    pub ttl: u32,
    /// 缓存大小
    pub cache_size: usize,
}

impl GossipProtocol {
    pub fn new(config: GossipConfig) -> Self {
        Self {
            known_messages: Arc::new(RwLock::new(
                LruCache::new(config.cache_size.try_into().unwrap())
            )),
            config,
        }
    }
    
    /// 广播消息
    pub async fn broadcast(
        &self,
        node: &P2PNode,
        message: Message
    ) -> Result<(), NetworkError> {
        let msg_hash = self.hash_message(&message);
        
        // 检查是否已广播
        if self.known_messages.write().await.get(&msg_hash).is_some() {
            return Ok(()); // 已处理过
        }
        
        // 标记为已知
        self.known_messages.write().await.put(msg_hash, ());
        
        // 选择对等节点
        let peers = self.select_peers(node, self.config.fanout).await?;
        
        // 发送到选中的对等节点
        for peer_id in peers {
            if let Err(e) = self.send_to_peer(node, &peer_id, &message).await {
                eprintln!("发送到节点 {:?} 失败: {}", peer_id, e);
            }
        }
        
        Ok(())
    }
    
    async fn select_peers(
        &self,
        node: &P2PNode,
        count: usize
    ) -> Result<Vec<NodeId>, NetworkError> {
        use rand::seq::SliceRandom;
        
        let peers = node.peers.read().await;
        let peer_ids: Vec<NodeId> = peers.keys().cloned().collect();
        
        let mut rng = rand::thread_rng();
        let selected: Vec<NodeId> = peer_ids
            .choose_multiple(&mut rng, count)
            .cloned()
            .collect();
        
        Ok(selected)
    }
    
    async fn send_to_peer(
        &self,
        node: &P2PNode,
        peer_id: &NodeId,
        message: &Message
    ) -> Result<(), NetworkError> {
        // 发送消息到指定对等节点
        Ok(())
    }
    
    fn hash_message(&self, message: &Message) -> Hash {
        use sha3::{Digest, Keccak256};
        let encoded = message.encode();
        let hash = Keccak256::digest(&encoded);
        Hash::from_slice(hash.as_slice())
    }
}
```

### 5.2 交易传播

```rust
/// 交易传播器
pub struct TransactionPropagator {
    gossip: Arc<GossipProtocol>,
    /// 交易池
    tx_pool: Arc<RwLock<TransactionPool>>,
}

impl TransactionPropagator {
    pub fn new(gossip: Arc<GossipProtocol>) -> Self {
        Self {
            gossip,
            tx_pool: Arc::new(RwLock::new(TransactionPool::new())),
        }
    }
    
    /// 传播交易
    pub async fn propagate_transaction(
        &self,
        node: &P2PNode,
        tx: Transaction
    ) -> Result<(), NetworkError> {
        // 添加到交易池
        self.tx_pool.write().await.add(tx.clone())?;
        
        // 创建交易消息
        let message = Message {
            header: MessageHeader {
                version: 1,
                message_type: MessageType::Transactions,
                length: 0,
                checksum: 0,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            payload: Self::encode_transactions(vec![tx]),
        };
        
        // 通过Gossip广播
        self.gossip.broadcast(node, message).await?;
        
        Ok(())
    }
    
    fn encode_transactions(txs: Vec<Transaction>) -> Vec<u8> {
        // 编码交易列表
        vec![] // 示例
    }
}

#[derive(Debug)]
struct TransactionPool {
    transactions: HashMap<Hash, Transaction>,
}

impl TransactionPool {
    fn new() -> Self {
        Self {
            transactions: HashMap::new(),
        }
    }
    
    fn add(&mut self, tx: Transaction) -> Result<(), NetworkError> {
        let tx_hash = tx.hash();
        self.transactions.insert(tx_hash, tx);
        Ok(())
    }
}
```

### 5.3 区块传播

```rust
/// 区块传播器
pub struct BlockPropagator {
    gossip: Arc<GossipProtocol>,
}

impl BlockPropagator {
    pub fn new(gossip: Arc<GossipProtocol>) -> Self {
        Self { gossip }
    }
    
    /// 传播新区块
    pub async fn propagate_block(
        &self,
        node: &P2PNode,
        block: Block
    ) -> Result<(), NetworkError> {
        // 先广播区块哈希（轻量级）
        self.broadcast_block_hash(node, &block).await?;
        
        // 对于直接连接的节点，发送完整区块
        self.send_full_block_to_connected(node, &block).await?;
        
        Ok(())
    }
    
    async fn broadcast_block_hash(
        &self,
        node: &P2PNode,
        block: &Block
    ) -> Result<(), NetworkError> {
        let message = Message {
            header: MessageHeader {
                version: 1,
                message_type: MessageType::NewBlockHashes,
                length: 0,
                checksum: 0,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            payload: Self::encode_block_hash(block),
        };
        
        self.gossip.broadcast(node, message).await?;
        Ok(())
    }
    
    async fn send_full_block_to_connected(
        &self,
        node: &P2PNode,
        block: &Block
    ) -> Result<(), NetworkError> {
        // 向直接连接的节点发送完整区块
        Ok(())
    }
    
    fn encode_block_hash(block: &Block) -> Vec<u8> {
        // 编码区块哈希和号码
        let mut data = Vec::new();
        data.extend_from_slice(block.hash().as_bytes());
        data.extend_from_slice(&block.number.to_be_bytes());
        data
    }
}
```

## 6. NAT穿透

### 6.1 UPnP

```rust
/// UPnP NAT穿透
pub struct UpnpNatTraversal {
    gateway: Option<igd::Gateway>,
}

impl UpnpNatTraversal {
    pub async fn new() -> Result<Self, NetworkError> {
        let gateway = igd::search_gateway(Default::default())?;
        
        Ok(Self {
            gateway: Some(gateway),
        })
    }
    
    /// 添加端口映射
    pub async fn add_port_mapping(
        &self,
        internal_port: u16,
        external_port: u16,
        protocol: Protocol,
        description: &str
    ) -> Result<(), NetworkError> {
        if let Some(ref gateway) = self.gateway {
            let protocol_str = match protocol {
                Protocol::TCP => igd::PortMappingProtocol::TCP,
                Protocol::UDP => igd::PortMappingProtocol::UDP,
            };
            
            gateway.add_port(
                protocol_str,
                external_port,
                internal_port,
                0, // 租期（0表示永久）
                description,
            )?;
            
            println!("成功添加端口映射: {}:{} -> {}", 
                protocol_str, external_port, internal_port);
        }
        
        Ok(())
    }
    
    /// 获取外部IP
    pub async fn get_external_ip(&self) -> Result<std::net::IpAddr, NetworkError> {
        if let Some(ref gateway) = self.gateway {
            let ip = gateway.get_external_ip()?;
            Ok(ip)
        } else {
            Err(NetworkError::NoGateway)
        }
    }
    
    /// 删除端口映射
    pub async fn remove_port_mapping(
        &self,
        external_port: u16,
        protocol: Protocol
    ) -> Result<(), NetworkError> {
        if let Some(ref gateway) = self.gateway {
            let protocol_str = match protocol {
                Protocol::TCP => igd::PortMappingProtocol::TCP,
                Protocol::UDP => igd::PortMappingProtocol::UDP,
            };
            
            gateway.remove_port(protocol_str, external_port)?;
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Protocol {
    TCP,
    UDP,
}
```

### 6.2 STUN/TURN

```rust
/// STUN客户端
pub struct StunClient {
    server: String,
}

impl StunClient {
    pub fn new(server: String) -> Self {
        Self { server }
    }
    
    /// 获取公网地址
    pub async fn get_mapped_address(&self) -> Result<SocketAddr, NetworkError> {
        // 实现STUN协议获取公网地址
        // 这里简化处理
        Ok("0.0.0.0:0".parse().unwrap())
    }
}

/// TURN客户端
pub struct TurnClient {
    server: String,
    username: String,
    password: String,
}

impl TurnClient {
    pub fn new(server: String, username: String, password: String) -> Self {
        Self {
            server,
            username,
            password,
        }
    }
    
    /// 建立TURN会话
    pub async fn create_allocation(&self) -> Result<TurnAllocation, NetworkError> {
        // 实现TURN协议
        Ok(TurnAllocation {
            relay_addr: "0.0.0.0:0".parse().unwrap(),
        })
    }
}

#[derive(Debug)]
pub struct TurnAllocation {
    pub relay_addr: SocketAddr,
}
```

### 6.3 Hole Punching

```rust
/// UDP Hole Punching
pub struct HolePunching {
    local_addr: SocketAddr,
}

impl HolePunching {
    pub fn new(local_addr: SocketAddr) -> Self {
        Self { local_addr }
    }
    
    /// 执行hole punching
    pub async fn punch_hole(
        &self,
        peer_addr: SocketAddr
    ) -> Result<UdpSocket, NetworkError> {
        let socket = UdpSocket::bind(self.local_addr).await?;
        
        // 1. 向对等节点发送数据包
        socket.send_to(b"punch", peer_addr).await?;
        
        // 2. 等待响应
        let mut buf = [0u8; 1024];
        let (len, _) = socket.recv_from(&mut buf).await?;
        
        if len > 0 {
            println!("Hole punching成功");
            Ok(socket)
        } else {
            Err(NetworkError::HolePunchingFailed)
        }
    }
}
```

## 7. 网络安全

### 7.1 TLS加密

```rust
use tokio_rustls::{TlsAcceptor, TlsConnector};
use rustls::{ServerConfig, ClientConfig};

/// TLS加密传输
pub struct TlsTransport {
    acceptor: TlsAcceptor,
    connector: TlsConnector,
}

impl TlsTransport {
    pub fn new(server_config: ServerConfig, client_config: ClientConfig) -> Self {
        Self {
            acceptor: TlsAcceptor::from(Arc::new(server_config)),
            connector: TlsConnector::from(Arc::new(client_config)),
        }
    }
    
    /// 接受TLS连接
    pub async fn accept(&self, stream: TcpStream) -> Result<TlsStream, NetworkError> {
        let tls_stream = self.acceptor.accept(stream).await?;
        Ok(tls_stream)
    }
    
    /// 建立TLS连接
    pub async fn connect(
        &self,
        stream: TcpStream,
        domain: &str
    ) -> Result<TlsStream, NetworkError> {
        let domain = rustls::ServerName::try_from(domain)?;
        let tls_stream = self.connector.connect(domain, stream).await?;
        Ok(tls_stream)
    }
}

type TlsStream = tokio_rustls::client::TlsStream<TcpStream>;
```

### 7.2 节点认证

```rust
/// 节点认证
pub struct NodeAuthentication {
    /// 本地私钥
    private_key: secp256k1::SecretKey,
    /// 已认证节点
    authenticated_nodes: Arc<RwLock<HashMap<NodeId, PublicKey>>>,
}

impl NodeAuthentication {
    pub fn new(private_key: secp256k1::SecretKey) -> Self {
        Self {
            private_key,
            authenticated_nodes: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// 生成握手消息
    pub fn create_handshake(&self) -> HandshakeMessage {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // 签名
        let message = format!("handshake:{}", timestamp);
        let signature = self.sign(message.as_bytes());
        
        HandshakeMessage {
            node_id: self.get_node_id(),
            timestamp,
            signature,
        }
    }
    
    /// 验证握手消息
    pub fn verify_handshake(&self, handshake: &HandshakeMessage) -> Result<bool, NetworkError> {
        let message = format!("handshake:{}", handshake.timestamp);
        
        // 验证签名
        let public_key = self.recover_public_key(message.as_bytes(), &handshake.signature)?;
        
        // 验证节点ID
        let expected_node_id = Self::node_id_from_public_key(&public_key);
        if expected_node_id != handshake.node_id {
            return Ok(false);
        }
        
        // 保存认证信息
        self.authenticated_nodes.write().await
            .insert(handshake.node_id.clone(), public_key);
        
        Ok(true)
    }
    
    fn sign(&self, message: &[u8]) -> Vec<u8> {
        use secp256k1::{Secp256k1, Message as Secp256k1Message};
        use sha3::{Digest, Keccak256};
        
        let secp = Secp256k1::new();
        let msg_hash = Keccak256::digest(message);
        let msg = Secp256k1Message::from_slice(&msg_hash).unwrap();
        
        secp.sign_ecdsa(&msg, &self.private_key).serialize_compact().to_vec()
    }
    
    fn recover_public_key(&self, message: &[u8], signature: &[u8]) -> Result<PublicKey, NetworkError> {
        // 从签名恢复公钥
        Ok(PublicKey::default()) // 示例
    }
    
    fn get_node_id(&self) -> NodeId {
        // 从私钥派生节点ID
        NodeId::random()
    }
    
    fn node_id_from_public_key(public_key: &PublicKey) -> NodeId {
        // 从公钥派生节点ID
        NodeId::random() // 示例
    }
}

#[derive(Debug)]
pub struct HandshakeMessage {
    pub node_id: NodeId,
    pub timestamp: u64,
    pub signature: Vec<u8>,
}

#[derive(Debug, Default, Clone)]
pub struct PublicKey {
    // 简化实现
}
```

### 7.3 DDoS防护

```rust
/// DDoS防护
pub struct DDoSProtection {
    /// 速率限制器
    rate_limiter: Arc<RwLock<HashMap<SocketAddr, RateLimiter>>>,
    /// 黑名单
    blacklist: Arc<RwLock<HashSet<SocketAddr>>>,
    /// 配置
    config: DDoSConfig,
}

#[derive(Debug, Clone)]
pub struct DDoSConfig {
    /// 每秒最大请求数
    pub max_requests_per_second: u32,
    /// 突发请求数
    pub burst_size: u32,
    /// 封禁时长
    pub ban_duration: Duration,
}

struct RateLimiter {
    tokens: f64,
    last_update: SystemTime,
    max_tokens: f64,
    refill_rate: f64,
}

impl DDoSProtection {
    pub fn new(config: DDoSConfig) -> Self {
        Self {
            rate_limiter: Arc::new(RwLock::new(HashMap::new())),
            blacklist: Arc::new(RwLock::new(HashSet::new())),
            config,
        }
    }
    
    /// 检查是否允许请求
    pub async fn allow_request(&self, addr: SocketAddr) -> bool {
        // 检查黑名单
        if self.blacklist.read().await.contains(&addr) {
            return false;
        }
        
        // 速率限制
        let mut limiters = self.rate_limiter.write().await;
        let limiter = limiters.entry(addr).or_insert_with(|| RateLimiter {
            tokens: self.config.burst_size as f64,
            last_update: SystemTime::now(),
            max_tokens: self.config.burst_size as f64,
            refill_rate: self.config.max_requests_per_second as f64,
        });
        
        limiter.refill();
        
        if limiter.tokens >= 1.0 {
            limiter.tokens -= 1.0;
            true
        } else {
            // 超出限制，加入黑名单
            drop(limiters);
            self.blacklist.write().await.insert(addr);
            println!("地址 {} 因超出速率限制被封禁", addr);
            false
        }
    }
}

impl RateLimiter {
    fn refill(&mut self) {
        let now = SystemTime::now();
        let elapsed = now.duration_since(self.last_update).unwrap().as_secs_f64();
        
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.max_tokens);
        self.last_update = now;
    }
}
```

## 8. 性能优化

### 8.1 连接池

已在连接管理部分实现。

### 8.2 消息压缩

```rust
use flate2::Compression;
use flate2::write::{ZlibEncoder, ZlibDecoder};
use std::io::Write;

/// 消息压缩
pub struct MessageCompression;

impl MessageCompression {
    /// 压缩数据
    pub fn compress(data: &[u8]) -> Result<Vec<u8>, NetworkError> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)?;
        Ok(encoder.finish()?)
    }
    
    /// 解压数据
    pub fn decompress(data: &[u8]) -> Result<Vec<u8>, NetworkError> {
        let mut decoder = ZlibDecoder::new(Vec::new());
        decoder.write_all(data)?;
        Ok(decoder.finish()?)
    }
    
    /// 判断是否值得压缩
    pub fn should_compress(data: &[u8]) -> bool {
        data.len() > 1024 // 大于1KB才压缩
    }
}
```

### 8.3 流量控制

```rust
/// 流量控制
pub struct FlowControl {
    /// 发送窗口
    send_window: Arc<RwLock<u32>>,
    /// 接收窗口
    recv_window: Arc<RwLock<u32>>,
    /// 配置
    config: FlowControlConfig,
}

#[derive(Debug, Clone)]
pub struct FlowControlConfig {
    /// 初始窗口大小
    pub initial_window_size: u32,
    /// 最大窗口大小
    pub max_window_size: u32,
}

impl FlowControl {
    pub fn new(config: FlowControlConfig) -> Self {
        Self {
            send_window: Arc::new(RwLock::new(config.initial_window_size)),
            recv_window: Arc::new(RwLock::new(config.initial_window_size)),
            config,
        }
    }
    
    /// 检查是否可以发送
    pub async fn can_send(&self, size: u32) -> bool {
        *self.send_window.read().await >= size
    }
    
    /// 消耗发送窗口
    pub async fn consume_send_window(&self, size: u32) {
        let mut window = self.send_window.write().await;
        *window = window.saturating_sub(size);
    }
    
    /// 更新发送窗口
    pub async fn update_send_window(&self, size: u32) {
        let mut window = self.send_window.write().await;
        *window = (*window + size).min(self.config.max_window_size);
    }
}
```

## 9. 监控与诊断

### 9.1 网络指标

```rust
/// 网络指标收集
pub struct NetworkMetrics {
    /// 发送字节数
    pub bytes_sent: AtomicU64,
    /// 接收字节数
    pub bytes_received: AtomicU64,
    /// 发送消息数
    pub messages_sent: AtomicU64,
    /// 接收消息数
    pub messages_received: AtomicU64,
    /// 活跃连接数
    pub active_connections: AtomicUsize,
}

impl NetworkMetrics {
    pub fn new() -> Self {
        Self {
            bytes_sent: AtomicU64::new(0),
            bytes_received: AtomicU64::new(0),
            messages_sent: AtomicU64::new(0),
            messages_received: AtomicU64::new(0),
            active_connections: AtomicUsize::new(0),
        }
    }
    
    pub fn record_send(&self, bytes: u64) {
        self.bytes_sent.fetch_add(bytes, Ordering::Relaxed);
        self.messages_sent.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn record_receive(&self, bytes: u64) {
        self.bytes_received.fetch_add(bytes, Ordering::Relaxed);
        self.messages_received.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn get_stats(&self) -> NetworkStats {
        NetworkStats {
            bytes_sent: self.bytes_sent.load(Ordering::Relaxed),
            bytes_received: self.bytes_received.load(Ordering::Relaxed),
            messages_sent: self.messages_sent.load(Ordering::Relaxed),
            messages_received: self.messages_received.load(Ordering::Relaxed),
            active_connections: self.active_connections.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug)]
pub struct NetworkStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub active_connections: usize,
}
```

### 9.2 连接状态

已在连接管理部分实现。

### 9.3 故障诊断

已在问题诊断与解决文档中详细说明。

## 10. 总结

本文档全面介绍了区块链网络层的实现，包括：

1. **P2P网络架构**：网络拓扑、节点发现（Kademlia DHT）、连接管理
2. **传输层协议**：TCP/IP、QUIC、WebSocket
3. **消息协议**：协议格式、消息类型、序列化（RLP/Protobuf/JSON）
4. **同步机制**：区块同步、状态同步、快照同步
5. **广播与传播**：Gossip协议、交易传播、区块传播
6. **NAT穿透**：UPnP、STUN/TURN、Hole Punching
7. **网络安全**：TLS加密、节点认证、DDoS防护
8. **性能优化**：连接池、消息压缩、流量控制
9. **监控与诊断**：网络指标、连接状态、故障诊断

**关键要点**：

- 使用Kademlia DHT实现高效节点发现
- 支持多种传输协议（TCP、QUIC、WebSocket）
- Gossip协议实现高效消息传播
- 全面的NAT穿透支持
- 多层安全防护机制
- 完善的性能优化策略

---

**文档版本**: v1.0.0  
**最后更新**: 2025年10月17日  
**作者**: Rust区块链技术团队  
**相关文档**:

- [12_ARCHITECTURE_DESIGN.md](./12_ARCHITECTURE_DESIGN.md) - 架构设计
- [11_PERFORMANCE_OPTIMIZATION.md](./11_PERFORMANCE_OPTIMIZATION.md) - 性能优化
- [19_SECURITY_BEST_PRACTICES.md](./19_SECURITY_BEST_PRACTICES.md) - 安全最佳实践
