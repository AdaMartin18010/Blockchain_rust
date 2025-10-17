# 协议规范详解

## 📋 目录

- [协议规范详解](#协议规范详解)
  - [📋 目录](#-目录)
  - [1. 区块链核心协议](#1-区块链核心协议)
    - [1.1 比特币协议](#11-比特币协议)
    - [1.2 以太坊协议](#12-以太坊协议)
    - [1.3 EVM规范](#13-evm规范)
  - [2. 共识协议规范](#2-共识协议规范)
    - [2.1 PoW规范](#21-pow规范)
    - [2.2 PoS规范](#22-pos规范)
    - [2.3 BFT规范](#23-bft规范)
  - [3. 网络协议](#3-网络协议)
    - [3.1 P2P协议](#31-p2p协议)
    - [3.2 RLPx协议](#32-rlpx协议)
    - [3.3 DevP2P协议](#33-devp2p协议)
  - [4. 数据结构规范](#4-数据结构规范)
    - [4.1 区块结构](#41-区块结构)
    - [4.2 交易结构](#42-交易结构)
    - [4.3 状态树结构](#43-状态树结构)
  - [5. 加密标准](#5-加密标准)
    - [5.1 哈希算法](#51-哈希算法)
    - [5.2 签名算法](#52-签名算法)
    - [5.3 加密算法](#53-加密算法)
  - [6. RPC协议](#6-rpc协议)
    - [6.1 JSON-RPC规范](#61-json-rpc规范)
    - [6.2 以太坊RPC API](#62-以太坊rpc-api)
    - [6.3 自定义RPC](#63-自定义rpc)
  - [7. EIP/ERC标准](#7-eiperc标准)
    - [7.1 核心EIP](#71-核心eip)
    - [7.2 代币标准](#72-代币标准)
    - [7.3 应用标准](#73-应用标准)
  - [8. 跨链协议](#8-跨链协议)
    - [8.1 IBC协议](#81-ibc协议)
    - [8.2 桥接协议](#82-桥接协议)
    - [8.3 中继协议](#83-中继协议)
  - [9. Layer 2协议](#9-layer-2协议)
    - [9.1 Rollup规范](#91-rollup规范)
    - [9.2 State Channel](#92-state-channel)
    - [9.3 Plasma规范](#93-plasma规范)
  - [10. 总结](#10-总结)

## 1. 区块链核心协议

### 1.1 比特币协议

```rust
/// 比特币协议实现
pub struct BitcoinProtocol {
    /// 网络类型
    network: Network,
    /// 协议版本
    version: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

impl BitcoinProtocol {
    /// 比特币区块结构
    pub fn block_structure() -> BlockSpec {
        BlockSpec {
            version: 4,                    // 区块版本
            prev_block_hash: [0u8; 32],   // 前一区块哈希
            merkle_root: [0u8; 32],       // Merkle根
            timestamp: 0,                   // 时间戳
            bits: 0,                        // 难度目标
            nonce: 0,                       // 随机数
            txn_count: 0,                   // 交易数量
        }
    }
    
    /// 比特币交易结构
    pub fn transaction_structure() -> TxSpec {
        TxSpec {
            version: 2,                     // 交易版本
            lock_time: 0,                   // 锁定时间
            inputs: vec![],                 // 输入
            outputs: vec![],                // 输出
        }
    }
    
    /// PoW验证规则
    pub fn pow_validation_rules() -> PoWRules {
        PoWRules {
            target_adjustment_interval: 2016,  // 难度调整间隔（区块数）
            target_timespan: 14 * 24 * 60 * 60, // 目标时间跨度（2周）
            max_target: u256::from_str_radix(
                "00000000FFFF0000000000000000000000000000000000000000000000000000",
                16
            ).unwrap(),
            min_difficulty: 1,
        }
    }
    
    /// UTXO验证
    pub fn validate_utxo(tx: &Transaction, utxo_set: &UtxoSet) -> Result<(), ProtocolError> {
        let mut total_input = 0u64;
        
        // 验证所有输入
        for input in &tx.inputs {
            // 检查UTXO是否存在
            let utxo = utxo_set.get(&input.prev_output)
                .ok_or(ProtocolError::UtxoNotFound)?;
            
            // 验证脚本
            if !Self::verify_script(&input.script_sig, &utxo.script_pubkey)? {
                return Err(ProtocolError::InvalidScript);
            }
            
            total_input += utxo.value;
        }
        
        // 计算总输出
        let total_output: u64 = tx.outputs.iter().map(|o| o.value).sum();
        
        // 验证输入 >= 输出 + 手续费
        if total_input < total_output {
            return Err(ProtocolError::InsufficientFunds);
        }
        
        Ok(())
    }
    
    fn verify_script(script_sig: &[u8], script_pubkey: &[u8]) -> Result<bool, ProtocolError> {
        // 简化的脚本验证
        Ok(true)
    }
}

#[derive(Debug)]
pub struct BlockSpec {
    version: i32,
    prev_block_hash: [u8; 32],
    merkle_root: [u8; 32],
    timestamp: u32,
    bits: u32,
    nonce: u32,
    txn_count: u64,
}

#[derive(Debug)]
pub struct TxSpec {
    version: i32,
    lock_time: u32,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
}

#[derive(Debug)]
pub struct TxInput {
    prev_output: OutPoint,
    script_sig: Vec<u8>,
    sequence: u32,
}

#[derive(Debug)]
pub struct TxOutput {
    value: u64,
    script_pubkey: Vec<u8>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct OutPoint {
    txid: Hash,
    vout: u32,
}

#[derive(Debug)]
pub struct PoWRules {
    target_adjustment_interval: u32,
    target_timespan: u64,
    max_target: U256,
    min_difficulty: u32,
}

pub struct UtxoSet {
    utxos: HashMap<OutPoint, Utxo>,
}

impl UtxoSet {
    fn get(&self, outpoint: &OutPoint) -> Option<&Utxo> {
        self.utxos.get(outpoint)
    }
}

#[derive(Debug)]
pub struct Utxo {
    value: u64,
    script_pubkey: Vec<u8>,
}
```

### 1.2 以太坊协议

```rust
/// 以太坊协议实现
pub struct EthereumProtocol {
    /// 链ID
    chain_id: u64,
    /// 协议版本
    version: ProtocolVersion,
}

#[derive(Debug, Clone, Copy)]
pub enum ProtocolVersion {
    Frontier,
    Homestead,
    Byzantium,
    Constantinople,
    Istanbul,
    Berlin,
    London,
    Shanghai,
    Cancun,
}

impl EthereumProtocol {
    /// 以太坊区块结构
    pub fn block_structure() -> EthBlockSpec {
        EthBlockSpec {
            // 区块头
            header: BlockHeader {
                parent_hash: Hash::zero(),
                uncle_hash: Hash::zero(),
                coinbase: Address::zero(),
                state_root: Hash::zero(),
                transactions_root: Hash::zero(),
                receipts_root: Hash::zero(),
                logs_bloom: Bloom::default(),
                difficulty: U256::zero(),
                number: 0,
                gas_limit: 15_000_000,
                gas_used: 0,
                timestamp: 0,
                extra_data: vec![],
                mix_hash: Hash::zero(),
                nonce: 0,
                base_fee_per_gas: None, // EIP-1559
            },
            // 区块体
            transactions: vec![],
            uncles: vec![],
        }
    }
    
    /// 交易类型
    pub fn transaction_types() -> Vec<TxType> {
        vec![
            TxType::Legacy,           // 传统交易
            TxType::AccessList,       // EIP-2930
            TxType::DynamicFee,       // EIP-1559
            TxType::Blob,             // EIP-4844
        ]
    }
    
    /// EIP-1559规范
    pub fn eip1559_spec() -> EIP1559Spec {
        EIP1559Spec {
            base_fee_max_change_denominator: 8,
            elasticity_multiplier: 2,
            initial_base_fee: 1_000_000_000, // 1 Gwei
            target_gas_used: 15_000_000,
        }
    }
    
    /// Gas计算规则
    pub fn gas_costs() -> GasCosts {
        GasCosts {
            // 基础成本
            g_zero: 0,              // 零字节
            g_base: 2,              // 基础Gas
            g_very_low: 3,          // 极低成本操作
            g_low: 5,               // 低成本操作
            g_mid: 8,               // 中等成本操作
            g_high: 10,             // 高成本操作
            
            // 存储操作
            g_sset: 20_000,         // SSTORE from zero
            g_sreset: 5_000,        // SSTORE to zero
            r_sclear: 15_000,       // Storage清除退款
            
            // 交易成本
            g_transaction: 21_000,  // 交易基础成本
            g_tx_data_zero: 4,      // 零字节数据
            g_tx_data_nonzero: 16,  // 非零字节数据
            
            // 合约创建
            g_create: 32_000,       // CREATE操作
            g_code_deposit: 200,    // 代码存储（每字节）
            
            // 其他
            g_call: 700,            // CALL操作
            g_call_value: 9_000,    // 带value的CALL
            g_new_account: 25_000,  // 创建新账户
        }
    }
    
    /// 状态转换规则
    pub fn apply_transaction(
        state: &mut WorldState,
        tx: &Transaction,
        block_env: &BlockEnvironment
    ) -> Result<TransactionReceipt, ProtocolError> {
        // 1. 检查nonce
        let sender_account = state.get_account(&tx.sender())?;
        if sender_account.nonce != tx.nonce {
            return Err(ProtocolError::InvalidNonce);
        }
        
        // 2. 检查balance
        let max_cost = tx.gas_limit * tx.gas_price() + tx.value;
        if sender_account.balance < max_cost {
            return Err(ProtocolError::InsufficientBalance);
        }
        
        // 3. 扣除Gas预付款
        state.sub_balance(&tx.sender(), max_cost)?;
        
        // 4. 增加nonce
        state.increment_nonce(&tx.sender())?;
        
        // 5. 执行交易
        let result = if tx.to.is_none() {
            // 合约部署
            Self::create_contract(state, tx, block_env)?
        } else {
            // 合约调用或转账
            Self::call(state, tx, block_env)?
        };
        
        // 6. 退还剩余Gas
        let gas_refund = (tx.gas_limit - result.gas_used) * tx.gas_price();
        state.add_balance(&tx.sender(), gas_refund)?;
        
        // 7. 支付矿工费
        state.add_balance(&block_env.coinbase, result.gas_used * tx.gas_price())?;
        
        // 8. 生成收据
        Ok(TransactionReceipt {
            status: result.status,
            gas_used: result.gas_used,
            logs: result.logs,
            logs_bloom: Bloom::from_logs(&result.logs),
        })
    }
    
    fn create_contract(
        state: &mut WorldState,
        tx: &Transaction,
        block_env: &BlockEnvironment
    ) -> Result<ExecutionResult, ProtocolError> {
        // 简化实现
        Ok(ExecutionResult::default())
    }
    
    fn call(
        state: &mut WorldState,
        tx: &Transaction,
        block_env: &BlockEnvironment
    ) -> Result<ExecutionResult, ProtocolError> {
        // 简化实现
        Ok(ExecutionResult::default())
    }
}

#[derive(Debug)]
pub struct EthBlockSpec {
    header: BlockHeader,
    transactions: Vec<Transaction>,
    uncles: Vec<BlockHeader>,
}

#[derive(Debug, Clone)]
pub struct BlockHeader {
    parent_hash: Hash,
    uncle_hash: Hash,
    coinbase: Address,
    state_root: Hash,
    transactions_root: Hash,
    receipts_root: Hash,
    logs_bloom: Bloom,
    difficulty: U256,
    number: u64,
    gas_limit: u64,
    gas_used: u64,
    timestamp: u64,
    extra_data: Vec<u8>,
    mix_hash: Hash,
    nonce: u64,
    base_fee_per_gas: Option<U256>,
}

#[derive(Debug, Clone, Copy)]
pub enum TxType {
    Legacy,
    AccessList,
    DynamicFee,
    Blob,
}

#[derive(Debug)]
pub struct EIP1559Spec {
    base_fee_max_change_denominator: u64,
    elasticity_multiplier: u64,
    initial_base_fee: u64,
    target_gas_used: u64,
}

#[derive(Debug)]
pub struct GasCosts {
    g_zero: u64,
    g_base: u64,
    g_very_low: u64,
    g_low: u64,
    g_mid: u64,
    g_high: u64,
    g_sset: u64,
    g_sreset: u64,
    r_sclear: u64,
    g_transaction: u64,
    g_tx_data_zero: u64,
    g_tx_data_nonzero: u64,
    g_create: u64,
    g_code_deposit: u64,
    g_call: u64,
    g_call_value: u64,
    g_new_account: u64,
}

#[derive(Debug)]
pub struct BlockEnvironment {
    coinbase: Address,
    timestamp: u64,
    number: u64,
    difficulty: U256,
    gas_limit: u64,
    base_fee: Option<U256>,
}

pub struct WorldState {
    // 简化实现
}

impl WorldState {
    fn get_account(&self, address: &Address) -> Result<Account, ProtocolError> {
        Ok(Account::default())
    }
    
    fn sub_balance(&mut self, address: &Address, amount: U256) -> Result<(), ProtocolError> {
        Ok(())
    }
    
    fn add_balance(&mut self, address: &Address, amount: U256) -> Result<(), ProtocolError> {
        Ok(())
    }
    
    fn increment_nonce(&mut self, address: &Address) -> Result<(), ProtocolError> {
        Ok(())
    }
}

#[derive(Debug, Default)]
struct Account {
    nonce: u64,
    balance: U256,
    storage_root: Hash,
    code_hash: Hash,
}

#[derive(Debug, Default)]
struct ExecutionResult {
    status: bool,
    gas_used: u64,
    logs: Vec<Log>,
}

#[derive(Debug)]
pub struct TransactionReceipt {
    status: bool,
    gas_used: u64,
    logs: Vec<Log>,
    logs_bloom: Bloom,
}

#[derive(Debug, Clone)]
pub struct Log {
    address: Address,
    topics: Vec<Hash>,
    data: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct Bloom([u8; 256]);

impl Bloom {
    fn from_logs(logs: &[Log]) -> Self {
        // 简化实现
        Self::default()
    }
}
```

### 1.3 EVM规范

```rust
/// EVM（以太坊虚拟机）规范
pub struct EVMSpecification {
    /// EVM版本
    version: EVMVersion,
}

#[derive(Debug, Clone, Copy)]
pub enum EVMVersion {
    Frontier,
    Homestead,
    Byzantium,
    Constantinople,
    Istanbul,
    Berlin,
    London,
    Shanghai,
    Cancun,
}

impl EVMSpecification {
    /// 操作码规范
    pub fn opcodes() -> Vec<Opcode> {
        vec![
            // 算术操作
            Opcode { code: 0x01, name: "ADD", gas: 3, stack_input: 2, stack_output: 1 },
            Opcode { code: 0x02, name: "MUL", gas: 5, stack_input: 2, stack_output: 1 },
            Opcode { code: 0x03, name: "SUB", gas: 3, stack_input: 2, stack_output: 1 },
            Opcode { code: 0x04, name: "DIV", gas: 5, stack_input: 2, stack_output: 1 },
            
            // 比较操作
            Opcode { code: 0x10, name: "LT", gas: 3, stack_input: 2, stack_output: 1 },
            Opcode { code: 0x11, name: "GT", gas: 3, stack_input: 2, stack_output: 1 },
            Opcode { code: 0x14, name: "EQ", gas: 3, stack_input: 2, stack_output: 1 },
            
            // 位运算
            Opcode { code: 0x16, name: "AND", gas: 3, stack_input: 2, stack_output: 1 },
            Opcode { code: 0x17, name: "OR", gas: 3, stack_input: 2, stack_output: 1 },
            Opcode { code: 0x18, name: "XOR", gas: 3, stack_input: 2, stack_output: 1 },
            
            // 栈操作
            Opcode { code: 0x50, name: "POP", gas: 2, stack_input: 1, stack_output: 0 },
            Opcode { code: 0x51, name: "MLOAD", gas: 3, stack_input: 1, stack_output: 1 },
            Opcode { code: 0x52, name: "MSTORE", gas: 3, stack_input: 2, stack_output: 0 },
            
            // 存储操作
            Opcode { code: 0x54, name: "SLOAD", gas: 800, stack_input: 1, stack_output: 1 },
            Opcode { code: 0x55, name: "SSTORE", gas: 20000, stack_input: 2, stack_output: 0 },
            
            // 控制流
            Opcode { code: 0x56, name: "JUMP", gas: 8, stack_input: 1, stack_output: 0 },
            Opcode { code: 0x57, name: "JUMPI", gas: 10, stack_input: 2, stack_output: 0 },
            
            // 环境信息
            Opcode { code: 0x30, name: "ADDRESS", gas: 2, stack_input: 0, stack_output: 1 },
            Opcode { code: 0x33, name: "CALLER", gas: 2, stack_input: 0, stack_output: 1 },
            Opcode { code: 0x34, name: "CALLVALUE", gas: 2, stack_input: 0, stack_output: 1 },
            
            // 调用操作
            Opcode { code: 0xF0, name: "CREATE", gas: 32000, stack_input: 3, stack_output: 1 },
            Opcode { code: 0xF1, name: "CALL", gas: 700, stack_input: 7, stack_output: 1 },
            Opcode { code: 0xF4, name: "DELEGATECALL", gas: 700, stack_input: 6, stack_output: 1 },
            Opcode { code: 0xFA, name: "STATICCALL", gas: 700, stack_input: 6, stack_output: 1 },
            
            // 其他
            Opcode { code: 0x00, name: "STOP", gas: 0, stack_input: 0, stack_output: 0 },
            Opcode { code: 0xFD, name: "REVERT", gas: 0, stack_input: 2, stack_output: 0 },
            Opcode { code: 0xFF, name: "SELFDESTRUCT", gas: 5000, stack_input: 1, stack_output: 0 },
        ]
    }
    
    /// 预编译合约
    pub fn precompiled_contracts() -> Vec<PrecompiledContract> {
        vec![
            PrecompiledContract {
                address: Address::from_low_u64_be(1),
                name: "ecrecover",
                gas_cost: 3000,
            },
            PrecompiledContract {
                address: Address::from_low_u64_be(2),
                name: "sha256",
                gas_cost: 60,
            },
            PrecompiledContract {
                address: Address::from_low_u64_be(3),
                name: "ripemd160",
                gas_cost: 600,
            },
            PrecompiledContract {
                address: Address::from_low_u64_be(4),
                name: "identity",
                gas_cost: 15,
            },
            PrecompiledContract {
                address: Address::from_low_u64_be(5),
                name: "modexp",
                gas_cost: 0, // 动态计算
            },
            PrecompiledContract {
                address: Address::from_low_u64_be(6),
                name: "ecadd",
                gas_cost: 150,
            },
            PrecompiledContract {
                address: Address::from_low_u64_be(7),
                name: "ecmul",
                gas_cost: 6000,
            },
            PrecompiledContract {
                address: Address::from_low_u64_be(8),
                name: "ecpairing",
                gas_cost: 0, // 动态计算
            },
            PrecompiledContract {
                address: Address::from_low_u64_be(9),
                name: "blake2f",
                gas_cost: 0, // 动态计算
            },
        ]
    }
    
    /// EVM执行规则
    pub fn execution_rules() -> ExecutionRules {
        ExecutionRules {
            stack_limit: 1024,
            call_depth_limit: 1024,
            memory_gas_cost: MemoryGasCost {
                linear_cost_per_word: 3,
                quadratic_cost_divisor: 512,
            },
        }
    }
}

#[derive(Debug)]
pub struct Opcode {
    code: u8,
    name: &'static str,
    gas: u64,
    stack_input: usize,
    stack_output: usize,
}

#[derive(Debug)]
pub struct PrecompiledContract {
    address: Address,
    name: &'static str,
    gas_cost: u64,
}

#[derive(Debug)]
pub struct ExecutionRules {
    stack_limit: usize,
    call_depth_limit: usize,
    memory_gas_cost: MemoryGasCost,
}

#[derive(Debug)]
pub struct MemoryGasCost {
    linear_cost_per_word: u64,
    quadratic_cost_divisor: u64,
}
```

## 2. 共识协议规范

### 2.1 PoW规范

已在共识理论文档中详细说明。

### 2.2 PoS规范

已在共识理论文档中详细说明。

### 2.3 BFT规范

已在共识理论文档中详细说明。

## 3. 网络协议

### 3.1 P2P协议

已在网络实现文档中详细说明。

### 3.2 RLPx协议

```rust
/// RLPx协议（以太坊P2P加密传输协议）
pub struct RLPxProtocol {
    /// ECDH密钥对
    key_pair: (SecretKey, PublicKey),
}

impl RLPxProtocol {
    /// RLPx握手
    pub async fn handshake(
        &self,
        stream: &mut TcpStream,
        remote_id: &PublicKey
    ) -> Result<RLPxConnection, ProtocolError> {
        // 1. 发送auth消息
        let auth_msg = self.create_auth_message(remote_id)?;
        stream.write_all(&auth_msg).await?;
        
        // 2. 接收ack消息
        let mut ack_buffer = vec![0u8; 210];
        stream.read_exact(&mut ack_buffer).await?;
        let ack = self.parse_ack_message(&ack_buffer)?;
        
        // 3. 派生加密密钥
        let secrets = self.derive_secrets(&ack)?;
        
        Ok(RLPxConnection {
            stream: stream.try_clone()?,
            secrets,
            frame_codec: FrameCodec::new(),
        })
    }
    
    fn create_auth_message(&self, remote_id: &PublicKey) -> Result<Vec<u8>, ProtocolError> {
        use sha3::{Digest, Keccak256};
        
        // 生成临时密钥
        let ephemeral_key = SecretKey::random();
        
        // 构造auth消息
        let mut auth = Vec::new();
        
        // 签名
        let shared_secret = self.ecdh(remote_id)?;
        let signature = self.sign(&shared_secret)?;
        auth.extend_from_slice(&signature);
        
        // 公钥哈希
        let pubkey_hash = Keccak256::digest(self.key_pair.1.serialize());
        auth.extend_from_slice(&pubkey_hash);
        
        // 临时公钥
        auth.extend_from_slice(&ephemeral_key.public_key().serialize());
        
        // Nonce
        let nonce = rand::random::<[u8; 32]>();
        auth.extend_from_slice(&nonce);
        
        // 加密auth消息
        let encrypted = self.encrypt_auth(&auth, remote_id)?;
        
        Ok(encrypted)
    }
    
    fn parse_ack_message(&self, data: &[u8]) -> Result<AckMessage, ProtocolError> {
        // 解密和解析ack消息
        Ok(AckMessage {
            ephemeral_pubkey: PublicKey::default(),
            nonce: [0u8; 32],
        })
    }
    
    fn derive_secrets(&self, ack: &AckMessage) -> Result<Secrets, ProtocolError> {
        // 派生MAC和加密密钥
        Ok(Secrets {
            aes_secret: [0u8; 32],
            mac_secret: [0u8; 32],
        })
    }
    
    fn ecdh(&self, remote_pubkey: &PublicKey) -> Result<Vec<u8>, ProtocolError> {
        // ECDH密钥交换
        Ok(vec![])
    }
    
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, ProtocolError> {
        // ECDSA签名
        Ok(vec![])
    }
    
    fn encrypt_auth(&self, data: &[u8], remote_pubkey: &PublicKey) -> Result<Vec<u8>, ProtocolError> {
        // ECIES加密
        Ok(vec![])
    }
}

#[derive(Debug)]
pub struct RLPxConnection {
    stream: TcpStream,
    secrets: Secrets,
    frame_codec: FrameCodec,
}

#[derive(Debug)]
struct AckMessage {
    ephemeral_pubkey: PublicKey,
    nonce: [u8; 32],
}

#[derive(Debug)]
struct Secrets {
    aes_secret: [u8; 32],
    mac_secret: [u8; 32],
}

struct FrameCodec {
    // RLP帧编解码器
}

impl FrameCodec {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Default)]
struct SecretKey {
    // 简化实现
}

impl SecretKey {
    fn random() -> Self {
        Self::default()
    }
    
    fn public_key(&self) -> PublicKey {
        PublicKey::default()
    }
}

#[derive(Debug, Default, Clone)]
struct PublicKey {
    // 简化实现
}

impl PublicKey {
    fn serialize(&self) -> Vec<u8> {
        vec![]
    }
}
```

### 3.3 DevP2P协议

```rust
/// DevP2P协议栈
pub struct DevP2PProtocol {
    /// 支持的子协议
    subprotocols: Vec<SubProtocol>,
}

#[derive(Debug)]
pub struct SubProtocol {
    name: String,
    version: u8,
    length: u16, // 消息ID范围长度
}

impl DevP2PProtocol {
    /// 以太坊wire协议
    pub fn eth_protocol() -> SubProtocol {
        SubProtocol {
            name: "eth".to_string(),
            version: 68,
            length: 17,
        }
    }
    
    /// 节点发现协议
    pub fn disc_protocol() -> SubProtocol {
        SubProtocol {
            name: "disc".to_string(),
            version: 5,
            length: 16,
        }
    }
    
    /// Hello消息
    pub fn hello_message() -> HelloMessage {
        HelloMessage {
            protocol_version: 5,
            client_id: "rust-blockchain/v1.0.0".to_string(),
            capabilities: vec![
                Capability { name: "eth".to_string(), version: 68 },
                Capability { name: "snap".to_string(), version: 1 },
            ],
            listen_port: 30303,
            node_id: vec![0u8; 64],
        }
    }
}

#[derive(Debug)]
pub struct HelloMessage {
    protocol_version: u8,
    client_id: String,
    capabilities: Vec<Capability>,
    listen_port: u16,
    node_id: Vec<u8>,
}

#[derive(Debug)]
pub struct Capability {
    name: String,
    version: u8,
}
```

## 4. 数据结构规范

### 4.1 区块结构

已在以太坊协议部分说明。

### 4.2 交易结构

已在以太坊协议部分说明。

### 4.3 状态树结构

```rust
/// Merkle Patricia Trie规范
pub struct MPTSpecification;

impl MPTSpecification {
    /// 节点类型
    pub fn node_types() -> Vec<NodeType> {
        vec![
            NodeType::Null,         // 空节点
            NodeType::Branch,       // 分支节点（17个子节点）
            NodeType::Extension,    // 扩展节点（压缩路径）
            NodeType::Leaf,         // 叶子节点（存储值）
        ]
    }
    
    /// RLP编码规则
    pub fn rlp_encoding_rules() -> RLPRules {
        RLPRules {
            single_byte_max: 0x7f,
            short_string_max: 55,
            long_string_prefix: 0xb7,
            short_list_max: 55,
            long_list_prefix: 0xf7,
        }
    }
}

#[derive(Debug)]
pub enum NodeType {
    Null,
    Branch,
    Extension,
    Leaf,
}

#[derive(Debug)]
pub struct RLPRules {
    single_byte_max: u8,
    short_string_max: u8,
    long_string_prefix: u8,
    short_list_max: u8,
    long_list_prefix: u8,
}
```

## 5. 加密标准

### 5.1 哈希算法

```rust
/// 哈希算法规范
pub struct HashAlgorithms;

impl HashAlgorithms {
    /// Keccak-256
    pub fn keccak256(data: &[u8]) -> [u8; 32] {
        use sha3::{Digest, Keccak256};
        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
    
    /// SHA-256
    pub fn sha256(data: &[u8]) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
    
    /// RIPEMD-160
    pub fn ripemd160(data: &[u8]) -> [u8; 20] {
        use ripemd::{Digest, Ripemd160};
        let mut hasher = Ripemd160::new();
        hasher.update(data);
        hasher.finalize().into()
    }
    
    /// Blake2b
    pub fn blake2b(data: &[u8]) -> [u8; 64] {
        use blake2::{Blake2b512, Digest};
        let mut hasher = Blake2b512::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}
```

### 5.2 签名算法

```rust
/// 数字签名算法规范
pub struct SignatureAlgorithms;

impl SignatureAlgorithms {
    /// ECDSA (secp256k1)
    pub fn ecdsa_sign(
        message: &[u8],
        private_key: &[u8; 32]
    ) -> Result<[u8; 65], ProtocolError> {
        use secp256k1::{Secp256k1, Message, SecretKey};
        
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(private_key)?;
        let message = Message::from_slice(message)?;
        
        let signature = secp.sign_ecdsa_recoverable(&message, &secret_key);
        let (recovery_id, sig_bytes) = signature.serialize_compact();
        
        let mut result = [0u8; 65];
        result[..64].copy_from_slice(&sig_bytes);
        result[64] = recovery_id.to_i32() as u8;
        
        Ok(result)
    }
    
    /// ECDSA验证
    pub fn ecdsa_verify(
        message: &[u8],
        signature: &[u8; 65],
        public_key: &[u8; 64]
    ) -> Result<bool, ProtocolError> {
        // 验证签名
        Ok(true)
    }
    
    /// BLS签名
    pub fn bls_sign(
        message: &[u8],
        private_key: &[u8; 32]
    ) -> Result<[u8; 96], ProtocolError> {
        // BLS签名实现
        Ok([0u8; 96])
    }
    
    /// BLS聚合签名
    pub fn bls_aggregate(signatures: &[[u8; 96]]) -> Result<[u8; 96], ProtocolError> {
        // BLS聚合签名
        Ok([0u8; 96])
    }
}
```

### 5.3 加密算法

```rust
/// 加密算法规范
pub struct EncryptionAlgorithms;

impl EncryptionAlgorithms {
    /// AES-256-GCM
    pub fn aes_gcm_encrypt(
        plaintext: &[u8],
        key: &[u8; 32],
        nonce: &[u8; 12]
    ) -> Result<Vec<u8>, ProtocolError> {
        use aes_gcm::{Aes256Gcm, Key, Nonce};
        use aes_gcm::aead::{Aead, NewAead};
        
        let cipher = Aes256Gcm::new(Key::from_slice(key));
        let nonce = Nonce::from_slice(nonce);
        
        cipher.encrypt(nonce, plaintext)
            .map_err(|e| ProtocolError::EncryptionError(e.to_string()))
    }
    
    /// AES-256-GCM解密
    pub fn aes_gcm_decrypt(
        ciphertext: &[u8],
        key: &[u8; 32],
        nonce: &[u8; 12]
    ) -> Result<Vec<u8>, ProtocolError> {
        use aes_gcm::{Aes256Gcm, Key, Nonce};
        use aes_gcm::aead::{Aead, NewAead};
        
        let cipher = Aes256Gcm::new(Key::from_slice(key));
        let nonce = Nonce::from_slice(nonce);
        
        cipher.decrypt(nonce, ciphertext)
            .map_err(|e| ProtocolError::DecryptionError(e.to_string()))
    }
    
    /// ChaCha20-Poly1305
    pub fn chacha20_poly1305_encrypt(
        plaintext: &[u8],
        key: &[u8; 32],
        nonce: &[u8; 12]
    ) -> Result<Vec<u8>, ProtocolError> {
        use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
        use chacha20poly1305::aead::{Aead, NewAead};
        
        let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
        let nonce = Nonce::from_slice(nonce);
        
        cipher.encrypt(nonce, plaintext)
            .map_err(|e| ProtocolError::EncryptionError(e.to_string()))
    }
}
```

## 6. RPC协议

### 6.1 JSON-RPC规范

```rust
/// JSON-RPC 2.0规范
pub struct JsonRpcSpec;

impl JsonRpcSpec {
    /// 请求格式
    pub fn request_format() -> JsonRpcRequest {
        JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "eth_blockNumber".to_string(),
            params: vec![],
            id: 1,
        }
    }
    
    /// 响应格式
    pub fn response_format() -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(serde_json::json!("0x1b4")),
            error: None,
            id: 1,
        }
    }
    
    /// 错误格式
    pub fn error_format() -> JsonRpcError {
        JsonRpcError {
            code: -32600,
            message: "Invalid Request".to_string(),
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Vec<serde_json::Value>,
    id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
    id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}
```

### 6.2 以太坊RPC API

```rust
/// 以太坊JSON-RPC API规范
pub struct EthereumRpcApi;

impl EthereumRpcApi {
    /// API方法列表
    pub fn methods() -> Vec<RpcMethod> {
        vec![
            // 基础查询
            RpcMethod {
                name: "eth_blockNumber",
                description: "Returns the number of most recent block",
                params: vec![],
                returns: "QUANTITY - integer of the current block number",
            },
            RpcMethod {
                name: "eth_getBalance",
                description: "Returns the balance of the account",
                params: vec!["DATA - address", "QUANTITY|TAG - block number"],
                returns: "QUANTITY - integer of the current balance in wei",
            },
            
            // 交易相关
            RpcMethod {
                name: "eth_sendTransaction",
                description: "Creates new message call transaction",
                params: vec!["Object - transaction object"],
                returns: "DATA - 32 Bytes - transaction hash",
            },
            RpcMethod {
                name: "eth_getTransactionReceipt",
                description: "Returns the receipt of a transaction",
                params: vec!["DATA - transaction hash"],
                returns: "Object - transaction receipt object",
            },
            
            // 合约调用
            RpcMethod {
                name: "eth_call",
                description: "Executes a new message call immediately",
                params: vec!["Object - call object", "QUANTITY|TAG - block number"],
                returns: "DATA - return value of executed contract",
            },
            RpcMethod {
                name: "eth_estimateGas",
                description: "Generates and returns an estimate of gas",
                params: vec!["Object - call object"],
                returns: "QUANTITY - amount of gas used",
            },
            
            // 区块相关
            RpcMethod {
                name: "eth_getBlockByNumber",
                description: "Returns information about a block by number",
                params: vec!["QUANTITY|TAG", "Boolean - full tx objects"],
                returns: "Object - block object",
            },
            
            // 日志过滤
            RpcMethod {
                name: "eth_getLogs",
                description: "Returns an array of all logs matching filter",
                params: vec!["Object - filter options"],
                returns: "Array - array of log objects",
            },
        ]
    }
}

#[derive(Debug)]
pub struct RpcMethod {
    name: &'static str,
    description: &'static str,
    params: Vec<&'static str>,
    returns: &'static str,
}
```

### 6.3 自定义RPC

```rust
/// 自定义RPC API
pub struct CustomRpcApi {
    handlers: HashMap<String, Box<dyn RpcHandler>>,
}

pub trait RpcHandler: Send + Sync {
    fn handle(&self, params: Vec<serde_json::Value>) -> Result<serde_json::Value, RpcError>;
}

impl CustomRpcApi {
    pub fn new() -> Self {
        let mut api = Self {
            handlers: HashMap::new(),
        };
        
        // 注册自定义方法
        api.register("custom_getNodeInfo", Box::new(GetNodeInfoHandler));
        api.register("custom_getPeerCount", Box::new(GetPeerCountHandler));
        
        api
    }
    
    pub fn register(&mut self, method: &str, handler: Box<dyn RpcHandler>) {
        self.handlers.insert(method.to_string(), handler);
    }
    
    pub fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        match self.handlers.get(&request.method) {
            Some(handler) => {
                match handler.handle(request.params) {
                    Ok(result) => JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        result: Some(result),
                        error: None,
                        id: request.id,
                    },
                    Err(error) => JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        result: None,
                        error: Some(JsonRpcError {
                            code: error.code,
                            message: error.message,
                            data: None,
                        }),
                        id: request.id,
                    },
                }
            }
            None => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
                id: request.id,
            },
        }
    }
}

struct GetNodeInfoHandler;

impl RpcHandler for GetNodeInfoHandler {
    fn handle(&self, _params: Vec<serde_json::Value>) -> Result<serde_json::Value, RpcError> {
        Ok(serde_json::json!({
            "version": "1.0.0",
            "node_id": "0x1234...",
            "peer_count": 10,
        }))
    }
}

struct GetPeerCountHandler;

impl RpcHandler for GetPeerCountHandler {
    fn handle(&self, _params: Vec<serde_json::Value>) -> Result<serde_json::Value, RpcError> {
        Ok(serde_json::json!(10))
    }
}

#[derive(Debug)]
pub struct RpcError {
    code: i32,
    message: String,
}
```

## 7. EIP/ERC标准

### 7.1 核心EIP

```rust
/// 核心EIP标准
pub struct CoreEIPs;

impl CoreEIPs {
    pub fn eip_list() -> Vec<EIP> {
        vec![
            EIP {
                number: 1559,
                title: "Fee market change for ETH 1.0 chain",
                status: EIPStatus::Final,
                type_: EIPType::Core,
                description: "引入base fee和priority fee机制",
            },
            EIP {
                number: 2930,
                title: "Optional access lists",
                status: EIPStatus::Final,
                type_: EIPType::Core,
                description: "为交易添加可选的访问列表",
            },
            EIP {
                number: 4844,
                title: "Shard Blob Transactions",
                status: EIPStatus::Draft,
                type_: EIPType::Core,
                description: "Proto-danksharding，为分片做准备",
            },
        ]
    }
}

#[derive(Debug)]
pub struct EIP {
    number: u32,
    title: &'static str,
    status: EIPStatus,
    type_: EIPType,
    description: &'static str,
}

#[derive(Debug)]
pub enum EIPStatus {
    Draft,
    Review,
    LastCall,
    Final,
    Stagnant,
    Withdrawn,
}

#[derive(Debug)]
pub enum EIPType {
    Core,
    Networking,
    Interface,
    ERC,
    Meta,
    Informational,
}
```

### 7.2 代币标准

```rust
/// ERC代币标准
pub struct TokenStandards;

impl TokenStandards {
    /// ERC-20接口
    pub fn erc20_interface() -> Vec<FunctionSignature> {
        vec![
            FunctionSignature {
                name: "totalSupply",
                signature: "totalSupply()",
                returns: "uint256",
            },
            FunctionSignature {
                name: "balanceOf",
                signature: "balanceOf(address)",
                returns: "uint256",
            },
            FunctionSignature {
                name: "transfer",
                signature: "transfer(address,uint256)",
                returns: "bool",
            },
            FunctionSignature {
                name: "transferFrom",
                signature: "transferFrom(address,address,uint256)",
                returns: "bool",
            },
            FunctionSignature {
                name: "approve",
                signature: "approve(address,uint256)",
                returns: "bool",
            },
            FunctionSignature {
                name: "allowance",
                signature: "allowance(address,address)",
                returns: "uint256",
            },
        ]
    }
    
    /// ERC-721接口（NFT）
    pub fn erc721_interface() -> Vec<FunctionSignature> {
        vec![
            FunctionSignature {
                name: "balanceOf",
                signature: "balanceOf(address)",
                returns: "uint256",
            },
            FunctionSignature {
                name: "ownerOf",
                signature: "ownerOf(uint256)",
                returns: "address",
            },
            FunctionSignature {
                name: "safeTransferFrom",
                signature: "safeTransferFrom(address,address,uint256)",
                returns: "void",
            },
            FunctionSignature {
                name: "approve",
                signature: "approve(address,uint256)",
                returns: "void",
            },
        ]
    }
    
    /// ERC-1155接口（多代币）
    pub fn erc1155_interface() -> Vec<FunctionSignature> {
        vec![
            FunctionSignature {
                name: "balanceOf",
                signature: "balanceOf(address,uint256)",
                returns: "uint256",
            },
            FunctionSignature {
                name: "balanceOfBatch",
                signature: "balanceOfBatch(address[],uint256[])",
                returns: "uint256[]",
            },
            FunctionSignature {
                name: "safeTransferFrom",
                signature: "safeTransferFrom(address,address,uint256,uint256,bytes)",
                returns: "void",
            },
            FunctionSignature {
                name: "safeBatchTransferFrom",
                signature: "safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)",
                returns: "void",
            },
        ]
    }
}

#[derive(Debug)]
pub struct FunctionSignature {
    name: &'static str,
    signature: &'static str,
    returns: &'static str,
}
```

### 7.3 应用标准

已在NFT生态系统文档中详细说明。

## 8. 跨链协议

### 8.1 IBC协议

已在行业标准文档中详细说明。

### 8.2 桥接协议

已在行业标准文档中详细说明。

### 8.3 中继协议

已在行业标准文档中详细说明。

## 9. Layer 2协议

### 9.1 Rollup规范

已在Web3技术文档中详细说明。

### 9.2 State Channel

已在Web3技术文档中详细说明。

### 9.3 Plasma规范

已在Web3技术文档中详细说明。

## 10. 总结

本文档全面介绍了区块链协议规范，涵盖：

1. **核心协议**：比特币协议、以太坊协议、EVM规范
2. **共识协议**：PoW、PoS、BFT规范
3. **网络协议**：P2P、RLPx、DevP2P协议
4. **数据结构**：区块、交易、状态树结构
5. **加密标准**：哈希、签名、加密算法
6. **RPC协议**：JSON-RPC、以太坊RPC API、自定义RPC
7. **EIP/ERC标准**：核心EIP、代币标准、应用标准
8. **跨链协议**：IBC、桥接、中继协议
9. **Layer 2协议**：Rollup、State Channel、Plasma

**关键要点**：

- 遵循标准协议确保互操作性
- 理解EVM执行模型和Gas机制
- 掌握RLPx和DevP2P网络协议
- 熟悉ERC代币标准
- 关注最新的EIP提案

---

**文档版本**: v1.0.0  
**最后更新**: 2025年10月17日  
**作者**: Rust区块链技术团队  
**相关文档**:

- [16_INTERNATIONAL_STANDARDS.md](./16_INTERNATIONAL_STANDARDS.md) - 国际标准
- [17_INDUSTRY_STANDARDS.md](./17_INDUSTRY_STANDARDS.md) - 行业标准
- [15_NETWORK_IMPLEMENTATION.md](./15_NETWORK_IMPLEMENTATION.md) - 网络实现
