# 共识算法实现

## 📋 目录

- [共识算法实现](#共识算法实现)
  - [📋 目录](#-目录)
  - [1. 共识算法概述](#1-共识算法概述)
    - [1.1 共识算法接口设计](#11-共识算法接口设计)
  - [2. 工作量证明(PoW)](#2-工作量证明pow)
    - [2.1 PoW基础实现](#21-pow基础实现)
  - [3. 权益证明(PoS)](#3-权益证明pos)
    - [3.1 PoS实现](#31-pos实现)
  - [4. 委托权益证明(DPoS)](#4-委托权益证明dpos)
    - [4.1 DPoS实现](#41-dpos实现)
  - [5. 实用拜占庭容错(PBFT)](#5-实用拜占庭容错pbft)
    - [5.1 PBFT实现](#51-pbft实现)
  - [6. Raft共识](#6-raft共识)
    - [6.1 Raft实现（简化版）](#61-raft实现简化版)
  - [7. 混合共识](#7-混合共识)
    - [7.1 PoW + PoS混合](#71-pow--pos混合)
  - [8. 性能对比与选择](#8-性能对比与选择)
    - [8.1 共识算法对比](#81-共识算法对比)
    - [8.2 选择建议](#82-选择建议)
  - [9. 总结](#9-总结)

## 1. 共识算法概述

### 1.1 共识算法接口设计

```rust
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// 共识算法trait
#[async_trait]
pub trait ConsensusAlgorithm: Send + Sync {
    type Block: Clone + Send + Sync;
    type Proof: Clone + Send + Sync;
    type Config: Clone + Send + Sync;
    
    // 创建新区块
    async fn create_block(
        &self,
        transactions: Vec<Transaction>,
        previous_hash: BlockHash,
    ) -> Result<Self::Block, ConsensusError>;
    
    // 验证区块
    async fn verify_block(
        &self,
        block: &Self::Block,
    ) -> Result<bool, ConsensusError>;
    
    // 选择下一个出块者
    async fn select_validator(
        &self,
        validators: &[Validator],
    ) -> Result<Validator, ConsensusError>;
    
    // 达成共识
    async fn reach_consensus(
        &self,
        block: &Self::Block,
    ) -> Result<ConsensusResult, ConsensusError>;
    
    // 获取当前难度/权重
    fn get_difficulty(&self) -> u32;
    
    // 更新共识状态
    async fn update_state(
        &mut self,
        block: &Self::Block,
    ) -> Result<(), ConsensusError>;
}

// 共识结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    pub approved: bool,
    pub signatures: Vec<Signature>,
    pub voting_power: u64,
    pub timestamp: u64,
}

// 共识错误
#[derive(Debug, thiserror::Error)]
pub enum ConsensusError {
    #[error("Invalid proof")]
    InvalidProof,
    
    #[error("Insufficient voting power")]
    InsufficientVotingPower,
    
    #[error("Timeout reached")]
    Timeout,
    
    #[error("Byzantine behavior detected")]
    ByzantineBehavior,
    
    #[error("Network error: {0}")]
    NetworkError(String),
}
```

## 2. 工作量证明(PoW)

### 2.1 PoW基础实现

```rust
use sha2::{Sha256, Digest};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct ProofOfWork {
    config: PoWConfig,
    current_difficulty: Arc<RwLock<u32>>,
}

#[derive(Debug, Clone)]
pub struct PoWConfig {
    pub initial_difficulty: u32,
    pub block_time: u64,  // 目标出块时间(秒)
    pub adjustment_interval: u64,  // 难度调整间隔
    pub max_nonce: u64,
}

impl Default for PoWConfig {
    fn default() -> Self {
        Self {
            initial_difficulty: 0x1d00ffff,
            block_time: 600,  // 10分钟
            adjustment_interval: 2016,  // 约2周
            max_nonce: u64::MAX,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoWBlock {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: BlockHash,
    pub merkle_root: BlockHash,
    pub nonce: u64,
    pub difficulty: u32,
    pub hash: BlockHash,
}

impl ProofOfWork {
    pub fn new(config: PoWConfig) -> Self {
        let initial_difficulty = config.initial_difficulty;
        Self {
            config,
            current_difficulty: Arc::new(RwLock::new(initial_difficulty)),
        }
    }
    
    // 挖矿函数
    pub async fn mine_block(
        &self,
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: BlockHash,
    ) -> Result<PoWBlock, ConsensusError> {
        let merkle_root = Self::calculate_merkle_root(&transactions);
        let difficulty = *self.current_difficulty.read().await;
        let target = Self::difficulty_to_target(difficulty);
        
        let mut nonce = 0u64;
        let start_time = std::time::Instant::now();
        
        loop {
            let timestamp = current_timestamp();
            
            let hash = Self::calculate_hash(
                index,
                timestamp,
                &previous_hash,
                &merkle_root,
                nonce,
                difficulty,
            );
            
            if Self::hash_meets_target(&hash, &target) {
                println!("Block mined! Nonce: {}, Time: {:?}", nonce, start_time.elapsed());
                
                return Ok(PoWBlock {
                    index,
                    timestamp,
                    transactions,
                    previous_hash,
                    merkle_root,
                    nonce,
                    difficulty,
                    hash,
                });
            }
            
            nonce += 1;
            
            if nonce >= self.config.max_nonce {
                return Err(ConsensusError::InvalidProof);
            }
            
            // 每100万次尝试检查一次是否需要更新时间戳
            if nonce % 1_000_000 == 0 {
                tokio::task::yield_now().await;
            }
        }
    }
    
    // 并行挖矿
    pub async fn mine_block_parallel(
        &self,
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: BlockHash,
        num_threads: usize,
    ) -> Result<PoWBlock, ConsensusError> {
        use tokio::sync::mpsc;
        
        let merkle_root = Self::calculate_merkle_root(&transactions);
        let difficulty = *self.current_difficulty.read().await;
        let target = Self::difficulty_to_target(difficulty);
        
        let (tx, mut rx) = mpsc::channel(1);
        let mut handles = Vec::new();
        
        for thread_id in 0..num_threads {
            let tx = tx.clone();
            let transactions = transactions.clone();
            let target = target.clone();
            
            let handle = tokio::spawn(async move {
                let nonce_start = thread_id as u64 * (u64::MAX / num_threads as u64);
                let nonce_end = (thread_id + 1) as u64 * (u64::MAX / num_threads as u64);
                
                for nonce in nonce_start..nonce_end {
                    let timestamp = current_timestamp();
                    
                    let hash = Self::calculate_hash(
                        index,
                        timestamp,
                        &previous_hash,
                        &merkle_root,
                        nonce,
                        difficulty,
                    );
                    
                    if Self::hash_meets_target(&hash, &target) {
                        let block = PoWBlock {
                            index,
                            timestamp,
                            transactions,
                            previous_hash,
                            merkle_root,
                            nonce,
                            difficulty,
                            hash,
                        };
                        let _ = tx.send(block).await;
                        break;
                    }
                    
                    if nonce % 10_000 == 0 {
                        tokio::task::yield_now().await;
                    }
                }
            });
            
            handles.push(handle);
        }
        
        drop(tx);
        
        // 等待第一个找到解的线程
        if let Some(block) = rx.recv().await {
            // 取消其他线程
            for handle in handles {
                handle.abort();
            }
            Ok(block)
        } else {
            Err(ConsensusError::InvalidProof)
        }
    }
    
    // 计算哈希
    fn calculate_hash(
        index: u64,
        timestamp: u64,
        previous_hash: &BlockHash,
        merkle_root: &BlockHash,
        nonce: u64,
        difficulty: u32,
    ) -> BlockHash {
        let mut hasher = Sha256::new();
        
        hasher.update(&index.to_le_bytes());
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(previous_hash);
        hasher.update(merkle_root);
        hasher.update(&nonce.to_le_bytes());
        hasher.update(&difficulty.to_le_bytes());
        
        hasher.finalize().into()
    }
    
    // 难度转目标值
    fn difficulty_to_target(difficulty: u32) -> [u8; 32] {
        let mut target = [0xffu8; 32];
        let exponent = (difficulty >> 24) as usize;
        let coefficient = difficulty & 0x00ffffff;
        
        if exponent <= 3 {
            let shift = 8 * (3 - exponent);
            let value = coefficient >> shift;
            target[31] = (value & 0xff) as u8;
            target[30] = ((value >> 8) & 0xff) as u8;
            target[29] = ((value >> 16) & 0xff) as u8;
        } else {
            let pos = 32 - exponent;
            if pos < 29 {
                target[pos + 2] = (coefficient & 0xff) as u8;
                target[pos + 1] = ((coefficient >> 8) & 0xff) as u8;
                target[pos] = ((coefficient >> 16) & 0xff) as u8;
            }
        }
        
        target
    }
    
    // 检查哈希是否满足目标
    fn hash_meets_target(hash: &BlockHash, target: &[u8; 32]) -> bool {
        for i in 0..32 {
            if hash[i] < target[i] {
                return true;
            } else if hash[i] > target[i] {
                return false;
            }
        }
        false
    }
    
    // 难度调整
    pub async fn adjust_difficulty(
        &self,
        blocks: &[PoWBlock],
    ) -> Result<(), ConsensusError> {
        if blocks.len() < self.config.adjustment_interval as usize {
            return Ok(());
        }
        
        let start_time = blocks[0].timestamp;
        let end_time = blocks[blocks.len() - 1].timestamp;
        let time_taken = end_time - start_time;
        
        let expected_time = self.config.block_time * self.config.adjustment_interval;
        
        let mut difficulty = *self.current_difficulty.read().await;
        
        if time_taken < expected_time / 2 {
            // 时间太短，增加难度
            difficulty = (difficulty as f64 * 2.0) as u32;
        } else if time_taken > expected_time * 2 {
            // 时间太长，降低难度
            difficulty = (difficulty as f64 / 2.0) as u32;
        } else {
            // 正常调整
            let ratio = expected_time as f64 / time_taken as f64;
            difficulty = (difficulty as f64 * ratio) as u32;
        }
        
        // 限制调整幅度（最多4倍）
        let current = *self.current_difficulty.read().await;
        difficulty = difficulty.clamp(current / 4, current * 4);
        
        *self.current_difficulty.write().await = difficulty;
        
        Ok(())
    }
    
    fn calculate_merkle_root(transactions: &[Transaction]) -> BlockHash {
        if transactions.is_empty() {
            return [0u8; 32];
        }
        
        let mut hashes: Vec<[u8; 32]> = transactions
            .iter()
            .map(|tx| tx.id)
            .collect();
        
        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();
            
            for chunk in hashes.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(&chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(&chunk[1]);
                } else {
                    hasher.update(&chunk[0]);
                }
                new_hashes.push(hasher.finalize().into());
            }
            
            hashes = new_hashes;
        }
        
        hashes[0]
    }
}

#[async_trait]
impl ConsensusAlgorithm for ProofOfWork {
    type Block = PoWBlock;
    type Proof = u64;  // nonce
    type Config = PoWConfig;
    
    async fn create_block(
        &self,
        transactions: Vec<Transaction>,
        previous_hash: BlockHash,
    ) -> Result<Self::Block, ConsensusError> {
        let index = 0; // 应该从区块链获取
        self.mine_block(index, transactions, previous_hash).await
    }
    
    async fn verify_block(&self, block: &Self::Block) -> Result<bool, ConsensusError> {
        // 重新计算哈希
        let calculated_hash = Self::calculate_hash(
            block.index,
            block.timestamp,
            &block.previous_hash,
            &block.merkle_root,
            block.nonce,
            block.difficulty,
        );
        
        if calculated_hash != block.hash {
            return Ok(false);
        }
        
        // 验证难度
        let target = Self::difficulty_to_target(block.difficulty);
        Ok(Self::hash_meets_target(&block.hash, &target))
    }
    
    async fn select_validator(&self, _validators: &[Validator]) -> Result<Validator, ConsensusError> {
        // PoW不需要选择验证者
        Err(ConsensusError::InvalidProof)
    }
    
    async fn reach_consensus(&self, block: &Self::Block) -> Result<ConsensusResult, ConsensusError> {
        let verified = self.verify_block(block).await?;
        
        Ok(ConsensusResult {
            approved: verified,
            signatures: Vec::new(),
            voting_power: 0,
            timestamp: current_timestamp(),
        })
    }
    
    fn get_difficulty(&self) -> u32 {
        // 需要使用blocking版本或改为async
        0x1d00ffff
    }
    
    async fn update_state(&mut self, _block: &Self::Block) -> Result<(), ConsensusError> {
        Ok(())
    }
}
```

## 3. 权益证明(PoS)

### 3.1 PoS实现

```rust
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProofOfStake {
    config: PoSConfig,
    validators: Arc<RwLock<HashMap<Address, ValidatorInfo>>>,
    current_epoch: Arc<RwLock<u64>>,
}

#[derive(Debug, Clone)]
pub struct PoSConfig {
    pub min_stake: u64,
    pub epoch_duration: u64,  // 每个epoch的区块数
    pub slash_percentage: u8,  // 惩罚百分比
    pub reward_per_block: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    pub address: Address,
    pub stake: u64,
    pub locked_until: u64,
    pub reputation: f64,
    pub blocks_produced: u64,
    pub blocks_missed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoSBlock {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: BlockHash,
    pub merkle_root: BlockHash,
    pub validator: Address,
    pub signature: Signature,
    pub hash: BlockHash,
}

impl ProofOfStake {
    pub fn new(config: PoSConfig) -> Self {
        Self {
            config,
            validators: Arc::new(RwLock::new(HashMap::new())),
            current_epoch: Arc::new(RwLock::new(0)),
        }
    }
    
    // 注册验证者
    pub async fn register_validator(
        &self,
        address: Address,
        stake: u64,
        lock_duration: u64,
    ) -> Result<(), ConsensusError> {
        if stake < self.config.min_stake {
            return Err(ConsensusError::InsufficientVotingPower);
        }
        
        let mut validators = self.validators.write().await;
        
        let validator_info = ValidatorInfo {
            address,
            stake,
            locked_until: current_timestamp() + lock_duration,
            reputation: 1.0,
            blocks_produced: 0,
            blocks_missed: 0,
        };
        
        validators.insert(address, validator_info);
        Ok(())
    }
    
    // 选择下一个验证者（基于权益加权随机）
    pub async fn select_next_validator(&self) -> Result<Address, ConsensusError> {
        let validators = self.validators.read().await;
        
        if validators.is_empty() {
            return Err(ConsensusError::InsufficientVotingPower);
        }
        
        // 计算总权益
        let total_stake: u64 = validators.values()
            .map(|v| (v.stake as f64 * v.reputation) as u64)
            .sum();
        
        // 加权随机选择
        let mut rng = rand::thread_rng();
        let mut random_stake = rng.gen_range(0..total_stake);
        
        for (address, validator) in validators.iter() {
            let weighted_stake = (validator.stake as f64 * validator.reputation) as u64;
            
            if random_stake < weighted_stake {
                return Ok(*address);
            }
            
            random_stake -= weighted_stake;
        }
        
        // 降级选择第一个验证者
        Ok(*validators.keys().next().unwrap())
    }
    
    // 创建区块
    pub async fn create_pos_block(
        &self,
        validator: Address,
        transactions: Vec<Transaction>,
        previous_hash: BlockHash,
        private_key: &PrivateKey,
    ) -> Result<PoSBlock, ConsensusError> {
        // 验证validator是否有权限
        {
            let validators = self.validators.read().await;
            if !validators.contains_key(&validator) {
                return Err(ConsensusError::InvalidProof);
            }
        }
        
        let index = 0; // 应从区块链获取
        let timestamp = current_timestamp();
        let merkle_root = Self::calculate_merkle_root(&transactions);
        
        // 计算区块哈希
        let hash = Self::calculate_block_hash(
            index,
            timestamp,
            &previous_hash,
            &merkle_root,
            &validator,
        );
        
        // 签名
        let signature = private_key.sign(&hash)?;
        
        Ok(PoSBlock {
            index,
            timestamp,
            transactions,
            previous_hash,
            merkle_root,
            validator,
            signature,
            hash,
        })
    }
    
    // 验证区块
    pub async fn verify_pos_block(&self, block: &PoSBlock) -> Result<bool, ConsensusError> {
        // 验证validator
        let validators = self.validators.read().await;
        let validator_info = validators.get(&block.validator)
            .ok_or(ConsensusError::InvalidProof)?;
        
        // 验证哈希
        let calculated_hash = Self::calculate_block_hash(
            block.index,
            block.timestamp,
            &block.previous_hash,
            &block.merkle_root,
            &block.validator,
        );
        
        if calculated_hash != block.hash {
            return Ok(false);
        }
        
        // 验证签名
        let public_key = PublicKey::from_address(&block.validator);
        if !public_key.verify(&block.hash, &block.signature) {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    // 更新验证者状态
    pub async fn update_validator_stats(
        &self,
        validator: &Address,
        produced: bool,
    ) -> Result<(), ConsensusError> {
        let mut validators = self.validators.write().await;
        
        if let Some(info) = validators.get_mut(validator) {
            if produced {
                info.blocks_produced += 1;
                // 增加声誉
                info.reputation = (info.reputation * 1.01).min(2.0);
            } else {
                info.blocks_missed += 1;
                // 降低声誉
                info.reputation = (info.reputation * 0.99).max(0.1);
            }
        }
        
        Ok(())
    }
    
    // 惩罚机制（slashing）
    pub async fn slash_validator(
        &self,
        validator: &Address,
        reason: SlashReason,
    ) -> Result<(), ConsensusError> {
        let mut validators = self.validators.write().await;
        
        if let Some(info) = validators.get_mut(validator) {
            let slash_amount = (info.stake * self.config.slash_percentage as u64) / 100;
            info.stake = info.stake.saturating_sub(slash_amount);
            info.reputation = (info.reputation * 0.5).max(0.1);
            
            println!("Validator {} slashed {} tokens for {:?}", 
                     hex::encode(validator), slash_amount, reason);
            
            // 如果权益低于最小值，移除验证者
            if info.stake < self.config.min_stake {
                validators.remove(validator);
            }
        }
        
        Ok(())
    }
    
    // 计算区块哈希
    fn calculate_block_hash(
        index: u64,
        timestamp: u64,
        previous_hash: &BlockHash,
        merkle_root: &BlockHash,
        validator: &Address,
    ) -> BlockHash {
        let mut hasher = Sha256::new();
        
        hasher.update(&index.to_le_bytes());
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(previous_hash);
        hasher.update(merkle_root);
        hasher.update(validator);
        
        hasher.finalize().into()
    }
    
    fn calculate_merkle_root(transactions: &[Transaction]) -> BlockHash {
        if transactions.is_empty() {
            return [0u8; 32];
        }
        
        let mut hashes: Vec<[u8; 32]> = transactions
            .iter()
            .map(|tx| tx.id)
            .collect();
        
        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();
            
            for chunk in hashes.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(&chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(&chunk[1]);
                } else {
                    hasher.update(&chunk[0]);
                }
                new_hashes.push(hasher.finalize().into());
            }
            
            hashes = new_hashes;
        }
        
        hashes[0]
    }
}

#[derive(Debug, Clone)]
pub enum SlashReason {
    DoubleSign,
    InvalidBlock,
    Offline,
    MaliciousBehavior,
}
```

## 4. 委托权益证明(DPoS)

### 4.1 DPoS实现

```rust
#[derive(Debug, Clone)]
pub struct DelegatedProofOfStake {
    config: DPoSConfig,
    witnesses: Arc<RwLock<Vec<Witness>>>,
    delegations: Arc<RwLock<HashMap<Address, Vec<Delegation>>>>,
    current_round: Arc<RwLock<u64>>,
}

#[derive(Debug, Clone)]
pub struct DPoSConfig {
    pub witness_count: usize,  // 见证人数量
    pub block_interval: u64,   // 出块间隔(秒)
    pub round_duration: u64,   // 轮次时长
    pub min_delegation: u64,   // 最小委托量
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Witness {
    pub address: Address,
    pub total_votes: u64,
    pub blocks_produced: u64,
    pub missed_blocks: u64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation {
    pub delegator: Address,
    pub witness: Address,
    pub amount: u64,
    pub timestamp: u64,
}

impl DelegatedProofOfStake {
    pub fn new(config: DPoSConfig) -> Self {
        Self {
            config,
            witnesses: Arc::new(RwLock::new(Vec::new())),
            delegations: Arc::new(RwLock::new(HashMap::new())),
            current_round: Arc::new(RwLock::new(0)),
        }
    }
    
    // 注册见证人
    pub async fn register_witness(
        &self,
        address: Address,
    ) -> Result<(), ConsensusError> {
        let mut witnesses = self.witnesses.write().await;
        
        let witness = Witness {
            address,
            total_votes: 0,
            blocks_produced: 0,
            missed_blocks: 0,
            is_active: true,
        };
        
        witnesses.push(witness);
        Ok(())
    }
    
    // 委托投票
    pub async fn delegate(
        &self,
        delegator: Address,
        witness: Address,
        amount: u64,
    ) -> Result<(), ConsensusError> {
        if amount < self.config.min_delegation {
            return Err(ConsensusError::InsufficientVotingPower);
        }
        
        let delegation = Delegation {
            delegator,
            witness,
            amount,
            timestamp: current_timestamp(),
        };
        
        // 添加委托
        let mut delegations = self.delegations.write().await;
        delegations.entry(witness)
            .or_insert_with(Vec::new)
            .push(delegation);
        
        // 更新见证人票数
        let mut witnesses = self.witnesses.write().await;
        if let Some(w) = witnesses.iter_mut().find(|w| w.address == witness) {
            w.total_votes += amount;
        }
        
        Ok(())
    }
    
    // 选举活跃见证人
    pub async fn elect_active_witnesses(&self) -> Result<Vec<Address>, ConsensusError> {
        let mut witnesses = self.witnesses.write().await;
        
        // 按票数排序
        witnesses.sort_by(|a, b| b.total_votes.cmp(&a.total_votes));
        
        // 选择前N个
        let active_witnesses: Vec<Address> = witnesses
            .iter()
            .take(self.config.witness_count)
            .map(|w| w.address)
            .collect();
        
        // 更新活跃状态
        for witness in witnesses.iter_mut() {
            witness.is_active = active_witnesses.contains(&witness.address);
        }
        
        Ok(active_witnesses)
    }
    
    // 获取当前出块见证人
    pub async fn get_current_witness(&self, block_num: u64) -> Result<Address, ConsensusError> {
        let witnesses = self.witnesses.read().await;
        let active_witnesses: Vec<&Witness> = witnesses
            .iter()
            .filter(|w| w.is_active)
            .collect();
        
        if active_witnesses.is_empty() {
            return Err(ConsensusError::InsufficientVotingPower);
        }
        
        let index = (block_num as usize % active_witnesses.len()) as usize;
        Ok(active_witnesses[index].address)
    }
    
    // 创建DPoS区块
    pub async fn create_dpos_block(
        &self,
        witness: Address,
        transactions: Vec<Transaction>,
        previous_hash: BlockHash,
        private_key: &PrivateKey,
    ) -> Result<PoSBlock, ConsensusError> {
        // 验证witness是否活跃
        let witnesses = self.witnesses.read().await;
        let witness_info = witnesses.iter()
            .find(|w| w.address == witness && w.is_active)
            .ok_or(ConsensusError::InvalidProof)?;
        
        drop(witnesses);
        
        // 创建区块（复用PoS区块结构）
        let index = 0;
        let timestamp = current_timestamp();
        let merkle_root = Self::calculate_merkle_root(&transactions);
        
        let hash = Self::calculate_block_hash(
            index,
            timestamp,
            &previous_hash,
            &merkle_root,
            &witness,
        );
        
        let signature = private_key.sign(&hash)?;
        
        // 更新见证人统计
        let mut witnesses = self.witnesses.write().await;
        if let Some(w) = witnesses.iter_mut().find(|w| w.address == witness) {
            w.blocks_produced += 1;
        }
        
        Ok(PoSBlock {
            index,
            timestamp,
            transactions,
            previous_hash,
            merkle_root,
            validator: witness,
            signature,
            hash,
        })
    }
    
    // 处理错过的区块
    pub async fn handle_missed_block(&self, witness: Address) -> Result<(), ConsensusError> {
        let mut witnesses = self.witnesses.write().await;
        
        if let Some(w) = witnesses.iter_mut().find(|w| w.address == witness) {
            w.missed_blocks += 1;
            
            // 如果错过太多区块，取消活跃状态
            if w.missed_blocks > 10 {
                w.is_active = false;
                println!("Witness {} deactivated due to missed blocks", hex::encode(witness));
            }
        }
        
        Ok(())
    }
    
    fn calculate_merkle_root(transactions: &[Transaction]) -> BlockHash {
        if transactions.is_empty() {
            return [0u8; 32];
        }
        
        let mut hashes: Vec<[u8; 32]> = transactions
            .iter()
            .map(|tx| tx.id)
            .collect();
        
        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();
            
            for chunk in hashes.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(&chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(&chunk[1]);
                } else {
                    hasher.update(&chunk[0]);
                }
                new_hashes.push(hasher.finalize().into());
            }
            
            hashes = new_hashes;
        }
        
        hashes[0]
    }
    
    fn calculate_block_hash(
        index: u64,
        timestamp: u64,
        previous_hash: &BlockHash,
        merkle_root: &BlockHash,
        witness: &Address,
    ) -> BlockHash {
        let mut hasher = Sha256::new();
        
        hasher.update(&index.to_le_bytes());
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(previous_hash);
        hasher.update(merkle_root);
        hasher.update(witness);
        
        hasher.finalize().into()
    }
}
```

## 5. 实用拜占庭容错(PBFT)

### 5.1 PBFT实现

```rust
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct PBFT {
    config: PBFTConfig,
    node_id: NodeId,
    nodes: Vec<NodeId>,
    view: Arc<RwLock<u64>>,
    sequence_number: Arc<RwLock<u64>>,
    states: Arc<RwLock<HashMap<u64, ConsensusState>>>,
}

#[derive(Debug, Clone)]
pub struct PBFTConfig {
    pub f: usize,  // 最大容错节点数
    pub timeout: Duration,
    pub checkpoint_interval: u64,
}

#[derive(Debug, Clone)]
struct ConsensusState {
    sequence: u64,
    view: u64,
    pre_prepare: Option<PrePrepareMessage>,
    prepares: Vec<PrepareMessage>,
    commits: Vec<CommitMessage>,
    committed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrePrepareMessage {
    pub view: u64,
    pub sequence: u64,
    pub digest: BlockHash,
    pub block: Block,
    pub signature: Signature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepareMessage {
    pub view: u64,
    pub sequence: u64,
    pub digest: BlockHash,
    pub node_id: NodeId,
    pub signature: Signature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitMessage {
    pub view: u64,
    pub sequence: u64,
    pub digest: BlockHash,
    pub node_id: NodeId,
    pub signature: Signature,
}

impl PBFT {
    pub fn new(config: PBFTConfig, node_id: NodeId, nodes: Vec<NodeId>) -> Self {
        Self {
            config,
            node_id,
            nodes,
            view: Arc::new(RwLock::new(0)),
            sequence_number: Arc::new(RwLock::new(0)),
            states: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    // 主节点：发起pre-prepare
    pub async fn pre_prepare(
        &self,
        block: Block,
        private_key: &PrivateKey,
    ) -> Result<PrePrepareMessage, ConsensusError> {
        let view = *self.view.read().await;
        let mut sequence = self.sequence_number.write().await;
        *sequence += 1;
        let seq = *sequence;
        
        let digest = Self::calculate_digest(&block);
        let message_bytes = Self::pre_prepare_bytes(view, seq, &digest);
        let signature = private_key.sign(&message_bytes)?;
        
        let pre_prepare = PrePrepareMessage {
            view,
            sequence: seq,
            digest,
            block,
            signature,
        };
        
        // 保存状态
        let mut states = self.states.write().await;
        states.insert(seq, ConsensusState {
            sequence: seq,
            view,
            pre_prepare: Some(pre_prepare.clone()),
            prepares: Vec::new(),
            commits: Vec::new(),
            committed: false,
        });
        
        Ok(pre_prepare)
    }
    
    // 副本节点：接收pre-prepare并发送prepare
    pub async fn on_pre_prepare(
        &self,
        pre_prepare: PrePrepareMessage,
        private_key: &PrivateKey,
    ) -> Result<PrepareMessage, ConsensusError> {
        // 验证pre-prepare消息
        self.verify_pre_prepare(&pre_prepare).await?;
        
        let digest = Self::calculate_digest(&pre_prepare.block);
        let message_bytes = Self::prepare_bytes(
            pre_prepare.view,
            pre_prepare.sequence,
            &digest,
        );
        let signature = private_key.sign(&message_bytes)?;
        
        let prepare = PrepareMessage {
            view: pre_prepare.view,
            sequence: pre_prepare.sequence,
            digest,
            node_id: self.node_id,
            signature,
        };
        
        // 更新状态
        let mut states = self.states.write().await;
        if let Some(state) = states.get_mut(&pre_prepare.sequence) {
            state.pre_prepare = Some(pre_prepare);
        }
        
        Ok(prepare)
    }
    
    // 接收prepare消息
    pub async fn on_prepare(
        &self,
        prepare: PrepareMessage,
    ) -> Result<Option<CommitMessage>, ConsensusError> {
        // 验证prepare消息
        self.verify_prepare(&prepare).await?;
        
        // 保存prepare消息
        let mut states = self.states.write().await;
        if let Some(state) = states.get_mut(&prepare.sequence) {
            state.prepares.push(prepare.clone());
            
            // 检查是否收到足够的prepare消息 (2f)
            if state.prepares.len() >= 2 * self.config.f {
                // 进入prepared状态，发送commit
                drop(states);
                return self.send_commit(prepare.view, prepare.sequence, prepare.digest).await;
            }
        }
        
        Ok(None)
    }
    
    // 发送commit消息
    async fn send_commit(
        &self,
        view: u64,
        sequence: u64,
        digest: BlockHash,
    ) -> Result<Option<CommitMessage>, ConsensusError> {
        let message_bytes = Self::commit_bytes(view, sequence, &digest);
        let private_key = PrivateKey::default(); // 应该从配置获取
        let signature = private_key.sign(&message_bytes)?;
        
        let commit = CommitMessage {
            view,
            sequence,
            digest,
            node_id: self.node_id,
            signature,
        };
        
        Ok(Some(commit))
    }
    
    // 接收commit消息
    pub async fn on_commit(
        &self,
        commit: CommitMessage,
    ) -> Result<bool, ConsensusError> {
        // 验证commit消息
        self.verify_commit(&commit).await?;
        
        // 保存commit消息
        let mut states = self.states.write().await;
        if let Some(state) = states.get_mut(&commit.sequence) {
            state.commits.push(commit.clone());
            
            // 检查是否收到足够的commit消息 (2f + 1)
            if state.commits.len() >= 2 * self.config.f + 1 {
                state.committed = true;
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    // 获取主节点
    pub async fn get_primary(&self) -> NodeId {
        let view = *self.view.read().await;
        let index = (view as usize) % self.nodes.len();
        self.nodes[index]
    }
    
    // 是否是主节点
    pub async fn is_primary(&self) -> bool {
        self.get_primary().await == self.node_id
    }
    
    // 视图切换
    pub async fn view_change(&self) -> Result<(), ConsensusError> {
        let mut view = self.view.write().await;
        *view += 1;
        println!("View changed to {}", *view);
        Ok(())
    }
    
    // 验证消息
    async fn verify_pre_prepare(&self, pre_prepare: &PrePrepareMessage) -> Result<(), ConsensusError> {
        let view = *self.view.read().await;
        if pre_prepare.view != view {
            return Err(ConsensusError::InvalidProof);
        }
        
        // 验证摘要
        let digest = Self::calculate_digest(&pre_prepare.block);
        if digest != pre_prepare.digest {
            return Err(ConsensusError::InvalidProof);
        }
        
        Ok(())
    }
    
    async fn verify_prepare(&self, prepare: &PrepareMessage) -> Result<(), ConsensusError> {
        let view = *self.view.read().await;
        if prepare.view != view {
            return Err(ConsensusError::InvalidProof);
        }
        
        Ok(())
    }
    
    async fn verify_commit(&self, commit: &CommitMessage) -> Result<(), ConsensusError> {
        let view = *self.view.read().await;
        if commit.view != view {
            return Err(ConsensusError::InvalidProof);
        }
        
        Ok(())
    }
    
    // 辅助函数
    fn calculate_digest(block: &Block) -> BlockHash {
        let mut hasher = Sha256::new();
        hasher.update(&bincode::serialize(block).unwrap());
        hasher.finalize().into()
    }
    
    fn pre_prepare_bytes(view: u64, sequence: u64, digest: &BlockHash) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&view.to_le_bytes());
        bytes.extend_from_slice(&sequence.to_le_bytes());
        bytes.extend_from_slice(digest);
        bytes.extend_from_slice(b"pre-prepare");
        bytes
    }
    
    fn prepare_bytes(view: u64, sequence: u64, digest: &BlockHash) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&view.to_le_bytes());
        bytes.extend_from_slice(&sequence.to_le_bytes());
        bytes.extend_from_slice(digest);
        bytes.extend_from_slice(b"prepare");
        bytes
    }
    
    fn commit_bytes(view: u64, sequence: u64, digest: &BlockHash) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&view.to_le_bytes());
        bytes.extend_from_slice(&sequence.to_le_bytes());
        bytes.extend_from_slice(digest);
        bytes.extend_from_slice(b"commit");
        bytes
    }
}
```

## 6. Raft共识

### 6.1 Raft实现（简化版）

```rust
#[derive(Debug, Clone)]
pub struct Raft {
    node_id: NodeId,
    nodes: Vec<NodeId>,
    state: Arc<RwLock<RaftState>>,
    current_term: Arc<RwLock<u64>>,
    voted_for: Arc<RwLock<Option<NodeId>>>,
    log: Arc<RwLock<Vec<LogEntry>>>,
    commit_index: Arc<RwLock<u64>>,
    last_applied: Arc<RwLock<u64>>,
}

#[derive(Debug, Clone, PartialEq)]
enum RaftState {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LogEntry {
    term: u64,
    index: u64,
    command: Vec<u8>,
}

impl Raft {
    pub fn new(node_id: NodeId, nodes: Vec<NodeId>) -> Self {
        Self {
            node_id,
            nodes,
            state: Arc::new(RwLock::new(RaftState::Follower)),
            current_term: Arc::new(RwLock::new(0)),
            voted_for: Arc::new(RwLock::new(None)),
            log: Arc::new(RwLock::new(Vec::new())),
            commit_index: Arc::new(RwLock::new(0)),
            last_applied: Arc::new(RwLock::new(0)),
        }
    }
    
    // 请求投票
    pub async fn request_vote(
        &self,
        term: u64,
        candidate_id: NodeId,
        last_log_index: u64,
        last_log_term: u64,
    ) -> Result<bool, ConsensusError> {
        let mut current_term = self.current_term.write().await;
        
        if term < *current_term {
            return Ok(false);
        }
        
        if term > *current_term {
            *current_term = term;
            *self.state.write().await = RaftState::Follower;
            *self.voted_for.write().await = None;
        }
        
        let mut voted_for = self.voted_for.write().await;
        
        if voted_for.is_some() && *voted_for != Some(candidate_id) {
            return Ok(false);
        }
        
        // 检查日志是否至少和自己一样新
        let log = self.log.read().await;
        let my_last_index = log.len() as u64;
        let my_last_term = log.last().map(|e| e.term).unwrap_or(0);
        
        if last_log_term < my_last_term || 
           (last_log_term == my_last_term && last_log_index < my_last_index) {
            return Ok(false);
        }
        
        *voted_for = Some(candidate_id);
        Ok(true)
    }
    
    // 追加日志
    pub async fn append_entries(
        &self,
        term: u64,
        leader_id: NodeId,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<LogEntry>,
        leader_commit: u64,
    ) -> Result<bool, ConsensusError> {
        let mut current_term = self.current_term.write().await;
        
        if term < *current_term {
            return Ok(false);
        }
        
        if term > *current_term {
            *current_term = term;
            *self.state.write().await = RaftState::Follower;
            *self.voted_for.write().await = None;
        }
        
        // 检查日志一致性
        let mut log = self.log.write().await;
        
        if prev_log_index > 0 {
            if log.len() < prev_log_index as usize {
                return Ok(false);
            }
            
            if log[prev_log_index as usize - 1].term != prev_log_term {
                return Ok(false);
            }
        }
        
        // 追加新日志
        if !entries.is_empty() {
            log.truncate(prev_log_index as usize);
            log.extend(entries);
        }
        
        // 更新commit index
        if leader_commit > *self.commit_index.read().await {
            let mut commit_index = self.commit_index.write().await;
            *commit_index = leader_commit.min(log.len() as u64);
        }
        
        Ok(true)
    }
}
```

## 7. 混合共识

### 7.1 PoW + PoS混合

```rust
#[derive(Debug, Clone)]
pub struct HybridConsensus {
    pow: ProofOfWork,
    pos: ProofOfStake,
    config: HybridConfig,
}

#[derive(Debug, Clone)]
pub struct HybridConfig {
    pub pow_blocks: u64,  // PoW区块数
    pub pos_blocks: u64,  // PoS区块数
    pub transition_threshold: f64,  // 转换阈值
}

impl HybridConsensus {
    pub fn new(
        pow_config: PoWConfig,
        pos_config: PoSConfig,
        hybrid_config: HybridConfig,
    ) -> Self {
        Self {
            pow: ProofOfWork::new(pow_config),
            pos: ProofOfStake::new(pos_config),
            config: hybrid_config,
        }
    }
    
    // 根据区块高度选择共识算法
    pub async fn select_consensus(&self, block_height: u64) -> ConsensusType {
        let cycle_length = self.config.pow_blocks + self.config.pos_blocks;
        let position = block_height % cycle_length;
        
        if position < self.config.pow_blocks {
            ConsensusType::PoW
        } else {
            ConsensusType::PoS
        }
    }
    
    pub async fn create_hybrid_block(
        &self,
        block_height: u64,
        transactions: Vec<Transaction>,
        previous_hash: BlockHash,
    ) -> Result<HybridBlock, ConsensusError> {
        match self.select_consensus(block_height).await {
            ConsensusType::PoW => {
                let pow_block = self.pow.mine_block(block_height, transactions, previous_hash).await?;
                Ok(HybridBlock::PoW(pow_block))
            }
            ConsensusType::PoS => {
                let validator = self.pos.select_next_validator().await?;
                let private_key = PrivateKey::default(); // 应从配置获取
                let pos_block = self.pos.create_pos_block(
                    validator,
                    transactions,
                    previous_hash,
                    &private_key
                ).await?;
                Ok(HybridBlock::PoS(pos_block))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum HybridBlock {
    PoW(PoWBlock),
    PoS(PoSBlock),
}

#[derive(Debug, Clone)]
pub enum ConsensusType {
    PoW,
    PoS,
}
```

## 8. 性能对比与选择

### 8.1 共识算法对比

| 算法 | TPS | 最终确认时间 | 能耗 | 去中心化程度 | 适用场景 |
|------|-----|------------|------|------------|---------|
| PoW | 7-10 | 60分钟 | 非常高 | 高 | 公链、高安全需求 |
| PoS | 100-1000 | 1-5分钟 | 低 | 中高 | 公链、节能需求 |
| DPoS | 1000-10000 | 秒级 | 非常低 | 中 | 高性能应用 |
| PBFT | 1000-5000 | 秒级 | 低 | 低 | 联盟链、金融 |
| Raft | 5000-10000 | 秒级 | 低 | 低 | 联盟链、私有链 |

### 8.2 选择建议

```rust
pub fn recommend_consensus(requirements: &ConsensusRequirements) -> ConsensusType {
    match requirements {
        // 公链、高安全
        _ if requirements.decentralization > 0.8 && requirements.security > 0.9 => {
            ConsensusType::PoW
        }
        // 公链、节能
        _ if requirements.decentralization > 0.7 && requirements.energy_efficiency > 0.8 => {
            ConsensusType::PoS
        }
        // 高性能、中等去中心化
        _ if requirements.tps > 1000 && requirements.decentralization > 0.5 => {
            ConsensusType::DPoS
        }
        // 联盟链、强一致性
        _ if requirements.consistency > 0.9 && requirements.nodes < 100 => {
            ConsensusType::PBFT
        }
        // 默认
        _ => ConsensusType::PoS
    }
}

pub struct ConsensusRequirements {
    pub tps: u64,
    pub decentralization: f64,  // 0-1
    pub security: f64,  // 0-1
    pub energy_efficiency: f64,  // 0-1
    pub consistency: f64,  // 0-1
    pub nodes: usize,
}
```

## 9. 总结

本文档实现了主流共识算法：

1. **PoW**: 高安全性、去中心化，但能耗高
2. **PoS**: 节能、高效，但需要质押机制
3. **DPoS**: 高性能、低延迟，但去中心化程度较低
4. **PBFT**: 强一致性、高性能，适合联盟链
5. **Raft**: 简单易用，适合私有链

每种算法都有其适用场景，需根据具体需求选择。

---

**文档版本**: v1.0.0  
**最后更新**: 2025年10月15日  
**作者**: 共识算法专家  
**审核**: 区块链架构师
