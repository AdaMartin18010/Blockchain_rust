# Web3技术栈

## 📋 目录

- [Web3技术栈](#web3技术栈)
  - [📋 目录](#-目录)
  - [1. Web3概述](#1-web3概述)
    - [1.1 Web1 vs Web2 vs Web3](#11-web1-vs-web2-vs-web3)
    - [1.2 Web3核心特征](#12-web3核心特征)
    - [1.3 Web3技术架构](#13-web3技术架构)
  - [2. 前端技术](#2-前端技术)
    - [2.1 Web3.js / Ethers.js](#21-web3js--ethersjs)
    - [2.2 Wallet连接](#22-wallet连接)
    - [2.3 去中心化前端](#23-去中心化前端)
  - [3. 智能合约交互](#3-智能合约交互)
    - [3.1 合约调用](#31-合约调用)
    - [3.2 事件监听](#32-事件监听)
    - [3.3 交易管理](#33-交易管理)
  - [4. 去中心化存储](#4-去中心化存储)
    - [4.1 IPFS](#41-ipfs)
    - [4.2 Arweave](#42-arweave)
    - [4.3 Filecoin](#43-filecoin)
  - [5. 去中心化身份(DID)](#5-去中心化身份did)
    - [5.1 DID规范](#51-did规范)
    - [5.2 ENS域名系统](#52-ens域名系统)
    - [5.3 可验证凭证](#53-可验证凭证)
  - [6. Web3通信协议](#6-web3通信协议)
    - [6.1 libp2p](#61-libp2p)
    - [6.2 Whisper](#62-whisper)
    - [6.3 XMTP](#63-xmtp)
  - [7. 预言机服务](#7-预言机服务)
    - [7.1 Chainlink](#71-chainlink)
    - [7.2 Band Protocol](#72-band-protocol)
    - [7.3 自定义预言机](#73-自定义预言机)
  - [8. Web3开发框架](#8-web3开发框架)
    - [8.1 Hardhat](#81-hardhat)
    - [8.2 Foundry](#82-foundry)
    - [8.3 Truffle](#83-truffle)
  - [9. Web3 UI/UX](#9-web3-uiux)
    - [9.1 Web3设计模式](#91-web3设计模式)
    - [9.2 用户体验优化](#92-用户体验优化)
    - [9.3 组件库](#93-组件库)
  - [10. Web3安全](#10-web3安全)
    - [10.1 前端安全](#101-前端安全)
    - [10.2 交易签名验证](#102-交易签名验证)
    - [10.3 钓鱼防护](#103-钓鱼防护)
  - [11. 实战项目：构建Web3 DApp](#11-实战项目构建web3-dapp)
    - [11.1 项目初始化](#111-项目初始化)
    - [11.2 智能合约开发](#112-智能合约开发)
    - [11.3 前端集成](#113-前端集成)
  - [12. Web3未来展望](#12-web3未来展望)
  - [总结](#总结)
  - [相关文档](#相关文档)
  - [参考资料](#参考资料)
  - [实用工具](#实用工具)

## 1. Web3概述

### 1.1 Web1 vs Web2 vs Web3

```rust
/// Web演进历史
#[derive(Debug, Clone)]
pub struct WebEvolution {
    pub web1: WebEra,
    pub web2: WebEra,
    pub web3: WebEra,
}

#[derive(Debug, Clone)]
pub struct WebEra {
    pub name: String,
    pub period: String,
    pub characteristics: Vec<String>,
    pub key_features: Vec<String>,
    pub examples: Vec<String>,
}

pub fn compare_web_eras() -> WebEvolution {
    WebEvolution {
        web1: WebEra {
            name: "Web 1.0 - 只读网络".to_string(),
            period: "1990-2004".to_string(),
            characteristics: vec![
                "静态HTML页面".to_string(),
                "单向信息流".to_string(),
                "内容创建者少".to_string(),
                "用户只能浏览".to_string(),
            ],
            key_features: vec![
                "个人网站".to_string(),
                "门户网站".to_string(),
                "目录服务".to_string(),
            ],
            examples: vec![
                "Yahoo Directory".to_string(),
                "GeoCities".to_string(),
                "个人主页".to_string(),
            ],
        },
        web2: WebEra {
            name: "Web 2.0 - 读写网络".to_string(),
            period: "2004-至今".to_string(),
            characteristics: vec![
                "动态内容".to_string(),
                "用户生成内容(UGC)".to_string(),
                "社交媒体".to_string(),
                "平台主导".to_string(),
                "数据中心化".to_string(),
            ],
            key_features: vec![
                "社交网络".to_string(),
                "云计算".to_string(),
                "移动互联网".to_string(),
                "SaaS".to_string(),
            ],
            examples: vec![
                "Facebook".to_string(),
                "Twitter".to_string(),
                "YouTube".to_string(),
                "Google".to_string(),
            ],
        },
        web3: WebEra {
            name: "Web 3.0 - 读写拥有网络".to_string(),
            period: "2020-未来".to_string(),
            characteristics: vec![
                "去中心化".to_string(),
                "用户拥有数据".to_string(),
                "无需许可".to_string(),
                "加密货币原生".to_string(),
                "智能合约驱动".to_string(),
            ],
            key_features: vec![
                "区块链".to_string(),
                "去中心化身份".to_string(),
                "代币经济".to_string(),
                "DAO治理".to_string(),
            ],
            examples: vec![
                "Uniswap".to_string(),
                "OpenSea".to_string(),
                "Lens Protocol".to_string(),
                "ENS".to_string(),
            ],
        },
    }
}
```

### 1.2 Web3核心特征

```rust
/// Web3核心特征
#[derive(Debug)]
pub struct Web3CoreFeatures {
    /// 去中心化
    pub decentralization: DecentralizationFeature,
    
    /// 无需许可
    pub permissionless: PermissionlessFeature,
    
    /// 用户主权
    pub user_sovereignty: UserSovereigntyFeature,
    
    /// 原生支付
    pub native_payments: NativePaymentFeature,
    
    /// 可组合性
    pub composability: ComposabilityFeature,
}

#[derive(Debug)]
pub struct DecentralizationFeature {
    pub no_single_point_of_failure: bool,
    pub censorship_resistant: bool,
    pub distributed_governance: bool,
}

#[derive(Debug)]
pub struct PermissionlessFeature {
    pub open_participation: bool,
    pub no_gatekeepers: bool,
    pub global_access: bool,
}

#[derive(Debug)]
pub struct UserSovereigntyFeature {
    pub own_data: bool,
    pub own_identity: bool,
    pub own_assets: bool,
    pub portability: bool,
}

#[derive(Debug)]
pub struct NativePaymentFeature {
    pub crypto_native: bool,
    pub programmable_money: bool,
    pub instant_settlement: bool,
    pub global_payments: bool,
}

#[derive(Debug)]
pub struct ComposabilityFeature {
    pub interoperable_protocols: bool,
    pub money_legos: bool,
    pub open_standards: bool,
}
```

### 1.3 Web3技术架构

```rust
/// Web3技术栈
#[derive(Debug)]
pub struct Web3TechStack {
    /// 区块链层
    pub blockchain_layer: BlockchainLayer,
    
    /// 协议层
    pub protocol_layer: ProtocolLayer,
    
    /// 应用层
    pub application_layer: ApplicationLayer,
    
    /// 接入层
    pub access_layer: AccessLayer,
}

#[derive(Debug)]
pub struct BlockchainLayer {
    pub networks: Vec<String>,  // Ethereum, Polygon, etc.
    pub consensus: String,       // PoW, PoS, etc.
    pub smart_contracts: bool,
}

#[derive(Debug)]
pub struct ProtocolLayer {
    pub defi: Vec<String>,      // Uniswap, Aave, etc.
    pub nft: Vec<String>,        // OpenSea, Rarible, etc.
    pub governance: Vec<String>, // Snapshot, Aragon, etc.
    pub storage: Vec<String>,    // IPFS, Arweave, etc.
}

#[derive(Debug)]
pub struct ApplicationLayer {
    pub dapps: Vec<String>,
    pub wallets: Vec<String>,
    pub explorers: Vec<String>,
}

#[derive(Debug)]
pub struct AccessLayer {
    pub web3_providers: Vec<String>,  // Infura, Alchemy, etc.
    pub sdks: Vec<String>,             // Web3.js, Ethers.js, etc.
    pub apis: Vec<String>,
}
```

## 2. 前端技术

### 2.1 Web3.js / Ethers.js

```rust
/// Web3前端集成(Rust视角，实际使用JavaScript/TypeScript)
/// 以下是概念模型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Web3 Provider
#[derive(Debug)]
pub struct Web3Provider {
    pub rpc_url: String,
    pub chain_id: u64,
    pub network_name: String,
}

impl Web3Provider {
    pub fn new(rpc_url: String, chain_id: u64, network_name: String) -> Self {
        Self {
            rpc_url,
            chain_id,
            network_name,
        }
    }
    
    /// 连接到提供商
    pub async fn connect(&self) -> Result<Connection, Web3Error> {
        println!("Connecting to {} at {}", self.network_name, self.rpc_url);
        Ok(Connection {
            provider: self.clone(),
            connected: true,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub provider: Web3Provider,
    pub connected: bool,
}

impl Connection {
    /// 获取区块号
    pub async fn get_block_number(&self) -> Result<u64, Web3Error> {
        // 实际实现会调用RPC
        Ok(1000000)
    }
    
    /// 获取余额
    pub async fn get_balance(&self, address: &str) -> Result<u128, Web3Error> {
        // 实际实现会调用eth_getBalance
        Ok(1000000000000000000) // 1 ETH in wei
    }
    
    /// 获取交易
    pub async fn get_transaction(&self, tx_hash: &str) -> Result<Transaction, Web3Error> {
        Ok(Transaction {
            hash: tx_hash.to_string(),
            from: "0x0".to_string(),
            to: "0x0".to_string(),
            value: 0,
            data: vec![],
            nonce: 0,
            gas_limit: 21000,
            gas_price: 20000000000,
        })
    }
}

/// 交易结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: u128,
    pub data: Vec<u8>,
    pub nonce: u64,
    pub gas_limit: u64,
    pub gas_price: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum Web3Error {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
    
    #[error("Contract error: {0}")]
    ContractError(String),
}
```

### 2.2 Wallet连接

```rust
/// 钱包连接器
#[derive(Debug)]
pub struct WalletConnector {
    pub connected_wallet: Option<ConnectedWallet>,
    pub supported_wallets: Vec<WalletType>,
}

#[derive(Debug, Clone)]
pub struct ConnectedWallet {
    pub address: String,
    pub wallet_type: WalletType,
    pub chain_id: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WalletType {
    MetaMask,
    WalletConnect,
    Coinbase,
    Ledger,
    Trezor,
    Other(String),
}

impl WalletConnector {
    pub fn new() -> Self {
        Self {
            connected_wallet: None,
            supported_wallets: vec![
                WalletType::MetaMask,
                WalletType::WalletConnect,
                WalletType::Coinbase,
            ],
        }
    }
    
    /// 连接钱包
    pub async fn connect_wallet(
        &mut self,
        wallet_type: WalletType,
    ) -> Result<ConnectedWallet, Web3Error> {
        // 实际实现会打开钱包弹窗
        println!("Requesting wallet connection: {:?}", wallet_type);
        
        let wallet = ConnectedWallet {
            address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
            wallet_type,
            chain_id: 1,
        };
        
        self.connected_wallet = Some(wallet.clone());
        
        Ok(wallet)
    }
    
    /// 断开钱包
    pub fn disconnect_wallet(&mut self) {
        self.connected_wallet = None;
    }
    
    /// 切换网络
    pub async fn switch_network(&mut self, chain_id: u64) -> Result<(), Web3Error> {
        if let Some(wallet) = &mut self.connected_wallet {
            wallet.chain_id = chain_id;
            Ok(())
        } else {
            Err(Web3Error::ConnectionError("No wallet connected".to_string()))
        }
    }
    
    /// 签名消息
    pub async fn sign_message(&self, message: &str) -> Result<String, Web3Error> {
        if self.connected_wallet.is_none() {
            return Err(Web3Error::ConnectionError("No wallet connected".to_string()));
        }
        
        // 实际实现会调用钱包签名
        Ok(format!("0x{}", "a".repeat(130)))
    }
}

/// WalletConnect示例
pub struct WalletConnectSession {
    pub session_id: String,
    pub bridge_url: String,
    pub connected: bool,
}

impl WalletConnectSession {
    pub async fn create() -> Result<Self, Web3Error> {
        Ok(Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            bridge_url: "https://bridge.walletconnect.org".to_string(),
            connected: false,
        })
    }
    
    pub async fn connect(&mut self) -> Result<String, Web3Error> {
        // 生成二维码
        let qr_data = format!("wc:{}@1?bridge={}&key={}", 
                             self.session_id, 
                             self.bridge_url,
                             "key");
        
        self.connected = true;
        
        Ok(qr_data)
    }
}
```

### 2.3 去中心化前端

```rust
/// IPFS前端托管
#[derive(Debug)]
pub struct IPFSFrontend {
    pub ipfs_gateway: String,
    pub content_hash: Option<String>,
}

impl IPFSFrontend {
    pub fn new(gateway: String) -> Self {
        Self {
            ipfs_gateway: gateway,
            content_hash: None,
        }
    }
    
    /// 上传前端资源到IPFS
    pub async fn upload_app(&mut self, files: Vec<File>) -> Result<String, Web3Error> {
        // 实际实现会上传到IPFS节点
        let content_hash = "QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG".to_string();
        
        self.content_hash = Some(content_hash.clone());
        
        Ok(content_hash)
    }
    
    /// 获取访问URL
    pub fn get_url(&self) -> Option<String> {
        self.content_hash.as_ref().map(|hash| {
            format!("{}/ipfs/{}", self.ipfs_gateway, hash)
        })
    }
    
    /// 固定内容(防止被垃圾回收)
    pub async fn pin_content(&self, hash: &str) -> Result<(), Web3Error> {
        println!("Pinning content: {}", hash);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    pub content: Vec<u8>,
}

/// ENS域名绑定
pub struct ENSBinding {
    pub domain: String,
    pub content_hash: String,
}

impl ENSBinding {
    /// 设置ENS内容哈希
    pub async fn set_content_hash(
        &self,
        wallet: &ConnectedWallet,
    ) -> Result<(), Web3Error> {
        println!("Setting {} -> {}", self.domain, self.content_hash);
        // 实际实现会调用ENS合约
        Ok(())
    }
    
    /// 解析ENS
    pub async fn resolve_ens(domain: &str) -> Result<String, Web3Error> {
        // 实际实现会查询ENS合约
        Ok("QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG".to_string())
    }
}
```

## 3. 智能合约交互

### 3.1 合约调用

```rust
/// 合约接口
#[derive(Debug, Clone)]
pub struct ContractInterface {
    pub address: String,
    pub abi: Vec<ABIFunction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABIFunction {
    pub name: String,
    pub inputs: Vec<ABIParameter>,
    pub outputs: Vec<ABIParameter>,
    pub state_mutability: StateMutability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABIParameter {
    pub name: String,
    pub param_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateMutability {
    Pure,
    View,
    Nonpayable,
    Payable,
}

/// 合约实例
pub struct ContractInstance {
    pub interface: ContractInterface,
    pub connection: Connection,
}

impl ContractInstance {
    /// 调用只读函数
    pub async fn call_view_function(
        &self,
        function_name: &str,
        params: Vec<String>,
    ) -> Result<String, Web3Error> {
        println!("Calling view function: {}({:?})", function_name, params);
        
        // 实际实现会调用eth_call
        Ok("return_value".to_string())
    }
    
    /// 发送交易(写函数)
    pub async fn send_transaction(
        &self,
        function_name: &str,
        params: Vec<String>,
        from: &str,
        value: u128,
    ) -> Result<String, Web3Error> {
        println!("Sending transaction: {}({:?}) from {} value {}", 
                 function_name, params, from, value);
        
        // 实际实现会发送eth_sendTransaction
        Ok("0x".to_string() + &"1".repeat(64))
    }
    
    /// 估算gas
    pub async fn estimate_gas(
        &self,
        function_name: &str,
        params: Vec<String>,
    ) -> Result<u64, Web3Error> {
        // 实际实现会调用eth_estimateGas
        Ok(50000)
    }
}

/// ERC-20代币接口示例
pub struct ERC20Contract {
    pub contract: ContractInstance,
}

impl ERC20Contract {
    /// 查询余额
    pub async fn balance_of(&self, account: &str) -> Result<u128, Web3Error> {
        let result = self.contract
            .call_view_function("balanceOf", vec![account.to_string()])
            .await?;
        
        // 解析返回值
        Ok(u128::from_str_radix(&result.trim_start_matches("0x"), 16)
           .unwrap_or(0))
    }
    
    /// 转账
    pub async fn transfer(
        &self,
        from: &str,
        to: &str,
        amount: u128,
    ) -> Result<String, Web3Error> {
        self.contract.send_transaction(
            "transfer",
            vec![to.to_string(), amount.to_string()],
            from,
            0,
        ).await
    }
    
    /// 授权
    pub async fn approve(
        &self,
        owner: &str,
        spender: &str,
        amount: u128,
    ) -> Result<String, Web3Error> {
        self.contract.send_transaction(
            "approve",
            vec![spender.to_string(), amount.to_string()],
            owner,
            0,
        ).await
    }
}
```

### 3.2 事件监听

```rust
use tokio::sync::mpsc;

/// 事件监听器
pub struct EventListener {
    pub contract_address: String,
    pub events: Vec<EventFilter>,
}

#[derive(Debug, Clone)]
pub struct EventFilter {
    pub event_name: String,
    pub topics: Vec<String>,
    pub from_block: u64,
    pub to_block: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLog {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub block_number: u64,
    pub transaction_hash: String,
    pub log_index: u64,
}

impl EventListener {
    /// 监听事件
    pub async fn listen(
        &self,
        connection: &Connection,
    ) -> Result<mpsc::Receiver<EventLog>, Web3Error> {
        let (tx, rx) = mpsc::channel(100);
        
        // 实际实现会使用WebSocket或轮询
        tokio::spawn(async move {
            // 模拟事件流
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                
                let event = EventLog {
                    address: "0x0".to_string(),
                    topics: vec!["0x0".to_string()],
                    data: "0x0".to_string(),
                    block_number: 1000000,
                    transaction_hash: "0x0".to_string(),
                    log_index: 0,
                };
                
                if tx.send(event).await.is_err() {
                    break;
                }
            }
        });
        
        Ok(rx)
    }
    
    /// 获取历史事件
    pub async fn get_past_events(
        &self,
        connection: &Connection,
        filter: &EventFilter,
    ) -> Result<Vec<EventLog>, Web3Error> {
        // 实际实现会调用eth_getLogs
        Ok(vec![])
    }
}

/// Transfer事件监听示例
pub async fn listen_transfer_events(
    token_address: &str,
    connection: &Connection,
) -> Result<(), Web3Error> {
    let listener = EventListener {
        contract_address: token_address.to_string(),
        events: vec![EventFilter {
            event_name: "Transfer".to_string(),
            topics: vec!["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string()],
            from_block: 0,
            to_block: None,
        }],
    };
    
    let mut rx = listener.listen(connection).await?;
    
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            println!("Transfer event: {:?}", event);
            // 解析事件数据
        }
    });
    
    Ok(())
}
```

### 3.3 交易管理

```rust
/// 交易构建器
pub struct TransactionBuilder {
    pub from: String,
    pub to: String,
    pub value: u128,
    pub data: Vec<u8>,
    pub gas_limit: Option<u64>,
    pub gas_price: Option<u64>,
    pub nonce: Option<u64>,
}

impl TransactionBuilder {
    pub fn new(from: String, to: String) -> Self {
        Self {
            from,
            to,
            value: 0,
            data: vec![],
            gas_limit: None,
            gas_price: None,
            nonce: None,
        }
    }
    
    pub fn value(mut self, value: u128) -> Self {
        self.value = value;
        self
    }
    
    pub fn data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }
    
    pub fn gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = Some(gas_limit);
        self
    }
    
    pub async fn send(self, connection: &Connection) -> Result<String, Web3Error> {
        // 构建并发送交易
        let tx_hash = format!("0x{}", "2".repeat(64));
        Ok(tx_hash)
    }
}

/// 交易监控
pub struct TransactionMonitor {
    pub tx_hash: String,
}

impl TransactionMonitor {
    /// 等待交易确认
    pub async fn wait_for_confirmation(
        &self,
        connection: &Connection,
        confirmations: u64,
    ) -> Result<TransactionReceipt, Web3Error> {
        // 轮询交易状态
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            
            // 检查交易是否被确认
            // 实际实现会调用eth_getTransactionReceipt
            
            return Ok(TransactionReceipt {
                transaction_hash: self.tx_hash.clone(),
                block_number: 1000000,
                status: true,
                gas_used: 21000,
            });
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransactionReceipt {
    pub transaction_hash: String,
    pub block_number: u64,
    pub status: bool,
    pub gas_used: u64,
}

/// 批量交易
pub struct BatchTransaction {
    pub transactions: Vec<TransactionBuilder>,
}

impl BatchTransaction {
    pub async fn send_all(
        self,
        connection: &Connection,
    ) -> Result<Vec<String>, Web3Error> {
        let mut tx_hashes = Vec::new();
        
        for tx in self.transactions {
            let hash = tx.send(connection).await?;
            tx_hashes.push(hash);
        }
        
        Ok(tx_hashes)
    }
}
```

## 4. 去中心化存储

### 4.1 IPFS

```rust
use reqwest;

/// IPFS客户端
pub struct IPFSClient {
    pub api_url: String,
    pub gateway_url: String,
}

impl IPFSClient {
    pub fn new(api_url: String, gateway_url: String) -> Self {
        Self {
            api_url,
            gateway_url,
        }
    }
    
    /// 添加文件
    pub async fn add_file(&self, content: Vec<u8>) -> Result<String, Web3Error> {
        // 实际实现会调用IPFS API
        let cid = "QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG".to_string();
        
        println!("Added file to IPFS: {}", cid);
        
        Ok(cid)
    }
    
    /// 获取文件
    pub async fn get_file(&self, cid: &str) -> Result<Vec<u8>, Web3Error> {
        let url = format!("{}/ipfs/{}", self.gateway_url, cid);
        
        // 实际实现会HTTP GET
        Ok(vec![])
    }
    
    /// 添加JSON数据
    pub async fn add_json<T: Serialize>(&self, data: &T) -> Result<String, Web3Error> {
        let json_bytes = serde_json::to_vec(data)
            .map_err(|e| Web3Error::ConnectionError(e.to_string()))?;
        
        self.add_file(json_bytes).await
    }
    
    /// 获取JSON数据
    pub async fn get_json<T: for<'de> Deserialize<'de>>(
        &self,
        cid: &str,
    ) -> Result<T, Web3Error> {
        let bytes = self.get_file(cid).await?;
        
        serde_json::from_slice(&bytes)
            .map_err(|e| Web3Error::ConnectionError(e.to_string()))
    }
    
    /// 固定文件
    pub async fn pin(&self, cid: &str) -> Result<(), Web3Error> {
        println!("Pinning CID: {}", cid);
        Ok(())
    }
    
    /// 取消固定
    pub async fn unpin(&self, cid: &str) -> Result<(), Web3Error> {
        println!("Unpinning CID: {}", cid);
        Ok(())
    }
}

/// NFT元数据示例
#[derive(Debug, Serialize, Deserialize)]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<NFTAttribute>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NFTAttribute {
    pub trait_type: String,
    pub value: String,
}

/// 上传NFT元数据到IPFS
pub async fn upload_nft_metadata(
    ipfs: &IPFSClient,
    metadata: NFTMetadata,
) -> Result<String, Web3Error> {
    // 先上传图片
    // let image_cid = ipfs.add_file(image_data).await?;
    
    // 更新元数据中的图片URL
    // metadata.image = format!("ipfs://{}", image_cid);
    
    // 上传元数据
    let metadata_cid = ipfs.add_json(&metadata).await?;
    
    Ok(metadata_cid)
}
```

### 4.2 Arweave

```rust
/// Arweave客户端
pub struct ArweaveClient {
    pub gateway_url: String,
}

impl ArweaveClient {
    pub fn new(gateway_url: String) -> Self {
        Self { gateway_url }
    }
    
    /// 上传数据(永久存储)
    pub async fn upload(
        &self,
        data: Vec<u8>,
        tags: Vec<(String, String)>,
    ) -> Result<String, Web3Error> {
        // 实际实现会创建Arweave交易
        let tx_id = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG".to_string();
        
        println!("Uploaded to Arweave: {}", tx_id);
        
        Ok(tx_id)
    }
    
    /// 获取数据
    pub async fn get(&self, tx_id: &str) -> Result<Vec<u8>, Web3Error> {
        let url = format!("{}/{}", self.gateway_url, tx_id);
        
        // 实际实现会HTTP GET
        Ok(vec![])
    }
    
    /// 查询交易
    pub async fn query_transactions(
        &self,
        tags: Vec<(String, String)>,
    ) -> Result<Vec<String>, Web3Error> {
        // 实际实现会使用GraphQL查询
        Ok(vec![])
    }
}

/// Arweave标签
pub struct ArweaveTags {
    pub tags: Vec<(String, String)>,
}

impl ArweaveTags {
    pub fn new() -> Self {
        Self { tags: vec![] }
    }
    
    pub fn add_tag(mut self, name: String, value: String) -> Self {
        self.tags.push((name, value));
        self
    }
    
    pub fn content_type(self, content_type: &str) -> Self {
        self.add_tag("Content-Type".to_string(), content_type.to_string())
    }
    
    pub fn app_name(self, app_name: &str) -> Self {
        self.add_tag("App-Name".to_string(), app_name.to_string())
    }
}
```

### 4.3 Filecoin

```rust
/// Filecoin存储交易
pub struct FilecoinClient {
    pub lotus_url: String,
}

impl FilecoinClient {
    /// 创建存储交易
    pub async fn create_storage_deal(
        &self,
        data_cid: &str,
        miner: &str,
        duration: u64,
        price: u64,
    ) -> Result<String, Web3Error> {
        println!("Creating storage deal with miner {} for {} epochs",
                 miner, duration);
        
        // 实际实现会调用Lotus API
        Ok("deal_id".to_string())
    }
    
    /// 查询交易状态
    pub async fn check_deal_status(&self, deal_id: &str) -> Result<DealStatus, Web3Error> {
        Ok(DealStatus::Active)
    }
}

#[derive(Debug, Clone)]
pub enum DealStatus {
    Pending,
    Active,
    Expired,
    Failed,
}
```

## 5. 去中心化身份(DID)

### 5.1 DID规范

```rust
/// DID文档
#[derive(Debug, Serialize, Deserialize)]
pub struct DIDDocument {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    
    pub id: String,  // did:method:identifier
    
    pub verification_method: Vec<VerificationMethod>,
    
    pub authentication: Vec<String>,
    
    pub assertion_method: Option<Vec<String>>,
    
    pub service: Option<Vec<Service>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    
    #[serde(rename = "type")]
    pub method_type: String,
    
    pub controller: String,
    
    pub public_key_base58: Option<String>,
    
    pub public_key_jwk: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    
    #[serde(rename = "type")]
    pub service_type: String,
    
    pub service_endpoint: String,
}

/// DID创建
pub fn create_did(method: &str, identifier: &str) -> String {
    format!("did:{}:{}", method, identifier)
}

/// 示例：以太坊地址的DID
pub fn eth_address_to_did(address: &str) -> String {
    create_did("ethr", address)
}
```

### 5.2 ENS域名系统

```rust
/// ENS解析器
pub struct ENSResolver {
    pub contract: ContractInstance,
}

impl ENSResolver {
    /// 解析域名到地址
    pub async fn resolve(&self, domain: &str) -> Result<String, Web3Error> {
        // 计算域名哈希
        let node = Self::namehash(domain);
        
        // 调用resolver合约
        self.contract
            .call_view_function("addr", vec![node])
            .await
    }
    
    /// 反向解析(地址到域名)
    pub async fn reverse_resolve(&self, address: &str) -> Result<String, Web3Error> {
        let reverse_domain = format!("{}.addr.reverse", 
                                    address.trim_start_matches("0x"));
        
        self.contract
            .call_view_function("name", vec![Self::namehash(&reverse_domain)])
            .await
    }
    
    /// 设置域名解析
    pub async fn set_address(
        &self,
        domain: &str,
        address: &str,
        from: &str,
    ) -> Result<String, Web3Error> {
        let node = Self::namehash(domain);
        
        self.contract
            .send_transaction(
                "setAddr",
                vec![node, address.to_string()],
                from,
                0,
            )
            .await
    }
    
    /// Namehash算法
    fn namehash(domain: &str) -> String {
        use sha3::{Digest, Keccak256};
        
        let mut node = [0u8; 32];
        
        if !domain.is_empty() {
            let labels: Vec<&str> = domain.split('.').rev().collect();
            
            for label in labels {
                let mut hasher = Keccak256::new();
                hasher.update(node);
                hasher.update(Keccak256::digest(label.as_bytes()));
                node = hasher.finalize().into();
            }
        }
        
        format!("0x{}", hex::encode(node))
    }
    
    /// 获取内容哈希
    pub async fn get_content_hash(&self, domain: &str) -> Result<String, Web3Error> {
        let node = Self::namehash(domain);
        
        self.contract
            .call_view_function("contenthash", vec![node])
            .await
    }
}
```

### 5.3 可验证凭证

```rust
/// 可验证凭证(Verifiable Credential)
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiableCredential {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    
    pub id: String,
    
    #[serde(rename = "type")]
    pub credential_type: Vec<String>,
    
    pub issuer: String,
    
    pub issuance_date: String,
    
    pub expiration_date: Option<String>,
    
    pub credential_subject: serde_json::Value,
    
    pub proof: Proof,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proof {
    #[serde(rename = "type")]
    pub proof_type: String,
    
    pub created: String,
    
    pub verification_method: String,
    
    pub proof_purpose: String,
    
    pub jws: String,
}

/// VC发行者
pub struct VCIssuer {
    pub did: String,
    pub private_key: Vec<u8>,
}

impl VCIssuer {
    /// 发行凭证
    pub fn issue_credential(
        &self,
        subject: serde_json::Value,
        credential_type: Vec<String>,
    ) -> Result<VerifiableCredential, Web3Error> {
        let now = chrono::Utc::now().to_rfc3339();
        
        let credential = VerifiableCredential {
            context: vec![
                "https://www.w3.org/2018/credentials/v1".to_string(),
            ],
            id: format!("urn:uuid:{}", uuid::Uuid::new_v4()),
            credential_type,
            issuer: self.did.clone(),
            issuance_date: now.clone(),
            expiration_date: None,
            credential_subject: subject,
            proof: Proof {
                proof_type: "EcdsaSecp256k1Signature2019".to_string(),
                created: now,
                verification_method: format!("{}#keys-1", self.did),
                proof_purpose: "assertionMethod".to_string(),
                jws: "...".to_string(), // 实际实现会签名
            },
        };
        
        Ok(credential)
    }
}

/// VC验证者
pub struct VCVerifier;

impl VCVerifier {
    /// 验证凭证
    pub fn verify_credential(
        &self,
        credential: &VerifiableCredential,
    ) -> Result<bool, Web3Error> {
        // 1. 验证签名
        // 2. 验证发行者
        // 3. 验证过期时间
        // 4. 验证撤销状态
        
        Ok(true)
    }
}
```

## 6. Web3通信协议

### 6.1 libp2p

```rust
/// libp2p节点(简化概念模型)
pub struct Libp2pNode {
    pub peer_id: String,
    pub listen_addresses: Vec<String>,
    pub peers: Vec<PeerInfo>,
}

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub peer_id: String,
    pub addresses: Vec<String>,
    pub protocols: Vec<String>,
}

impl Libp2pNode {
    /// 连接到peer
    pub async fn connect(&mut self, address: &str) -> Result<(), Web3Error> {
        println!("Connecting to peer: {}", address);
        Ok(())
    }
    
    /// 发布消息(PubSub)
    pub async fn publish(&self, topic: &str, message: &[u8]) -> Result<(), Web3Error> {
        println!("Publishing to topic {}: {} bytes", topic, message.len());
        Ok(())
    }
    
    /// 订阅主题
    pub async fn subscribe(&mut self, topic: &str) -> Result<(), Web3Error> {
        println!("Subscribed to topic: {}", topic);
        Ok(())
    }
}
```

### 6.2 Whisper

```rust
/// Whisper消息
pub struct WhisperMessage {
    pub payload: Vec<u8>,
    pub topic: [u8; 4],
    pub ttl: u64,
    pub pow: f64,
}

pub struct WhisperClient {
    pub connection: Connection,
}

impl WhisperClient {
    /// 发送消息
    pub async fn send_message(&self, message: WhisperMessage) -> Result<String, Web3Error> {
        println!("Sending Whisper message");
        Ok("message_hash".to_string())
    }
    
    /// 创建过滤器
    pub async fn new_filter(&self, topics: Vec<[u8; 4]>) -> Result<String, Web3Error> {
        Ok("filter_id".to_string())
    }
    
    /// 获取消息
    pub async fn get_messages(&self, filter_id: &str) -> Result<Vec<WhisperMessage>, Web3Error> {
        Ok(vec![])
    }
}
```

### 6.3 XMTP

```rust
/// XMTP客户端(去中心化消息协议)
pub struct XMTPClient {
    pub wallet: ConnectedWallet,
}

impl XMTPClient {
    /// 发送消息
    pub async fn send_message(
        &self,
        to: &str,
        content: &str,
    ) -> Result<(), Web3Error> {
        println!("Sending XMTP message to {}: {}", to, content);
        Ok(())
    }
    
    /// 接收消息
    pub async fn receive_messages(&self) -> Result<Vec<Message>, Web3Error> {
        Ok(vec![])
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub content: String,
    pub timestamp: u64,
}
```

## 7. 预言机服务

### 7.1 Chainlink

```rust
/// Chainlink价格聚合器
pub struct ChainlinkPriceFeed {
    pub aggregator_contract: ContractInstance,
}

impl ChainlinkPriceFeed {
    /// 获取最新价格
    pub async fn get_latest_price(&self) -> Result<i128, Web3Error> {
        let result = self.aggregator_contract
            .call_view_function("latestRoundData", vec![])
            .await?;
        
        // 解析返回值(roundId, answer, startedAt, updatedAt, answeredInRound)
        Ok(0)
    }
    
    /// 获取历史价格
    pub async fn get_historical_price(&self, round_id: u64) -> Result<i128, Web3Error> {
        self.aggregator_contract
            .call_view_function("getRoundData", vec![round_id.to_string()])
            .await?;
        
        Ok(0)
    }
}
```

### 7.2 Band Protocol

```rust
/// Band Protocol客户端
pub struct BandProtocolClient {
    pub contract: ContractInstance,
}

impl BandProtocolClient {
    /// 查询数据
    pub async fn get_reference_data(
        &self,
        base: &str,
        quote: &str,
    ) -> Result<(u64, u64), Web3Error> {
        // 返回(rate, lastUpdatedBase, lastUpdatedQuote)
        Ok((100, 0))
    }
}
```

### 7.3 自定义预言机

```rust
/// 自定义预言机
pub struct CustomOracle {
    pub contract: ContractInstance,
    pub api_url: String,
}

impl CustomOracle {
    /// 请求数据
    pub async fn request_data(
        &self,
        job_id: &str,
        params: serde_json::Value,
    ) -> Result<String, Web3Error> {
        // 发送请求到预言机节点
        let request_id = self.contract
            .send_transaction(
                "requestData",
                vec![job_id.to_string(), params.to_string()],
                "0x0",
                0,
            )
            .await?;
        
        Ok(request_id)
    }
    
    /// 查询结果
    pub async fn get_result(&self, request_id: &str) -> Result<Vec<u8>, Web3Error> {
        self.contract
            .call_view_function("getResult", vec![request_id.to_string()])
            .await?;
        
        Ok(vec![])
    }
}
```

## 8. Web3开发框架

### 8.1 Hardhat

```rust
/// Hardhat项目配置(概念模型)
#[derive(Debug, Serialize, Deserialize)]
pub struct HardhatConfig {
    pub solidity: SolidityConfig,
    pub networks: HashMap<String, NetworkConfig>,
    pub paths: PathsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolidityConfig {
    pub version: String,
    pub settings: CompilerSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerSettings {
    pub optimizer: OptimizerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizerConfig {
    pub enabled: bool,
    pub runs: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub url: String,
    pub chain_id: u64,
    pub accounts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathsConfig {
    pub sources: String,
    pub tests: String,
    pub artifacts: String,
}
```

### 8.2 Foundry

```rust
/// Foundry配置
#[derive(Debug, Serialize, Deserialize)]
pub struct FoundryConfig {
    pub profile: ProfileConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub src: String,
    pub out: String,
    pub libs: Vec<String>,
    pub solc_version: String,
    pub optimizer: bool,
    pub optimizer_runs: u32,
}
```

### 8.3 Truffle

```rust
/// Truffle配置
#[derive(Debug, Serialize, Deserialize)]
pub struct TruffleConfig {
    pub networks: HashMap<String, TruffleNetworkConfig>,
    pub compilers: CompilersConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TruffleNetworkConfig {
    pub host: String,
    pub port: u16,
    pub network_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilersConfig {
    pub solc: SolcConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolcConfig {
    pub version: String,
}
```

## 9. Web3 UI/UX

### 9.1 Web3设计模式

```rust
/// Web3 UI组件模式
pub enum Web3UIPattern {
    /// 钱包连接按钮
    WalletConnectButton {
        label: String,
        supported_wallets: Vec<WalletType>,
    },
    
    /// 网络切换器
    NetworkSwitcher {
        networks: Vec<NetworkInfo>,
        current_network: u64,
    },
    
    /// 交易确认弹窗
    TransactionModal {
        transaction: Transaction,
        status: TransactionStatus,
    },
    
    /// 余额显示
    BalanceDisplay {
        token_symbol: String,
        balance: u128,
        usd_value: Option<f64>,
    },
    
    /// Gas费用估算
    GasEstimator {
        estimated_gas: u64,
        gas_price: u64,
        total_cost: u128,
    },
}

#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub name: String,
    pub chain_id: u64,
    pub rpc_url: String,
    pub explorer_url: String,
    pub currency_symbol: String,
}

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    Pending,
    Confirmed(u64),  // confirmations
    Failed(String),
}
```

### 9.2 用户体验优化

```rust
/// 交易状态管理
pub struct TransactionStateManager {
    pub pending_txs: Vec<PendingTransaction>,
}

#[derive(Debug, Clone)]
pub struct PendingTransaction {
    pub hash: String,
    pub description: String,
    pub submitted_at: u64,
    pub status: TransactionStatus,
}

impl TransactionStateManager {
    /// 添加待处理交易
    pub fn add_transaction(&mut self, tx: PendingTransaction) {
        self.pending_txs.push(tx);
    }
    
    /// 更新交易状态
    pub fn update_status(&mut self, tx_hash: &str, status: TransactionStatus) {
        if let Some(tx) = self.pending_txs.iter_mut().find(|t| t.hash == tx_hash) {
            tx.status = status;
        }
    }
    
    /// 移除已完成交易
    pub fn remove_completed(&mut self) {
        self.pending_txs.retain(|tx| {
            !matches!(tx.status, TransactionStatus::Confirmed(_))
        });
    }
}

/// 错误提示
pub struct ErrorHandler;

impl ErrorHandler {
    pub fn user_friendly_error(error: &Web3Error) -> String {
        match error {
            Web3Error::ConnectionError(msg) => {
                format!("连接错误: {}。请检查网络连接或切换RPC节点。", msg)
            }
            Web3Error::TransactionError(msg) => {
                if msg.contains("insufficient funds") {
                    "余额不足，请确保账户有足够的资金。".to_string()
                } else if msg.contains("gas") {
                    "Gas费用不足或设置过低，请增加Gas限制。".to_string()
                } else {
                    format!("交易失败: {}", msg)
                }
            }
            Web3Error::ContractError(msg) => {
                format!("智能合约错误: {}。请检查合约调用参数。", msg)
            }
        }
    }
}
```

### 9.3 组件库

```rust
/// Web3 React组件(概念模型)
pub enum Web3Component {
    ConnectButton {
        on_connect: fn(ConnectedWallet),
    },
    
    NetworkSelector {
        networks: Vec<NetworkInfo>,
        on_switch: fn(u64),
    },
    
    TokenBalance {
        token_address: String,
        user_address: String,
    },
    
    TransactionList {
        transactions: Vec<PendingTransaction>,
    },
    
    NFTGallery {
        nfts: Vec<NFTItem>,
        on_select: fn(NFTItem),
    },
}

#[derive(Debug, Clone)]
pub struct NFTItem {
    pub token_id: String,
    pub name: String,
    pub image_url: String,
    pub metadata: NFTMetadata,
}
```

## 10. Web3安全

### 10.1 前端安全

```rust
/// 输入验证
pub struct InputValidator;

impl InputValidator {
    /// 验证以太坊地址
    pub fn validate_eth_address(address: &str) -> bool {
        if !address.starts_with("0x") {
            return false;
        }
        
        if address.len() != 42 {
            return false;
        }
        
        // 验证校验和(实际实现需要完整的校验和验证)
        true
    }
    
    /// 验证金额
    pub fn validate_amount(amount: &str, decimals: u8) -> Result<u128, String> {
        let parts: Vec<&str> = amount.split('.').collect();
        
        if parts.len() > 2 {
            return Err("Invalid amount format".to_string());
        }
        
        let integer_part: u128 = parts[0].parse()
            .map_err(|_| "Invalid integer part".to_string())?;
        
        let fractional_part: u128 = if parts.len() == 2 {
            if parts[1].len() > decimals as usize {
                return Err("Too many decimal places".to_string());
            }
            
            let padded = format!("{:0<width$}", parts[1], width = decimals as usize);
            padded.parse().map_err(|_| "Invalid fractional part".to_string())?
        } else {
            0
        };
        
        let total = integer_part * 10u128.pow(decimals as u32) + fractional_part;
        
        Ok(total)
    }
}
```

### 10.2 交易签名验证

```rust
/// 签名验证
pub struct SignatureVerifier;

impl SignatureVerifier {
    /// 验证消息签名
    pub fn verify_signature(
        message: &str,
        signature: &str,
        expected_signer: &str,
    ) -> Result<bool, Web3Error> {
        // 实际实现会使用ethers库进行签名恢复
        // 1. 对消息进行以太坊前缀处理
        // 2. 从签名恢复公钥
        // 3. 从公钥派生地址
        // 4. 比较地址
        
        Ok(true)
    }
    
    /// EIP-712结构化数据签名
    pub fn verify_typed_data(
        domain: serde_json::Value,
        types: serde_json::Value,
        value: serde_json::Value,
        signature: &str,
    ) -> Result<bool, Web3Error> {
        // 实际实现会构建EIP-712哈希并验证
        Ok(true)
    }
}
```

### 10.3 钓鱼防护

```rust
/// 钓鱼检测
pub struct PhishingDetector {
    pub known_scams: Vec<String>,
    pub verified_domains: Vec<String>,
}

impl PhishingDetector {
    /// 检查URL是否安全
    pub fn check_url(&self, url: &str) -> SecurityLevel {
        if self.verified_domains.iter().any(|d| url.contains(d)) {
            return SecurityLevel::Safe;
        }
        
        if self.known_scams.iter().any(|s| url.contains(s)) {
            return SecurityLevel::Dangerous;
        }
        
        SecurityLevel::Unknown
    }
    
    /// 检查合约地址
    pub fn check_contract(&self, address: &str) -> SecurityLevel {
        // 实际实现会查询合约验证数据库
        SecurityLevel::Unknown
    }
    
    /// 分析交易风险
    pub fn analyze_transaction(&self, tx: &Transaction) -> Vec<SecurityWarning> {
        let mut warnings = Vec::new();
        
        // 检查大额转账
        if tx.value > 1000000000000000000 {  // > 1 ETH
            warnings.push(SecurityWarning::LargeTransfer);
        }
        
        // 检查未知合约
        if !tx.data.is_empty() {
            warnings.push(SecurityWarning::SmartContractInteraction);
        }
        
        warnings
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SecurityLevel {
    Safe,
    Unknown,
    Suspicious,
    Dangerous,
}

#[derive(Debug, Clone)]
pub enum SecurityWarning {
    LargeTransfer,
    SmartContractInteraction,
    UnverifiedContract,
    HighGasPrice,
    TokenApproval,
}
```

## 11. 实战项目：构建Web3 DApp

### 11.1 项目初始化

```rust
/// DApp项目结构
pub struct DAppProject {
    pub name: String,
    pub frontend: FrontendConfig,
    pub contracts: ContractsConfig,
    pub deployment: DeploymentConfig,
}

pub struct FrontendConfig {
    pub framework: String,  // React, Vue, etc.
    pub web3_library: String,  // ethers.js, web3.js
    pub ui_library: Option<String>,  // RainbowKit, Web3Modal
}

pub struct ContractsConfig {
    pub language: String,  // Solidity, Vyper
    pub compiler_version: String,
    pub test_framework: String,  // Hardhat, Foundry
}

pub struct DeploymentConfig {
    pub networks: Vec<String>,
    pub ipfs_gateway: Option<String>,
    pub ens_domain: Option<String>,
}
```

### 11.2 智能合约开发

```solidity
// SimpleToken.sol - 示例ERC-20代币合约
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SimpleToken {
    string public name = "Simple Token";
    string public symbol = "SIM";
    uint8 public decimals = 18;
    uint256 public totalSupply;
    
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    
    constructor(uint256 initialSupply) {
        totalSupply = initialSupply * 10 ** uint256(decimals);
        balanceOf[msg.sender] = totalSupply;
    }
    
    function transfer(address to, uint256 value) public returns (bool) {
        require(balanceOf[msg.sender] >= value, "Insufficient balance");
        balanceOf[msg.sender] -= value;
        balanceOf[to] += value;
        emit Transfer(msg.sender, to, value);
        return true;
    }
    
    function approve(address spender, uint256 value) public returns (bool) {
        allowance[msg.sender][spender] = value;
        emit Approval(msg.sender, spender, value);
        return true;
    }
    
    function transferFrom(address from, address to, uint256 value) public returns (bool) {
        require(balanceOf[from] >= value, "Insufficient balance");
        require(allowance[from][msg.sender] >= value, "Insufficient allowance");
        
        balanceOf[from] -= value;
        balanceOf[to] += value;
        allowance[from][msg.sender] -= value;
        
        emit Transfer(from, to, value);
        return true;
    }
}
```

### 11.3 前端集成

```typescript
// 前端集成示例(TypeScript)
/*
import { ethers } from 'ethers';
import SimpleTokenABI from './SimpleToken.json';

class TokenDApp {
    provider: ethers.providers.Web3Provider;
    signer: ethers.Signer;
    contract: ethers.Contract;
    
    async connect() {
        // 连接MetaMask
        this.provider = new ethers.providers.Web3Provider(window.ethereum);
        await this.provider.send("eth_requestAccounts", []);
        this.signer = this.provider.getSigner();
        
        // 连接合约
        const contractAddress = "0x...";
        this.contract = new ethers.Contract(
            contractAddress,
            SimpleTokenABI,
            this.signer
        );
    }
    
    async getBalance(address: string): Promise<string> {
        const balance = await this.contract.balanceOf(address);
        return ethers.utils.formatEther(balance);
    }
    
    async transfer(to: string, amount: string): Promise<ethers.ContractTransaction> {
        const tx = await this.contract.transfer(
            to,
            ethers.utils.parseEther(amount)
        );
        await tx.wait();
        return tx;
    }
}
*/
```

## 12. Web3未来展望

```rust
/// Web3未来趋势
#[derive(Debug)]
pub enum Web3FutureTrend {
    /// Account Abstraction (账户抽象)
    AccountAbstraction {
        social_recovery: bool,
        gas_sponsorship: bool,
        batch_transactions: bool,
    },
    
    /// Layer 2扩容
    Layer2Scaling {
        optimistic_rollups: bool,
        zk_rollups: bool,
        validium: bool,
    },
    
    /// 链抽象
    ChainAbstraction {
        cross_chain_messaging: bool,
        unified_liquidity: bool,
        chain_agnostic_apps: bool,
    },
    
    /// AI + Web3
    AIWeb3 {
        ai_agents: bool,
        personalized_defi: bool,
        automated_governance: bool,
    },
    
    /// 隐私增强
    PrivacyEnhancement {
        zero_knowledge_proofs: bool,
        fully_homomorphic_encryption: bool,
        confidential_smart_contracts: bool,
    },
    
    /// 物联网集成
    IoTIntegration {
        device_identity: bool,
        micro_payments: bool,
        supply_chain_tracking: bool,
    },
}

/// Web3发展方向
pub struct Web3Roadmap {
    pub scalability: ScalabilityGoals,
    pub usability: UsabilityGoals,
    pub interoperability: InteroperabilityGoals,
    pub sustainability: SustainabilityGoals,
}

#[derive(Debug)]
pub struct ScalabilityGoals {
    pub target_tps: u64,
    pub target_latency_ms: u64,
    pub target_cost_per_tx: f64,
}

#[derive(Debug)]
pub struct UsabilityGoals {
    pub onboarding_time_minutes: u32,
    pub mobile_first: bool,
    pub social_login: bool,
}

#[derive(Debug)]
pub struct InteroperabilityGoals {
    pub cross_chain_assets: bool,
    pub unified_identity: bool,
    pub composable_protocols: bool,
}

#[derive(Debug)]
pub struct SustainabilityGoals {
    pub carbon_neutral: bool,
    pub energy_efficient_consensus: bool,
    pub green_mining: bool,
}
```

## 总结

本文档全面介绍了Web3技术栈：

1. **Web3概述**: 演进历史、核心特征、技术架构
2. **前端技术**: Web3.js/Ethers.js、钱包连接、去中心化前端
3. **智能合约交互**: 合约调用、事件监听、交易管理
4. **去中心化存储**: IPFS、Arweave、Filecoin
5. **去中心化身份**: DID、ENS、可验证凭证
6. **Web3通信**: libp2p、Whisper、XMTP
7. **预言机**: Chainlink、Band Protocol、自定义预言机
8. **开发框架**: Hardhat、Foundry、Truffle
9. **UI/UX**: 设计模式、用户体验、组件库
10. **安全**: 前端安全、签名验证、钓鱼防护
11. **实战**: 完整DApp开发流程
12. **未来**: Account Abstraction、Layer2、链抽象、AI、隐私

---

**文档版本**: v1.0.0  
**最后更新**: 2025年10月17日  
**作者**: Web3技术专家  
**审核**: 全栈区块链工程师

## 相关文档

- [DeFi应用指南](./22_DEFI_APPLICATIONS.md)
- [NFT生态系统](./23_NFT_ECOSYSTEM.md)
- [智能合约开发](./21_SMART_CONTRACT_DEVELOPMENT.md)
- [安全最佳实践](./19_SECURITY_BEST_PRACTICES.md)

## 参考资料

- Ethereum Documentation
- Web3.js Documentation
- Ethers.js Documentation
- IPFS Documentation
- W3C DID Specification

## 实用工具

```rust
// 辅助依赖
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono;
use sha3::{Digest, Keccak256};
use hex;

// 类型别名
pub type Address = String;
pub type Bytes = Vec<u8>;
pub type Hash = String;
```
