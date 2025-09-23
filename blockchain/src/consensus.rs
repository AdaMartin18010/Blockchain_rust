//! # 共识算法模块
//! 
//! 实现多种共识算法：PoW, PoS, DPoS, PBFT
//! Implements multiple consensus algorithms: PoW, PoS, DPoS, PBFT

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rand::Rng;

use crate::simple_blockchain::{Blockchain, Block, Transaction, BlockHash};

/// 共识算法类型
/// Consensus algorithm types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub enum ConsensusType {
    ProofOfWork,    // 工作量证明
    ProofOfStake,   // 权益证明
    DelegatedProofOfStake, // 委托权益证明
    PracticalByzantineFaultTolerance, // 实用拜占庭容错
}

/// 共识配置
/// Consensus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ConsensusConfig {
    pub consensus_type: ConsensusType,
    pub difficulty: usize,
    pub block_time: Duration,
    pub stake_threshold: u64,  // 权益阈值
    pub delegate_count: usize, // 委托者数量
    pub byzantine_threshold: usize, // 拜占庭容错阈值
}

/// 验证者信息
/// Validator information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Validator {
    pub address: String,
    pub stake: u64,
    pub voting_power: u64,
    pub is_active: bool,
    pub last_block_time: u64,
}

/// 委托者信息
/// Delegate information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Delegate {
    pub address: String,
    pub votes: u64,
    pub productivity: f64,
    pub is_active: bool,
    pub block_count: u64,
}

/// PBFT 消息类型
/// PBFT message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum PBFTMessage {
    PrePrepare {
        view: u64,
        sequence: u64,
        block_hash: BlockHash<32>,
        sender: String,
    },
    Prepare {
        view: u64,
        sequence: u64,
        block_hash: BlockHash<32>,
        sender: String,
    },
    Commit {
        view: u64,
        sequence: u64,
        block_hash: BlockHash<32>,
        sender: String,
    },
    ViewChange {
        view: u64,
        sequence: u64,
        sender: String,
    },
}

/// 共识引擎
/// Consensus engine
#[allow(dead_code)]
pub struct ConsensusEngine {
    config: ConsensusConfig,
    validators: HashMap<String, Validator>,
    delegates: HashMap<String, Delegate>,
    current_view: u64,
    sequence_number: u64,
    prepared_blocks: HashMap<(u64, u64), BlockHash<32>>,
    committed_blocks: HashMap<(u64, u64), BlockHash<32>>,
}

#[allow(dead_code)]
impl ConsensusEngine {
    /// 创建新的共识引擎
    /// Create new consensus engine
    pub fn new(config: ConsensusConfig) -> Self {
        Self {
            config,
            validators: HashMap::new(),
            delegates: HashMap::new(),
            current_view: 0,
            sequence_number: 0,
            prepared_blocks: HashMap::new(),
            committed_blocks: HashMap::new(),
        }
    }

    /// 添加验证者
    /// Add validator
    #[allow(dead_code)]
    pub fn add_validator(&mut self, validator: Validator) {
        self.validators.insert(validator.address.clone(), validator);
    }

    /// 添加委托者
    /// Add delegate
    #[allow(dead_code)]
    pub fn add_delegate(&mut self, delegate: Delegate) {
        self.delegates.insert(delegate.address.clone(), delegate);
    }

    /// 选择下一个区块生产者
    /// Select next block producer
    pub fn select_block_producer(&self, blockchain: &Blockchain) -> Result<String, String> {
        match self.config.consensus_type {
            ConsensusType::ProofOfWork => {
                self.select_pow_producer()
            }
            ConsensusType::ProofOfStake => {
                self.select_pos_producer(blockchain)
            }
            ConsensusType::DelegatedProofOfStake => {
                self.select_dpos_producer()
            }
            ConsensusType::PracticalByzantineFaultTolerance => {
                self.select_pbft_producer()
            }
        }
    }

    /// 验证区块
    /// Validate block
    pub fn validate_block(&self, block: &Block, blockchain: &Blockchain) -> Result<(), String> {
        match self.config.consensus_type {
            ConsensusType::ProofOfWork => {
                self.validate_pow_block(block)
            }
            ConsensusType::ProofOfStake => {
                self.validate_pos_block(block, blockchain)
            }
            ConsensusType::DelegatedProofOfStake => {
                self.validate_dpos_block(block)
            }
            ConsensusType::PracticalByzantineFaultTolerance => {
                self.validate_pbft_block(block)
            }
        }
    }

