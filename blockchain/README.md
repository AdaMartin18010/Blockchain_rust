# Blockchain Rust Library

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
blockchain = { path = "path/to/blockchain" }
tokio = { version = "1.0", features = ["full"] }
```

### 基本使用

```rust
use blockchain::core::{Block, Transaction};
use blockchain::components::cryptography::HashEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建哈希引擎
    let hash_engine = HashEngine::new();
    
    // 计算哈希
    let data = b"Hello, Blockchain!";
    let hash = hash_engine.sha256(data);
    println!("Hash: {}", hex::encode(hash));
    
    // 创建交易
    let transaction = Transaction::new(vec![], vec![], 0);
    
    // 创建区块
    let block = Block::new(0, [0u8; 32], vec![transaction], 1234567890);
    
    println!("Block created: {}", hex::encode(block.hash()));
    
    Ok(())
}
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
