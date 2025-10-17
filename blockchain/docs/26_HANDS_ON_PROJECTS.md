# 实践项目指南

## 📋 目录

- [实践项目指南](#实践项目指南)
  - [📋 目录](#-目录)
  - [1. 项目概述](#1-项目概述)
    - [1.1 学习路径](#11-学习路径)
    - [1.2 项目难度分级](#12-项目难度分级)
    - [1.3 开发环境准备](#13-开发环境准备)
  - [2. 初级项目](#2-初级项目)
    - [2.1 项目1: 简单区块链](#21-项目1-简单区块链)
    - [2.2 项目2: 基础加密货币](#22-项目2-基础加密货币)
    - [2.3 项目3: 投票系统](#23-项目3-投票系统)
  - [3. 中级项目](#3-中级项目)
    - [3.1 项目4: ERC-20代币](#31-项目4-erc-20代币)
    - [3.2 项目5: NFT市场](#32-项目5-nft市场)
    - [3.3 项目6: 简单DEX](#33-项目6-简单dex)
  - [4. 高级项目](#4-高级项目)
    - [4.1 项目7: 借贷协议](#41-项目7-借贷协议)
    - [4.2 项目8: DAO治理平台](#42-项目8-dao治理平台)
    - [4.3 项目9: 跨链桥](#43-项目9-跨链桥)
  - [5. 专家级项目](#5-专家级项目)
    - [5.1 项目10: Layer 2 Rollup](#51-项目10-layer-2-rollup)
    - [5.2 项目11: 零知识证明应用](#52-项目11-零知识证明应用)
    - [5.3 项目12: 完整公链](#53-项目12-完整公链)
  - [6. 全栈DApp项目](#6-全栈dapp项目)
    - [6.1 项目13: 去中心化社交网络](#61-项目13-去中心化社交网络)
    - [6.2 项目14: DeFi仪表板](#62-项目14-defi仪表板)
    - [6.3 项目15: Web3游戏](#63-项目15-web3游戏)
  - [7. 项目开发最佳实践](#7-项目开发最佳实践)
    - [7.1 代码规范](#71-代码规范)
    - [7.2 测试策略](#72-测试策略)
    - [7.3 文档编写](#73-文档编写)
  - [8. 部署与运维](#8-部署与运维)
    - [8.1 测试网部署](#81-测试网部署)
    - [8.2 主网部署](#82-主网部署)
    - [8.3 监控与维护](#83-监控与维护)
  - [9. 项目展示与开源](#9-项目展示与开源)
    - [9.1 GitHub项目管理](#91-github项目管理)
    - [9.2 技术博客撰写](#92-技术博客撰写)
    - [9.3 社区参与](#93-社区参与)
  - [10. 进阶学习资源](#10-进阶学习资源)
  - [总结](#总结)
  - [相关文档](#相关文档)
  - [项目模板](#项目模板)

## 1. 项目概述

### 1.1 学习路径

```rust
/// 区块链学习路径
#[derive(Debug)]
pub struct LearningPath {
    pub stages: Vec<LearningStage>,
    pub estimated_duration: Duration,
}

#[derive(Debug)]
pub struct LearningStage {
    pub name: String,
    pub level: DifficultyLevel,
    pub topics: Vec<String>,
    pub projects: Vec<String>,
    pub duration_weeks: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

pub fn create_learning_path() -> LearningPath {
    LearningPath {
        stages: vec![
            LearningStage {
                name: "基础阶段".to_string(),
                level: DifficultyLevel::Beginner,
                topics: vec![
                    "区块链基础概念".to_string(),
                    "哈希函数与加密".to_string(),
                    "数据结构".to_string(),
                    "Rust基础语法".to_string(),
                ],
                projects: vec![
                    "简单区块链".to_string(),
                    "基础加密货币".to_string(),
                ],
                duration_weeks: 4,
            },
            LearningStage {
                name: "进阶阶段".to_string(),
                level: DifficultyLevel::Intermediate,
                topics: vec![
                    "智能合约".to_string(),
                    "共识算法".to_string(),
                    "P2P网络".to_string(),
                    "DeFi原理".to_string(),
                ],
                projects: vec![
                    "ERC-20代币".to_string(),
                    "NFT市场".to_string(),
                    "简单DEX".to_string(),
                ],
                duration_weeks: 8,
            },
            LearningStage {
                name: "高级阶段".to_string(),
                level: DifficultyLevel::Advanced,
                topics: vec![
                    "协议设计".to_string(),
                    "系统优化".to_string(),
                    "安全审计".to_string(),
                    "跨链技术".to_string(),
                ],
                projects: vec![
                    "借贷协议".to_string(),
                    "DAO平台".to_string(),
                    "跨链桥".to_string(),
                ],
                duration_weeks: 12,
            },
            LearningStage {
                name: "专家阶段".to_string(),
                level: DifficultyLevel::Expert,
                topics: vec![
                    "Layer 2技术".to_string(),
                    "零知识证明".to_string(),
                    "形式化验证".to_string(),
                    "公链开发".to_string(),
                ],
                projects: vec![
                    "Layer 2 Rollup".to_string(),
                    "ZK应用".to_string(),
                    "完整公链".to_string(),
                ],
                duration_weeks: 16,
            },
        ],
        estimated_duration: Duration::from_weeks(40),
    }
}

#[derive(Debug)]
pub struct Duration {
    pub weeks: u32,
}

impl Duration {
    pub fn from_weeks(weeks: u32) -> Self {
        Self { weeks }
    }
}
```

### 1.2 项目难度分级

```rust
/// 项目元数据
#[derive(Debug, Clone)]
pub struct ProjectMetadata {
    pub id: u32,
    pub name: String,
    pub difficulty: DifficultyLevel,
    pub estimated_hours: u32,
    pub prerequisites: Vec<String>,
    pub skills_learned: Vec<String>,
    pub technologies: Vec<String>,
}

/// 项目库
pub struct ProjectCatalog {
    pub projects: Vec<ProjectMetadata>,
}

impl ProjectCatalog {
    pub fn new() -> Self {
        Self {
            projects: vec![
                ProjectMetadata {
                    id: 1,
                    name: "简单区块链".to_string(),
                    difficulty: DifficultyLevel::Beginner,
                    estimated_hours: 20,
                    prerequisites: vec!["Rust基础".to_string()],
                    skills_learned: vec![
                        "区块链数据结构".to_string(),
                        "哈希计算".to_string(),
                        "工作量证明".to_string(),
                    ],
                    technologies: vec!["Rust".to_string(), "SHA-256".to_string()],
                },
                ProjectMetadata {
                    id: 2,
                    name: "ERC-20代币".to_string(),
                    difficulty: DifficultyLevel::Intermediate,
                    estimated_hours: 30,
                    prerequisites: vec![
                        "智能合约基础".to_string(),
                        "Solidity语法".to_string(),
                    ],
                    skills_learned: vec![
                        "代币标准".to_string(),
                        "合约部署".to_string(),
                        "事件日志".to_string(),
                    ],
                    technologies: vec![
                        "Solidity".to_string(),
                        "Hardhat".to_string(),
                        "OpenZeppelin".to_string(),
                    ],
                },
                // 更多项目...
            ],
        }
    }
    
    /// 按难度筛选项目
    pub fn filter_by_difficulty(&self, level: DifficultyLevel) -> Vec<&ProjectMetadata> {
        self.projects.iter()
            .filter(|p| p.difficulty == level)
            .collect()
    }
    
    /// 推荐下一个项目
    pub fn recommend_next(&self, completed: &[u32]) -> Option<&ProjectMetadata> {
        self.projects.iter()
            .find(|p| !completed.contains(&p.id))
    }
}
```

### 1.3 开发环境准备

```rust
/// 开发环境配置
#[derive(Debug)]
pub struct DevelopmentEnvironment {
    pub rust_toolchain: RustToolchain,
    pub blockchain_tools: Vec<BlockchainTool>,
    pub ide_setup: IDESetup,
    pub dependencies: Vec<String>,
}

#[derive(Debug)]
pub struct RustToolchain {
    pub version: String,
    pub components: Vec<String>,
}

#[derive(Debug)]
pub struct BlockchainTool {
    pub name: String,
    pub purpose: String,
    pub install_command: String,
}

#[derive(Debug)]
pub struct IDESetup {
    pub editor: String,
    pub extensions: Vec<String>,
    pub linter: String,
    pub formatter: String,
}

pub fn setup_development_environment() -> DevelopmentEnvironment {
    DevelopmentEnvironment {
        rust_toolchain: RustToolchain {
            version: "1.75.0".to_string(),
            components: vec![
                "rustc".to_string(),
                "cargo".to_string(),
                "rustfmt".to_string(),
                "clippy".to_string(),
            ],
        },
        blockchain_tools: vec![
            BlockchainTool {
                name: "Foundry".to_string(),
                purpose: "Solidity开发框架".to_string(),
                install_command: "curl -L https://foundry.paradigm.xyz | bash".to_string(),
            },
            BlockchainTool {
                name: "Hardhat".to_string(),
                purpose: "以太坊开发环境".to_string(),
                install_command: "npm install --save-dev hardhat".to_string(),
            },
        ],
        ide_setup: IDESetup {
            editor: "VSCode".to_string(),
            extensions: vec![
                "rust-analyzer".to_string(),
                "solidity".to_string(),
                "prettier".to_string(),
            ],
            linter: "clippy".to_string(),
            formatter: "rustfmt".to_string(),
        },
        dependencies: vec![
            "sha2".to_string(),
            "serde".to_string(),
            "tokio".to_string(),
            "ethers".to_string(),
        ],
    }
}
```

## 2. 初级项目

### 2.1 项目1: 简单区块链

```rust
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// 项目1: 简单区块链实现
/// 学习目标: 理解区块链基本数据结构和工作量证明

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleBlock {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl SimpleBlock {
    /// 创建新区块
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut block = Self {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        
        block.hash = block.calculate_hash();
        block
    }
    
    /// 计算区块哈希
    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    /// 挖矿(工作量证明)
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        
        loop {
            self.hash = self.calculate_hash();
            
            if self.hash.starts_with(&target) {
                println!("Block mined: {}", self.hash);
                break;
            }
            
            self.nonce += 1;
        }
    }
}

/// 简单区块链
pub struct SimpleBlockchain {
    pub chain: Vec<SimpleBlock>,
    pub difficulty: usize,
}

impl SimpleBlockchain {
    /// 创建创世区块
    pub fn new(difficulty: usize) -> Self {
        let mut genesis = SimpleBlock::new(
            0,
            "Genesis Block".to_string(),
            "0".to_string(),
        );
        genesis.mine_block(difficulty);
        
        Self {
            chain: vec![genesis],
            difficulty,
        }
    }
    
    /// 获取最新区块
    pub fn get_latest_block(&self) -> &SimpleBlock {
        self.chain.last().unwrap()
    }
    
    /// 添加新区块
    pub fn add_block(&mut self, data: String) {
        let previous_hash = self.get_latest_block().hash.clone();
        let index = self.chain.len() as u64;
        
        let mut new_block = SimpleBlock::new(index, data, previous_hash);
        new_block.mine_block(self.difficulty);
        
        self.chain.push(new_block);
    }
    
    /// 验证区块链
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            
            // 验证当前区块哈希
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }
            
            // 验证链接
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
            
            // 验证工作量证明
            if !current_block.hash.starts_with(&"0".repeat(self.difficulty)) {
                return false;
            }
        }
        
        true
    }
}

/// 测试代码
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_blockchain() {
        let mut blockchain = SimpleBlockchain::new(2);
        
        blockchain.add_block("Block 1 Data".to_string());
        blockchain.add_block("Block 2 Data".to_string());
        
        assert!(blockchain.is_valid());
        assert_eq!(blockchain.chain.len(), 3);
    }
}
```

### 2.2 项目2: 基础加密货币

```rust
use std::collections::HashMap;

/// 项目2: 基础加密货币
/// 学习目标: 交易、UTXO模型、钱包

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: u64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            from,
            to,
            amount,
            timestamp,
        }
    }
    
    pub fn hash(&self) -> String {
        let input = format!("{}{}{}{}", self.from, self.to, self.amount, self.timestamp);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

/// 加密货币区块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoBlock {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl CryptoBlock {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut block = Self {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        
        block.hash = block.calculate_hash();
        block
    }
    
    pub fn calculate_hash(&self) -> String {
        let tx_data: String = self.transactions.iter()
            .map(|tx| tx.hash())
            .collect();
        
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, tx_data, self.previous_hash, self.nonce
        );
        
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        
        loop {
            self.hash = self.calculate_hash();
            
            if self.hash.starts_with(&target) {
                break;
            }
            
            self.nonce += 1;
        }
    }
}

/// 加密货币
pub struct SimpleCryptocurrency {
    pub chain: Vec<CryptoBlock>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: u64,
    pub balances: HashMap<String, u64>,
}

impl SimpleCryptocurrency {
    pub fn new(difficulty: usize, mining_reward: u64) -> Self {
        let genesis = CryptoBlock::new(0, vec![], "0".to_string());
        
        Self {
            chain: vec![genesis],
            difficulty,
            pending_transactions: vec![],
            mining_reward,
            balances: HashMap::new(),
        }
    }
    
    /// 创建交易
    pub fn create_transaction(&mut self, from: String, to: String, amount: u64) -> Result<(), String> {
        // 验证余额
        let balance = self.get_balance(&from);
        
        if balance < amount {
            return Err("Insufficient balance".to_string());
        }
        
        let transaction = Transaction::new(from, to, amount);
        self.pending_transactions.push(transaction);
        
        Ok(())
    }
    
    /// 挖矿
    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        // 添加挖矿奖励交易
        let reward_tx = Transaction::new(
            "system".to_string(),
            miner_address.clone(),
            self.mining_reward,
        );
        
        self.pending_transactions.push(reward_tx);
        
        // 创建新区块
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let index = self.chain.len() as u64;
        
        let mut block = CryptoBlock::new(
            index,
            self.pending_transactions.clone(),
            previous_hash,
        );
        
        block.mine_block(self.difficulty);
        
        // 更新余额
        for tx in &block.transactions {
            if tx.from != "system" {
                *self.balances.entry(tx.from.clone()).or_insert(0) -= tx.amount;
            }
            *self.balances.entry(tx.to.clone()).or_insert(0) += tx.amount;
        }
        
        self.chain.push(block);
        self.pending_transactions.clear();
    }
    
    /// 获取余额
    pub fn get_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }
}

#[cfg(test)]
mod crypto_tests {
    use super::*;
    
    #[test]
    fn test_cryptocurrency() {
        let mut crypto = SimpleCryptocurrency::new(2, 100);
        
        // 挖矿获得初始资金
        crypto.mine_pending_transactions("miner1".to_string());
        assert_eq!(crypto.get_balance("miner1"), 100);
        
        // 创建交易
        crypto.create_transaction(
            "miner1".to_string(),
            "user1".to_string(),
            50,
        ).unwrap();
        
        crypto.mine_pending_transactions("miner1".to_string());
        
        assert_eq!(crypto.get_balance("miner1"), 150); // 100 - 50 + 100
        assert_eq!(crypto.get_balance("user1"), 50);
    }
}
```

### 2.3 项目3: 投票系统

```rust
/// 项目3: 去中心化投票系统
/// 学习目标: 智能合约、状态管理、权限控制

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub creator: String,
    pub created_at: u64,
    pub deadline: u64,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub executed: bool,
}

pub struct VotingSystem {
    pub proposals: HashMap<u64, Proposal>,
    pub voters: HashSet<String>,
    pub votes: HashMap<u64, HashMap<String, bool>>, // proposal_id -> (voter -> vote)
    next_proposal_id: u64,
}

impl VotingSystem {
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            voters: HashSet::new(),
            votes: HashMap::new(),
            next_proposal_id: 1,
        }
    }
    
    /// 注册选民
    pub fn register_voter(&mut self, address: String) -> Result<(), String> {
        if self.voters.contains(&address) {
            return Err("Voter already registered".to_string());
        }
        
        self.voters.insert(address);
        Ok(())
    }
    
    /// 创建提案
    pub fn create_proposal(
        &mut self,
        creator: String,
        title: String,
        description: String,
        duration_seconds: u64,
    ) -> Result<u64, String> {
        if !self.voters.contains(&creator) {
            return Err("Only registered voters can create proposals".to_string());
        }
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let proposal_id = self.next_proposal_id;
        self.next_proposal_id += 1;
        
        let proposal = Proposal {
            id: proposal_id,
            title,
            description,
            creator,
            created_at: now,
            deadline: now + duration_seconds,
            yes_votes: 0,
            no_votes: 0,
            executed: false,
        };
        
        self.proposals.insert(proposal_id, proposal);
        self.votes.insert(proposal_id, HashMap::new());
        
        Ok(proposal_id)
    }
    
    /// 投票
    pub fn vote(
        &mut self,
        proposal_id: u64,
        voter: String,
        support: bool,
    ) -> Result<(), String> {
        // 验证选民
        if !self.voters.contains(&voter) {
            return Err("Not a registered voter".to_string());
        }
        
        // 验证提案存在
        let proposal = self.proposals.get_mut(&proposal_id)
            .ok_or("Proposal not found")?;
        
        // 检查截止时间
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if now > proposal.deadline {
            return Err("Voting period has ended".to_string());
        }
        
        // 检查是否已投票
        let proposal_votes = self.votes.get_mut(&proposal_id).unwrap();
        
        if proposal_votes.contains_key(&voter) {
            return Err("Already voted".to_string());
        }
        
        // 记录投票
        proposal_votes.insert(voter, support);
        
        if support {
            proposal.yes_votes += 1;
        } else {
            proposal.no_votes += 1;
        }
        
        Ok(())
    }
    
    /// 获取提案结果
    pub fn get_result(&self, proposal_id: u64) -> Result<VoteResult, String> {
        let proposal = self.proposals.get(&proposal_id)
            .ok_or("Proposal not found")?;
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if now <= proposal.deadline {
            return Err("Voting is still ongoing".to_string());
        }
        
        let total_votes = proposal.yes_votes + proposal.no_votes;
        let passed = proposal.yes_votes > proposal.no_votes;
        
        Ok(VoteResult {
            proposal_id,
            yes_votes: proposal.yes_votes,
            no_votes: proposal.no_votes,
            total_votes,
            passed,
        })
    }
}

#[derive(Debug)]
pub struct VoteResult {
    pub proposal_id: u64,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub total_votes: u64,
    pub passed: bool,
}

#[cfg(test)]
mod voting_tests {
    use super::*;
    
    #[test]
    fn test_voting_system() {
        let mut system = VotingSystem::new();
        
        // 注册选民
        system.register_voter("voter1".to_string()).unwrap();
        system.register_voter("voter2".to_string()).unwrap();
        system.register_voter("voter3".to_string()).unwrap();
        
        // 创建提案
        let proposal_id = system.create_proposal(
            "voter1".to_string(),
            "Proposal 1".to_string(),
            "Description".to_string(),
            3600,
        ).unwrap();
        
        // 投票
        system.vote(proposal_id, "voter1".to_string(), true).unwrap();
        system.vote(proposal_id, "voter2".to_string(), true).unwrap();
        system.vote(proposal_id, "voter3".to_string(), false).unwrap();
        
        let proposal = system.proposals.get(&proposal_id).unwrap();
        assert_eq!(proposal.yes_votes, 2);
        assert_eq!(proposal.no_votes, 1);
    }
}
```

## 3. 中级项目

### 3.1 项目4: ERC-20代币

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * 项目4: ERC-20代币标准实现
 * 学习目标: 代币标准、事件、授权机制
 */
contract ERC20Token {
    string public name;
    string public symbol;
    uint8 public decimals;
    uint256 public totalSupply;
    
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    
    constructor(
        string memory _name,
        string memory _symbol,
        uint8 _decimals,
        uint256 _initialSupply
    ) {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;
        totalSupply = _initialSupply * 10 ** uint256(_decimals);
        balanceOf[msg.sender] = totalSupply;
        
        emit Transfer(address(0), msg.sender, totalSupply);
    }
    
    function transfer(address _to, uint256 _value) public returns (bool success) {
        require(_to != address(0), "Invalid recipient");
        require(balanceOf[msg.sender] >= _value, "Insufficient balance");
        
        balanceOf[msg.sender] -= _value;
        balanceOf[_to] += _value;
        
        emit Transfer(msg.sender, _to, _value);
        return true;
    }
    
    function approve(address _spender, uint256 _value) public returns (bool success) {
        allowance[msg.sender][_spender] = _value;
        emit Approval(msg.sender, _spender, _value);
        return true;
    }
    
    function transferFrom(
        address _from,
        address _to,
        uint256 _value
    ) public returns (bool success) {
        require(_to != address(0), "Invalid recipient");
        require(balanceOf[_from] >= _value, "Insufficient balance");
        require(allowance[_from][msg.sender] >= _value, "Insufficient allowance");
        
        balanceOf[_from] -= _value;
        balanceOf[_to] += _value;
        allowance[_from][msg.sender] -= _value;
        
        emit Transfer(_from, _to, _value);
        return true;
    }
}
```

### 3.2 项目5: NFT市场

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * 项目5: NFT市场
 * 学习目标: ERC-721、市场机制、版税
 */
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract NFTMarketplace is ERC721URIStorage, Ownable {
    uint256 private _tokenIdCounter;
    uint256 public marketFee = 25; // 2.5%
    
    struct Listing {
        uint256 price;
        address seller;
        bool active;
    }
    
    mapping(uint256 => Listing) public listings;
    mapping(uint256 => uint256) public royalties; // tokenId => royalty in basis points
    mapping(uint256 => address) public creators;
    
    event NFTMinted(uint256 indexed tokenId, address indexed creator, string uri);
    event NFTListed(uint256 indexed tokenId, uint256 price, address indexed seller);
    event NFTSold(uint256 indexed tokenId, uint256 price, address indexed buyer, address indexed seller);
    event NFTUnlisted(uint256 indexed tokenId);
    
    constructor() ERC721("NFT Marketplace", "NFTM") {}
    
    function mintNFT(string memory uri, uint256 royaltyPercentage) public returns (uint256) {
        require(royaltyPercentage <= 1000, "Royalty too high"); // Max 10%
        
        uint256 tokenId = _tokenIdCounter++;
        _safeMint(msg.sender, tokenId);
        _setTokenURI(tokenId, uri);
        
        creators[tokenId] = msg.sender;
        royalties[tokenId] = royaltyPercentage;
        
        emit NFTMinted(tokenId, msg.sender, uri);
        return tokenId;
    }
    
    function listNFT(uint256 tokenId, uint256 price) public {
        require(ownerOf(tokenId) == msg.sender, "Not the owner");
        require(price > 0, "Price must be positive");
        
        listings[tokenId] = Listing({
            price: price,
            seller: msg.sender,
            active: true
        });
        
        emit NFTListed(tokenId, price, msg.sender);
    }
    
    function buyNFT(uint256 tokenId) public payable {
        Listing memory listing = listings[tokenId];
        require(listing.active, "NFT not for sale");
        require(msg.value >= listing.price, "Insufficient payment");
        
        address seller = listing.seller;
        address creator = creators[tokenId];
        uint256 price = listing.price;
        
        // 计算费用
        uint256 marketFeeAmount = (price * marketFee) / 1000;
        uint256 royaltyAmount = (price * royalties[tokenId]) / 1000;
        uint256 sellerAmount = price - marketFeeAmount - royaltyAmount;
        
        // 转移NFT
        _transfer(seller, msg.sender, tokenId);
        
        // 分配资金
        payable(seller).transfer(sellerAmount);
        payable(creator).transfer(royaltyAmount);
        payable(owner()).transfer(marketFeeAmount);
        
        // 退还多余的资金
        if (msg.value > price) {
            payable(msg.sender).transfer(msg.value - price);
        }
        
        // 取消listing
        listings[tokenId].active = false;
        
        emit NFTSold(tokenId, price, msg.sender, seller);
    }
    
    function unlistNFT(uint256 tokenId) public {
        require(listings[tokenId].seller == msg.sender, "Not the seller");
        
        listings[tokenId].active = false;
        
        emit NFTUnlisted(tokenId);
    }
}
```

### 3.3 项目6: 简单DEX

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * 项目6: 简单的去中心化交易所
 * 学习目标: AMM、流动性池、价格计算
 */
interface IERC20 {
    function transfer(address to, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 amount) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

contract SimpleDEX {
    address public tokenA;
    address public tokenB;
    
    uint256 public reserveA;
    uint256 public reserveB;
    
    uint256 public totalLiquidity;
    mapping(address => uint256) public liquidity;
    
    uint256 public constant FEE = 3; // 0.3% fee
    
    event LiquidityAdded(address indexed provider, uint256 amountA, uint256 amountB, uint256 liquidity);
    event LiquidityRemoved(address indexed provider, uint256 amountA, uint256 amountB, uint256 liquidity);
    event Swap(address indexed trader, address tokenIn, uint256 amountIn, uint256 amountOut);
    
    constructor(address _tokenA, address _tokenB) {
        tokenA = _tokenA;
        tokenB = _tokenB;
    }
    
    function addLiquidity(uint256 amountA, uint256 amountB) external returns (uint256) {
        require(amountA > 0 && amountB > 0, "Invalid amounts");
        
        uint256 liquidityMinted;
        
        if (totalLiquidity == 0) {
            liquidityMinted = sqrt(amountA * amountB);
        } else {
            uint256 liquidityA = (amountA * totalLiquidity) / reserveA;
            uint256 liquidityB = (amountB * totalLiquidity) / reserveB;
            liquidityMinted = min(liquidityA, liquidityB);
        }
        
        require(liquidityMinted > 0, "Insufficient liquidity minted");
        
        // 转移代币
        IERC20(tokenA).transferFrom(msg.sender, address(this), amountA);
        IERC20(tokenB).transferFrom(msg.sender, address(this), amountB);
        
        // 更新储备
        reserveA += amountA;
        reserveB += amountB;
        
        // 铸造流动性代币
        liquidity[msg.sender] += liquidityMinted;
        totalLiquidity += liquidityMinted;
        
        emit LiquidityAdded(msg.sender, amountA, amountB, liquidityMinted);
        
        return liquidityMinted;
    }
    
    function removeLiquidity(uint256 liquidityAmount) external returns (uint256, uint256) {
        require(liquidity[msg.sender] >= liquidityAmount, "Insufficient liquidity");
        
        uint256 amountA = (liquidityAmount * reserveA) / totalLiquidity;
        uint256 amountB = (liquidityAmount * reserveB) / totalLiquidity;
        
        require(amountA > 0 && amountB > 0, "Insufficient liquidity burned");
        
        // 更新储备
        reserveA -= amountA;
        reserveB -= amountB;
        
        // 销毁流动性代币
        liquidity[msg.sender] -= liquidityAmount;
        totalLiquidity -= liquidityAmount;
        
        // 转移代币
        IERC20(tokenA).transfer(msg.sender, amountA);
        IERC20(tokenB).transfer(msg.sender, amountB);
        
        emit LiquidityRemoved(msg.sender, amountA, amountB, liquidityAmount);
        
        return (amountA, amountB);
    }
    
    function swapAForB(uint256 amountIn, uint256 minAmountOut) external returns (uint256) {
        require(amountIn > 0, "Invalid input amount");
        
        uint256 amountInWithFee = amountIn * (1000 - FEE);
        uint256 amountOut = (reserveB * amountInWithFee) / (reserveA * 1000 + amountInWithFee);
        
        require(amountOut >= minAmountOut, "Slippage too high");
        require(amountOut < reserveB, "Insufficient liquidity");
        
        // 转移代币
        IERC20(tokenA).transferFrom(msg.sender, address(this), amountIn);
        IERC20(tokenB).transfer(msg.sender, amountOut);
        
        // 更新储备
        reserveA += amountIn;
        reserveB -= amountOut;
        
        emit Swap(msg.sender, tokenA, amountIn, amountOut);
        
        return amountOut;
    }
    
    function swapBForA(uint256 amountIn, uint256 minAmountOut) external returns (uint256) {
        require(amountIn > 0, "Invalid input amount");
        
        uint256 amountInWithFee = amountIn * (1000 - FEE);
        uint256 amountOut = (reserveA * amountInWithFee) / (reserveB * 1000 + amountInWithFee);
        
        require(amountOut >= minAmountOut, "Slippage too high");
        require(amountOut < reserveA, "Insufficient liquidity");
        
        // 转移代币
        IERC20(tokenB).transferFrom(msg.sender, address(this), amountIn);
        IERC20(tokenA).transfer(msg.sender, amountOut);
        
        // 更新储备
        reserveB += amountIn;
        reserveA -= amountOut;
        
        emit Swap(msg.sender, tokenB, amountIn, amountOut);
        
        return amountOut;
    }
    
    function getAmountOut(uint256 amountIn, bool aToB) public view returns (uint256) {
        if (aToB) {
            uint256 amountInWithFee = amountIn * (1000 - FEE);
            return (reserveB * amountInWithFee) / (reserveA * 1000 + amountInWithFee);
        } else {
            uint256 amountInWithFee = amountIn * (1000 - FEE);
            return (reserveA * amountInWithFee) / (reserveB * 1000 + amountInWithFee);
        }
    }
    
    // 辅助函数
    function sqrt(uint256 x) internal pure returns (uint256) {
        if (x == 0) return 0;
        uint256 z = (x + 1) / 2;
        uint256 y = x;
        while (z < y) {
            y = z;
            z = (x / z + z) / 2;
        }
        return y;
    }
    
    function min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }
}
```

## 4. 高级项目

### 4.1 项目7: 借贷协议

由于篇幅限制，这里提供核心概念：

```rust
/// 项目7: 借贷协议核心逻辑(Rust概念模型)
/// 学习目标: 超额抵押、利率模型、清算机制

use std::collections::HashMap;

pub struct LendingProtocol {
    /// 市场配置
    pub markets: HashMap<String, Market>,
    
    /// 用户账户
    pub accounts: HashMap<String, Account>,
    
    /// 全局参数
    pub liquidation_threshold: f64,  // 清算阈值
    pub liquidation_bonus: f64,      // 清算奖励
    pub close_factor: f64,           // 每次可清算比例
}

pub struct Market {
    pub asset: String,
    pub total_deposits: u128,
    pub total_borrows: u128,
    pub deposit_rate: f64,
    pub borrow_rate: f64,
    pub collateral_factor: f64,
    pub reserve_factor: f64,
}

pub struct Account {
    pub deposits: HashMap<String, u128>,
    pub borrows: HashMap<String, u128>,
}

impl LendingProtocol {
    /// 计算利率
    pub fn calculate_interest_rate(&self, market: &Market) -> (f64, f64) {
        let utilization_rate = if market.total_deposits == 0 {
            0.0
        } else {
            market.total_borrows as f64 / market.total_deposits as f64
        };
        
        // 简化的利率模型
        let borrow_rate = 0.02 + utilization_rate * 0.20;  // 基础利率2% + 利用率相关
        let deposit_rate = borrow_rate * utilization_rate * (1.0 - market.reserve_factor);
        
        (deposit_rate, borrow_rate)
    }
    
    /// 计算健康度
    pub fn calculate_health_factor(&self, user: &str) -> f64 {
        // 健康度 = 抵押价值 * 抵押系数 / 借款价值
        // 实际实现需要价格预言机
        1.5 // 示例值
    }
}
```

### 4.2 项目8: DAO治理平台

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * 项目8: DAO治理平台
 * 学习目标: 提案系统、投票机制、时间锁
 */
contract DAOGovernance {
    struct Proposal {
        uint256 id;
        address proposer;
        string description;
        uint256 forVotes;
        uint256 againstVotes;
        uint256 startBlock;
        uint256 endBlock;
        bool executed;
        bool canceled;
        mapping(address => Receipt) receipts;
    }
    
    struct Receipt {
        bool hasVoted;
        bool support;
        uint256 votes;
    }
    
    mapping(uint256 => Proposal) public proposals;
    uint256 public proposalCount;
    
    address public governanceToken;
    uint256 public proposalThreshold;  // 提案最低代币数
    uint256 public quorum;             // 法定人数
    uint256 public votingPeriod;       // 投票期限(区块数)
    
    event ProposalCreated(uint256 indexed proposalId, address indexed proposer, string description);
    event VoteCast(address indexed voter, uint256 indexed proposalId, bool support, uint256 votes);
    event ProposalExecuted(uint256 indexed proposalId);
    
    constructor(
        address _governanceToken,
        uint256 _proposalThreshold,
        uint256 _quorum,
        uint256 _votingPeriod
    ) {
        governanceToken = _governanceToken;
        proposalThreshold = _proposalThreshold;
        quorum = _quorum;
        votingPeriod = _votingPeriod;
    }
    
    function propose(string memory description) external returns (uint256) {
        // 验证提案者持有足够代币
        require(
            IERC20(governanceToken).balanceOf(msg.sender) >= proposalThreshold,
            "Insufficient tokens to propose"
        );
        
        uint256 proposalId = proposalCount++;
        Proposal storage proposal = proposals[proposalId];
        
        proposal.id = proposalId;
        proposal.proposer = msg.sender;
        proposal.description = description;
        proposal.startBlock = block.number;
        proposal.endBlock = block.number + votingPeriod;
        
        emit ProposalCreated(proposalId, msg.sender, description);
        
        return proposalId;
    }
    
    function castVote(uint256 proposalId, bool support) external {
        Proposal storage proposal = proposals[proposalId];
        
        require(block.number >= proposal.startBlock, "Voting not started");
        require(block.number <= proposal.endBlock, "Voting ended");
        require(!proposal.receipts[msg.sender].hasVoted, "Already voted");
        
        uint256 votes = IERC20(governanceToken).balanceOf(msg.sender);
        require(votes > 0, "No voting power");
        
        if (support) {
            proposal.forVotes += votes;
        } else {
            proposal.againstVotes += votes;
        }
        
        Receipt storage receipt = proposal.receipts[msg.sender];
        receipt.hasVoted = true;
        receipt.support = support;
        receipt.votes = votes;
        
        emit VoteCast(msg.sender, proposalId, support, votes);
    }
    
    function execute(uint256 proposalId) external {
        Proposal storage proposal = proposals[proposalId];
        
        require(block.number > proposal.endBlock, "Voting not ended");
        require(!proposal.executed, "Already executed");
        require(!proposal.canceled, "Proposal canceled");
        
        uint256 totalVotes = proposal.forVotes + proposal.againstVotes;
        require(totalVotes >= quorum, "Quorum not reached");
        require(proposal.forVotes > proposal.againstVotes, "Proposal rejected");
        
        proposal.executed = true;
        
        // 执行提案逻辑
        
        emit ProposalExecuted(proposalId);
    }
}
```

### 4.3 项目9: 跨链桥

```rust
/// 项目9: 简化的跨链桥
/// 学习目标: 跨链通信、资产锁定、消息验证

use std::collections::HashMap;

pub struct CrossChainBridge {
    /// 链A的锁定资产
    pub locked_assets_chain_a: HashMap<String, u128>,
    
    /// 链B的映射资产
    pub wrapped_assets_chain_b: HashMap<String, u128>,
    
    /// 已处理的交易
    pub processed_transfers: HashMap<String, bool>,
    
    /// 验证者集合
    pub validators: Vec<String>,
    
    /// 最小确认数
    pub min_confirmations: u32,
}

impl CrossChainBridge {
    pub fn new(validators: Vec<String>, min_confirmations: u32) -> Self {
        Self {
            locked_assets_chain_a: HashMap::new(),
            wrapped_assets_chain_b: HashMap::new(),
            processed_transfers: HashMap::new(),
            validators,
            min_confirmations,
        }
    }
    
    /// 锁定资产(链A)
    pub fn lock_asset(
        &mut self,
        user: String,
        amount: u128,
        target_chain: String,
    ) -> Result<String, String> {
        // 生成唯一转账ID
        let transfer_id = format!("{}-{}-{}", user, amount, target_chain);
        
        if self.processed_transfers.contains_key(&transfer_id) {
            return Err("Transfer already processed".to_string());
        }
        
        // 锁定资产
        *self.locked_assets_chain_a.entry(user.clone()).or_insert(0) += amount;
        
        // 标记为已处理
        self.processed_transfers.insert(transfer_id.clone(), true);
        
        Ok(transfer_id)
    }
    
    /// 铸造映射资产(链B)
    pub fn mint_wrapped_asset(
        &mut self,
        user: String,
        amount: u128,
        transfer_id: String,
        signatures: Vec<String>,
    ) -> Result<(), String> {
        // 验证签名数量
        if signatures.len() < self.min_confirmations as usize {
            return Err("Insufficient confirmations".to_string());
        }
        
        // 验证转账ID未被使用
        if self.wrapped_assets_chain_b.contains_key(&transfer_id) {
            return Err("Transfer already minted".to_string());
        }
        
        // 铸造映射资产
        *self.wrapped_assets_chain_b.entry(user).or_insert(0) += amount;
        self.wrapped_assets_chain_b.insert(transfer_id, amount);
        
        Ok(())
    }
    
    /// 销毁映射资产(链B)
    pub fn burn_wrapped_asset(
        &mut self,
        user: String,
        amount: u128,
    ) -> Result<String, String> {
        let balance = self.wrapped_assets_chain_b.get(&user).copied().unwrap_or(0);
        
        if balance < amount {
            return Err("Insufficient balance".to_string());
        }
        
        // 销毁映射资产
        *self.wrapped_assets_chain_b.get_mut(&user).unwrap() -= amount;
        
        // 生成解锁证明
        let unlock_id = format!("{}-unlock-{}", user, amount);
        
        Ok(unlock_id)
    }
    
    /// 解锁原始资产(链A)
    pub fn unlock_asset(
        &mut self,
        user: String,
        amount: u128,
        unlock_id: String,
        signatures: Vec<String>,
    ) -> Result<(), String> {
        // 验证签名
        if signatures.len() < self.min_confirmations as usize {
            return Err("Insufficient confirmations".to_string());
        }
        
        let locked = self.locked_assets_chain_a.get(&user).copied().unwrap_or(0);
        
        if locked < amount {
            return Err("Insufficient locked assets".to_string());
        }
        
        // 解锁资产
        *self.locked_assets_chain_a.get_mut(&user).unwrap() -= amount;
        
        Ok(())
    }
}
```

## 5. 专家级项目

### 5.1 项目10: Layer 2 Rollup

```rust
/// 项目10: Layer 2 Optimistic Rollup(简化版)
/// 学习目标: Layer 2扩容、状态根、欺诈证明

pub struct OptimisticRollup {
    /// L2状态根
    pub state_root: [u8; 32],
    
    /// 批次序列
    pub batches: Vec<Batch>,
    
    /// 挑战期(秒)
    pub challenge_period: u64,
    
    /// L1合约地址
    pub l1_contract: String,
}

pub struct Batch {
    pub batch_id: u64,
    pub transactions: Vec<L2Transaction>,
    pub state_root: [u8; 32],
    pub submitted_at: u64,
    pub challenged: bool,
}

pub struct L2Transaction {
    pub from: String,
    pub to: String,
    pub value: u128,
    pub nonce: u64,
    pub data: Vec<u8>,
}

impl OptimisticRollup {
    /// 提交批次到L1
    pub fn submit_batch(&mut self, transactions: Vec<L2Transaction>) -> u64 {
        let batch_id = self.batches.len() as u64;
        
        // 执行交易并计算新状态根
        let new_state_root = self.execute_batch(&transactions);
        
        let batch = Batch {
            batch_id,
            transactions,
            state_root: new_state_root,
            submitted_at: current_timestamp(),
            challenged: false,
        };
        
        self.batches.push(batch);
        self.state_root = new_state_root;
        
        batch_id
    }
    
    /// 执行批次
    fn execute_batch(&self, transactions: &[L2Transaction]) -> [u8; 32] {
        // 简化实现：实际需要完整的状态转换
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(&self.state_root);
        
        for tx in transactions {
            hasher.update(tx.from.as_bytes());
            hasher.update(tx.to.as_bytes());
            hasher.update(&tx.value.to_le_bytes());
        }
        
        hasher.finalize().into()
    }
    
    /// 挑战批次
    pub fn challenge_batch(
        &mut self,
        batch_id: u64,
        fraud_proof: FraudProof,
    ) -> Result<(), String> {
        let batch = self.batches.get_mut(batch_id as usize)
            .ok_or("Batch not found")?;
        
        let now = current_timestamp();
        if now > batch.submitted_at + self.challenge_period {
            return Err("Challenge period expired".to_string());
        }
        
        // 验证欺诈证明
        if self.verify_fraud_proof(batch, &fraud_proof) {
            batch.challenged = true;
            // 回滚状态
            return Ok(());
        }
        
        Err("Invalid fraud proof".to_string())
    }
    
    fn verify_fraud_proof(&self, batch: &Batch, proof: &FraudProof) -> bool {
        // 验证某个交易执行不正确
        true // 简化实现
    }
}

pub struct FraudProof {
    pub tx_index: usize,
    pub pre_state: Vec<u8>,
    pub post_state: Vec<u8>,
    pub witness: Vec<u8>,
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
```

### 5.2 项目11: 零知识证明应用

```rust
/// 项目11: 零知识证明应用
/// 学习目标: ZK-SNARK、隐私计算、证明生成

/// 简化的ZK证明系统
pub struct ZKProofSystem {
    /// 公共参数
    pub setup_params: SetupParams,
}

pub struct SetupParams {
    pub proving_key: Vec<u8>,
    pub verification_key: Vec<u8>,
}

pub struct Proof {
    pub data: Vec<u8>,
}

impl ZKProofSystem {
    /// 生成证明
    pub fn generate_proof(
        &self,
        witness: &Witness,
        public_inputs: &[u64],
    ) -> Result<Proof, String> {
        // 实际实现需要使用ZK库(如bellman, ark-crypto等)
        println!("Generating ZK proof for inputs: {:?}", public_inputs);
        
        // 简化：直接返回模拟证明
        Ok(Proof {
            data: vec![1, 2, 3, 4],
        })
    }
    
    /// 验证证明
    pub fn verify_proof(
        &self,
        proof: &Proof,
        public_inputs: &[u64],
    ) -> bool {
        // 实际实现需要ZK验证逻辑
        println!("Verifying ZK proof");
        true
    }
}

pub struct Witness {
    pub secret_data: Vec<u64>,
}

/// 应用示例：隐私转账
pub struct PrivateTransfer {
    pub zk_system: ZKProofSystem,
}

impl PrivateTransfer {
    /// 创建隐私转账
    pub fn create_transfer(
        &self,
        from_balance: u64,
        to_balance: u64,
        amount: u64,
    ) -> Result<(Proof, Vec<u64>), String> {
        // 私有见证
        let witness = Witness {
            secret_data: vec![from_balance, to_balance, amount],
        };
        
        // 公开输入(承诺值)
        let public_inputs = vec![
            self.hash_balance(from_balance),
            self.hash_balance(to_balance - amount),
        ];
        
        // 生成证明
        let proof = self.zk_system.generate_proof(&witness, &public_inputs)?;
        
        Ok((proof, public_inputs))
    }
    
    fn hash_balance(&self, balance: u64) -> u64 {
        // 简化的哈希函数
        balance % 1000000
    }
}
```

### 5.3 项目12: 完整公链

```rust
/// 项目12: 完整公链架构
/// 学习目标: 系统集成、共识、网络、存储

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Blockchain {
    /// 核心组件
    pub consensus: Arc<dyn ConsensusEngine>,
    pub network: Arc<P2PNetwork>,
    pub storage: Arc<dyn Storage>,
    pub mempool: Arc<RwLock<Mempool>>,
    pub state: Arc<RwLock<WorldState>>,
    
    /// 配置
    pub config: ChainConfig,
}

pub struct ChainConfig {
    pub chain_id: u64,
    pub block_time: u64,
    pub max_block_size: usize,
    pub genesis_hash: [u8; 32],
}

/// 共识引擎接口
pub trait ConsensusEngine: Send + Sync {
    fn propose_block(&self, transactions: Vec<Transaction>) -> Block;
    fn validate_block(&self, block: &Block) -> bool;
    fn finalize_block(&self, block: &Block);
}

/// 存储接口
pub trait Storage: Send + Sync {
    fn get_block(&self, hash: &[u8; 32]) -> Option<Block>;
    fn put_block(&self, block: &Block);
    fn get_state(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn put_state(&self, key: &[u8], value: &[u8]);
}

/// 内存池
pub struct Mempool {
    pub pending: Vec<Transaction>,
    pub max_size: usize,
}

/// 世界状态
pub struct WorldState {
    pub accounts: HashMap<String, Account>,
    pub nonces: HashMap<String, u64>,
}

pub struct Account {
    pub balance: u128,
    pub code: Vec<u8>,
    pub storage: HashMap<Vec<u8>, Vec<u8>>,
}

/// P2P网络
pub struct P2PNetwork {
    pub peers: Vec<PeerInfo>,
    pub local_peer_id: String,
}

pub struct PeerInfo {
    pub peer_id: String,
    pub address: String,
}

/// 区块
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

pub struct BlockHeader {
    pub number: u64,
    pub timestamp: u64,
    pub parent_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub tx_root: [u8; 32],
    pub hash: [u8; 32],
}

pub struct Transaction {
    pub from: String,
    pub to: String,
    pub value: u128,
    pub data: Vec<u8>,
    pub nonce: u64,
    pub signature: Vec<u8>,
}

impl Blockchain {
    /// 启动区块链
    pub async fn start(&self) {
        // 启动网络层
        // 启动共识
        // 同步区块
        // 开始出块
        println!("Blockchain started");
    }
    
    /// 提交交易
    pub async fn submit_transaction(&self, tx: Transaction) -> Result<(), String> {
        let mut mempool = self.mempool.write().await;
        
        if mempool.pending.len() >= mempool.max_size {
            return Err("Mempool full".to_string());
        }
        
        mempool.pending.push(tx);
        Ok(())
    }
    
    /// 产生新区块
    pub async fn produce_block(&self) -> Result<Block, String> {
        let mempool = self.mempool.read().await;
        let transactions = mempool.pending.clone();
        drop(mempool);
        
        let block = self.consensus.propose_block(transactions);
        
        // 验证并执行区块
        if self.consensus.validate_block(&block) {
            self.execute_block(&block).await?;
            self.storage.put_block(&block);
            self.consensus.finalize_block(&block);
            
            // 清理mempool
            let mut mempool = self.mempool.write().await;
            mempool.pending.clear();
            
            Ok(block)
        } else {
            Err("Block validation failed".to_string())
        }
    }
    
    /// 执行区块
    async fn execute_block(&self, block: &Block) -> Result<(), String> {
        let mut state = self.state.write().await;
        
        for tx in &block.transactions {
            // 执行交易
            let from_account = state.accounts.get_mut(&tx.from)
                .ok_or("Account not found")?;
            
            if from_account.balance < tx.value {
                return Err("Insufficient balance".to_string());
            }
            
            from_account.balance -= tx.value;
            
            let to_account = state.accounts.entry(tx.to.clone())
                .or_insert(Account {
                    balance: 0,
                    code: vec![],
                    storage: HashMap::new(),
                });
            
            to_account.balance += tx.value;
        }
        
        Ok(())
    }
}

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
```

## 6. 全栈DApp项目

### 6.1 项目13: 去中心化社交网络

```typescript
/**
 * 项目13: 去中心化社交网络
 * 学习目标: Web3前端、IPFS、社交图谱
 */

// 前端组件示例(React + TypeScript)
/*
interface Post {
    id: string;
    author: string;
    content: string;
    ipfsHash: string;
    timestamp: number;
    likes: number;
}

const SocialNetwork: React.FC = () => {
    const [posts, setPosts] = useState<Post[]>([]);
    const [connected, setConnected] = useState(false);
    const [account, setAccount] = useState<string>('');
    
    const connectWallet = async () => {
        const provider = new ethers.providers.Web3Provider(window.ethereum);
        await provider.send("eth_requestAccounts", []);
        const signer = provider.getSigner();
        const address = await signer.getAddress();
        
        setAccount(address);
        setConnected(true);
    };
    
    const createPost = async (content: string) => {
        // 上传到IPFS
        const ipfsHash = await uploadToIPFS(content);
        
        // 调用智能合约
        const contract = getContract();
        const tx = await contract.createPost(ipfsHash);
        await tx.wait();
        
        loadPosts();
    };
    
    const loadPosts = async () => {
        const contract = getContract();
        const postCount = await contract.getPostCount();
        
        const loadedPosts: Post[] = [];
        for (let i = 0; i < postCount; i++) {
            const post = await contract.getPost(i);
            const content = await fetchFromIPFS(post.ipfsHash);
            
            loadedPosts.push({
                id: post.id,
                author: post.author,
                content,
                ipfsHash: post.ipfsHash,
                timestamp: post.timestamp.toNumber(),
                likes: post.likes.toNumber(),
            });
        }
        
        setPosts(loadedPosts);
    };
    
    return (
        <div>
            {!connected ? (
                <button onClick={connectWallet}>Connect Wallet</button>
            ) : (
                <>
                    <CreatePostForm onSubmit={createPost} />
                    <PostList posts={posts} />
                </>
            )}
        </div>
    );
};
*/
```

### 6.2 项目14: DeFi仪表板

```rust
/// 项目14: DeFi仪表板
/// 学习目标: 数据聚合、实时价格、收益计算

/// DeFi仪表板后端服务
pub struct DeFiDashboard {
    pub protocols: Vec<Protocol>,
    pub price_feeds: HashMap<String, f64>,
}

pub struct Protocol {
    pub name: String,
    pub tvl: f64,
    pub apy: f64,
    pub user_position: UserPosition,
}

pub struct UserPosition {
    pub deposited: f64,
    pub borrowed: f64,
    pub earned: f64,
    pub health_factor: f64,
}

impl DeFiDashboard {
    /// 获取用户总览
    pub async fn get_user_overview(&self, address: &str) -> UserOverview {
        let mut total_deposited = 0.0;
        let mut total_borrowed = 0.0;
        let mut total_earned = 0.0;
        
        for protocol in &self.protocols {
            total_deposited += protocol.user_position.deposited;
            total_borrowed += protocol.user_position.borrowed;
            total_earned += protocol.user_position.earned;
        }
        
        UserOverview {
            total_deposited,
            total_borrowed,
            total_earned,
            net_worth: total_deposited - total_borrowed + total_earned,
            protocols: self.protocols.clone(),
        }
    }
    
    /// 计算总APY
    pub fn calculate_portfolio_apy(&self) -> f64 {
        let total_deposited: f64 = self.protocols.iter()
            .map(|p| p.user_position.deposited)
            .sum();
        
        if total_deposited == 0.0 {
            return 0.0;
        }
        
        let weighted_apy: f64 = self.protocols.iter()
            .map(|p| p.apy * p.user_position.deposited)
            .sum();
        
        weighted_apy / total_deposited
    }
}

pub struct UserOverview {
    pub total_deposited: f64,
    pub total_borrowed: f64,
    pub total_earned: f64,
    pub net_worth: f64,
    pub protocols: Vec<Protocol>,
}
```

### 6.3 项目15: Web3游戏

```rust
/// 项目15: Web3游戏
/// 学习目标: NFT游戏资产、链上逻辑、游戏经济

/// 游戏角色NFT
pub struct GameCharacter {
    pub token_id: u64,
    pub owner: String,
    pub attributes: CharacterAttributes,
    pub equipment: Vec<ItemNFT>,
    pub level: u32,
    pub experience: u64,
}

pub struct CharacterAttributes {
    pub strength: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub vitality: u32,
}

pub struct ItemNFT {
    pub token_id: u64,
    pub item_type: ItemType,
    pub rarity: Rarity,
    pub stats: HashMap<String, u32>,
}

pub enum ItemType {
    Weapon,
    Armor,
    Accessory,
    Consumable,
}

pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

/// 游戏逻辑
pub struct GameLogic {
    pub characters: HashMap<u64, GameCharacter>,
    pub items: HashMap<u64, ItemNFT>,
}

impl GameLogic {
    /// 战斗系统
    pub fn battle(
        &self,
        char1_id: u64,
        char2_id: u64,
    ) -> BattleResult {
        let char1 = self.characters.get(&char1_id).unwrap();
        let char2 = self.characters.get(&char2_id).unwrap();
        
        let power1 = self.calculate_power(char1);
        let power2 = self.calculate_power(char2);
        
        // 简化的战斗逻辑
        let winner = if power1 > power2 { char1_id } else { char2_id };
        
        BattleResult {
            winner,
            exp_gained: 100,
            items_dropped: vec![],
        }
    }
    
    fn calculate_power(&self, character: &GameCharacter) -> u32 {
        let base_power = character.attributes.strength 
            + character.attributes.agility 
            + character.attributes.intelligence;
        
        let equipment_bonus: u32 = character.equipment.iter()
            .map(|item| item.stats.values().sum::<u32>())
            .sum();
        
        base_power + equipment_bonus
    }
}

pub struct BattleResult {
    pub winner: u64,
    pub exp_gained: u64,
    pub items_dropped: Vec<ItemNFT>,
}
```

## 7. 项目开发最佳实践

### 7.1 代码规范

```rust
/// 代码规范检查清单
pub struct CodeStyleGuide {
    pub naming_conventions: NamingConventions,
    pub documentation: DocumentationStandards,
    pub error_handling: ErrorHandlingPractices,
}

pub struct NamingConventions {
    pub snake_case_for_variables: bool,
    pub camel_case_for_types: bool,
    pub descriptive_names: bool,
}

pub struct DocumentationStandards {
    pub module_level_docs: bool,
    pub function_docs: bool,
    pub complex_logic_comments: bool,
}

pub struct ErrorHandlingPractices {
    pub use_result_type: bool,
    pub custom_error_types: bool,
    pub descriptive_errors: bool,
}
```

### 7.2 测试策略

```rust
/// 测试策略
pub enum TestStrategy {
    UnitTests,
    IntegrationTests,
    EndToEndTests,
    FuzzTesting,
    PropertyBasedTesting,
}

#[cfg(test)]
mod test_examples {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        // 单元测试示例
        let blockchain = SimpleBlockchain::new(2);
        assert_eq!(blockchain.chain.len(), 1);
    }
    
    #[tokio::test]
    async fn test_async_functionality() {
        // 异步测试示例
        let mut crypto = SimpleCryptocurrency::new(2, 100);
        crypto.mine_pending_transactions("miner1".to_string());
        assert_eq!(crypto.get_balance("miner1"), 100);
    }
}
```

### 7.3 文档编写

```rust
/// 文档结构
pub struct ProjectDocumentation {
    pub readme: String,
    pub architecture: String,
    pub api_reference: String,
    pub tutorials: Vec<Tutorial>,
    pub changelog: String,
}

pub struct Tutorial {
    pub title: String,
    pub level: DifficultyLevel,
    pub content: String,
    pub code_examples: Vec<String>,
}
```

## 8. 部署与运维

### 8.1 测试网部署

```typescript
// 部署脚本示例(Hardhat)
/*
async function main() {
    const [deployer] = await ethers.getSigners();
    
    console.log("Deploying contracts with account:", deployer.address);
    
    const Token = await ethers.getContractFactory("ERC20Token");
    const token = await Token.deploy("My Token", "MTK", 18, 1000000);
    
    await token.deployed();
    
    console.log("Token deployed to:", token.address);
    
    // 验证合约
    await hre.run("verify:verify", {
        address: token.address,
        constructorArguments: ["My Token", "MTK", 18, 1000000],
    });
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
*/
```

### 8.2 主网部署

```rust
/// 主网部署检查清单
pub struct MainnetDeploymentChecklist {
    pub security_audit_completed: bool,
    pub testnet_tested: bool,
    pub gas_optimization: bool,
    pub emergency_pause_mechanism: bool,
    pub upgrade_strategy: bool,
    pub monitoring_setup: bool,
    pub insurance_considered: bool,
}
```

### 8.3 监控与维护

```rust
/// 监控系统
pub struct MonitoringSystem {
    pub metrics: Metrics,
    pub alerts: Vec<Alert>,
}

pub struct Metrics {
    pub transaction_count: u64,
    pub gas_price: u64,
    pub block_time: f64,
    pub active_users: u64,
    pub tvl: f64,
}

pub struct Alert {
    pub level: AlertLevel,
    pub message: String,
    pub timestamp: u64,
}

pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}
```

## 9. 项目展示与开源

### 9.1 GitHub项目管理

```markdown
    # README.md模板

    ## 项目名称

    简短描述

    ## 特性

    - 特性1
    - 特性2

    ## 快速开始

    ### 安装
    ```bash
    cargo build --release
    ```

    ### 使用

    ```rust
    // 代码示例
    ```

    ## 文档

    完整文档: [链接]

    ## 贡献

    欢迎贡献!

    ## 许可证

    MIT

```

### 9.2 技术博客撰写

```rust
/// 博客文章结构
pub struct BlogPost {
    pub title: String,
    pub introduction: String,
    pub problem_statement: String,
    pub solution: String,
    pub implementation: String,
    pub results: String,
    pub conclusion: String,
    pub code_snippets: Vec<CodeSnippet>,
}

pub struct CodeSnippet {
    pub language: String,
    pub code: String,
    pub explanation: String,
}
```

### 9.3 社区参与

```rust
/// 社区活动
pub enum CommunityActivity {
    OpenSource {
        contributions: u32,
        repositories: Vec<String>,
    },
    
    TechnicalWriting {
        articles: Vec<String>,
        tutorials: Vec<String>,
    },
    
    Speaking {
        conferences: Vec<String>,
        workshops: Vec<String>,
    },
    
    Mentoring {
        mentees: u32,
        projects_guided: u32,
    },
}
```

## 10. 进阶学习资源

```rust
/// 学习资源
pub struct LearningResources {
    pub books: Vec<Book>,
    pub online_courses: Vec<Course>,
    pub documentation: Vec<Documentation>,
    pub communities: Vec<Community>,
}

pub struct Book {
    pub title: String,
    pub author: String,
    pub topics: Vec<String>,
}

pub struct Course {
    pub name: String,
    pub platform: String,
    pub level: DifficultyLevel,
}

pub struct Documentation {
    pub name: String,
    pub url: String,
    pub topics: Vec<String>,
}

pub struct Community {
    pub name: String,
    pub platform: String,
    pub focus: String,
}

pub fn recommended_resources() -> LearningResources {
    LearningResources {
        books: vec![
            Book {
                title: "Mastering Ethereum".to_string(),
                author: "Andreas M. Antonopoulos".to_string(),
                topics: vec!["Ethereum".to_string(), "Smart Contracts".to_string()],
            },
        ],
        online_courses: vec![
            Course {
                name: "Blockchain Specialization".to_string(),
                platform: "Coursera".to_string(),
                level: DifficultyLevel::Intermediate,
            },
        ],
        documentation: vec![
            Documentation {
                name: "Ethereum Docs".to_string(),
                url: "https://ethereum.org/developers".to_string(),
                topics: vec!["Ethereum".to_string()],
            },
            Documentation {
                name: "Rust Book".to_string(),
                url: "https://doc.rust-lang.org/book/".to_string(),
                topics: vec!["Rust".to_string()],
            },
        ],
        communities: vec![
            Community {
                name: "Ethereum Research".to_string(),
                platform: "Discord".to_string(),
                focus: "Protocol Research".to_string(),
            },
        ],
    }
}
```

## 总结

本实践项目指南涵盖了从初级到专家级的15个完整项目：

**初级项目(1-3)**:

- 简单区块链
- 基础加密货币
- 投票系统

**中级项目(4-6)**:

- ERC-20代币
- NFT市场
- 简单DEX

**高级项目(7-9)**:

- 借贷协议
- DAO治理平台
- 跨链桥

**专家级项目(10-12)**:

- Layer 2 Rollup
- 零知识证明应用
- 完整公链

**全栈项目(13-15)**:

- 去中心化社交网络
- DeFi仪表板
- Web3游戏

通过完成这些项目，你将掌握区块链开发的全部核心技能！

---

**文档版本**: v1.0.0  
**最后更新**: 2025年10月17日  
**作者**: 区块链教育专家  
**审核**: 实战项目导师

## 相关文档

- [Rust语言实现](./12_RUST_IMPLEMENTATION.md)
- [共识算法实现](./14_CONSENSUS_IMPLEMENTATION.md)
- [DeFi应用指南](./22_DEFI_APPLICATIONS.md)
- [Web3技术栈](./24_WEB3_TECHNOLOGIES.md)

## 项目模板

访问 GitHub 仓库获取所有项目的完整模板代码：

- <https://github.com/blockchain-rust/project-templates>
