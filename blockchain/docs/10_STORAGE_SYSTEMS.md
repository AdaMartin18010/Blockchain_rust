# 存储系统设计

## 📋 目录

- [1. 存储架构概览](#1-存储架构概览)
  - [1.1 存储层次结构](#11-存储层次结构)
  - [1.2 存储类型](#12-存储类型)
  - [1.3 数据组织](#13-数据组织)
- [2. 区块存储](#2-区块存储)
  - [2.1 区块数据结构](#21-区块数据结构)
  - [2.2 区块索引](#22-区块索引)
  - [2.3 区块检索](#23-区块检索)
- [3. 状态存储](#3-状态存储)
  - [3.1 Merkle Patricia Trie](#31-merkle-patricia-trie)
  - [3.2 状态快照](#32-状态快照)
  - [3.3 状态剪枝](#33-状态剪枝)
- [4. 交易存储](#4-交易存储)
  - [4.1 交易池设计](#41-交易池设计)
  - [4.2 交易索引](#42-交易索引)
  - [4.3 历史交易查询](#43-历史交易查询)
- [5. 存储引擎](#5-存储引擎)
  - [5.1 RocksDB集成](#51-rocksdb集成)
  - [5.2 LMDB集成](#52-lmdb集成)
  - [5.3 自定义存储引擎](#53-自定义存储引擎)
- [6. 缓存策略](#6-缓存策略)
  - [6.1 LRU缓存](#61-lru缓存)
  - [6.2 多级缓存](#62-多级缓存)
  - [6.3 预读优化](#63-预读优化)
- [7. 数据持久化](#7-数据持久化)
  - [7.1 WAL日志](#71-wal日志)
  - [7.2 检查点](#72-检查点)
  - [7.3 崩溃恢复](#73-崩溃恢复)
- [8. 存储优化](#8-存储优化)
  - [8.1 压缩策略](#81-压缩策略)
  - [8.2 数据分片](#82-数据分片)
  - [8.3 归档与剪枝](#83-归档与剪枝)

## 1. 存储架构概览

### 1.1 存储层次结构

```rust
/// 存储层次结构
#[derive(Debug)]
pub struct StorageArchitecture {
    /// L1: 内存缓存层
    cache_layer: Arc<CacheLayer>,
    /// L2: 持久化存储层
    storage_layer: Arc<StorageLayer>,
    /// L3: 归档存储层
    archive_layer: Option<Arc<ArchiveLayer>>,
}

/// 缓存层（内存）
#[derive(Debug)]
pub struct CacheLayer {
    /// 区块缓存
    block_cache: Arc<RwLock<LruCache<Hash, Block>>>,
    /// 状态缓存
    state_cache: Arc<RwLock<LruCache<Hash, StateNode>>>,
    /// 交易缓存
    tx_cache: Arc<RwLock<LruCache<Hash, Transaction>>>,
}

/// 持久化存储层
#[derive(Debug)]
pub struct StorageLayer {
    /// 区块存储
    block_store: Arc<BlockStore>,
    /// 状态存储
    state_store: Arc<StateStore>,
    /// 交易存储
    tx_store: Arc<TransactionStore>,
}

/// 归档存储层
#[derive(Debug)]
pub struct ArchiveLayer {
    /// 历史区块归档
    archive_store: Arc<ArchiveStore>,
}

impl StorageArchitecture {
    /// 读取区块（多层查找）
    pub async fn get_block(&self, hash: &Hash) -> Result<Option<Block>, Error> {
        // 1. 尝试从缓存读取
        if let Some(block) = self.cache_layer.get_block(hash).await? {
            return Ok(Some(block));
        }
        
        // 2. 从持久化层读取
        if let Some(block) = self.storage_layer.get_block(hash).await? {
            // 写入缓存
            self.cache_layer.put_block(hash.clone(), block.clone()).await?;
            return Ok(Some(block));
        }
        
        // 3. 从归档层读取
        if let Some(archive) = &self.archive_layer {
            if let Some(block) = archive.get_block(hash).await? {
                return Ok(Some(block));
            }
        }
        
        Ok(None)
    }
    
    /// 写入区块
    pub async fn put_block(&self, hash: Hash, block: Block) -> Result<(), Error> {
        // 1. 写入缓存
        self.cache_layer.put_block(hash.clone(), block.clone()).await?;
        
        // 2. 持久化到存储层
        self.storage_layer.put_block(hash, block).await?;
        
        Ok(())
    }
}
```

### 1.2 存储类型

```rust
/// 存储类型
#[derive(Debug, Clone)]
pub enum StorageType {
    /// 全节点存储：存储所有历史数据
    FullNode {
        /// 是否启用归档模式
        archive: bool,
        /// 是否启用剪枝
        pruning: bool,
    },
    /// 轻节点存储：只存储区块头
    LightNode {
        /// 区块头数量限制
        header_limit: usize,
    },
    /// 验证节点存储：存储最近N个区块的状态
    ValidatorNode {
        /// 状态历史深度
        state_depth: u64,
    },
    /// 归档节点存储：存储所有历史状态
    ArchiveNode,
}

/// 存储配置
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// 存储类型
    pub storage_type: StorageType,
    /// 数据目录
    pub data_dir: PathBuf,
    /// 缓存大小
    pub cache_size: usize,
    /// 是否启用压缩
    pub compression: bool,
    /// 是否启用校验和
    pub checksum: bool,
}
```

### 1.3 数据组织

```rust
/// 数据组织结构
pub struct DataOrganization {
    /// 区块数据：按高度组织
    /// data/blocks/000000/000001.blk
    blocks_by_height: PathBuf,
    
    /// 区块索引：哈希 -> 文件位置
    /// data/indexes/block_hash.idx
    block_index: PathBuf,
    
    /// 状态数据：Merkle树节点
    /// data/state/nodes/
    state_nodes: PathBuf,
    
    /// 交易数据：按哈希组织
    /// data/transactions/
    transactions: PathBuf,
    
    /// 元数据
    /// data/metadata/
    metadata: PathBuf,
}

impl DataOrganization {
    /// 获取区块文件路径
    pub fn block_file_path(&self, height: u64) -> PathBuf {
        let folder = format!("{:06}", height / 1000);
        let filename = format!("{:06}.blk", height % 1000);
        self.blocks_by_height.join(folder).join(filename)
    }
    
    /// 获取区块索引路径
    pub fn block_index_path(&self) -> PathBuf {
        self.block_index.join("block_hash.idx")
    }
}
```

## 2. 区块存储

### 2.1 区块数据结构

```rust
use serde::{Serialize, Deserialize};

/// 区块存储项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredBlock {
    /// 区块数据
    pub block: Block,
    /// 区块高度
    pub height: u64,
    /// 区块大小（字节）
    pub size: u64,
    /// 存储时间戳
    pub stored_at: SystemTime,
    /// 确认数
    pub confirmations: u64,
}

/// 区块元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMetadata {
    /// 区块哈希
    pub hash: Hash,
    /// 区块高度
    pub height: u64,
    /// 文件偏移量
    pub file_offset: u64,
    /// 区块大小
    pub size: u32,
    /// 交易数量
    pub tx_count: u32,
}
```

### 2.2 区块索引

```rust
use rocksdb::{DB, Options};

/// 区块存储实现
#[derive(Debug)]
pub struct BlockStore {
    /// RocksDB实例
    db: Arc<DB>,
    /// 区块数据目录
    data_dir: PathBuf,
    /// 区块索引
    index: Arc<RwLock<HashMap<Hash, BlockMetadata>>>,
}

impl BlockStore {
    /// 创建区块存储
    pub fn new(data_dir: PathBuf) -> Result<Self, Error> {
        let db_path = data_dir.join("blocks_db");
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(rocksdb::DBCompressionType::Lz4);
        
        let db = DB::open(&opts, db_path)?;
        
        Ok(Self {
            db: Arc::new(db),
            data_dir,
            index: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// 存储区块
    pub async fn put_block(&self, hash: Hash, block: Block) -> Result<(), Error> {
        let height = block.header.height;
        
        // 1. 序列化区块
        let stored_block = StoredBlock {
            block: block.clone(),
            height,
            size: 0, // 计算实际大小
            stored_at: SystemTime::now(),
            confirmations: 0,
        };
        let data = bincode::serialize(&stored_block)?;
        
        // 2. 写入RocksDB
        self.db.put(hash.as_bytes(), &data)?;
        
        // 3. 更新索引
        let metadata = BlockMetadata {
            hash: hash.clone(),
            height,
            file_offset: 0,
            size: data.len() as u32,
            tx_count: block.transactions.len() as u32,
        };
        self.index.write().await.insert(hash.clone(), metadata);
        
        // 4. 创建高度索引
        self.db.put(
            format!("height:{}", height).as_bytes(),
            hash.as_bytes()
        )?;
        
        Ok(())
    }
    
    /// 读取区块
    pub async fn get_block(&self, hash: &Hash) -> Result<Option<Block>, Error> {
        match self.db.get(hash.as_bytes())? {
            Some(data) => {
                let stored_block: StoredBlock = bincode::deserialize(&data)?;
                Ok(Some(stored_block.block))
            },
            None => Ok(None),
        }
    }
    
    /// 根据高度获取区块
    pub async fn get_block_by_height(&self, height: u64) -> Result<Option<Block>, Error> {
        // 1. 从高度索引获取哈希
        let key = format!("height:{}", height);
        match self.db.get(key.as_bytes())? {
            Some(hash_bytes) => {
                let hash = Hash::from_bytes(&hash_bytes)?;
                self.get_block(&hash).await
            },
            None => Ok(None),
        }
    }
    
    /// 获取区块元数据
    pub async fn get_block_metadata(&self, hash: &Hash) -> Result<Option<BlockMetadata>, Error> {
        Ok(self.index.read().await.get(hash).cloned())
    }
    
    /// 批量读取区块
    pub async fn get_blocks(&self, hashes: &[Hash]) -> Result<Vec<Block>, Error> {
        let mut blocks = Vec::new();
        
        for hash in hashes {
            if let Some(block) = self.get_block(hash).await? {
                blocks.push(block);
            }
        }
        
        Ok(blocks)
    }
}
```

### 2.3 区块检索

```rust
/// 区块查询接口
impl BlockStore {
    /// 获取最新区块
    pub async fn get_latest_block(&self) -> Result<Option<Block>, Error> {
        // 从元数据中获取最新高度
        let latest_height_key = b"latest_height";
        match self.db.get(latest_height_key)? {
            Some(height_bytes) => {
                let height = u64::from_be_bytes(height_bytes.as_slice().try_into()?);
                self.get_block_by_height(height).await
            },
            None => Ok(None),
        }
    }
    
    /// 获取区块范围
    pub async fn get_block_range(
        &self,
        start_height: u64,
        end_height: u64
    ) -> Result<Vec<Block>, Error> {
        let mut blocks = Vec::new();
        
        for height in start_height..=end_height {
            if let Some(block) = self.get_block_by_height(height).await? {
                blocks.push(block);
            }
        }
        
        Ok(blocks)
    }
    
    /// 查询包含指定交易的区块
    pub async fn find_block_by_transaction(&self, tx_hash: &Hash) -> Result<Option<Block>, Error> {
        // 从交易索引查找
        let key = format!("tx:{}", tx_hash);
        match self.db.get(key.as_bytes())? {
            Some(block_hash_bytes) => {
                let block_hash = Hash::from_bytes(&block_hash_bytes)?;
                self.get_block(&block_hash).await
            },
            None => Ok(None),
        }
    }
}
```

## 3. 状态存储

### 3.1 Merkle Patricia Trie

```rust
/// Merkle Patricia Trie 节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MPTNode {
    /// 叶子节点
    Leaf {
        key: Vec<u8>,
        value: Vec<u8>,
    },
    /// 扩展节点
    Extension {
        prefix: Vec<u8>,
        child: Hash,
    },
    /// 分支节点（16个子节点 + 可选值）
    Branch {
        children: [Option<Hash>; 16],
        value: Option<Vec<u8>>,
    },
    /// 空节点
    Empty,
}

/// Merkle Patricia Trie 实现
#[derive(Debug)]
pub struct MerklePatriciaTrie {
    /// 存储引擎
    db: Arc<DB>,
    /// 根哈希
    root: Arc<RwLock<Hash>>,
}

impl MerklePatriciaTrie {
    /// 插入键值对
    pub async fn insert(&self, key: &[u8], value: Vec<u8>) -> Result<(), Error> {
        let root = *self.root.read().await;
        let new_root = self.insert_node(root, key, value, 0).await?;
        *self.root.write().await = new_root;
        Ok(())
    }
    
    /// 递归插入节点
    async fn insert_node(
        &self,
        node_hash: Hash,
        key: &[u8],
        value: Vec<u8>,
        depth: usize
    ) -> Result<Hash, Error> {
        // 读取节点
        let node = self.get_node(&node_hash).await?;
        
        match node {
            MPTNode::Empty => {
                // 创建新的叶子节点
                let leaf = MPTNode::Leaf {
                    key: key[depth..].to_vec(),
                    value,
                };
                self.put_node(&leaf).await
            },
            
            MPTNode::Leaf { key: leaf_key, value: leaf_value } => {
                // 找到共同前缀
                let common_prefix_len = Self::common_prefix_length(
                    &key[depth..],
                    &leaf_key
                );
                
                if common_prefix_len == leaf_key.len() && depth + common_prefix_len == key.len() {
                    // 键完全匹配，更新值
                    let new_leaf = MPTNode::Leaf {
                        key: leaf_key,
                        value,
                    };
                    self.put_node(&new_leaf).await
                } else {
                    // 需要分裂节点
                    self.split_leaf(
                        &leaf_key,
                        leaf_value,
                        &key[depth..],
                        value,
                        common_prefix_len
                    ).await
                }
            },
            
            MPTNode::Extension { prefix, child } => {
                let common_prefix_len = Self::common_prefix_length(
                    &key[depth..],
                    &prefix
                );
                
                if common_prefix_len == prefix.len() {
                    // 沿着扩展节点继续
                    let new_child = self.insert_node(
                        child,
                        key,
                        value,
                        depth + prefix.len()
                    ).await?;
                    
                    let ext = MPTNode::Extension {
                        prefix,
                        child: new_child,
                    };
                    self.put_node(&ext).await
                } else {
                    // 分裂扩展节点
                    self.split_extension(
                        &prefix,
                        child,
                        &key[depth..],
                        value,
                        common_prefix_len
                    ).await
                }
            },
            
            MPTNode::Branch { mut children, value: branch_value } => {
                if depth == key.len() {
                    // 在分支节点设置值
                    let branch = MPTNode::Branch {
                        children,
                        value: Some(value),
                    };
                    self.put_node(&branch).await
                } else {
                    // 继续沿着分支
                    let nibble = key[depth] as usize;
                    let child_hash = children[nibble].unwrap_or(Hash::empty());
                    let new_child = self.insert_node(
                        child_hash,
                        key,
                        value,
                        depth + 1
                    ).await?;
                    
                    children[nibble] = Some(new_child);
                    let branch = MPTNode::Branch {
                        children,
                        value: branch_value,
                    };
                    self.put_node(&branch).await
                }
            },
        }
    }
    
    /// 查询键值
    pub async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        let root = *self.root.read().await;
        self.get_node_value(root, key, 0).await
    }
    
    /// 递归查询节点值
    async fn get_node_value(
        &self,
        node_hash: Hash,
        key: &[u8],
        depth: usize
    ) -> Result<Option<Vec<u8>>, Error> {
        let node = self.get_node(&node_hash).await?;
        
        match node {
            MPTNode::Empty => Ok(None),
            
            MPTNode::Leaf { key: leaf_key, value } => {
                if &key[depth..] == leaf_key.as_slice() {
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            },
            
            MPTNode::Extension { prefix, child } => {
                if key[depth..].starts_with(&prefix) {
                    self.get_node_value(child, key, depth + prefix.len()).await
                } else {
                    Ok(None)
                }
            },
            
            MPTNode::Branch { children, value } => {
                if depth == key.len() {
                    Ok(value)
                } else {
                    let nibble = key[depth] as usize;
                    if let Some(child_hash) = children[nibble] {
                        self.get_node_value(child_hash, key, depth + 1).await
                    } else {
                        Ok(None)
                    }
                }
            },
        }
    }
    
    /// 计算共同前缀长度
    fn common_prefix_length(a: &[u8], b: &[u8]) -> usize {
        a.iter().zip(b.iter()).take_while(|(x, y)| x == y).count()
    }
    
    /// 分裂叶子节点
    async fn split_leaf(
        &self,
        old_key: &[u8],
        old_value: Vec<u8>,
        new_key: &[u8],
        new_value: Vec<u8>,
        common_prefix_len: usize
    ) -> Result<Hash, Error> {
        // 实现叶子节点分裂逻辑
        todo!()
    }
    
    /// 分裂扩展节点
    async fn split_extension(
        &self,
        prefix: &[u8],
        child: Hash,
        new_key: &[u8],
        new_value: Vec<u8>,
        common_prefix_len: usize
    ) -> Result<Hash, Error> {
        // 实现扩展节点分裂逻辑
        todo!()
    }
    
    /// 存储节点
    async fn put_node(&self, node: &MPTNode) -> Result<Hash, Error> {
        let data = bincode::serialize(node)?;
        let hash = Hash::hash(&data);
        self.db.put(hash.as_bytes(), &data)?;
        Ok(hash)
    }
    
    /// 读取节点
    async fn get_node(&self, hash: &Hash) -> Result<MPTNode, Error> {
        match self.db.get(hash.as_bytes())? {
            Some(data) => {
                let node: MPTNode = bincode::deserialize(&data)?;
                Ok(node)
            },
            None => Ok(MPTNode::Empty),
        }
    }
    
    /// 获取根哈希
    pub async fn root_hash(&self) -> Hash {
        *self.root.read().await
    }
}
```

### 3.2 状态快照

```rust
/// 状态快照管理器
#[derive(Debug)]
pub struct StateSnapshot {
    /// 快照ID
    id: SnapshotId,
    /// 区块高度
    block_height: u64,
    /// 状态根哈希
    state_root: Hash,
    /// 快照时间
    timestamp: SystemTime,
    /// 快照存储
    storage: Arc<DB>,
}

impl StateSnapshot {
    /// 创建快照
    pub async fn create(
        block_height: u64,
        state_root: Hash,
        mpt: &MerklePatriciaTrie
    ) -> Result<Self, Error> {
        let id = SnapshotId::new();
        let snapshot_path = format!("snapshots/snapshot_{}", id);
        
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let storage = Arc::new(DB::open(&opts, &snapshot_path)?);
        
        // 遍历并复制所有状态
        // 实现状态复制逻辑
        
        Ok(Self {
            id,
            block_height,
            state_root,
            timestamp: SystemTime::now(),
            storage,
        })
    }
    
    /// 从快照恢复
    pub async fn restore(&self, mpt: &MerklePatriciaTrie) -> Result<(), Error> {
        // 从快照恢复状态树
        Ok(())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct SnapshotId(uuid::Uuid);

impl SnapshotId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
```

### 3.3 状态剪枝

```rust
/// 状态剪枝管理器
#[derive(Debug)]
pub struct StatePruning {
    /// 保留深度（保留最近N个区块的状态）
    retention_depth: u64,
    /// 当前最新高度
    latest_height: Arc<RwLock<u64>>,
    /// 状态存储
    state_store: Arc<StateStore>,
}

impl StatePruning {
    /// 执行剪枝
    pub async fn prune(&self) -> Result<PruningStats, Error> {
        let latest = *self.latest_height.read().await;
        
        if latest < self.retention_depth {
            return Ok(PruningStats::default());
        }
        
        let prune_before = latest - self.retention_depth;
        
        let mut stats = PruningStats::default();
        
        // 标记可以删除的状态节点
        for height in 0..prune_before {
            let pruned = self.prune_height(height).await?;
            stats.blocks_pruned += 1;
            stats.nodes_removed += pruned;
        }
        
        Ok(stats)
    }
    
    /// 剪枝指定高度的状态
    async fn prune_height(&self, height: u64) -> Result<usize, Error> {
        // 实现状态剪枝逻辑
        // 1. 获取该高度的状态根
        // 2. 遍历状态树，标记不再被引用的节点
        // 3. 删除标记的节点
        Ok(0)
    }
}

#[derive(Debug, Default)]
pub struct PruningStats {
    pub blocks_pruned: u64,
    pub nodes_removed: usize,
    pub space_freed: u64,
}
```

## 4. 交易存储

### 4.1 交易池设计

```rust
use std::collections::BTreeMap;

/// 交易池
#[derive(Debug)]
pub struct Mempool {
    /// 待处理交易（按nonce排序）
    pending: Arc<RwLock<BTreeMap<Address, BTreeMap<u64, Transaction>>>>,
    /// 队列交易
    queued: Arc<RwLock<HashMap<Hash, Transaction>>>,
    /// 交易索引
    by_hash: Arc<RwLock<HashMap<Hash, Transaction>>>,
    /// 配置
    config: MempoolConfig,
}

#[derive(Debug, Clone)]
pub struct MempoolConfig {
    /// 最大交易数
    pub max_transactions: usize,
    /// 最小Gas价格
    pub min_gas_price: u64,
    /// 交易过期时间
    pub tx_ttl: Duration,
}

impl Mempool {
    /// 添加交易
    pub async fn add_transaction(&self, tx: Transaction) -> Result<(), Error> {
        let tx_hash = tx.hash();
        
        // 1. 验证交易
        self.validate_transaction(&tx)?;
        
        // 2. 检查交易池是否已满
        if self.is_full().await {
            self.evict_lowest_priority_tx().await?;
        }
        
        // 3. 添加到交易池
        {
            let mut by_hash = self.by_hash.write().await;
            by_hash.insert(tx_hash.clone(), tx.clone());
        }
        
        // 4. 根据账户状态分类
        let sender = tx.sender();
        let nonce = tx.nonce();
        
        let mut pending = self.pending.write().await;
        pending.entry(sender)
            .or_insert_with(BTreeMap::new)
            .insert(nonce, tx);
        
        Ok(())
    }
    
    /// 移除交易
    pub async fn remove_transaction(&self, tx_hash: &Hash) -> Result<(), Error> {
        let mut by_hash = self.by_hash.write().await;
        
        if let Some(tx) = by_hash.remove(tx_hash) {
            let sender = tx.sender();
            let nonce = tx.nonce();
            
            let mut pending = self.pending.write().await;
            if let Some(sender_txs) = pending.get_mut(&sender) {
                sender_txs.remove(&nonce);
                if sender_txs.is_empty() {
                    pending.remove(&sender);
                }
            }
        }
        
        Ok(())
    }
    
    /// 获取可打包的交易
    pub async fn get_pending_transactions(&self, limit: usize) -> Vec<Transaction> {
        let pending = self.pending.read().await;
        
        let mut txs = Vec::new();
        
        for (_sender, sender_txs) in pending.iter() {
            for (_nonce, tx) in sender_txs.iter() {
                if txs.len() >= limit {
                    return txs;
                }
                txs.push(tx.clone());
            }
        }
        
        txs
    }
    
    /// 验证交易
    fn validate_transaction(&self, tx: &Transaction) -> Result<(), Error> {
        // 1. 验证签名
        tx.verify_signature()?;
        
        // 2. 检查Gas价格
        if tx.gas_price() < self.config.min_gas_price {
            return Err(Error::GasPriceTooLow);
        }
        
        // 3. 其他验证
        
        Ok(())
    }
    
    /// 检查交易池是否已满
    async fn is_full(&self) -> bool {
        self.by_hash.read().await.len() >= self.config.max_transactions
    }
    
    /// 驱逐最低优先级交易
    async fn evict_lowest_priority_tx(&self) -> Result<(), Error> {
        // 找到Gas价格最低的交易并移除
        let by_hash = self.by_hash.read().await;
        
        if let Some((hash, _)) = by_hash.iter()
            .min_by_key(|(_, tx)| tx.gas_price()) {
            let hash = hash.clone();
            drop(by_hash);
            self.remove_transaction(&hash).await?;
        }
        
        Ok(())
    }
    
    /// 清理过期交易
    pub async fn cleanup_expired(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        
        loop {
            interval.tick().await;
            
            let now = SystemTime::now();
            let mut expired = Vec::new();
            
            {
                let by_hash = self.by_hash.read().await;
                for (hash, tx) in by_hash.iter() {
                    if let Ok(duration) = now.duration_since(tx.timestamp()) {
                        if duration > self.config.tx_ttl {
                            expired.push(hash.clone());
                        }
                    }
                }
            }
            
            for hash in expired {
                let _ = self.remove_transaction(&hash).await;
            }
        }
    }
}
```

### 4.2 交易索引

```rust
/// 交易索引管理器
#[derive(Debug)]
pub struct TransactionIndex {
    /// 存储引擎
    db: Arc<DB>,
}

impl TransactionIndex {
    /// 索引交易
    pub async fn index_transaction(
        &self,
        tx_hash: Hash,
        block_hash: Hash,
        block_height: u64,
        tx_index: u32
    ) -> Result<(), Error> {
        let index_entry = TransactionIndexEntry {
            block_hash,
            block_height,
            tx_index,
        };
        
        let data = bincode::serialize(&index_entry)?;
        self.db.put(
            format!("tx:{}", tx_hash).as_bytes(),
            &data
        )?;
        
        Ok(())
    }
    
    /// 查询交易位置
    pub async fn get_transaction_location(
        &self,
        tx_hash: &Hash
    ) -> Result<Option<TransactionIndexEntry>, Error> {
        let key = format!("tx:{}", tx_hash);
        match self.db.get(key.as_bytes())? {
            Some(data) => {
                let entry: TransactionIndexEntry = bincode::deserialize(&data)?;
                Ok(Some(entry))
            },
            None => Ok(None),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionIndexEntry {
    pub block_hash: Hash,
    pub block_height: u64,
    pub tx_index: u32,
}
```

### 4.3 历史交易查询

```rust
/// 交易存储
#[derive(Debug)]
pub struct TransactionStore {
    /// 存储引擎
    db: Arc<DB>,
    /// 交易索引
    index: Arc<TransactionIndex>,
}

impl TransactionStore {
    /// 存储交易
    pub async fn put_transaction(
        &self,
        tx: Transaction,
        block_hash: Hash,
        block_height: u64,
        tx_index: u32
    ) -> Result<(), Error> {
        let tx_hash = tx.hash();
        
        // 1. 存储交易数据
        let data = bincode::serialize(&tx)?;
        self.db.put(tx_hash.as_bytes(), &data)?;
        
        // 2. 创建索引
        self.index.index_transaction(
            tx_hash,
            block_hash,
            block_height,
            tx_index
        ).await?;
        
        Ok(())
    }
    
    /// 读取交易
    pub async fn get_transaction(&self, tx_hash: &Hash) -> Result<Option<Transaction>, Error> {
        match self.db.get(tx_hash.as_bytes())? {
            Some(data) => {
                let tx: Transaction = bincode::deserialize(&data)?;
                Ok(Some(tx))
            },
            None => Ok(None),
        }
    }
    
    /// 查询地址的所有交易
    pub async fn get_transactions_by_address(
        &self,
        address: &Address,
        limit: usize
    ) -> Result<Vec<Transaction>, Error> {
        // 从地址索引中查询
        let key_prefix = format!("addr:{}", address);
        let mut txs = Vec::new();
        
        let iter = self.db.prefix_iterator(key_prefix.as_bytes());
        
        for item in iter.take(limit) {
            let (_key, value) = item?;
            let tx_hash: Hash = bincode::deserialize(&value)?;
            
            if let Some(tx) = self.get_transaction(&tx_hash).await? {
                txs.push(tx);
            }
        }
        
        Ok(txs)
    }
}
```

## 5. 存储引擎

### 5.1 RocksDB集成

```rust
use rocksdb::{DB, Options, WriteBatch, IteratorMode};

/// RocksDB存储引擎
#[derive(Debug)]
pub struct RocksDBStorage {
    db: Arc<DB>,
    write_buffer_size: usize,
}

impl RocksDBStorage {
    /// 创建RocksDB实例
    pub fn new(path: &str) -> Result<Self, Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(rocksdb::DBCompressionType::Lz4);
        opts.set_write_buffer_size(64 * 1024 * 1024); // 64MB
        opts.set_max_write_buffer_number(3);
        opts.set_target_file_size_base(64 * 1024 * 1024);
        opts.set_level_zero_file_num_compaction_trigger(4);
        
        let db = DB::open(&opts, path)?;
        
        Ok(Self {
            db: Arc::new(db),
            write_buffer_size: 64 * 1024 * 1024,
        })
    }
    
    /// 批量写入
    pub fn batch_write(&self, operations: Vec<(Vec<u8>, Vec<u8>)>) -> Result<(), Error> {
        let mut batch = WriteBatch::default();
        
        for (key, value) in operations {
            batch.put(&key, &value);
        }
        
        self.db.write(batch)?;
        Ok(())
    }
    
    /// 范围查询
    pub fn range_scan(&self, start: &[u8], end: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Error> {
        let mut results = Vec::new();
        
        let iter = self.db.iterator(IteratorMode::From(start, rocksdb::Direction::Forward));
        
        for item in iter {
            let (key, value) = item?;
            if key.as_ref() >= end {
                break;
            }
            results.push((key.to_vec(), value.to_vec()));
        }
        
        Ok(results)
    }
}
```

### 5.2 LMDB集成

```rust
use lmdb::{Environment, Database, Transaction};

/// LMDB存储引擎
#[derive(Debug)]
pub struct LMDBStorage {
    env: Arc<Environment>,
    db: Database,
}

impl LMDBStorage {
    /// 创建LMDB实例
    pub fn new(path: &str, map_size: usize) -> Result<Self, Error> {
        let env = Environment::new()
            .set_map_size(map_size)
            .set_max_dbs(10)
            .open(Path::new(path))?;
        
        let db = env.open_db(None)?;
        
        Ok(Self {
            env: Arc::new(env),
            db,
        })
    }
    
    /// 读取
    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        let txn = self.env.begin_ro_txn()?;
        
        match txn.get(self.db, &key) {
            Ok(value) => Ok(Some(value.to_vec())),
            Err(lmdb::Error::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
    
    /// 写入
    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        let mut txn = self.env.begin_rw_txn()?;
        txn.put(self.db, &key, &value, lmdb::WriteFlags::empty())?;
        txn.commit()?;
        Ok(())
    }
}
```

### 5.3 自定义存储引擎

```rust
/// 存储引擎抽象接口
#[async_trait::async_trait]
pub trait StorageEngine: Send + Sync {
    /// 读取
    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error>;
    
    /// 写入
    async fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Error>;
    
    /// 删除
    async fn delete(&self, key: &[u8]) -> Result<(), Error>;
    
    /// 批量写入
    async fn batch_write(&self, operations: Vec<WriteOperation>) -> Result<(), Error>;
    
    /// 迭代器
    fn iter(&self, start: &[u8]) -> Box<dyn Iterator<Item = Result<(Vec<u8>, Vec<u8>), Error>>>;
}

#[derive(Debug, Clone)]
pub enum WriteOperation {
    Put { key: Vec<u8>, value: Vec<u8> },
    Delete { key: Vec<u8> },
}
```

## 6. 缓存策略

### 6.1 LRU缓存

```rust
use lru::LruCache;

/// LRU缓存实现
#[derive(Debug)]
pub struct LruCacheImpl<K, V> {
    cache: Arc<RwLock<LruCache<K, V>>>,
    hits: Arc<AtomicU64>,
    misses: Arc<AtomicU64>,
}

impl<K: Hash + Eq, V: Clone> LruCacheImpl<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(LruCache::new(capacity))),
            hits: Arc::new(AtomicU64::new(0)),
            misses: Arc::new(AtomicU64::new(0)),
        }
    }
    
    pub async fn get(&self, key: &K) -> Option<V> {
        match self.cache.write().await.get(key) {
            Some(value) => {
                self.hits.fetch_add(1, Ordering::Relaxed);
                Some(value.clone())
            },
            None => {
                self.misses.fetch_add(1, Ordering::Relaxed);
                None
            },
        }
    }
    
    pub async fn put(&self, key: K, value: V) {
        self.cache.write().await.put(key, value);
    }
    
    pub fn hit_rate(&self) -> f64 {
        let hits = self.hits.load(Ordering::Relaxed) as f64;
        let misses = self.misses.load(Ordering::Relaxed) as f64;
        let total = hits + misses;
        
        if total == 0.0 {
            0.0
        } else {
            hits / total
        }
    }
}
```

### 6.2 多级缓存

```rust
/// 多级缓存系统
#[derive(Debug)]
pub struct MultiLevelCache {
    /// L1: 内存缓存（热数据）
    l1_cache: Arc<LruCacheImpl<Hash, Vec<u8>>>,
    /// L2: SSD缓存（温数据）
    l2_cache: Option<Arc<LruCacheImpl<Hash, Vec<u8>>>>,
    /// L3: 持久化存储
    storage: Arc<dyn StorageEngine>,
}

impl MultiLevelCache {
    pub async fn get(&self, key: &Hash) -> Result<Option<Vec<u8>>, Error> {
        // 1. 尝试L1缓存
        if let Some(value) = self.l1_cache.get(key).await {
            return Ok(Some(value));
        }
        
        // 2. 尝试L2缓存
        if let Some(l2) = &self.l2_cache {
            if let Some(value) = l2.get(key).await {
                // 提升到L1
                self.l1_cache.put(key.clone(), value.clone()).await;
                return Ok(Some(value));
            }
        }
        
        // 3. 从存储读取
        if let Some(value) = self.storage.get(key.as_bytes()).await? {
            // 缓存到L2和L1
            if let Some(l2) = &self.l2_cache {
                l2.put(key.clone(), value.clone()).await;
            }
            self.l1_cache.put(key.clone(), value.clone()).await;
            return Ok(Some(value));
        }
        
        Ok(None)
    }
    
    pub async fn put(&self, key: Hash, value: Vec<u8>) -> Result<(), Error> {
        // 同时写入所有层
        self.l1_cache.put(key.clone(), value.clone()).await;
        
        if let Some(l2) = &self.l2_cache {
            l2.put(key.clone(), value.clone()).await;
        }
        
        self.storage.put(key.as_bytes(), &value).await?;
        
        Ok(())
    }
}
```

### 6.3 预读优化

```rust
/// 预读管理器
#[derive(Debug)]
pub struct Prefetcher {
    /// 预读窗口大小
    window_size: usize,
    /// 后台预读任务
    prefetch_tasks: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
}

impl Prefetcher {
    /// 预读区块范围
    pub async fn prefetch_blocks(
        &self,
        start_height: u64,
        cache: Arc<LruCacheImpl<Hash, Block>>,
        storage: Arc<BlockStore>
    ) {
        let end_height = start_height + self.window_size as u64;
        
        let handle = tokio::spawn(async move {
            for height in start_height..end_height {
                if let Ok(Some(block)) = storage.get_block_by_height(height).await {
                    let hash = block.hash();
                    cache.put(hash, block).await;
                }
            }
        });
        
        self.prefetch_tasks.write().await.push(handle);
    }
}
```

## 7. 数据持久化

### 7.1 WAL日志

```rust
/// Write-Ahead Log实现
#[derive(Debug)]
pub struct WriteAheadLog {
    /// WAL文件路径
    log_path: PathBuf,
    /// 当前WAL文件
    current_file: Arc<RwLock<File>>,
    /// LSN（日志序列号）
    lsn: Arc<AtomicU64>,
}

impl WriteAheadLog {
    /// 写入日志条目
    pub async fn append(&self, entry: LogEntry) -> Result<u64, Error> {
        let lsn = self.lsn.fetch_add(1, Ordering::SeqCst);
        
        let mut record = LogRecord {
            lsn,
            entry,
            checksum: 0,
        };
        
        // 计算校验和
        record.checksum = record.calculate_checksum();
        
        // 序列化
        let data = bincode::serialize(&record)?;
        
        // 写入文件
        let mut file = self.current_file.write().await;
        file.write_all(&data).await?;
        file.flush().await?;
        
        Ok(lsn)
    }
    
    /// 读取日志
    pub async fn read_from(&self, lsn: u64) -> Result<Vec<LogRecord>, Error> {
        let mut records = Vec::new();
        
        // 读取WAL文件并解析日志记录
        // 实现日志读取逻辑
        
        Ok(records)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRecord {
    pub lsn: u64,
    pub entry: LogEntry,
    pub checksum: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogEntry {
    Put { key: Vec<u8>, value: Vec<u8> },
    Delete { key: Vec<u8> },
    Commit,
    Abort,
}

impl LogRecord {
    fn calculate_checksum(&self) -> u32 {
        // 实现CRC32校验和
        0
    }
}
```

### 7.2 检查点

```rust
/// 检查点管理器
#[derive(Debug)]
pub struct CheckpointManager {
    /// 存储引擎
    storage: Arc<dyn StorageEngine>,
    /// WAL
    wal: Arc<WriteAheadLog>,
    /// 检查点间隔
    checkpoint_interval: Duration,
}

impl CheckpointManager {
    /// 创建检查点
    pub async fn create_checkpoint(&self) -> Result<Checkpoint, Error> {
        let checkpoint_id = uuid::Uuid::new_v4();
        let lsn = self.wal.lsn.load(Ordering::SeqCst);
        
        // 1. 刷新所有脏数据到磁盘
        // 2. 记录当前LSN
        // 3. 创建检查点元数据
        
        let checkpoint = Checkpoint {
            id: checkpoint_id,
            lsn,
            timestamp: SystemTime::now(),
        };
        
        Ok(checkpoint)
    }
    
    /// 定期创建检查点
    pub async fn periodic_checkpoint(&self) {
        let mut interval = tokio::time::interval(self.checkpoint_interval);
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.create_checkpoint().await {
                eprintln!("检查点创建失败: {:?}", e);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub id: uuid::Uuid,
    pub lsn: u64,
    pub timestamp: SystemTime,
}
```

### 7.3 崩溃恢复

```rust
/// 崩溃恢复管理器
#[derive(Debug)]
pub struct CrashRecovery {
    wal: Arc<WriteAheadLog>,
    storage: Arc<dyn StorageEngine>,
}

impl CrashRecovery {
    /// 执行崩溃恢复
    pub async fn recover(&self) -> Result<RecoveryStats, Error> {
        // 1. 找到最后一个检查点
        let checkpoint = self.find_latest_checkpoint().await?;
        
        // 2. 从检查点之后的WAL重放日志
        let records = self.wal.read_from(checkpoint.lsn).await?;
        
        let mut stats = RecoveryStats::default();
        
        for record in records {
            match record.entry {
                LogEntry::Put { key, value } => {
                    self.storage.put(&key, &value).await?;
                    stats.operations_replayed += 1;
                },
                LogEntry::Delete { key } => {
                    self.storage.delete(&key).await?;
                    stats.operations_replayed += 1;
                },
                LogEntry::Commit => {
                    stats.transactions_committed += 1;
                },
                LogEntry::Abort => {
                    stats.transactions_aborted += 1;
                },
            }
        }
        
        Ok(stats)
    }
    
    async fn find_latest_checkpoint(&self) -> Result<Checkpoint, Error> {
        // 查找最新的检查点
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct RecoveryStats {
    pub operations_replayed: usize,
    pub transactions_committed: usize,
    pub transactions_aborted: usize,
}
```

## 8. 存储优化

### 8.1 压缩策略

```rust
use flate2::Compression;
use flate2::write::{GzEncoder, GzDecoder};

/// 数据压缩器
#[derive(Debug)]
pub struct DataCompressor {
    compression_level: Compression,
}

impl DataCompressor {
    /// 压缩数据
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let mut encoder = GzEncoder::new(Vec::new(), self.compression_level);
        encoder.write_all(data)?;
        let compressed = encoder.finish()?;
        Ok(compressed)
    }
    
    /// 解压数据
    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let mut decoder = GzDecoder::new(Vec::new());
        decoder.write_all(data)?;
        let decompressed = decoder.finish()?;
        Ok(decompressed)
    }
}
```

### 8.2 数据分片

```rust
/// 数据分片管理器
#[derive(Debug)]
pub struct DataSharding {
    /// 分片数量
    shard_count: usize,
    /// 分片存储
    shards: Vec<Arc<dyn StorageEngine>>,
}

impl DataSharding {
    /// 计算分片ID
    fn shard_id(&self, key: &[u8]) -> usize {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.shard_count
    }
    
    /// 读取数据
    pub async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        let shard_id = self.shard_id(key);
        self.shards[shard_id].get(key).await
    }
    
    /// 写入数据
    pub async fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        let shard_id = self.shard_id(key);
        self.shards[shard_id].put(key, value).await
    }
}
```

### 8.3 归档与剪枝

```rust
/// 归档管理器
#[derive(Debug)]
pub struct ArchiveManager {
    /// 活跃存储
    active_storage: Arc<dyn StorageEngine>,
    /// 归档存储
    archive_storage: Arc<dyn StorageEngine>,
    /// 归档阈值（区块年龄）
    archive_threshold: Duration,
}

impl ArchiveManager {
    /// 归档旧数据
    pub async fn archive_old_data(&self) -> Result<ArchiveStats, Error> {
        let mut stats = ArchiveStats::default();
        let cutoff_time = SystemTime::now() - self.archive_threshold;
        
        // 查找需要归档的数据
        // 移动到归档存储
        // 从活跃存储中删除
        
        Ok(stats)
    }
}

#[derive(Debug, Default)]
pub struct ArchiveStats {
    pub blocks_archived: u64,
    pub data_size_archived: u64,
}
```

## 9. 总结

本文档详细介绍了区块链存储系统的设计与实现，包括：

1. **存储架构**：多层存储结构、存储类型、数据组织
2. **区块存储**：数据结构、索引、检索
3. **状态存储**：Merkle Patricia Trie、快照、剪枝
4. **交易存储**：交易池、索引、历史查询
5. **存储引擎**：RocksDB、LMDB、自定义引擎
6. **缓存策略**：LRU缓存、多级缓存、预读优化
7. **数据持久化**：WAL、检查点、崩溃恢复
8. **存储优化**：压缩、分片、归档

这些实现为构建高效、可靠、可扩展的区块链存储系统提供了完整的技术方案。

---

**文档版本**: v1.0.0  
**最后更新**: 2025年10月17日  
**作者**: Rust区块链技术团队  
**相关文档**:
- [08_ARCHITECTURE_DESIGN.md](./08_ARCHITECTURE_DESIGN.md) - 系统架构设计
- [09_NETWORK_PROTOCOLS.md](./09_NETWORK_PROTOCOLS.md) - 网络协议设计
- [11_PERFORMANCE_OPTIMIZATION.md](./11_PERFORMANCE_OPTIMIZATION.md) - 性能优化策略

