# 使用指南

本指南将帮助您快速上手使用区块链库。

## 🚀 快速开始

### 1. 基本设置

```rust
use blockchain::core::{Block, Transaction, State};
use blockchain::components::cryptography::{HashEngine, SignatureEngine};
use blockchain::components::storage::{BlockStorage, TransactionStorage};
use blockchain::components::consensus::ProofOfWork;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 您的代码
    Ok(())
}
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
println!("SHA256: {}", hex::encode(sha256_hash));

// Blake2b哈希
let blake2b_hash = hash_engine.blake2b(data);
println!("Blake2b: {}", hex::encode(&blake2b_hash[..32]));

// 双重SHA256
let double_hash = hash_engine.double_sha256(data);
println!("Double SHA256: {}", hex::encode(double_hash));
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
println!("签名验证: {}", if is_valid { "成功" } else { "失败" });
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
println!("存储的区块数量: {}", stats.total_blocks);
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
println!("区块验证: {}", if is_valid { "成功" } else { "失败" });
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
println!("合约地址: {}", contract_address);

// 执行合约
let input = b"function call data...";
let result = vm.execute(bytecode, input).await?;
println!("执行结果: {}", hex::encode(result));
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
for (key, value) in stats {
    println!("{}: {}", key, value);
}
```

## 🧪 测试

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_block_creation() {
        let block = Block::new(0, [0u8; 32], vec![], 1234567890);
        assert_eq!(block.header.height, 0);
    }
    
    #[test]
    fn test_hash_calculation() {
        let hash_engine = HashEngine::new();
        let data = b"test data";
        let hash = hash_engine.sha256(data);
        assert_eq!(hash.len(), 32);
    }
}
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
