//! # 区块链应用主程序
//!
//! 展示 Rust 1.89 特性在区块链开发中的应用
//! Demonstrates the application of Rust 1.89 features in blockchain development

mod cryptography;
mod network;
mod simple_blockchain;
mod smart_contract;
mod tools;
mod types;
mod monitoring;
mod cli;
mod consensus;
#[cfg(feature = "advanced")]
mod performance;
#[cfg(feature = "advanced")]
mod security;

// 新增的高级模块
#[cfg(feature = "crypto-advanced")]
mod advanced_cryptography_simple;

#[cfg(feature = "smart-contracts")]
mod smart_contract_engine;

#[cfg(feature = "p2p")]
mod p2p_network;

#[cfg(feature = "database")]
mod database;

mod web_api;

use simple_blockchain::*;
use std::io::{self, Write};

fn main() {
    println!("🚀 区块链应用演示 - Rust 最新特性展示");
    println!("🚀 Blockchain Application Demo - Latest Rust Features Showcase");
    println!();

    // 演示 Rust 特性
    demonstrate_rust_features();

    // 演示高级功能
    demonstrate_advanced_features();

    // 交互式区块链演示
    interactive_blockchain_demo();
}

/// 演示 Rust 特性
/// Demonstrate Rust features
fn demonstrate_rust_features() {
    println!("📋 Rust 1.89 特性演示");
    println!("📋 Rust 1.89 Features Demo");
    println!();

    // 1. 常量泛型推断
    println!("1️⃣ 常量泛型推断 (Const Generic Inference)");
    let data = b"Hello, Blockchain!";
    let hash: BlockHash<32> = BlockHash::<32>::from_data(data);
    println!("   数据: {:?}", String::from_utf8_lossy(data));
    println!("   哈希: {}", hash.to_string());
    println!();

    // 2. 生命周期语法检查
    println!("2️⃣ 生命周期语法检查 (Lifetime Syntax Check)");
    let _blockchain = Blockchain::new(2);
    let transaction = Transaction::new("alice".to_string(), "bob".to_string(), 100);
    let validation_result = transaction.validate();
    println!("   交易验证结果: {:?}", validation_result.is_valid);
    if !validation_result.errors.is_empty() {
        println!("   错误: {:?}", validation_result.errors);
    }
    println!();

    // 3. Result::flatten 简化错误处理
    println!("3️⃣ Result::flatten 简化错误处理 (Result::flatten Error Handling)");
    let mut blockchain = Blockchain::new(1); // 低难度用于演示
    let transaction = Transaction::new("genesis".to_string(), "alice".to_string(), 100);

    match blockchain.add_transaction(transaction) {
        Ok(_) => println!("   ✅ 交易添加成功"),
        Err(e) => println!("   ❌ 交易添加失败: {}", e),
    }
    println!();

    // 4. 挖矿演示
    println!("4️⃣ 挖矿演示 (Mining Demo)");
    println!("   开始挖矿...");
    match blockchain.mine_pending_transactions("miner".to_string()) {
        Ok(_) => {
            println!("   ✅ 挖矿成功！");
            println!("   链长度: {}", blockchain.get_chain_length());
            println!("   Alice 余额: {}", blockchain.get_balance("alice"));
            println!("   Miner 余额: {}", blockchain.get_balance("miner"));
        }
        Err(e) => println!("   ❌ 挖矿失败: {}", e),
    }
    println!();

    // 5. 链验证
    println!("5️⃣ 链验证 (Chain Validation)");
    let is_valid = blockchain.is_valid_chain();
    println!(
        "   链是否有效: {}",
        if is_valid { "✅ 是" } else { "❌ 否" }
    );
    println!();
}

/// 交互式区块链演示
/// Interactive blockchain demo
fn interactive_blockchain_demo() {
    println!("🎮 交互式区块链演示");
    println!("🎮 Interactive Blockchain Demo");
    println!();

    let mut blockchain = Blockchain::new(2);

    loop {
        println!("请选择操作 (Please select an operation):");
        println!("1. 添加交易 (Add Transaction)");
        println!("2. 挖矿 (Mining)");
        println!("3. 查看余额 (Check Balance)");
        println!("4. 查看链信息 (View Chain Info)");
        println!("5. 退出 (Exit)");
        print!("请输入选择 (1-5): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => add_transaction_interactive(&mut blockchain),
            "2" => mine_block_interactive(&mut blockchain),
            "3" => check_balance_interactive(&blockchain),
            "4" => view_chain_info(&blockchain),
            "5" => {
                println!("👋 再见！(Goodbye!)");
                break;
            }
            _ => println!("❌ 无效选择，请重新输入 (Invalid choice, please try again)"),
        }

        println!();
    }
}

