# 开发指南

## 📋 目录

- [开发指南](#开发指南)
  - [📋 目录](#-目录)
  - [1. 代码规范](#1-代码规范)
    - [1.1 Rust编码规范](#11-rust编码规范)
    - [1.2 Solidity编码规范](#12-solidity编码规范)
    - [1.3 命名约定](#13-命名约定)
  - [2. 项目结构](#2-项目结构)
    - [2.1 目录组织](#21-目录组织)
    - [2.2 模块划分](#22-模块划分)
    - [2.3 依赖管理](#23-依赖管理)
  - [3. 开发流程](#3-开发流程)
    - [3.1 Git工作流](#31-git工作流)
    - [3.2 代码审查](#32-代码审查)
    - [3.3 持续集成](#33-持续集成)
  - [4. 文档规范](#4-文档规范)
    - [4.1 代码注释](#41-代码注释)
    - [4.2 API文档](#42-api文档)
    - [4.3 用户文档](#43-用户文档)
  - [5. 错误处理](#5-错误处理)
    - [5.1 错误类型设计](#51-错误类型设计)
    - [5.2 错误传播](#52-错误传播)
    - [5.3 日志记录](#53-日志记录)
  - [6. 性能优化](#6-性能优化)
    - [6.1 性能测试](#61-性能测试)
    - [6.2 性能分析](#62-性能分析)
    - [6.3 优化策略](#63-优化策略)
  - [7. 安全开发](#7-安全开发)
    - [7.1 安全审计](#71-安全审计)
    - [7.2 漏洞修复](#72-漏洞修复)
    - [7.3 安全测试](#73-安全测试)
  - [8. 工具链](#8-工具链)
    - [8.1 开发工具](#81-开发工具)
    - [8.2 调试工具](#82-调试工具)
    - [8.3 部署工具](#83-部署工具)
  - [9. 最佳实践](#9-最佳实践)
    - [9.1 设计模式](#91-设计模式)
    - [9.2 代码复用](#92-代码复用)
    - [9.3 可维护性](#93-可维护性)
  - [10. 总结](#10-总结)

## 1. 代码规范

### 1.1 Rust编码规范

```rust
//! 模块级文档注释
//! 
//! 详细说明模块的用途和设计思路

/// Rust编码规范示例
pub mod rust_coding_standards {
    use std::sync::Arc;
    
    /// 结构体注释：说明结构体的用途
    /// 
    /// # 示例
    /// 
    /// ```
    /// let account = Account::new(address);
    /// ```
    pub struct Account {
        /// 账户地址
        address: Address,
        /// 余额（单位：wei）
        balance: u128,
        /// 交易nonce
        nonce: u64,
    }
    
    impl Account {
        /// 创建新账户
        /// 
        /// # 参数
        /// 
        /// * `address` - 账户地址
        /// 
        /// # 返回值
        /// 
        /// 返回新创建的账户实例
        pub fn new(address: Address) -> Self {
            Self {
                address,
                balance: 0,
                nonce: 0,
            }
        }
        
        /// 转账
        /// 
        /// # 错误
        /// 
        /// 如果余额不足，返回 `AccountError::InsufficientBalance`
        pub fn transfer(&mut self, amount: u128) -> Result<(), AccountError> {
            if self.balance < amount {
                return Err(AccountError::InsufficientBalance);
            }
            
            self.balance -= amount;
            Ok(())
        }
    }
    
    // 命名规范：
    // - 类型名称：PascalCase（如 Account、BlockHeader）
    // - 函数名称：snake_case（如 create_block、verify_signature）
    // - 常量：SCREAMING_SNAKE_CASE（如 MAX_BLOCK_SIZE）
    // - 生命周期：小写字母（如 'a, 'static）
    
    /// 常量定义
    pub const MAX_BLOCK_SIZE: usize = 1024 * 1024; // 1MB
    pub const MIN_GAS_LIMIT: u64 = 21_000;
    
    /// 使用类型别名提高可读性
    pub type Address = [u8; 20];
    pub type Hash = [u8; 32];
    
    /// 错误类型应该实现标准错误trait
    #[derive(Debug, thiserror::Error)]
    pub enum AccountError {
        #[error("Insufficient balance")]
        InsufficientBalance,
        #[error("Invalid address")]
        InvalidAddress,
    }
    
    // 格式化规范：
    // - 使用 rustfmt 自动格式化代码
    // - 行宽限制：100字符
    // - 缩进：4空格
    // - 末尾无空白
    
    /// 异步函数示例
    pub async fn fetch_block(number: u64) -> Result<Block, FetchError> {
        // 异步操作
        Ok(Block::default())
    }
    
    /// 泛型函数示例
    pub fn serialize<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, SerializationError> {
        serde_json::to_vec(value)
            .map_err(|e| SerializationError::JsonError(e.to_string()))
    }
    
    // 避免的反模式：
    // ❌ 过长的函数（超过100行）
    // ❌ 过深的嵌套（超过4层）
    // ❌ 未处理的Result
    // ❌ 使用unwrap()在库代码中
    // ❌ 公共API中的panic
    
    /// 良好的实践：
    /// 1. 使用 ? 操作符传播错误
    /// 2. 优先使用iter()而非循环
    /// 3. 使用 match 而非多个 if let
    /// 4. 利用类型系统保证正确性
    pub fn good_practices_example() -> Result<(), ExampleError> {
        // ✅ 使用 ? 传播错误
        let data = read_data()?;
        
        // ✅ 使用迭代器
        let sum: u64 = data.iter().sum();
        
        // ✅ 使用 match
        match sum {
            0 => println!("Empty"),
            1..=100 => println!("Small"),
            _ => println!("Large"),
        }
        
        Ok(())
    }
    
    fn read_data() -> Result<Vec<u64>, ExampleError> {
        Ok(vec![1, 2, 3])
    }
    
    #[derive(Debug, Default)]
    pub struct Block {
        // 简化实现
    }
    
    #[derive(Debug, thiserror::Error)]
    pub enum FetchError {
        #[error("Network error")]
        NetworkError,
    }
    
    #[derive(Debug, thiserror::Error)]
    pub enum SerializationError {
        #[error("JSON error: {0}")]
        JsonError(String),
    }
    
    #[derive(Debug, thiserror::Error)]
    pub enum ExampleError {
        #[error("Example error")]
        Example,
    }
}
```

### 1.2 Solidity编码规范

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title Solidity编码规范示例
 * @author Rust区块链技术团队
 * @notice 遵循Solidity Style Guide和最佳实践
 * @dev 详细的技术说明
 */
contract CodingStandardsExample {
    // 状态变量
    // 命名规范：
    // - 常量：UPPER_CASE_WITH_UNDERSCORES
    // - 状态变量：mixedCase
    // - 私有变量：_leadingUnderscore
    
    /// @notice 最大供应量
    uint256 public constant MAX_SUPPLY = 1_000_000 * 10**18;
    
    /// @notice 所有者地址
    address public owner;
    
    /// @dev 内部计数器
    uint256 private _counter;
    
    // 事件
    // 命名规范：PascalCase
    
    /// @notice 转账事件
    /// @param from 发送方地址
    /// @param to 接收方地址
    /// @param amount 转账金额
    event Transfer(
        address indexed from,
        address indexed to,
        uint256 amount
    );
    
    /// @notice 所有权转移事件
    /// @param previousOwner 前所有者
    /// @param newOwner 新所有者
    event OwnershipTransferred(
        address indexed previousOwner,
        address indexed newOwner
    );
    
    // 自定义错误（Gas高效）
    error Unauthorized();
    error InsufficientBalance(uint256 available, uint256 required);
    error InvalidAddress();
    
    // 修饰器
    // 命名规范：mixedCase
    
    /// @notice 仅所有者可调用
    modifier onlyOwner() {
        if (msg.sender != owner) revert Unauthorized();
        _;
    }
    
    /// @notice 地址有效性检查
    modifier validAddress(address _address) {
        if (_address == address(0)) revert InvalidAddress();
        _;
    }
    
    // 构造函数
    constructor() {
        owner = msg.sender;
    }
    
    // 外部函数
    // 可见性顺序：external > public > internal > private
    
    /// @notice 转移所有权
    /// @param newOwner 新所有者地址
    function transferOwnership(address newOwner)
        external
        onlyOwner
        validAddress(newOwner)
    {
        address previousOwner = owner;
        owner = newOwner;
        emit OwnershipTransferred(previousOwner, newOwner);
    }
    
    /// @notice 获取计数器值
    /// @return 当前计数器值
    function getCounter() external view returns (uint256) {
        return _counter;
    }
    
    // 公共函数
    function increment() public {
        _counter += 1;
    }
    
    // 内部函数
    // 命名规范：_leadingUnderscore
    function _internalFunction() internal {
        // 内部逻辑
    }
    
    // 私有函数
    function _privateFunction() private {
        // 私有逻辑
    }
    
    // 格式化规范：
    // - 缩进：4空格
    // - 行宽限制：120字符
    // - 大括号：函数和合约使用新行，if/for等使用同行
    // - 导入：按字母顺序排列
    
    // 安全最佳实践：
    // ✅ 使用 Checks-Effects-Interactions 模式
    // ✅ 防止重入攻击
    // ✅ 使用 SafeMath（或 Solidity 0.8+内置检查）
    // ✅ 验证外部调用返回值
    // ✅ 限制循环迭代次数
    
    /// @notice 安全转账示例
    function safeTransfer(address to, uint256 amount)
        external
        validAddress(to)
    {
        // Checks
        if (balanceOf[msg.sender] < amount) {
            revert InsufficientBalance({
                available: balanceOf[msg.sender],
                required: amount
            });
        }
        
        // Effects
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        
        // Interactions
        emit Transfer(msg.sender, to, amount);
    }
    
    mapping(address => uint256) public balanceOf;
}
```

### 1.3 命名约定

```rust
/// 命名约定参考指南
pub mod naming_conventions {
    
    // 1. 模块命名
    // - 使用 snake_case
    // - 避免过长的名称
    pub mod storage_layer {}
    pub mod consensus_engine {}
    
    // 2. 结构体和枚举
    // - 使用 PascalCase
    // - 名词或名词短语
    pub struct BlockHeader {}
    pub struct TransactionPool {}
    
    pub enum ConsensusState {
        Idle,
        Proposing,
        Voting,
    }
    
    // 3. trait命名
    // - 使用 PascalCase
    // - 通常以-able, -er结尾
    pub trait Serializable {}
    pub trait Validator {}
    
    // 4. 函数命名
    // - 使用 snake_case
    // - 动词开头
    pub fn create_block() {}
    pub fn verify_signature() {}
    pub fn is_valid() -> bool { true }
    pub fn has_data() -> bool { false }
    
    // 5. 常量
    // - 使用 SCREAMING_SNAKE_CASE
    pub const MAX_BLOCK_SIZE: usize = 1024;
    pub const DEFAULT_GAS_PRICE: u64 = 1_000_000_000;
    
    // 6. 静态变量
    pub static INSTANCE_COUNTER: std::sync::atomic::AtomicUsize = 
        std::sync::atomic::AtomicUsize::new(0);
    
    // 7. 泛型参数
    // - 单字母（简单情况）或 PascalCase（复杂情况）
    pub struct Container<T> {
        item: T,
    }
    
    pub struct Cache<Key, Value> {
        data: std::collections::HashMap<Key, Value>,
    }
    
    // 8. 生命周期
    // - 小写字母，简短有意义
    pub struct Reference<'a> {
        data: &'a str,
    }
    
    // 9. 缩写
    // - 短缩写：全大写（如 HTTP, URL）
    // - 长缩写：首字母大写（如 Html, Json）
    pub struct HTTPClient {}
    pub struct JsonParser {}
}
```

## 2. 项目结构

### 2.1 目录组织

```text
blockchain/
├── Cargo.toml              # 工作空间配置
├── README.md               # 项目说明
├── LICENSE                 # 许可证
├── .gitignore             # Git忽略文件
├── .github/               # GitHub配置
│   └── workflows/         # CI/CD工作流
│       ├── ci.yml
│       └── release.yml
├── docs/                  # 文档目录
│   ├── api/              # API文档
│   ├── guides/           # 指南
│   └── specifications/   # 规范
├── src/                   # 源代码目录
│   ├── lib.rs            # 库入口
│   ├── main.rs           # 主程序入口
│   ├── core/             # 核心模块
│   │   ├── mod.rs
│   │   ├── block.rs
│   │   ├── transaction.rs
│   │   └── blockchain.rs
│   ├── consensus/        # 共识模块
│   │   ├── mod.rs
│   │   ├── pow.rs
│   │   └── pos.rs
│   ├── network/          # 网络模块
│   │   ├── mod.rs
│   │   ├── p2p.rs
│   │   └── rpc.rs
│   ├── storage/          # 存储模块
│   │   ├── mod.rs
│   │   ├── database.rs
│   │   └── state.rs
│   └── utils/            # 工具模块
│       ├── mod.rs
│       └── crypto.rs
├── tests/                 # 集成测试
│   ├── common/           # 测试公共代码
│   ├── integration_test.rs
│   └── e2e_test.rs
├── benches/              # 性能测试
│   └── benchmark.rs
└── examples/             # 示例代码
    ├── simple_node.rs
    └── full_node.rs
```

### 2.2 模块划分

```rust
//! 模块划分最佳实践

/// 核心模块：包含最基本的区块链数据结构
pub mod core {
    pub mod block;
    pub mod transaction;
    pub mod blockchain;
    pub mod state;
}

/// 共识模块：实现各种共识算法
pub mod consensus {
    pub mod pow;
    pub mod pos;
    pub mod bft;
    
    // 公共接口
    pub use self::pow::ProofOfWork;
    pub use self::pos::ProofOfStake;
}

/// 网络模块：处理P2P通信和RPC
pub mod network {
    pub mod p2p;
    pub mod rpc;
    pub mod protocol;
}

/// 存储模块：数据持久化
pub mod storage {
    pub mod database;
    pub mod cache;
    pub mod merkle;
}

/// 加密模块：加密算法和签名
pub mod crypto {
    pub mod hash;
    pub mod signature;
    pub mod encryption;
}

/// 工具模块：通用工具函数
pub mod utils {
    pub mod encoding;
    pub mod time;
    pub mod validation;
}

/// 模块组织原则：
/// 1. 单一职责：每个模块只负责一个功能域
/// 2. 低耦合：模块间依赖最小化
/// 3. 高内聚：相关功能放在同一模块
/// 4. 公开接口：使用 pub use 重导出公共API
```

### 2.3 依赖管理

```toml
[package]
name = "blockchain"
version = "1.0.0"
edition = "2021"
rust-version = "1.70"
authors = ["Rust Blockchain Team"]
description = "A blockchain implementation in Rust"
license = "MIT"
repository = "https://github.com/example/blockchain"
keywords = ["blockchain", "cryptocurrency", "consensus"]
categories = ["cryptography", "network-programming"]

[dependencies]
# 异步运行时
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bincode = "1.3"

# 加密
sha3 = "0.10"
sha2 = "0.10"
secp256k1 = "0.27"
ed25519-dalek = "2.0"

# 数据库
rocksdb = "0.21"
sled = "0.34"

# 网络
libp2p = "0.53"

# 日志
tracing = "0.1"
tracing-subscriber = "0.3"

# 错误处理
thiserror = "1.0"
anyhow = "1.0"

[dev-dependencies]
# 测试框架
criterion = "0.5"
proptest = "1.4"
tempfile = "3.8"

[build-dependencies]
# 构建脚本依赖
built = "0.7"

[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"

[profile.bench]
inherits = "release"

# 依赖管理原则：
# 1. 锁定版本：使用 Cargo.lock
# 2. 定期更新：使用 cargo-audit 检查安全问题
# 3. 最小化依赖：只引入必要的依赖
# 4. Feature gates：按需启用功能
```

## 3. 开发流程

### 3.1 Git工作流

```bash
# Git工作流规范

# 1. 分支策略
# - main: 生产分支，只接受合并
# - develop: 开发分支
# - feature/*: 功能分支
# - hotfix/*: 紧急修复分支
# - release/*: 发布分支

# 2. 功能开发流程
# 创建功能分支
git checkout -b feature/add-consensus-algorithm develop

# 开发过程中定期提交
git add .
git commit -m "feat: add basic PoS consensus structure"

# 保持与develop同步
git fetch origin
git rebase origin/develop

# 完成后推送
git push origin feature/add-consensus-algorithm

# 在GitHub/GitLab上创建Pull Request

# 3. 提交信息规范（Conventional Commits）
# 格式：<type>(<scope>): <subject>

# 类型：
# - feat: 新功能
# - fix: Bug修复
# - docs: 文档变更
# - style: 代码格式（不影响代码运行）
# - refactor: 重构
# - perf: 性能优化
# - test: 测试相关
# - chore: 构建过程或辅助工具变动

# 示例：
git commit -m "feat(consensus): implement PoS validator selection"
git commit -m "fix(network): resolve peer connection timeout issue"
git commit -m "docs: update API documentation for RPC methods"

# 4. 代码审查检查清单
# ✓ 代码符合规范
# ✓ 测试覆盖率充分
# ✓ 文档已更新
# ✓ 无未解决的TODO
# ✓ 性能无明显退化
# ✓ 安全问题已考虑
```

### 3.2 代码审查

```rust
/// 代码审查清单
pub mod code_review {
    /// 代码审查检查项
    pub struct CodeReviewChecklist {
        items: Vec<CheckItem>,
    }
    
    #[derive(Debug)]
    pub struct CheckItem {
        category: &'static str,
        items: Vec<&'static str>,
    }
    
    impl CodeReviewChecklist {
        pub fn new() -> Self {
            Self {
                items: vec![
                    CheckItem {
                        category: "代码质量",
                        items: vec![
                            "代码清晰易读",
                            "函数职责单一",
                            "避免代码重复",
                            "复杂度合理",
                            "命名准确",
                        ],
                    },
                    CheckItem {
                        category: "功能正确性",
                        items: vec![
                            "实现符合需求",
                            "边界情况处理",
                            "错误处理完善",
                            "无逻辑错误",
                        ],
                    },
                    CheckItem {
                        category: "测试",
                        items: vec![
                            "单元测试充分",
                            "集成测试覆盖",
                            "测试用例有意义",
                            "测试可重复",
                        ],
                    },
                    CheckItem {
                        category: "性能",
                        items: vec![
                            "无明显性能问题",
                            "资源使用合理",
                            "算法复杂度适当",
                            "无内存泄漏",
                        ],
                    },
                    CheckItem {
                        category: "安全",
                        items: vec![
                            "输入验证",
                            "权限检查",
                            "敏感数据保护",
                            "无常见漏洞",
                        ],
                    },
                    CheckItem {
                        category: "文档",
                        items: vec![
                            "公共API有文档",
                            "复杂逻辑有注释",
                            "README已更新",
                            "示例代码正确",
                        ],
                    },
                ],
            }
        }
    }
}
```

### 3.3 持续集成

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

env:
  CARGO_TERM_COLOR: always

jobs:
  # 代码检查
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy
          
      - name: Check formatting
        run: cargo fmt -- --check
        
      - name: Clippy lints
        run: cargo clippy -- -D warnings
  
  # 构建测试
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, nightly]
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}
          
      - name: Build
        run: cargo build --verbose
        
      - name: Run tests
        run: cargo test --verbose
        
      - name: Run benchmarks
        run: cargo bench --no-run
  
  # 代码覆盖率
  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
        
      - name: Generate coverage
        run: cargo tarpaulin --out Xml
        
      - name: Upload to codecov
        uses: codecov/codecov-action@v3
  
  # 安全审计
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Security audit
        uses: actions-rs/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
```

## 4. 文档规范

### 4.1 代码注释

```rust
//! # 模块级文档
//! 
//! 详细说明模块的用途、设计思路和使用方法。
//! 
//! ## 示例
//! 
//! ```
//! use blockchain::core::Block;
//! 
//! let block = Block::new();
//! ```

/// 函数文档注释
/// 
/// 详细说明函数的用途和行为。
/// 
/// # 参数
/// 
/// * `data` - 要处理的数据
/// * `options` - 可选配置
/// 
/// # 返回值
/// 
/// 成功时返回 `Ok(result)`，失败时返回 `Err(error)`
/// 
/// # 错误
/// 
/// 如果数据无效，返回 `DataError::Invalid`
/// 
/// # 示例
/// 
/// ```
/// let result = process_data(&data, None)?;
/// assert_eq!(result.len(), expected_len);
/// ```
/// 
/// # 性能
/// 
/// 时间复杂度：O(n)
/// 空间复杂度：O(1)
/// 
/// # 安全
/// 
/// 此函数会验证所有输入，不会panic
pub fn process_data(
    data: &[u8],
    options: Option<ProcessOptions>
) -> Result<Vec<u8>, DataError> {
    // 实现
    Ok(vec![])
}

// 行内注释：解释复杂或不明显的代码
fn complex_calculation() -> u64 {
    let mut result = 0;
    
    // 使用快速幂算法计算
    for i in 0..10 {
        result += i.pow(2);
    }
    
    result
}

/// TODO注释：标记待办事项
/// TODO(username): Add input validation
/// FIXME: This has a bug in edge cases
/// HACK: Temporary workaround for upstream issue
/// NOTE: Important implementation detail

#[derive(Debug)]
pub struct ProcessOptions {
    // 字段注释
}

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("Invalid data")]
    Invalid,
}
```

### 4.2 API文档

```rust
//! # API Documentation
//! 
//! 本模块提供区块链核心API。
//! 
//! ## 快速开始
//! 
//! ```no_run
//! use blockchain::{Blockchain, Block};
//! 
//! let mut chain = Blockchain::new();
//! let block = Block::new(vec![], chain.latest_hash());
//! chain.add_block(block)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! 
//! ## 模块结构
//! 
//! - [`Blockchain`] - 区块链主结构
//! - [`Block`] - 区块结构
//! - [`Transaction`] - 交易结构
//! 
//! ## 特性
//! 
//! - PoW/PoS共识支持
//! - 智能合约虚拟机
//! - P2P网络
//! - 状态存储
//! 
//! ## 配置
//! 
//! 通过feature gates启用可选功能：
//! 
//! ```toml
//! [dependencies]
//! blockchain = { version = "1.0", features = ["rocksdb", "metrics"] }
//! ```

/// 主区块链结构
/// 
/// 管理区块链的核心数据结构和操作。
/// 
/// # 线程安全
/// 
/// `Blockchain` 实现了 `Send` 和 `Sync`，可以安全地在线程间共享。
/// 
/// # 示例
/// 
/// ```
/// # use blockchain::Blockchain;
/// let chain = Blockchain::new();
/// assert_eq!(chain.height(), 0);
/// ```
pub struct Blockchain {
    // 字段
}

impl Blockchain {
    /// 创建新的区块链
    /// 
    /// # 示例
    /// 
    /// ```
    /// # use blockchain::Blockchain;
    /// let chain = Blockchain::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
    
    /// 添加新区块
    /// 
    /// # 参数
    /// 
    /// * `block` - 要添加的区块
    /// 
    /// # 错误
    /// 
    /// - `BlockchainError::InvalidBlock` - 区块验证失败
    /// - `BlockchainError::DuplicateBlock` - 区块已存在
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// # use blockchain::{Blockchain, Block};
    /// # let mut chain = Blockchain::new();
    /// let block = Block::new(vec![], chain.latest_hash());
    /// chain.add_block(block)?;
    /// # Ok::<(), blockchain::BlockchainError>(())
    /// ```
    pub fn add_block(&mut self, block: Block) -> Result<(), BlockchainError> {
        Ok(())
    }
    
    /// 获取区块链高度
    pub fn height(&self) -> u64 {
        0
    }
    
    /// 获取最新区块哈希
    pub fn latest_hash(&self) -> Hash {
        Hash::default()
    }
}

#[derive(Debug, Default)]
pub struct Block {
    // 简化实现
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, prev_hash: Hash) -> Self {
        Self::default()
    }
}

#[derive(Debug)]
pub struct Transaction {
    // 简化实现
}

#[derive(Debug, Default)]
pub struct Hash([u8; 32]);

#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
    #[error("Invalid block")]
    InvalidBlock,
    #[error("Duplicate block")]
    DuplicateBlock,
}
```

### 4.3 用户文档

```markdown
# 用户文档规范

## 文档结构

### README.md
- 项目简介
- 功能特性
- 快速开始
- 安装说明
- 基本使用
- 贡献指南
- 许可证

### 安装指南（INSTALL.md）
- 系统要求
- 依赖安装
- 从源码构建
- 二进制安装
- Docker部署
- 验证安装

### 使用手册（USAGE.md）
- 基本概念
- 配置说明
- 命令参考
- API参考
- 最佳实践
- 常见问题

### 开发指南（DEVELOPMENT.md）
- 开发环境设置
- 代码结构
- 构建说明
- 测试指南
- 调试技巧
- 贡献流程

## 写作规范

1. **清晰简洁**
   - 使用简单直接的语言
   - 避免行话，或加以解释
   - 一个段落一个主题

2. **结构化**
   - 使用标题层次
   - 列表和表格
   - 代码示例
   - 图表说明

3. **示例驱动**
   - 提供实际可运行的示例
   - 从简单到复杂
   - 覆盖常见场景

4. **保持更新**
   - 随代码同步更新
   - 标注版本信息
   - 记录变更历史
```

## 5. 错误处理

### 5.1 错误类型设计

```rust
/// 错误处理最佳实践
pub mod error_handling {
    use thiserror::Error;
    
    /// 顶层错误类型
    #[derive(Debug, Error)]
    pub enum BlockchainError {
        /// 共识错误
        #[error("Consensus error: {0}")]
        Consensus(#[from] ConsensusError),
        
        /// 网络错误
        #[error("Network error: {0}")]
        Network(#[from] NetworkError),
        
        /// 存储错误
        #[error("Storage error: {0}")]
        Storage(#[from] StorageError),
        
        /// IO错误
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),
        
        /// 自定义错误
        #[error("{0}")]
        Custom(String),
    }
    
    /// 共识错误
    #[derive(Debug, Error)]
    pub enum ConsensusError {
        #[error("Invalid block: {reason}")]
        InvalidBlock { reason: String },
        
        #[error("Consensus timeout")]
        Timeout,
        
        #[error("Insufficient validators")]
        InsufficientValidators,
    }
    
    /// 网络错误
    #[derive(Debug, Error)]
    pub enum NetworkError {
        #[error("Connection failed: {0}")]
        ConnectionFailed(String),
        
        #[error("Peer timeout")]
        PeerTimeout,
        
        #[error("Invalid message")]
        InvalidMessage,
    }
    
    /// 存储错误
    #[derive(Debug, Error)]
    pub enum StorageError {
        #[error("Database error: {0}")]
        Database(String),
        
        #[error("Key not found")]
        NotFound,
        
        #[error("Corruption detected")]
        Corruption,
    }
    
    /// 错误上下文增强
    pub trait ErrorContext<T> {
        fn context(self, context: &str) -> Result<T, BlockchainError>;
    }
    
    impl<T, E> ErrorContext<T> for Result<T, E>
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        fn context(self, context: &str) -> Result<T, BlockchainError> {
            self.map_err(|e| {
                BlockchainError::Custom(format!("{}: {}", context, e))
            })
        }
    }
}
```

### 5.2 错误传播

```rust
/// 错误传播示例
pub mod error_propagation {
    use super::error_handling::*;
    
    /// 使用 ? 操作符传播错误
    pub fn process_block(data: &[u8]) -> Result<(), BlockchainError> {
        // 自动转换错误类型
        let block = parse_block(data)?;
        validate_block(&block)?;
        store_block(&block)?;
        
        Ok(())
    }
    
    /// 添加上下文信息
    pub fn sync_blockchain(peer: &str) -> Result<(), BlockchainError> {
        fetch_blocks(peer)
            .context("Failed to fetch blocks")?;
        
        apply_blocks()
            .context("Failed to apply blocks")?;
        
        Ok(())
    }
    
    /// 错误恢复
    pub fn resilient_operation() -> Result<(), BlockchainError> {
        match risky_operation() {
            Ok(result) => {
                println!("Success: {:?}", result);
                Ok(())
            }
            Err(BlockchainError::Network(NetworkError::PeerTimeout)) => {
                // 特定错误的恢复逻辑
                println!("Peer timeout, retrying...");
                retry_operation()
            }
            Err(e) => {
                // 其他错误直接传播
                Err(e)
            }
        }
    }
    
    fn parse_block(data: &[u8]) -> Result<Block, BlockchainError> {
        Ok(Block::default())
    }
    
    fn validate_block(block: &Block) -> Result<(), ConsensusError> {
        Ok(())
    }
    
    fn store_block(block: &Block) -> Result<(), StorageError> {
        Ok(())
    }
    
    fn fetch_blocks(peer: &str) -> Result<(), NetworkError> {
        Ok(())
    }
    
    fn apply_blocks() -> Result<(), BlockchainError> {
        Ok(())
    }
    
    fn risky_operation() -> Result<(), BlockchainError> {
        Ok(())
    }
    
    fn retry_operation() -> Result<(), BlockchainError> {
        Ok(())
    }
    
    #[derive(Debug, Default)]
    struct Block;
}
```

### 5.3 日志记录

```rust
/// 日志记录最佳实践
pub mod logging {
    use tracing::{info, warn, error, debug, trace, instrument};
    
    /// 结构化日志
    #[instrument(skip(data))]
    pub async fn process_transaction(tx_hash: &str, data: &[u8]) -> Result<(), Error> {
        info!("Processing transaction: {}", tx_hash);
        
        // 调试信息
        debug!("Transaction size: {} bytes", data.len());
        
        match validate(data) {
            Ok(_) => {
                info!("Transaction validated successfully");
            }
            Err(e) => {
                error!("Validation failed: {}", e);
                return Err(e);
            }
        }
        
        // 警告信息
        if data.len() > 1024 * 1024 {
            warn!("Large transaction: {} bytes", data.len());
        }
        
        Ok(())
    }
    
    /// 日志级别：
    /// - ERROR: 错误事件，需要立即关注
    /// - WARN: 警告事件，可能有问题但系统可继续运行
    /// - INFO: 信息事件，重要的运行时事件
    /// - DEBUG: 调试信息，详细的运行时信息
    /// - TRACE: 跟踪信息，最详细的信息
    
    fn validate(data: &[u8]) -> Result<(), Error> {
        Ok(())
    }
    
    #[derive(Debug, thiserror::Error)]
    #[error("Validation error")]
    pub struct Error;
}
```

## 6. 性能优化

### 6.1 性能测试

已在性能优化文档中详细说明。

### 6.2 性能分析

已在性能优化文档中详细说明。

### 6.3 优化策略

已在性能优化文档中详细说明。

## 7. 安全开发

### 7.1 安全审计

已在安全最佳实践文档中详细说明。

### 7.2 漏洞修复

已在安全最佳实践文档中详细说明。

### 7.3 安全测试

已在安全最佳实践文档中详细说明。

## 8. 工具链

### 8.1 开发工具

```bash
# Rust工具链
rustup update                    # 更新Rust
rustup component add rustfmt    # 代码格式化
rustup component add clippy     # 代码检查

# 开发工具
cargo install cargo-watch       # 自动重新编译
cargo install cargo-edit        # 依赖管理
cargo install cargo-outdated    # 检查过期依赖
cargo install cargo-audit       # 安全审计
cargo install cargo-deny        # 依赖许可检查
cargo install cargo-tarpaulin   # 代码覆盖率
cargo install cargo-expand      # 宏展开
cargo install cargo-flamegraph  # 性能分析

# 使用示例
cargo watch -x test             # 监视文件变化并运行测试
cargo outdated                  # 检查过期的依赖
cargo audit                     # 安全漏洞检查
cargo deny check                # 检查依赖许可证
```

### 8.2 调试工具

```rust
/// 调试工具和技巧
pub mod debugging {
    /// 使用dbg!宏调试
    pub fn debug_macro_example() {
        let x = 5;
        let y = 10;
        
        // dbg!会打印表达式和值
        dbg!(x + y);
        
        // 可以链式调用
        let result = dbg!(x + dbg!(y * 2));
        
        println!("Result: {}", result);
    }
    
    /// 条件断言
    pub fn assertions() {
        let x = 5;
        
        // debug_assert!只在debug构建中检查
        debug_assert!(x > 0);
        
        // assert!总是检查
        assert!(x > 0, "x must be positive, got: {}", x);
    }
    
    /// 使用tracing进行调试
    #[tracing::instrument]
    pub fn traced_function(input: i32) -> i32 {
        tracing::debug!("Processing input: {}", input);
        input * 2
    }
}

// GDB/LLDB调试
// rust-gdb ./target/debug/blockchain
// rust-lldb ./target/debug/blockchain

// VSCode调试配置（.vscode/launch.json）
// {
//   "version": "0.2.0",
//   "configurations": [
//     {
//       "type": "lldb",
//       "request": "launch",
//       "name": "Debug",
//       "program": "${workspaceFolder}/target/debug/blockchain",
//       "args": [],
//       "cwd": "${workspaceFolder}"
//     }
//   ]
// }
```

### 8.3 部署工具

```bash
# Docker部署
# Dockerfile
# FROM rust:1.70 as builder
# WORKDIR /app
# COPY . .
# RUN cargo build --release
# 
# FROM debian:bullseye-slim
# COPY --from=builder /app/target/release/blockchain /usr/local/bin/
# CMD ["blockchain"]

# 构建Docker镜像
docker build -t blockchain:latest .

# 运行容器
docker run -d -p 8545:8545 blockchain:latest

# Kubernetes部署
# kubectl apply -f deployment.yaml
# kubectl apply -f service.yaml

# 系统服务（systemd）
# /etc/systemd/system/blockchain.service
# [Unit]
# Description=Blockchain Node
# After=network.target
# 
# [Service]
# Type=simple
# User=blockchain
# ExecStart=/usr/local/bin/blockchain
# Restart=on-failure
# 
# [Install]
# WantedBy=multi-user.target

# 启用服务
# sudo systemctl enable blockchain
# sudo systemctl start blockchain
```

## 9. 最佳实践

### 9.1 设计模式

已在架构设计文档中详细说明。

### 9.2 代码复用

```rust
/// 代码复用策略
pub mod code_reuse {
    /// 1. 使用trait定义通用接口
    pub trait Storage {
        type Error;
        
        fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Self::Error>;
        fn put(&mut self, key: &[u8], value: &[u8]) -> Result<(), Self::Error>;
    }
    
    /// 2. 使用泛型实现通用逻辑
    pub fn process_items<T, F>(items: &[T], processor: F) -> Vec<T>
    where
        T: Clone,
        F: Fn(&T) -> T,
    {
        items.iter().map(processor).collect()
    }
    
    /// 3. 使用宏减少重复代码
    macro_rules! impl_from_error {
        ($error:ty, $variant:ident) => {
            impl From<$error> for BlockchainError {
                fn from(e: $error) -> Self {
                    BlockchainError::$variant(e)
                }
            }
        };
    }
    
    #[derive(Debug)]
    pub enum BlockchainError {
        Io(std::io::Error),
        Parse(String),
    }
    
    impl_from_error!(std::io::Error, Io);
}
```

### 9.3 可维护性

```rust
/// 可维护性原则
pub mod maintainability {
    /// SOLID原则
    
    // S: Single Responsibility Principle（单一职责原则）
    // 每个模块只负责一个功能
    
    // O: Open/Closed Principle（开闭原则）
    // 对扩展开放，对修改封闭
    pub trait Validator {
        fn validate(&self) -> bool;
    }
    
    // L: Liskov Substitution Principle（里氏替换原则）
    // 子类型必须能替换其基类型
    
    // I: Interface Segregation Principle（接口隔离原则）
    // 不应强迫客户端依赖它不使用的方法
    pub trait Reader {
        fn read(&self) -> Vec<u8>;
    }
    
    pub trait Writer {
        fn write(&mut self, data: &[u8]);
    }
    
    // D: Dependency Inversion Principle（依赖倒置原则）
    // 依赖抽象而非具体实现
    pub struct Service<S: Storage> {
        storage: S,
    }
    
    pub trait Storage {
        fn save(&mut self, data: &[u8]);
    }
    
    /// 其他原则：
    /// - DRY: Don't Repeat Yourself
    /// - KISS: Keep It Simple, Stupid
    /// - YAGNI: You Aren't Gonna Need It
}
```

## 10. 总结

本文档提供了全面的区块链开发指南，包括：

1. **代码规范**：Rust和Solidity编码标准、命名约定
2. **项目结构**：目录组织、模块划分、依赖管理
3. **开发流程**：Git工作流、代码审查、持续集成
4. **文档规范**：代码注释、API文档、用户文档
5. **错误处理**：错误类型设计、错误传播、日志记录
6. **性能优化**：测试、分析、优化策略
7. **安全开发**：审计、修复、测试
8. **工具链**：开发工具、调试工具、部署工具
9. **最佳实践**：设计模式、代码复用、可维护性

**关键要点**：

- 遵循编码规范确保代码质量
- 建立标准化的开发流程
- 编写清晰完整的文档
- 重视错误处理和日志
- 使用合适的工具提升效率
- 遵循最佳实践和设计原则

---

**文档版本**: v1.0.0  
**最后更新**: 2025年10月17日  
**作者**: Rust区块链技术团队  
**相关文档**:

- [21_TESTING_STRATEGIES.md](./21_TESTING_STRATEGIES.md) - 测试策略
- [19_SECURITY_BEST_PRACTICES.md](./19_SECURITY_BEST_PRACTICES.md) - 安全最佳实践
- [11_PERFORMANCE_OPTIMIZATION.md](./11_PERFORMANCE_OPTIMIZATION.md) - 性能优化