    /// 处理 PBFT 消息
    /// Handle PBFT message
    pub fn handle_pbft_message(&mut self, message: PBFTMessage) -> Result<Vec<PBFTMessage>, String> {
        match message {
            PBFTMessage::PrePrepare { view, sequence, block_hash, sender } => {
                self.handle_preprepare(view, sequence, block_hash, sender)
            }
            PBFTMessage::Prepare { view, sequence, block_hash, sender } => {
                self.handle_prepare(view, sequence, block_hash, sender)
            }
            PBFTMessage::Commit { view, sequence, block_hash, sender } => {
                self.handle_commit(view, sequence, block_hash, sender)
            }
            PBFTMessage::ViewChange { view, sequence, sender } => {
                self.handle_view_change(view, sequence, sender)
            }
        }
    }

    /// 获取共识统计
    /// Get consensus statistics
    pub fn get_consensus_stats(&self) -> ConsensusStats {
        ConsensusStats {
            consensus_type: self.config.consensus_type.clone(),
            total_validators: self.validators.len(),
            active_validators: self.validators.values().filter(|v| v.is_active).count(),
            total_delegates: self.delegates.len(),
            active_delegates: self.delegates.values().filter(|d| d.is_active).count(),
            current_view: self.current_view,
            sequence_number: self.sequence_number,
            total_stake: self.validators.values().map(|v| v.stake).sum(),
            total_votes: self.delegates.values().map(|d| d.votes).sum(),
        }
    }

    // PoW 相关方法
    fn select_pow_producer(&self) -> Result<String, String> {
        // PoW 中任何节点都可以尝试挖矿
        Ok("miner".to_string())
    }

    fn validate_pow_block(&self, block: &Block) -> Result<(), String> {
        // 验证工作量证明
        if !block.hash.meets_difficulty(self.config.difficulty) {
            return Err("Invalid proof of work".to_string());
        }
        Ok(())
    }

    // PoS 相关方法
    #[allow(unused_variables)]
    fn select_pos_producer(&self, blockchain: &Blockchain) -> Result<String, String> {
        let total_stake: u64 = self.validators.values().map(|v| v.stake).sum();
        if total_stake == 0 {
            return Err("No validators with stake".to_string());
        }

        let mut rng = rand::rng();
        let random_value = rng.random_range(0..total_stake);
        
        let mut current_stake = 0;
        for validator in self.validators.values() {
            if validator.is_active && validator.stake > 0 {
                current_stake += validator.stake;
                if current_stake > random_value {
                    return Ok(validator.address.clone());
                }
            }
        }

        Err("No valid validator found".to_string())
    }

    fn validate_pos_block(&self, block: &Block, blockchain: &Blockchain) -> Result<(), String> {
        // 验证权益证明
        if let Some(validator) = self.validators.get(&block.transactions[0].sender) {
            if !validator.is_active || validator.stake < self.config.stake_threshold {
                return Err("Invalid validator or insufficient stake".to_string());
            }
        } else {
            return Err("Validator not found".to_string());
        }

        // 验证时间间隔
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if let Some(last_block) = blockchain.get_latest_block() {
            let time_diff = current_time - last_block.timestamp;
            if time_diff < self.config.block_time.as_secs() {
                return Err("Block time too short".to_string());
            }
        }

        Ok(())
    }

    // DPoS 相关方法
    fn select_dpos_producer(&self) -> Result<String, String> {
        let active_delegates: Vec<&Delegate> = self.delegates
            .values()
            .filter(|d| d.is_active)
            .collect();

        if active_delegates.is_empty() {
            return Err("No active delegates".to_string());
        }

        // 按投票数排序，选择前 N 个委托者
        let mut sorted_delegates = active_delegates;
        sorted_delegates.sort_by(|a, b| b.votes.cmp(&a.votes));

        let delegate_count = std::cmp::min(self.config.delegate_count, sorted_delegates.len());
        let mut rng = rand::rng();
        let selected_index = rng.random_range(0..delegate_count);

        Ok(sorted_delegates[selected_index].address.clone())
    }

    fn validate_dpos_block(&self, block: &Block) -> Result<(), String> {
        // 验证委托者权限
        if let Some(delegate) = self.delegates.get(&block.transactions[0].sender) {
            if !delegate.is_active {
                return Err("Delegate is not active".to_string());
            }
        } else {
            return Err("Delegate not found".to_string());
        }

        Ok(())
    }

