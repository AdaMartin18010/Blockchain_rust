//! 文档生成工具

use blockchain::docs::generate_markdown_docs;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📚 生成API文档...");
    
    // 生成Markdown文档
    let markdown_docs = generate_markdown_docs();
    
    // 确保docs目录存在
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)?;
    }
    
    // 写入API文档
    let api_docs_path = docs_dir.join("API.md");
    fs::write(&api_docs_path, markdown_docs)?;
    println!("✅ API文档已生成: {}", api_docs_path.display());
    
    // 生成README文档
    let readme_content = generate_readme();
    let readme_path = Path::new("README.md");
    fs::write(readme_path, readme_content)?;
    println!("✅ README文档已生成: {}", readme_path.display());
    
    // 生成使用指南
    let usage_guide = generate_usage_guide();
    let usage_path = docs_dir.join("USAGE.md");
    fs::write(&usage_path, usage_guide)?;
    println!("✅ 使用指南已生成: {}", usage_path.display());
    
    println!("🎉 所有文档生成完成！");
    
    Ok(())
}

fn generate_readme() -> String {
    format!(r#"# Blockchain Rust Library

[![Rust](https://img.shields.io/badge/rust-1.90+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

一个用Rust语言实现的完整区块链系统，展示了现代区块链技术的核心组件和功能。

## 🚀 特性

- **现代化Rust技术栈**: 使用Rust 1.90+最新特性
- **高性能设计**: 并发安全的组件，高效的密码学操作
- **模块化架构**: 清晰的分层设计，可插拔的组件系统
- **全面测试**: 99个测试用例，覆盖所有核心功能
- **安全性**: 多种密码学算法，安全漏洞检测

## 📦 核心组件

### 🔐 密码学组件
- **哈希算法**: SHA256, SHA512, Blake2b, Blake2s
- **签名算法**: ECDSA (secp256k1), Ed25519
- **加密算法**: AES-GCM, ChaCha20-Poly1305

### 🌐 网络组件
- **P2P网络**: 点对点连接和通信
- **消息路由**: 高效的消息广播和路由
- **节点管理**: 节点状态跟踪和连接管理

### 💾 存储组件
- **区块存储**: 持久化区块数据
- **交易存储**: 交易池和确认状态管理
- **状态存储**: 状态历史和快照功能
- **Merkle存储**: 树版本管理和证明缓存

### ⚡ 共识机制
- **PoW**: 工作量证明算法
- **PoS**: 权益证明算法
- **DPoS**: 委托权益证明算法
- **PBFT**: 实用拜占庭容错算法

### 🤖 智能合约
- **虚拟机**: 字节码执行环境
- **编译器**: 源代码编译支持
- **运行时**: 合约执行环境

## 🛠️ 安装和使用

### 添加依赖

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
blockchain = {{ path = "path/to/blockchain" }}
tokio = {{ version = "1.0", features = ["full"] }}
```

### 基本使用

```rust
use blockchain::core::{{Block, Transaction}};
use blockchain::components::cryptography::HashEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    // 创建哈希引擎
    let hash_engine = HashEngine::new();
    
    // 计算哈希
    let data = b"Hello, Blockchain!";
    let hash = hash_engine.sha256(data);
    println!("Hash: {{}}", hex::encode(hash));
    
    // 创建交易
    let transaction = Transaction::new(vec![], vec![], 0);
    
    // 创建区块
    let block = Block::new(0, [0u8; 32], vec![transaction], 1234567890);
    
    println!("Block created: {{}}", hex::encode(block.hash()));
    
    Ok(())
}}
```

## 📚 文档

- [API文档](docs/API.md) - 完整的API参考
- [使用指南](docs/USAGE.md) - 详细的使用说明
- [示例代码](examples/) - 各种使用示例

## 🧪 测试

运行测试：

```bash
cargo test
```

运行基准测试：

```bash
cargo bench
```

## 📊 性能

项目包含全面的性能基准测试，涵盖：

- 哈希操作性能
- 签名验证性能
- 存储操作性能
- 网络通信性能
- 共识算法性能

## 🏗️ 项目结构

```
blockchain/
├── src/
│   ├── core/                 # 核心类型
│   ├── components/           # 组件实现
│   │   ├── cryptography/    # 密码学组件
│   │   ├── network/         # 网络组件
│   │   ├── storage/         # 存储组件
│   │   └── consensus/       # 共识组件
│   ├── smart_contracts/     # 智能合约
│   ├── layers/              # 分层架构
│   ├── algorithms/          # 算法模块
│   └── utils/               # 工具函数
├── examples/                # 示例代码
├── benches/                 # 基准测试
├── tests/                   # 集成测试
└── docs/                    # 文档
```

## 🤝 贡献

欢迎贡献代码！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详细信息。

## 📄 许可证

本项目采用 MIT 许可证。详情请见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

感谢所有为这个项目做出贡献的开发者和开源社区。

---

**版本**: 0.1.0  
**最后更新**: 2025年1月21日
"#)
}

fn generate_usage_guide() -> String {
    format!(r#"# 使用指南

本指南将帮助您快速上手使用区块链库。

## 🚀 快速开始

### 1. 基本设置

```rust
use blockchain::core::{{Block, Transaction, State}};
use blockchain::components::cryptography::{{HashEngine, SignatureEngine}};
use blockchain::components::storage::{{BlockStorage, TransactionStorage}};
use blockchain::components::consensus::ProofOfWork;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    // 您的代码
    Ok(())
}}
```

### 2. 创建创世区块

```rust
// 创建创世交易
let genesis_transaction = Transaction::new(vec![], vec![], 0);

