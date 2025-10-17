# 测试策略

## 📋 目录

- [测试策略](#测试策略)
  - [📋 目录](#-目录)
  - [1. 测试金字塔](#1-测试金字塔)
    - [1.1 单元测试](#11-单元测试)
    - [1.2 集成测试](#12-集成测试)
    - [1.3 端到端测试](#13-端到端测试)
  - [2. 单元测试](#2-单元测试)
    - [2.1 测试结构](#21-测试结构)
    - [2.2 测试覆盖率](#22-测试覆盖率)
    - [2.3 测试模式](#23-测试模式)
  - [3. 集成测试](#3-集成测试)
    - [3.1 模块集成](#31-模块集成)
    - [3.2 外部依赖](#32-外部依赖)
    - [3.3 异步测试](#33-异步测试)
  - [4. 属性测试](#4-属性测试)
    - [4.1 QuickCheck](#41-quickcheck)
    - [4.2 proptest](#42-proptest)
    - [4.3 属性定义](#43-属性定义)
  - [5. 模糊测试](#5-模糊测试)
    - [5.1 cargo-fuzz](#51-cargo-fuzz)
    - [5.2 目标选择](#52-目标选择)
    - [5.3 语料库管理](#53-语料库管理)
  - [6. 性能测试](#6-性能测试)
    - [6.1 Criterion基准测试](#61-criterion基准测试)
    - [6.2 性能回归](#62-性能回归)
    - [6.3 压力测试](#63-压力测试)
  - [7. 智能合约测试](#7-智能合约测试)
    - [7.1 Hardhat测试](#71-hardhat测试)
    - [7.2 Foundry测试](#72-foundry测试)
    - [7.3 测试网部署](#73-测试网部署)
  - [8. 测试工具](#8-测试工具)
    - [8.1 Mock框架](#81-mock框架)
    - [8.2 测试工具库](#82-测试工具库)
    - [8.3 CI/CD集成](#83-cicd集成)
  - [9. 测试最佳实践](#9-测试最佳实践)
    - [9.1 测试原则](#91-测试原则)
    - [9.2 测试组织](#92-测试组织)
    - [9.3 测试维护](#93-测试维护)
  - [10. 总结](#10-总结)

## 1. 测试金字塔

### 1.1 单元测试

```rust
/// 单元测试：测试最小的代码单元
pub mod unit_tests {
    /// 区块结构
    #[derive(Debug, PartialEq)]
    pub struct Block {
        pub index: u64,
        pub timestamp: u64,
        pub data: Vec<u8>,
        pub prev_hash: Hash,
        pub hash: Hash,
    }
    
    pub type Hash = [u8; 32];
    
    impl Block {
        /// 创建新区块
        pub fn new(index: u64, data: Vec<u8>, prev_hash: Hash) -> Self {
            let timestamp = current_timestamp();
            let hash = calculate_hash(index, timestamp, &data, &prev_hash);
            
            Self {
                index,
                timestamp,
                data,
                prev_hash,
                hash,
            }
        }
        
        /// 验证区块哈希
        pub fn is_valid(&self) -> bool {
            let calculated_hash = calculate_hash(
                self.index,
                self.timestamp,
                &self.data,
                &self.prev_hash,
            );
            
            self.hash == calculated_hash
        }
    }
    
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    fn calculate_hash(index: u64, timestamp: u64, data: &[u8], prev_hash: &Hash) -> Hash {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(index.to_le_bytes());
        hasher.update(timestamp.to_le_bytes());
        hasher.update(data);
        hasher.update(prev_hash);
        
        hasher.finalize().into()
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        /// 测试区块创建
        #[test]
        fn test_block_creation() {
            let prev_hash = [0u8; 32];
            let data = b"Genesis Block".to_vec();
            let block = Block::new(0, data.clone(), prev_hash);
            
            assert_eq!(block.index, 0);
            assert_eq!(block.data, data);
            assert_eq!(block.prev_hash, prev_hash);
            assert!(block.timestamp > 0);
        }
        
        /// 测试区块验证
        #[test]
        fn test_block_validation() {
            let prev_hash = [0u8; 32];
            let data = b"Test Block".to_vec();
            let block = Block::new(1, data, prev_hash);
            
            assert!(block.is_valid());
        }
        
        /// 测试无效区块
        #[test]
        fn test_invalid_block() {
            let prev_hash = [0u8; 32];
            let data = b"Test Block".to_vec();
            let mut block = Block::new(1, data, prev_hash);
            
            // 篡改数据
            block.data = b"Modified".to_vec();
            
            assert!(!block.is_valid());
        }
        
        /// 测试边界情况
        #[test]
        fn test_edge_cases() {
            // 空数据
            let block = Block::new(0, vec![], [0u8; 32]);
            assert!(block.is_valid());
            
            // 大数据
            let large_data = vec![0u8; 1024 * 1024]; // 1MB
            let block = Block::new(0, large_data, [0u8; 32]);
            assert!(block.is_valid());
        }
        
        /// 测试哈希唯一性
        #[test]
        fn test_hash_uniqueness() {
            let block1 = Block::new(0, b"data1".to_vec(), [0u8; 32]);
            let block2 = Block::new(0, b"data2".to_vec(), [0u8; 32]);
            
            assert_ne!(block1.hash, block2.hash);
        }
    }
}
```

### 1.2 集成测试

```rust
/// 集成测试：测试多个模块的协作
/// 位于 tests/ 目录下
// tests/blockchain_integration_test.rs

use blockchain::{Blockchain, Block, Transaction};

#[test]
fn test_blockchain_creation() {
    let chain = Blockchain::new();
    assert_eq!(chain.height(), 0);
    assert!(chain.is_valid());
}

#[test]
fn test_add_blocks() {
    let mut chain = Blockchain::new();
    
    let tx1 = Transaction::new("Alice", "Bob", 50);
    let block1 = Block::new(vec![tx1], chain.latest_hash());
    chain.add_block(block1).unwrap();
    
    let tx2 = Transaction::new("Bob", "Charlie", 30);
    let block2 = Block::new(vec![tx2], chain.latest_hash());
    chain.add_block(block2).unwrap();
    
    assert_eq!(chain.height(), 2);
    assert!(chain.is_valid());
}

#[test]
fn test_blockchain_validation() {
    let mut chain = Blockchain::new();
    
    // 添加有效区块
    let tx = Transaction::new("Alice", "Bob", 50);
    let block = Block::new(vec![tx], chain.latest_hash());
    chain.add_block(block).unwrap();
    
    assert!(chain.is_valid());
    
    // 尝试添加无效区块（错误的前置哈希）
    let invalid_block = Block::new(vec![], [0u8; 32]);
    assert!(chain.add_block(invalid_block).is_err());
}

#[tokio::test]
async fn test_async_operations() {
    let mut chain = Blockchain::new();
    
    // 异步创建交易
    let tx = create_transaction_async("Alice", "Bob", 50).await;
    let block = Block::new(vec![tx], chain.latest_hash());
    
    chain.add_block(block).unwrap();
    assert_eq!(chain.height(), 1);
}

async fn create_transaction_async(from: &str, to: &str, amount: u64) -> Transaction {
    // 模拟异步操作
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    Transaction::new(from, to, amount)
}

// Mock实现供测试使用
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }
    
    pub fn height(&self) -> u64 {
        self.blocks.len() as u64
    }
    
    pub fn is_valid(&self) -> bool {
        true
    }
    
    pub fn latest_hash(&self) -> [u8; 32] {
        [0u8; 32]
    }
    
    pub fn add_block(&mut self, block: Block) -> Result<(), String> {
        self.blocks.push(block);
        Ok(())
    }
}

pub struct Block {
    transactions: Vec<Transaction>,
    prev_hash: [u8; 32],
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, prev_hash: [u8; 32]) -> Self {
        Self {
            transactions,
            prev_hash,
        }
    }
}

pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

impl Transaction {
    pub fn new(from: &str, to: &str, amount: u64) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            amount,
        }
    }
}
```

### 1.3 端到端测试

```rust
/// 端到端测试：测试完整的用户场景
/// tests/e2e_test.rs

use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
#[ignore] // 标记为慢速测试
async fn test_full_node_lifecycle() {
    // 1. 启动节点
    let mut node = start_node().await;
    
    // 2. 等待节点启动
    sleep(Duration::from_secs(2)).await;
    
    // 3. 创建账户
    let account = create_account(&node).await.unwrap();
    
    // 4. 发送交易
    let tx_hash = send_transaction(&node, &account, "Bob", 100)
        .await
        .unwrap();
    
    // 5. 等待交易确认
    wait_for_confirmation(&node, &tx_hash).await.unwrap();
    
    // 6. 验证余额
    let balance = get_balance(&node, "Bob").await.unwrap();
    assert_eq!(balance, 100);
    
    // 7. 停止节点
    stop_node(node).await;
}

#[tokio::test]
#[ignore]
async fn test_multi_node_consensus() {
    // 启动多个节点
    let mut nodes = vec![];
    for i in 0..4 {
        let node = start_node_with_port(8000 + i).await;
        nodes.push(node);
    }
    
    // 等待节点互联
    sleep(Duration::from_secs(5)).await;
    
    // 在一个节点上创建交易
    let tx_hash = send_transaction(&nodes[0], "Alice", "Bob", 100)
        .await
        .unwrap();
    
    // 等待共识
    sleep(Duration::from_secs(10)).await;
    
    // 验证所有节点状态一致
    for node in &nodes {
        let balance = get_balance(node, "Bob").await.unwrap();
        assert_eq!(balance, 100);
    }
    
    // 停止所有节点
    for node in nodes {
        stop_node(node).await;
    }
}

// 辅助函数
async fn start_node() -> Node {
    Node {}
}

async fn start_node_with_port(port: u16) -> Node {
    Node {}
}

async fn create_account(node: &Node) -> Result<String, String> {
    Ok("account1".to_string())
}

async fn send_transaction(
    node: &Node,
    from: &str,
    to: &str,
    amount: u64,
) -> Result<String, String> {
    Ok("tx_hash".to_string())
}

async fn wait_for_confirmation(node: &Node, tx_hash: &str) -> Result<(), String> {
    Ok(())
}

async fn get_balance(node: &Node, account: &str) -> Result<u64, String> {
    Ok(100)
}

async fn stop_node(node: Node) {}

struct Node {}
```

## 2. 单元测试

### 2.1 测试结构

```rust
/// 测试组织和结构
pub mod test_structure {
    /// 使用测试模块组织测试
    pub struct Calculator;
    
    impl Calculator {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        pub fn subtract(a: i32, b: i32) -> i32 {
            a - b
        }
        
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
        
        pub fn divide(a: i32, b: i32) -> Option<i32> {
            if b == 0 {
                None
            } else {
                Some(a / b)
            }
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        /// 基本功能测试
        mod basic_operations {
            use super::*;
            
            #[test]
            fn test_add() {
                assert_eq!(Calculator::add(2, 3), 5);
                assert_eq!(Calculator::add(-1, 1), 0);
                assert_eq!(Calculator::add(0, 0), 0);
            }
            
            #[test]
            fn test_subtract() {
                assert_eq!(Calculator::subtract(5, 3), 2);
                assert_eq!(Calculator::subtract(0, 5), -5);
            }
            
            #[test]
            fn test_multiply() {
                assert_eq!(Calculator::multiply(3, 4), 12);
                assert_eq!(Calculator::multiply(-2, 3), -6);
                assert_eq!(Calculator::multiply(0, 100), 0);
            }
        }
        
        /// 边界情况测试
        mod edge_cases {
            use super::*;
            
            #[test]
            fn test_divide_normal() {
                assert_eq!(Calculator::divide(10, 2), Some(5));
                assert_eq!(Calculator::divide(7, 3), Some(2));
            }
            
            #[test]
            fn test_divide_by_zero() {
                assert_eq!(Calculator::divide(10, 0), None);
            }
            
            #[test]
            fn test_overflow() {
                // 使用 wrapping 方法测试溢出
                assert_eq!(
                    i32::MAX.wrapping_add(1),
                    i32::MIN
                );
            }
        }
        
        /// 使用测试夹具
        mod with_fixtures {
            use super::*;
            
            fn setup() -> TestContext {
                TestContext {
                    calc: Calculator,
                }
            }
            
            struct TestContext {
                calc: Calculator,
            }
            
            #[test]
            fn test_with_context() {
                let ctx = setup();
                assert_eq!(Calculator::add(1, 1), 2);
            }
        }
        
        /// 参数化测试（使用rstest）
        #[cfg(feature = "rstest")]
        mod parametrized {
            use super::*;
            use rstest::rstest;
            
            #[rstest]
            #[case(2, 3, 5)]
            #[case(0, 0, 0)]
            #[case(-1, 1, 0)]
            #[case(100, 200, 300)]
            fn test_add_cases(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
                assert_eq!(Calculator::add(a, b), expected);
            }
        }
    }
}
```

### 2.2 测试覆盖率

```bash
# 使用tarpaulin生成覆盖率报告
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html

# 生成XML格式（用于CI）
cargo tarpaulin --out Xml

# 排除某些文件
cargo tarpaulin --exclude-files 'tests/*'

# 设置最小覆盖率阈值
cargo tarpaulin --fail-under 80

# 覆盖率目标：
# - 核心模块：> 90%
# - 工具模块：> 80%
# - 总体：> 75%
```

### 2.3 测试模式

```rust
/// 常见测试模式
pub mod test_patterns {
    /// 1. Arrange-Act-Assert (AAA) 模式
    #[test]
    fn test_aaa_pattern() {
        // Arrange: 设置测试数据
        let input = vec![1, 2, 3, 4, 5];
        let expected = 15;
        
        // Act: 执行被测试的操作
        let result: i32 = input.iter().sum();
        
        // Assert: 验证结果
        assert_eq!(result, expected);
    }
    
    /// 2. Given-When-Then (GWT) 模式
    #[test]
    fn test_gwt_pattern() {
        // Given: 初始状态
        let mut account = Account::new(100);
        
        // When: 执行操作
        let result = account.withdraw(50);
        
        // Then: 验证结果
        assert!(result.is_ok());
        assert_eq!(account.balance(), 50);
    }
    
    /// 3. 表驱动测试
    #[test]
    fn test_table_driven() {
        let test_cases = vec![
            (2, 3, 5),
            (0, 0, 0),
            (-1, 1, 0),
            (100, -50, 50),
        ];
        
        for (a, b, expected) in test_cases {
            assert_eq!(a + b, expected, "Failed for {} + {}", a, b);
        }
    }
    
    /// 4. Builder模式测试
    #[test]
    fn test_builder_pattern() {
        let block = BlockBuilder::new()
            .with_index(1)
            .with_data(b"test".to_vec())
            .with_timestamp(12345)
            .build();
        
        assert_eq!(block.index, 1);
        assert_eq!(block.data, b"test");
    }
    
    struct Account {
        balance: u64,
    }
    
    impl Account {
        fn new(balance: u64) -> Self {
            Self { balance }
        }
        
        fn withdraw(&mut self, amount: u64) -> Result<(), String> {
            if amount > self.balance {
                return Err("Insufficient balance".to_string());
            }
            self.balance -= amount;
            Ok(())
        }
        
        fn balance(&self) -> u64 {
            self.balance
        }
    }
    
    struct Block {
        index: u64,
        data: Vec<u8>,
        timestamp: u64,
    }
    
    struct BlockBuilder {
        index: u64,
        data: Vec<u8>,
        timestamp: u64,
    }
    
    impl BlockBuilder {
        fn new() -> Self {
            Self {
                index: 0,
                data: vec![],
                timestamp: 0,
            }
        }
        
        fn with_index(mut self, index: u64) -> Self {
            self.index = index;
            self
        }
        
        fn with_data(mut self, data: Vec<u8>) -> Self {
            self.data = data;
            self
        }
        
        fn with_timestamp(mut self, timestamp: u64) -> Self {
            self.timestamp = timestamp;
            self
        }
        
        fn build(self) -> Block {
            Block {
                index: self.index,
                data: self.data,
                timestamp: self.timestamp,
            }
        }
    }
}
```

## 3. 集成测试

### 3.1 模块集成

已在前面的集成测试章节展示。

### 3.2 外部依赖

```rust
/// Mock外部依赖
#[cfg(test)]
mod mocking {
    use mockall::*;
    use async_trait::async_trait;
    
    /// 定义trait
    #[async_trait]
    pub trait Database: Send + Sync {
        async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, String>;
        async fn put(&self, key: &str, value: Vec<u8>) -> Result<(), String>;
    }
    
    /// 自动生成Mock
    #[automock]
    #[async_trait]
    impl Database for RealDatabase {
        async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, String> {
            Ok(None)
        }
        
        async fn put(&self, key: &str, value: Vec<u8>) -> Result<(), String> {
            Ok(())
        }
    }
    
    pub struct RealDatabase;
    
    /// 使用Mock进行测试
    #[tokio::test]
    async fn test_with_mock() {
        let mut mock_db = MockRealDatabase::new();
        
        // 设置期望
        mock_db
            .expect_get()
            .with(mockall::predicate::eq("key1"))
            .times(1)
            .returning(|_| Ok(Some(vec![1, 2, 3])));
        
        // 执行测试
        let result = mock_db.get("key1").await.unwrap();
        assert_eq!(result, Some(vec![1, 2, 3]));
    }
}
```

### 3.3 异步测试

```rust
/// 异步测试示例
#[cfg(test)]
mod async_tests {
    use tokio::time::{sleep, Duration};
    
    /// 基本异步测试
    #[tokio::test]
    async fn test_async_operation() {
        let result = async_function().await;
        assert_eq!(result, 42);
    }
    
    async fn async_function() -> i32 {
        sleep(Duration::from_millis(100)).await;
        42
    }
    
    /// 测试超时
    #[tokio::test]
    async fn test_timeout() {
        let result = tokio::time::timeout(
            Duration::from_secs(1),
            slow_operation()
        ).await;
        
        assert!(result.is_ok());
    }
    
    async fn slow_operation() -> i32 {
        sleep(Duration::from_millis(500)).await;
        42
    }
    
    /// 测试并发
    #[tokio::test]
    async fn test_concurrent_operations() {
        let handles: Vec<_> = (0..10)
            .map(|i| tokio::spawn(async move {
                async_function().await + i
            }))
            .collect();
        
        let results: Vec<_> = futures::future::join_all(handles)
            .await
            .into_iter()
            .map(|r| r.unwrap())
            .collect();
        
        assert_eq!(results.len(), 10);
    }
}
```

## 4. 属性测试

### 4.1 QuickCheck

```rust
/// QuickCheck属性测试
#[cfg(test)]
mod quickcheck_tests {
    use quickcheck::{quickcheck, TestResult};
    
    /// 测试加法交换律
    #[quickcheck]
    fn prop_addition_commutative(a: i32, b: i32) -> bool {
        a.wrapping_add(b) == b.wrapping_add(a)
    }
    
    /// 测试列表反转
    #[quickcheck]
    fn prop_reverse_involution(xs: Vec<i32>) -> bool {
        let reversed: Vec<_> = xs.iter().copied().rev().collect();
        let double_reversed: Vec<_> = reversed.iter().copied().rev().collect();
        xs == double_reversed
    }
    
    /// 条件属性测试
    #[quickcheck]
    fn prop_division(a: i32, b: i32) -> TestResult {
        if b == 0 {
            return TestResult::discard();
        }
        
        let result = a / b;
        TestResult::from_bool(result * b + (a % b) == a)
    }
}
```

### 4.2 proptest

```rust
/// Proptest属性测试
#[cfg(test)]
mod proptest_tests {
    use proptest::prelude::*;
    
    proptest! {
        /// 测试字符串编码
        #[test]
        fn test_string_encoding(s in "\\PC*") {
            let encoded = encode(&s);
            let decoded = decode(&encoded);
            prop_assert_eq!(s, decoded);
        }
        
        /// 测试数值范围
        #[test]
        fn test_bounded_number(n in 1..=100i32) {
            prop_assert!(n >= 1 && n <= 100);
        }
        
        /// 测试集合属性
        #[test]
        fn test_vec_properties(v in prop::collection::vec(any::<u32>(), 0..100)) {
            let sorted = {
                let mut v = v.clone();
                v.sort();
                v
            };
            
            // 排序后长度不变
            prop_assert_eq!(v.len(), sorted.len());
            
            // 排序后是有序的
            for i in 1..sorted.len() {
                prop_assert!(sorted[i-1] <= sorted[i]);
            }
        }
    }
    
    fn encode(s: &str) -> String {
        s.to_string()
    }
    
    fn decode(s: &str) -> String {
        s.to_string()
    }
}
```

### 4.3 属性定义

```rust
/// 属性测试指南
pub mod property_testing_guide {
    /// 常见属性类型：
    
    /// 1. 往返属性（Round-trip）
    /// 序列化后反序列化应该得到原值
    #[test]
    fn test_roundtrip_property() {
        let original = MyStruct { value: 42 };
        let serialized = serialize(&original);
        let deserialized = deserialize(&serialized);
        assert_eq!(original, deserialized);
    }
    
    /// 2. 不变量（Invariant）
    /// 某些属性在操作后保持不变
    #[test]
    fn test_invariant_property() {
        let mut list = vec![1, 2, 3, 4, 5];
        let original_len = list.len();
        
        list.sort();
        
        assert_eq!(list.len(), original_len);
    }
    
    /// 3. 幂等性（Idempotence）
    /// 多次应用操作结果相同
    #[test]
    fn test_idempotent_property() {
        let mut data = vec![3, 1, 4, 1, 5];
        
        data.sort();
        let first_sort = data.clone();
        
        data.sort();
        let second_sort = data.clone();
        
        assert_eq!(first_sort, second_sort);
    }
    
    /// 4. 交换律（Commutativity）
    #[test]
    fn test_commutative_property() {
        let a = 5;
        let b = 3;
        
        assert_eq!(a + b, b + a);
        assert_eq!(a * b, b * a);
    }
    
    /// 5. 结合律（Associativity）
    #[test]
    fn test_associative_property() {
        let a = 2;
        let b = 3;
        let c = 4;
        
        assert_eq!((a + b) + c, a + (b + c));
        assert_eq!((a * b) * c, a * (b * c));
    }
    
    #[derive(Debug, PartialEq)]
    struct MyStruct {
        value: i32,
    }
    
    fn serialize(s: &MyStruct) -> Vec<u8> {
        s.value.to_le_bytes().to_vec()
    }
    
    fn deserialize(bytes: &[u8]) -> MyStruct {
        let value = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        MyStruct { value }
    }
}
```

## 5. 模糊测试

### 5.1 cargo-fuzz

```bash
# 安装cargo-fuzz
cargo install cargo-fuzz

# 初始化fuzzing
cargo fuzz init

# 创建fuzz target
cargo fuzz add fuzz_target_name

# 运行fuzzing
cargo fuzz run fuzz_target_name

# 使用特定语料库
cargo fuzz run fuzz_target_name corpus/

# 限制运行时间
cargo fuzz run fuzz_target_name -- -max_total_time=60

# 最小化测试用例
cargo fuzz cmin fuzz_target_name
```

```rust
// fuzz/fuzz_targets/fuzz_parse_transaction.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use blockchain::Transaction;

fuzz_target!(|data: &[u8]| {
    // 尝试解析任意字节序列
    let _ = Transaction::from_bytes(data);
});

// fuzz/fuzz_targets/fuzz_consensus.rs
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 32 {
        return;
    }
    
    // 测试共识算法的鲁棒性
    let _ = verify_block_hash(data);
});

fn verify_block_hash(data: &[u8]) -> bool {
    // 模拟实现
    true
}
```

### 5.2 目标选择

```rust
/// Fuzzing目标选择指南
pub mod fuzzing_targets {
    /// 优先fuzz的目标：
    
    /// 1. 解析器和反序列化器
    pub fn parse_message(data: &[u8]) -> Result<Message, ParseError> {
        // 容易出现panic或溢出
        Ok(Message::default())
    }
    
    /// 2. 加密和签名验证
    pub fn verify_signature(data: &[u8], signature: &[u8]) -> bool {
        // 安全关键代码
        true
    }
    
    /// 3. 网络协议处理
    pub fn handle_p2p_message(data: &[u8]) -> Result<(), ProtocolError> {
        // 接受外部输入
        Ok(())
    }
    
    /// 4. 状态转换逻辑
    pub fn apply_transaction(state: &mut State, tx: &[u8]) -> Result<(), TxError> {
        // 复杂状态机
        Ok(())
    }
    
    /// 5. 数据压缩/解压缩
    pub fn decompress(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        // 可能的内存问题
        Ok(vec![])
    }
    
    #[derive(Debug, Default)]
    pub struct Message;
    
    #[derive(Debug)]
    pub struct ParseError;
    
    #[derive(Debug)]
    pub struct ProtocolError;
    
    pub struct State;
    
    #[derive(Debug)]
    pub struct TxError;
    
    #[derive(Debug)]
    pub struct CompressionError;
}
```

### 5.3 语料库管理

```bash
# 语料库目录结构
fuzz/
├── corpus/
│   ├── fuzz_target1/
│   │   ├── seed1
│   │   ├── seed2
│   │   └── ...
│   └── fuzz_target2/
│       └── ...
└── artifacts/
    ├── fuzz_target1/
    │   └── crash-xxxxx
    └── ...

# 添加种子文件
echo "test input" > fuzz/corpus/fuzz_target1/seed1

# 从现有测试生成语料库
cargo test -- --nocapture | grep "Input:" | cut -d: -f2 > fuzz/corpus/fuzz_target1/seed2

# 最小化语料库
cargo fuzz cmin fuzz_target_name

# 合并语料库
cargo fuzz merge fuzz_target_name corpus1/ corpus2/
```

## 6. 性能测试

### 6.1 Criterion基准测试

```rust
/// Criterion基准测试
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    
    a
}

/// 简单基准测试
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

/// 参数化基准测试
fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    
    for i in [10u64, 15, 20].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(i), i, |b, &i| {
            b.iter(|| fibonacci(black_box(i)));
        });
    }
    
    group.finish();
}

/// 比较实现
fn bench_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci_comparison");
    
    group.bench_function("recursive", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
    
    group.bench_function("iterative", |b| {
        b.iter(|| fibonacci_iterative(black_box(20)))
    });
    
    group.finish();
}

criterion_group!(benches, criterion_benchmark, bench_fibonacci, bench_comparison);
criterion_main!(benches);
```

### 6.2 性能回归

```toml
# Cargo.toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

```bash
# 运行基准测试
cargo bench

# 保存基线
cargo bench -- --save-baseline main

# 与基线比较
cargo bench -- --baseline main

# 生成HTML报告
# 报告位于 target/criterion/报告/index.html
```

### 6.3 压力测试

```rust
/// 压力测试示例
#[cfg(test)]
mod stress_tests {
    use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
    use std::thread;
    
    #[test]
    #[ignore] // 标记为慢速测试
    fn stress_test_concurrent_operations() {
        let counter = Arc::new(AtomicU64::new(0));
        let num_threads = 100;
        let operations_per_thread = 10000;
        
        let handles: Vec<_> = (0..num_threads)
            .map(|_| {
                let counter = Arc::clone(&counter);
                thread::spawn(move || {
                    for _ in 0..operations_per_thread {
                        counter.fetch_add(1, Ordering::SeqCst);
                    }
                })
            })
            .collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(
            counter.load(Ordering::SeqCst),
            num_threads * operations_per_thread
        );
    }
    
    #[tokio::test]
    #[ignore]
    async fn stress_test_async_throughput() {
        let num_requests = 10000;
        
        let start = std::time::Instant::now();
        
        let handles: Vec<_> = (0..num_requests)
            .map(|_| tokio::spawn(async {
                // 模拟异步操作
                tokio::time::sleep(tokio::time::Duration::from_micros(100)).await;
            }))
            .collect();
        
        futures::future::join_all(handles).await;
        
        let duration = start.elapsed();
        let throughput = num_requests as f64 / duration.as_secs_f64();
        
        println!("Throughput: {:.2} requests/sec", throughput);
        assert!(throughput > 1000.0, "Throughput too low");
    }
}
```

## 7. 智能合约测试

### 7.1 Hardhat测试

```javascript
// test/Token.test.js
const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Token", function () {
  let token;
  let owner;
  let addr1;
  let addr2;

  beforeEach(async function () {
    [owner, addr1, addr2] = await ethers.getSigners();
    
    const Token = await ethers.getContractFactory("Token");
    token = await Token.deploy("MyToken", "MTK", 1000000);
    await token.deployed();
  });

  describe("Deployment", function () {
    it("Should set the right owner", async function () {
      expect(await token.owner()).to.equal(owner.address);
    });

    it("Should assign the total supply to the owner", async function () {
      const ownerBalance = await token.balanceOf(owner.address);
      expect(await token.totalSupply()).to.equal(ownerBalance);
    });
  });

  describe("Transactions", function () {
    it("Should transfer tokens between accounts", async function () {
      await token.transfer(addr1.address, 50);
      expect(await token.balanceOf(addr1.address)).to.equal(50);

      await token.connect(addr1).transfer(addr2.address, 50);
      expect(await token.balanceOf(addr2.address)).to.equal(50);
    });

    it("Should fail if sender doesn't have enough tokens", async function () {
      const initialOwnerBalance = await token.balanceOf(owner.address);

      await expect(
        token.connect(addr1).transfer(owner.address, 1)
      ).to.be.revertedWith("Insufficient balance");

      expect(await token.balanceOf(owner.address)).to.equal(
        initialOwnerBalance
      );
    });
  });
});
```

### 7.2 Foundry测试

```solidity
// test/Token.t.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/Token.sol";

contract TokenTest is Test {
    Token public token;
    address public owner = address(1);
    address public user1 = address(2);
    address public user2 = address(3);

    function setUp() public {
        vm.prank(owner);
        token = new Token("MyToken", "MTK", 1000000);
    }

    function testInitialSupply() public {
        assertEq(token.totalSupply(), 1000000);
        assertEq(token.balanceOf(owner), 1000000);
    }

    function testTransfer() public {
        vm.prank(owner);
        token.transfer(user1, 100);
        assertEq(token.balanceOf(user1), 100);
        assertEq(token.balanceOf(owner), 999900);
    }

    function testFailTransferInsufficientBalance() public {
        vm.prank(user1);
        token.transfer(user2, 100);
    }

    function testFuzzTransfer(uint256 amount) public {
        vm.assume(amount <= 1000000);
        
        vm.prank(owner);
        token.transfer(user1, amount);
        assertEq(token.balanceOf(user1), amount);
    }

    function testCannotTransferToZeroAddress() public {
        vm.prank(owner);
        vm.expectRevert("Invalid address");
        token.transfer(address(0), 100);
    }
}
```

### 7.3 测试网部署

```javascript
// scripts/deploy.js
async function main() {
  const [deployer] = await ethers.getSigners();

  console.log("Deploying contracts with account:", deployer.address);
  console.log("Account balance:", (await deployer.getBalance()).toString());

  const Token = await ethers.getContractFactory("Token");
  const token = await Token.deploy("TestToken", "TST", 1000000);

  console.log("Token deployed to:", token.address);
  
  // 等待确认
  await token.deployTransaction.wait(5);
  
  // 验证合约
  await run("verify:verify", {
    address: token.address,
    constructorArguments: ["TestToken", "TST", 1000000],
  });
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
```

## 8. 测试工具

### 8.1 Mock框架

已在集成测试部分展示mockall的使用。

### 8.2 测试工具库

```rust
/// 测试工具函数库
pub mod test_utils {
    use std::path::PathBuf;
    use tempfile::TempDir;
    
    /// 创建临时目录
    pub fn create_temp_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }
    
    /// 创建测试配置
    pub fn create_test_config() -> Config {
        Config {
            data_dir: create_temp_dir().path().to_path_buf(),
            port: 0, // 自动分配端口
            log_level: "debug".to_string(),
        }
    }
    
    /// 等待条件
    pub async fn wait_for_condition<F>(
        mut check: F,
        timeout: std::time::Duration,
    ) -> Result<(), String>
    where
        F: FnMut() -> bool,
    {
        let start = std::time::Instant::now();
        
        while !check() {
            if start.elapsed() > timeout {
                return Err("Timeout waiting for condition".to_string());
            }
            
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        
        Ok(())
    }
    
    /// 生成测试数据
    pub fn generate_test_blocks(count: usize) -> Vec<Block> {
        (0..count)
            .map(|i| Block {
                index: i as u64,
                data: format!("Block {}", i).into_bytes(),
                hash: [i as u8; 32],
            })
            .collect()
    }
    
    #[derive(Debug)]
    pub struct Config {
        pub data_dir: PathBuf,
        pub port: u16,
        pub log_level: String,
    }
    
    #[derive(Debug)]
    pub struct Block {
        pub index: u64,
        pub data: Vec<u8>,
        pub hash: [u8; 32],
    }
}
```

### 8.3 CI/CD集成

已在开发指南的CI/CD章节展示。

## 9. 测试最佳实践

### 9.1 测试原则

```rust
/// 测试最佳实践原则
pub mod testing_principles {
    /// F.I.R.S.T 原则：
    
    /// Fast（快速）
    /// 测试应该快速运行，使开发者愿意频繁运行
    #[test]
    fn test_fast() {
        // 避免：
        // - 网络调用
        // - 文件I/O
        // - 复杂计算
        
        // 使用：
        // - 纯函数
        // - 内存操作
        // - Mock外部依赖
        assert_eq!(2 + 2, 4);
    }
    
    /// Independent（独立）
    /// 测试之间不应该有依赖关系
    #[test]
    fn test_independent_1() {
        let state = create_fresh_state();
        assert!(state.is_empty());
    }
    
    #[test]
    fn test_independent_2() {
        let state = create_fresh_state();
        state.add_item("test");
        assert_eq!(state.len(), 1);
    }
    
    /// Repeatable（可重复）
    /// 测试应该在任何环境下产生相同结果
    #[test]
    fn test_repeatable() {
        // 避免：
        // - 依赖当前时间
        // - 依赖随机数
        // - 依赖外部状态
        
        // 使用：
        // - 固定的输入
        // - 确定性算法
        // - Mock外部状态
        let input = vec![3, 1, 4, 1, 5];
        let mut sorted = input.clone();
        sorted.sort();
        assert_eq!(sorted, vec![1, 1, 3, 4, 5]);
    }
    
    /// Self-Validating（自我验证）
    /// 测试应该自动判断成功或失败
    #[test]
    fn test_self_validating() {
        let result = compute_answer();
        // 好：明确的断言
        assert_eq!(result, 42);
        
        // 坏：需要人工检查输出
        // println!("Result: {}", result);
    }
    
    /// Timely（及时）
    /// 测试应该在编写生产代码的同时或之前编写
    #[test]
    fn test_timely() {
        // TDD: 先写测试，后写实现
        let calculator = Calculator::new();
        assert_eq!(calculator.add(2, 3), 5);
    }
    
    fn create_fresh_state() -> State {
        State::new()
    }
    
    fn compute_answer() -> i32 {
        42
    }
    
    struct State {
        items: Vec<String>,
    }
    
    impl State {
        fn new() -> Self {
            Self { items: vec![] }
        }
        
        fn is_empty(&self) -> bool {
            self.items.is_empty()
        }
        
        fn add_item(&mut self, item: &str) {
            self.items.push(item.to_string());
        }
        
        fn len(&self) -> usize {
            self.items.len()
        }
    }
    
    struct Calculator;
    
    impl Calculator {
        fn new() -> Self {
            Self
        }
        
        fn add(&self, a: i32, b: i32) -> i32 {
            a + b
        }
    }
}
```

### 9.2 测试组织

```text
测试目录结构：

src/
├── lib.rs
├── core/
│   ├── mod.rs
│   ├── block.rs
│   └── block.rs
│       #[cfg(test)]
│       mod tests {
│           // 单元测试
│       }
│
tests/
├── common/
│   ├── mod.rs          # 测试工具
│   └── fixtures.rs     # 测试夹具
├── integration/
│   ├── blockchain_test.rs
│   └── consensus_test.rs
└── e2e/
    └── full_node_test.rs

benches/
├── block_processing.rs
└── consensus.rs
```

### 9.3 测试维护

```rust
/// 测试维护指南
pub mod test_maintenance {
    /// 1. 保持测试简单清晰
    #[test]
    fn test_good() {
        // 好：清晰的测试意图
        let account = Account::new(100);
        account.deposit(50);
        assert_eq!(account.balance(), 150);
    }
    
    #[test]
    fn test_bad() {
        // 坏：复杂的测试逻辑
        let mut account = Account::new(0);
        for i in 0..10 {
            if i % 2 == 0 {
                account.deposit(10);
            } else {
                account.withdraw(5);
            }
        }
        // 难以理解期望结果
    }
    
    /// 2. 使用描述性的测试名称
    #[test]
    fn test_withdraw_with_sufficient_balance_succeeds() {
        // 好：名称说明了测试内容
    }
    
    #[test]
    fn test1() {
        // 坏：无意义的名称
    }
    
    /// 3. 一个测试只测一个概念
    #[test]
    fn test_single_concept() {
        let account = Account::new(100);
        account.withdraw(50);
        assert_eq!(account.balance(), 50);
        // 只测试取款功能
    }
    
    /// 4. 及时更新或删除过时的测试
    #[test]
    #[ignore] // 标记待修复的测试
    fn test_needs_update() {
        // TODO: Update for new API
    }
    
    /// 5. 使用helpers减少重复
    fn create_test_account() -> Account {
        Account::new(100)
    }
    
    #[test]
    fn test_with_helper() {
        let account = create_test_account();
        // 使用helper减少重复代码
    }
    
    struct Account {
        balance: u64,
    }
    
    impl Account {
        fn new(balance: u64) -> Self {
            Self { balance }
        }
        
        fn deposit(&mut self, amount: u64) {
            self.balance += amount;
        }
        
        fn withdraw(&mut self, amount: u64) -> Result<(), String> {
            if amount > self.balance {
                return Err("Insufficient balance".to_string());
            }
            self.balance -= amount;
            Ok(())
        }
        
        fn balance(&self) -> u64 {
            self.balance
        }
    }
}
```

## 10. 总结

本文档提供了全面的测试策略指南，包括：

1. **测试金字塔**：单元测试、集成测试、端到端测试
2. **单元测试**：测试结构、覆盖率、测试模式
3. **集成测试**：模块集成、外部依赖、异步测试
4. **属性测试**：QuickCheck、proptest、属性定义
5. **模糊测试**：cargo-fuzz、目标选择、语料库管理
6. **性能测试**：Criterion基准测试、性能回归、压力测试
7. **智能合约测试**：Hardhat、Foundry、测试网部署
8. **测试工具**：Mock框架、工具库、CI/CD集成
9. **最佳实践**：测试原则、组织、维护

**关键要点**：

- 遵循测试金字塔，底层测试要多
- 单元测试要快速、独立、可重复
- 使用属性测试和模糊测试提高覆盖率
- 重视性能测试和回归检测
- 智能合约测试要全面细致
- 遵循F.I.R.S.T原则
- 保持测试简单清晰，易于维护

**测试覆盖率目标**：

- 核心模块：>90%
- 一般模块：>80%
- 总体：>75%

---

**文档版本**: v1.0.0  
**最后更新**: 2025年10月17日  
**作者**: Rust区块链技术团队  
**相关文档**:

- [20_DEVELOPMENT_GUIDELINES.md](./20_DEVELOPMENT_GUIDELINES.md) - 开发指南
- [19_SECURITY_BEST_PRACTICES.md](./19_SECURITY_BEST_PRACTICES.md) - 安全最佳实践
- [11_PERFORMANCE_OPTIMIZATION.md](./11_PERFORMANCE_OPTIMIZATION.md) - 性能优化