    // PBFT 相关方法
    fn select_pbft_producer(&self) -> Result<String, String> {
        let active_validators: Vec<&Validator> = self.validators
            .values()
            .filter(|v| v.is_active)
            .collect();

        if active_validators.len() < 3 * self.config.byzantine_threshold + 1 {
            return Err("Not enough validators for PBFT".to_string());
        }

        // 轮询选择主节点
        let primary_index = (self.current_view % active_validators.len() as u64) as usize;
        Ok(active_validators[primary_index].address.clone())
    }

    fn validate_pbft_block(&self, block: &Block) -> Result<(), String> {
        // PBFT 区块验证
        if let Some(validator) = self.validators.get(&block.transactions[0].sender) {
            if !validator.is_active {
                return Err("Validator is not active".to_string());
            }
        } else {
            return Err("Validator not found".to_string());
        }

        Ok(())
    }

    fn handle_preprepare(&mut self, view: u64, sequence: u64, block_hash: BlockHash<32>, sender: String) -> Result<Vec<PBFTMessage>, String> {
        if view != self.current_view {
            return Err("Invalid view number".to_string());
        }

        if sequence <= self.sequence_number {
            return Err("Invalid sequence number".to_string());
        }

        // 验证发送者是主节点
        if !self.is_primary(&sender) {
            return Err("Not primary node".to_string());
        }

        self.sequence_number = sequence;
        self.prepared_blocks.insert((view, sequence), block_hash.clone());

        // 发送 Prepare 消息
        let prepare_messages: Vec<PBFTMessage> = self.validators
            .keys()
            .filter(|addr| *addr != &sender)
            .map(|addr| PBFTMessage::Prepare {
                view,
                sequence,
                block_hash: block_hash.clone(),
                sender: addr.clone(),
            })
            .collect();

        Ok(prepare_messages)
    }

    fn handle_prepare(&mut self, view: u64, sequence: u64, block_hash: BlockHash<32>, sender: String) -> Result<Vec<PBFTMessage>, String> {
        if !self.validators.contains_key(&sender) {
            return Err("Unknown validator".to_string());
        }

        // 统计 Prepare 消息
        let prepare_count = self.prepared_blocks
            .values()
            .filter(|hash| **hash == block_hash)
            .count();

        if prepare_count >= 2 * self.config.byzantine_threshold + 1 {
            // 发送 Commit 消息
            let commit_messages: Vec<PBFTMessage> = self.validators
                .keys()
                .map(|addr| PBFTMessage::Commit {
                    view,
                    sequence,
                    block_hash: block_hash.clone(),
                    sender: addr.clone(),
                })
                .collect();

            return Ok(commit_messages);
        }

        Ok(vec![])
    }

    fn handle_commit(&mut self, view: u64, sequence: u64, block_hash: BlockHash<32>, sender: String) -> Result<Vec<PBFTMessage>, String> {
        if !self.validators.contains_key(&sender) {
            return Err("Unknown validator".to_string());
        }

        // 统计 Commit 消息
        let commit_count = self.committed_blocks
            .values()
            .filter(|hash| **hash == block_hash)
            .count();

        if commit_count >= 2 * self.config.byzantine_threshold + 1 {
            // 区块可以提交
            self.committed_blocks.insert((view, sequence), block_hash);
            return Ok(vec![]);
        }

        Ok(vec![])
    }

    fn handle_view_change(&mut self, view: u64, sequence: u64, sender: String) -> Result<Vec<PBFTMessage>, String> {
        if !self.validators.contains_key(&sender) {
            return Err("Unknown validator".to_string());
        }

        if view > self.current_view {
            self.current_view = view;
            self.sequence_number = sequence;
        }

        Ok(vec![])
    }

    fn is_primary(&self, address: &str) -> bool {
        let active_validators: Vec<&Validator> = self.validators
            .values()
            .filter(|v| v.is_active)
            .collect();

        if active_validators.is_empty() {
            return false;
        }

        let primary_index = (self.current_view % active_validators.len() as u64) as usize;
        active_validators[primary_index].address == address
    }
}

/// 共识统计信息
/// Consensus statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusStats {
    pub consensus_type: ConsensusType,
    pub total_validators: usize,
    pub active_validators: usize,
    pub total_delegates: usize,
    pub active_delegates: usize,
    pub current_view: u64,
    pub sequence_number: u64,
    pub total_stake: u64,
    pub total_votes: u64,
}