/// 交互式添加交易
/// Interactive add transaction
fn add_transaction_interactive(blockchain: &mut Blockchain) {
    print!("请输入发送者地址: ");
    io::stdout().flush().unwrap();
    let mut sender = String::new();
    io::stdin().read_line(&mut sender).unwrap();
    let sender = sender.trim().to_string();

    print!("请输入接收者地址: ");
    io::stdout().flush().unwrap();
    let mut receiver = String::new();
    io::stdin().read_line(&mut receiver).unwrap();
    let receiver = receiver.trim().to_string();

    print!("请输入金额: ");
    io::stdout().flush().unwrap();
    let mut amount_str = String::new();
    io::stdin().read_line(&mut amount_str).unwrap();

    match amount_str.trim().parse::<u64>() {
        Ok(amount) => {
            let transaction = Transaction::new(sender, receiver, amount);
            match blockchain.add_transaction(transaction) {
                Ok(_) => println!("✅ 交易添加成功"),
                Err(e) => println!("❌ 交易添加失败: {}", e),
            }
        }
        Err(_) => println!("❌ 无效金额"),
    }
}

/// 交互式挖矿
/// Interactive mining
fn mine_block_interactive(blockchain: &mut Blockchain) {
    print!("请输入挖矿奖励地址: ");
    io::stdout().flush().unwrap();
    let mut reward_address = String::new();
    io::stdin().read_line(&mut reward_address).unwrap();
    let reward_address = reward_address.trim();

    println!("⛏️ 开始挖矿...");
    match blockchain.mine_pending_transactions(reward_address.to_string()) {
        Ok(_) => {
            println!("✅ 挖矿成功！");
            println!("   链长度: {}", blockchain.get_chain_length());
        }
        Err(e) => println!("❌ 挖矿失败: {}", e),
    }
}

/// 交互式查看余额
/// Interactive check balance
fn check_balance_interactive(blockchain: &Blockchain) {
    print!("请输入地址: ");
    io::stdout().flush().unwrap();
    let mut address = String::new();
    io::stdin().read_line(&mut address).unwrap();
    let address = address.trim();

    let balance = blockchain.get_balance(address);
    println!("💰 {} 的余额: {}", address, balance);
}

/// 查看链信息
/// View chain info
fn view_chain_info(blockchain: &Blockchain) {
    println!("📊 链信息 (Chain Info):");
    println!("   链长度: {}", blockchain.get_chain_length());
    println!("   难度: {}", blockchain.difficulty);
    println!("   待处理交易数: {}", blockchain.pending_transactions.len());

    if let Some(latest_block) = blockchain.get_latest_block() {
        println!("   最新区块索引: {}", latest_block.index);
        println!("   最新区块哈希: {}", latest_block.hash.to_string());
        println!("   最新区块交易数: {}", latest_block.transactions.len());
    }

    println!(
        "   链是否有效: {}",
        if blockchain.is_valid_chain() {
            "✅ 是"
        } else {
            "❌ 否"
        }
    );
}

/// 演示高级功能
/// Demonstrate advanced features
fn demonstrate_advanced_features() {
    println!("🔬 高级功能演示");
    println!("🔬 Advanced Features Demo");
    println!();

    // 演示高级密码学功能
    #[cfg(feature = "crypto-advanced")]
    {
        println!("1️⃣ 高级密码学功能 (Advanced Cryptography)");
        demonstrate_advanced_cryptography();
    }

    // 演示智能合约功能
    #[cfg(feature = "smart-contracts")]
    {
        println!("2️⃣ 智能合约引擎 (Smart Contract Engine)");
        demonstrate_smart_contracts();
    }

    // 演示 P2P 网络功能
    #[cfg(feature = "p2p")]
    {
        println!("3️⃣ P2P 网络功能 (P2P Network)");
        demonstrate_p2p_network();
    }

    println!();
}