// 创建创世区块
let genesis_block = Block::new(0, [0u8; 32], vec![genesis_transaction], 1234567890);

// 存储区块
let mut block_storage = BlockStorage::new();
block_storage.initialize().await?;
block_storage.store_block(0, genesis_block).await?;
```

## 🔐 密码学操作

### 哈希计算

```rust
let hash_engine = HashEngine::new();

// SHA256哈希
let data = b"Hello, World!";
let sha256_hash = hash_engine.sha256(data);
println!("SHA256: {{}}", hex::encode(sha256_hash));

// Blake2b哈希
let blake2b_hash = hash_engine.blake2b(data);
println!("Blake2b: {{}}", hex::encode(&blake2b_hash[..32]));

// 双重SHA256
let double_hash = hash_engine.double_sha256(data);
println!("Double SHA256: {{}}", hex::encode(double_hash));
```

### 数字签名

```rust
let signature_engine = SignatureEngine::new();

// 生成密钥对
let (private_key, public_key) = signature_engine.generate_keypair("ed25519")?;

// 签名数据
let data = b"Important message";
let signature = signature_engine.sign(data, &private_key, "ed25519")?;

// 验证签名
let is_valid = signature_engine.verify(data, &signature, &public_key, "ed25519")?;
println!("签名验证: {{}}", if is_valid {{ "成功" }} else {{ "失败" }});
```

## 💾 存储操作

### 区块存储

```rust
let mut block_storage = BlockStorage::new();
block_storage.initialize().await?;

// 存储区块
let block = Block::new(1, [0u8; 32], vec![], 1234567890);
block_storage.store_block(1, block.clone()).await?;

// 获取区块
let retrieved_block = block_storage.get_block(1).await?;
assert!(retrieved_block.is_some());

// 获取统计信息
let stats = block_storage.get_stats().await?;
println!("存储的区块数量: {{}}", stats.total_blocks);
```

## ⚡ 共识算法

### 工作量证明 (PoW)

```rust
let pow = ProofOfWork::new(4); // 难度为4

// 创建区块
let mut block = Block::new(1, [0u8; 32], vec![], 1234567890);

// 挖矿
pow.mine_block(&mut block).await?;

// 验证区块
let is_valid = pow.validate_block(&block).await?;
println!("区块验证: {{}}", if is_valid {{ "成功" }} else {{ "失败" }});
```

## 🌐 网络操作

### P2P网络

```rust
let mut p2p_network = P2PNetwork::new();

// 启动网络
p2p_network.start(8080).await?;

// 连接到其他节点
p2p_network.connect_to_peer("127.0.0.1:8081").await?;

// 广播消息
let message = b"Hello, P2P Network!";
p2p_network.broadcast_message(message).await?;
```

## 🤖 智能合约

### 虚拟机操作

```rust
let mut vm = VirtualMachine::new();

// 部署合约
let bytecode = b"contract bytecode...";
let contract_address = vm.deploy(bytecode).await?;
println!("合约地址: {{}}", contract_address);

// 执行合约
let input = b"function call data...";
let result = vm.execute(bytecode, input).await?;
println!("执行结果: {{}}", hex::encode(result));
```

## 📊 监控和统计

### 系统监控

```rust
let mut monitor = Monitor::new();

// 记录指标
monitor.record_metric("blocks_mined", 1.0);
monitor.record_metric("transactions_processed", 5.0);
monitor.record_metric("network_peers", 2.0);

// 获取统计信息
let stats = monitor.get_statistics();
for (key, value) in stats {{
    println!("{{}}: {{}}", key, value);
}}
```

## 🧪 测试

### 单元测试

```rust
#[cfg(test)]
mod tests {{
    use super::*;
    
    #[tokio::test]
    async fn test_block_creation() {{
        let block = Block::new(0, [0u8; 32], vec![], 1234567890);
        assert_eq!(block.header.height, 0);
    }}
    
    #[test]
    fn test_hash_calculation() {{
        let hash_engine = HashEngine::new();
        let data = b"test data";
        let hash = hash_engine.sha256(data);
        assert_eq!(hash.len(), 32);
    }}
}}
```

## 📚 更多示例

查看 `examples/` 目录获取更多完整的使用示例：

- `complete_blockchain_demo.rs` - 完整的区块链演示
- `security_demo.rs` - 安全功能演示
- `performance_demo.rs` - 性能测试演示

## 🆘 常见问题

### Q: 如何处理大文件？
A: 使用流式处理或分块处理，避免一次性加载大量数据到内存。

### Q: 如何优化网络性能？
A: 使用连接池、消息批处理和异步I/O来提高网络性能。

### Q: 如何确保数据一致性？
A: 使用事务和锁机制来确保并发访问时的数据一致性。

## 📞 支持

如果您遇到问题或有任何疑问，请：

1. 查看 [API文档](API.md)
2. 查看示例代码
3. 提交 Issue
4. 参与讨论

---

**祝您使用愉快！** 🚀
"#)
}