/// 共识管理器
/// Consensus manager
#[allow(dead_code)]
pub struct ConsensusManager {
    engine: ConsensusEngine,
    blockchain: Blockchain,
    last_block_time: u64,
}

#[allow(dead_code)]
impl ConsensusManager {
    /// 创建新的共识管理器
    /// Create new consensus manager
    pub fn new(config: ConsensusConfig) -> Self {
        let blockchain = Blockchain::new(config.difficulty);
        Self {
            engine: ConsensusEngine::new(config),
            blockchain,
            last_block_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// 添加交易
    /// Add transaction
    #[allow(dead_code)]
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        self.blockchain.add_transaction(transaction)
    }

    /// 生成新区块
    /// Generate new block
    #[allow(dead_code)]
    pub fn generate_block(&mut self) -> Result<Block, String> {
        match self.engine.config.consensus_type {
            ConsensusType::ProofOfWork => {
                self.generate_pow_block()
            }
            ConsensusType::ProofOfStake => {
                self.generate_pos_block()
            }
            ConsensusType::DelegatedProofOfStake => {
                self.generate_dpos_block()
            }
            ConsensusType::PracticalByzantineFaultTolerance => {
                self.generate_pbft_block()
            }
        }
    }

    /// 验证并添加区块
    /// Validate and add block
    pub fn validate_and_add_block(&mut self, block: Block) -> Result<(), String> {
        // 验证区块
        self.engine.validate_block(&block, &self.blockchain)?;

        // 验证区块索引和前一个区块哈希
        if let Some(last_block) = self.blockchain.get_latest_block() {
            if block.index != last_block.index + 1 {
                return Err("Invalid block index".to_string());
            }
            if block.prev_hash != last_block.hash {
                return Err("Invalid previous hash".to_string());
            }
        }

        // 添加到区块链
        self.blockchain.chain.push(block.clone());
        self.blockchain.update_balances();
        self.blockchain.pending_transactions.clear();

        self.last_block_time = block.timestamp;
        Ok(())
    }

    /// 获取区块链
    /// Get blockchain
    pub fn get_blockchain(&self) -> &Blockchain {
        &self.blockchain
    }

    /// 获取共识统计
    /// Get consensus statistics
    pub fn get_consensus_stats(&self) -> ConsensusStats {
        self.engine.get_consensus_stats()
    }

    /// 处理 PBFT 消息
    /// Handle PBFT message
    pub fn handle_pbft_message(&mut self, message: PBFTMessage) -> Result<Vec<PBFTMessage>, String> {
        self.engine.handle_pbft_message(message)
    }

    /// 添加验证者
    /// Add validator
    pub fn add_validator(&mut self, validator: Validator) {
        self.engine.add_validator(validator);
    }

    /// 添加委托者
    /// Add delegate
    pub fn add_delegate(&mut self, delegate: Delegate) {
        self.engine.add_delegate(delegate);
    }

    // PoW 区块生成
    fn generate_pow_block(&mut self) -> Result<Block, String> {
        let miner_address = self.engine.select_block_producer(&self.blockchain)?;
        self.blockchain.mine_pending_transactions(miner_address)?;
        
        if let Some(block) = self.blockchain.get_latest_block() {
            Ok(block.clone())
        } else {
            Err("Failed to generate block".to_string())
        }
    }

    // PoS 区块生成
    fn generate_pos_block(&mut self) -> Result<Block, String> {
        let validator_address = self.engine.select_block_producer(&self.blockchain)?;
        
        // 创建权益证明区块
        let mut block = Block::new(
            self.blockchain.chain.len() as u64,
            self.blockchain.get_latest_block().unwrap().hash.clone(),
            self.blockchain.pending_transactions.clone(),
            self.engine.config.difficulty,
        );

        // 添加权益证明交易
        let stake_transaction = Transaction::new(
            "system".to_string(),
            validator_address,
            0, // 权益证明不需要金额
        );
        block.transactions.insert(0, stake_transaction);

        // 计算区块哈希
        block.hash = block.calculate_hash();

        Ok(block)
    }

    // DPoS 区块生成
    fn generate_dpos_block(&mut self) -> Result<Block, String> {
        let delegate_address = self.engine.select_block_producer(&self.blockchain)?;
        
        // 创建委托权益证明区块
        let mut block = Block::new(
            self.blockchain.chain.len() as u64,
            self.blockchain.get_latest_block().unwrap().hash.clone(),
            self.blockchain.pending_transactions.clone(),
            self.engine.config.difficulty,
        );

        // 添加委托者交易
        let delegate_transaction = Transaction::new(
            "system".to_string(),
            delegate_address,
            0,
        );
        block.transactions.insert(0, delegate_transaction);

        // 计算区块哈希
        block.hash = block.calculate_hash();

        Ok(block)
    }

    // PBFT 区块生成
    fn generate_pbft_block(&mut self) -> Result<Block, String> {
        let primary_address = self.engine.select_block_producer(&self.blockchain)?;
        
        // 创建 PBFT 区块
        let mut block = Block::new(
            self.blockchain.chain.len() as u64,
            self.blockchain.get_latest_block().unwrap().hash.clone(),
            self.blockchain.pending_transactions.clone(),
            self.engine.config.difficulty,
        );

        // 添加主节点交易
        let primary_transaction = Transaction::new(
            "system".to_string(),
            primary_address,
            0,
        );
        block.transactions.insert(0, primary_transaction);

        // 计算区块哈希
        block.hash = block.calculate_hash();

        Ok(block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_consensus_engine_creation() {
        let config = ConsensusConfig {
            consensus_type: ConsensusType::ProofOfWork,
            difficulty: 2,
            block_time: Duration::from_secs(10),
            stake_threshold: 1000,
            delegate_count: 21,
            byzantine_threshold: 1,
        };

        let engine = ConsensusEngine::new(config);
        assert_eq!(engine.current_view, 0);
        assert_eq!(engine.sequence_number, 0);
    }

    #[test]
    fn test_validator_management() {
        let mut engine = ConsensusEngine::new(ConsensusConfig {
            consensus_type: ConsensusType::ProofOfStake,
            difficulty: 2,
            block_time: Duration::from_secs(10),
            stake_threshold: 1000,
            delegate_count: 21,
            byzantine_threshold: 1,
        });

        let validator = Validator {
            address: "validator1".to_string(),
            stake: 5000,
            voting_power: 5000,
            is_active: true,
            last_block_time: 0,
        };

        engine.add_validator(validator);
        assert_eq!(engine.validators.len(), 1);
        assert_eq!(engine.validators["validator1"].stake, 5000);
    }

    #[test]
    fn test_delegate_management() {
        let mut engine = ConsensusEngine::new(ConsensusConfig {
            consensus_type: ConsensusType::DelegatedProofOfStake,
            difficulty: 2,
            block_time: Duration::from_secs(10),
            stake_threshold: 1000,
            delegate_count: 21,
            byzantine_threshold: 1,
        });

        let delegate = Delegate {
            address: "delegate1".to_string(),
            votes: 10000,
            productivity: 0.95,
            is_active: true,
            block_count: 100,
        };

        engine.add_delegate(delegate);
        assert_eq!(engine.delegates.len(), 1);
        assert_eq!(engine.delegates["delegate1"].votes, 10000);
    }

    #[test]
    fn test_consensus_stats() {
        let mut engine = ConsensusEngine::new(ConsensusConfig {
            consensus_type: ConsensusType::ProofOfStake,
            difficulty: 2,
            block_time: Duration::from_secs(10),
            stake_threshold: 1000,
            delegate_count: 21,
            byzantine_threshold: 1,
        });

        engine.add_validator(Validator {
            address: "validator1".to_string(),
            stake: 5000,
            voting_power: 5000,
            is_active: true,
            last_block_time: 0,
        });

        engine.add_delegate(Delegate {
            address: "delegate1".to_string(),
            votes: 10000,
            productivity: 0.95,
            is_active: true,
            block_count: 100,
        });

        let stats = engine.get_consensus_stats();
        assert_eq!(stats.total_validators, 1);
        assert_eq!(stats.total_delegates, 1);
        assert_eq!(stats.total_stake, 5000);
        assert_eq!(stats.total_votes, 10000);
    }

    #[test]
    fn test_consensus_manager() {
        let config = ConsensusConfig {
            consensus_type: ConsensusType::ProofOfWork,
            difficulty: 1,
            block_time: Duration::from_secs(10),
            stake_threshold: 1000,
            delegate_count: 21,
            byzantine_threshold: 1,
        };

        let mut manager = ConsensusManager::new(config);
        let transaction = Transaction::new("alice".to_string(), "bob".to_string(), 100);
        
        assert!(manager.add_transaction(transaction).is_ok());
        assert!(manager.generate_block().is_ok());
    }
}