/// 演示高级密码学功能
#[cfg(feature = "crypto-advanced")]
fn demonstrate_advanced_cryptography() {
    use advanced_cryptography_simple::*;

    // 演示多种哈希算法
    let data = b"Hello, Advanced Blockchain!";
    
    let sha256_hash = AdvancedHash::hash(data, HashAlgorithm::Sha256).unwrap();
    let sha512_hash = AdvancedHash::hash(data, HashAlgorithm::Sha512).unwrap();
    
    println!("   SHA256: {}", sha256_hash.to_hex());
    println!("   SHA512: {}", sha512_hash.to_hex());

    // 演示密钥生成和签名
    let secp_key_pair = AdvancedKeyPair::generate("secp256k1").unwrap();
    let ed_key_pair = AdvancedKeyPair::generate("ed25519").unwrap();
    
    let message = b"Sign this message";
    
    let secp_signature = AdvancedSignature::sign(message, &secp_key_pair).unwrap();
    let ed_signature = AdvancedSignature::sign(message, &ed_key_pair).unwrap();
    
    println!("   Secp256k1 签名验证: {}", secp_signature.verify(message).unwrap());
    println!("   Ed25519 签名验证: {}", ed_signature.verify(message).unwrap());

    // 演示地址生成
    let bitcoin_addr = AddressGenerator::generate_bitcoin_address(&secp_key_pair).unwrap();
    let ethereum_addr = AddressGenerator::generate_ethereum_address(&secp_key_pair).unwrap();
    
    println!("   比特币地址: {}", bitcoin_addr);
    println!("   以太坊地址: {}", ethereum_addr);

    // 演示 Merkle 树
    let merkle_data = vec![
        b"data1".to_vec(),
        b"data2".to_vec(),
        b"data3".to_vec(),
        b"data4".to_vec(),
    ];
    
    let merkle_tree = MerkleTree::new(merkle_data, HashAlgorithm::Sha256).unwrap();
    let proof = merkle_tree.generate_proof(0).unwrap();
    let leaf = &merkle_tree.leaves[0];
    let is_valid = merkle_tree.verify_proof(leaf, &proof, 0).unwrap();
    
    println!("   Merkle 树根: {}", merkle_tree.root.as_ref().unwrap().to_hex());
    println!("   Merkle 证明验证: {}", is_valid);
    println!();
}

/// 演示智能合约功能
#[cfg(feature = "smart-contracts")]
fn demonstrate_smart_contracts() {
    use smart_contract_engine::*;

    let mut engine = SmartContractEngine::new();
    
    let interface = ContractInterface {
        name: "DemoContract".to_string(),
        methods: vec![
            ContractMethod {
                name: "get_balance".to_string(),
                inputs: vec![],
                outputs: vec![ContractParameter {
                    name: "balance".to_string(),
                    param_type: "u64".to_string(),
                }],
                payable: false,
                constant: true,
            },
            ContractMethod {
                name: "transfer".to_string(),
                inputs: vec![
                    ContractParameter {
                        name: "to".to_string(),
                        param_type: "address".to_string(),
                    },
                    ContractParameter {
                        name: "amount".to_string(),
                        param_type: "u64".to_string(),
                    },
                ],
                outputs: vec![ContractParameter {
                    name: "success".to_string(),
                    param_type: "bool".to_string(),
                }],
                payable: false,
                constant: false,
            },
        ],
        events: vec![],
    };

    // 注意：这里使用空的 WASM 代码，实际应用中需要有效的 WASM 字节码
    let wasm_code = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]; // 最小 WASM 模块
    
    match engine.deploy_contract(wasm_code, "alice".to_string(), interface, 1000) {
        Ok(address) => {
            println!("   ✅ 智能合约部署成功: {}", address);
            
            let context = ExecutionContext {
                caller: "alice".to_string(),
                value: 0,
                gas_limit: 10000,
                gas_used: 0,
                block_height: 1,
                timestamp: 1234567890,
                contract_address: address.clone(),
            };
            
            // 尝试调用合约方法
            let result = engine.call_contract(&address, "get_balance", &[], context);
            match result {
                Ok(exec_result) => {
                    println!("   📊 合约执行结果: 成功={}, Gas消耗={}", 
                        exec_result.success, exec_result.gas_used);
                }
                Err(e) => {
                    println!("   ⚠️ 合约执行失败 (预期): {}", e);
                }
            }
        }
        Err(e) => {
            println!("   ⚠️ 智能合约部署失败 (预期): {}", e);
        }
    }
    
    println!();
}

/// 演示 P2P 网络功能
#[cfg(feature = "p2p")]
fn demonstrate_p2p_network() {
    use p2p_network::*;

    let message_handler = Arc::new(DefaultMessageHandler);
    
    let node = P2PNode::new(
        "demo_node".to_string(),
        "1.0.0".to_string(),
        vec!["blockchain".to_string(), "smart_contracts".to_string()],
        message_handler,
    );
    
    // 演示消息创建
    let handshake_msg = NetworkMessage::new(MessageType::Handshake(HandshakeMessage {
        version: "1.0.0".to_string(),
        node_id: "demo_node".to_string(),
        capabilities: vec!["blockchain".to_string()],
        timestamp: 1234567890,
    }));
    
    let serialized = handshake_msg.serialize().unwrap();
    let deserialized = NetworkMessage::deserialize(&serialized).unwrap();
    
    println!("   📨 消息序列化/反序列化测试: {}", 
        handshake_msg.id == deserialized.id);
    println!("   🔗 节点ID: {}", node.node_id);
    println!("   📋 支持的功能: {:?}", node.capabilities);
    println!("   🌐 对等节点数量: {}", 
        tokio::runtime::Handle::current().block_on(node.get_connected_peer_count()));
    
    println!();
}
