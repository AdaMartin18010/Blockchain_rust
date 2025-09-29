//! 完整的区块链演示
//! 
//! 这个示例展示了如何使用区块链库的各个组件来构建一个完整的区块链系统

use blockchain::core::{Block, Transaction, State, MerkleTree};
use blockchain::components::{
    cryptography::{HashEngine, SignatureEngine, EncryptionEngine},
    network::{P2PNetwork, MessageRouter, PeerManager},
    storage::{BlockStorage, TransactionStorage, StateStorage, MerkleStorage},
    consensus::{ProofOfWork, ProofOfStake, DelegatedProofOfStake, PBFT}
};
use blockchain::smart_contracts::{VirtualMachine, Compiler, Runtime};
use blockchain::algorithms::{ConsensusAlgorithms, CryptographicAlgorithms, OptimizationAlgorithms};
use blockchain::layers::application::ApplicationLayer;
use blockchain::monitoring::Monitor;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 区块链完整演示开始");
    println!("=" .repeat(50));
    
    // 1. 初始化密码学组件
    println!("\n📊 1. 初始化密码学组件");
    let hash_engine = HashEngine::new();
    let signature_engine = SignatureEngine::new();
    let encryption_engine = EncryptionEngine::new();
    
    // 演示哈希操作
    let test_data = b"Hello, Blockchain!";
    let sha256_hash = hash_engine.sha256(test_data);
    let blake2b_hash = hash_engine.blake2b(test_data);
    println!("   SHA256: {}", hex::encode(sha256_hash));
    println!("   Blake2b: {}", hex::encode(&blake2b_hash[..32]));
    
    // 演示签名操作
    let (private_key, public_key) = signature_engine.generate_keypair("ed25519")?;
    let signature = signature_engine.sign(test_data, &private_key, "ed25519")?;
    let is_valid = signature_engine.verify(test_data, &signature, &public_key, "ed25519")?;
    println!("   签名验证: {}", if is_valid { "✅ 成功" } else { "❌ 失败" });
    
    // 2. 创建存储组件
    println!("\n💾 2. 初始化存储组件");
    let mut block_storage = BlockStorage::new();
    let mut transaction_storage = TransactionStorage::new();
    let mut state_storage = StateStorage::new();
    let mut merkle_storage = MerkleStorage::new();
    
    // 初始化存储
    block_storage.initialize().await?;
    transaction_storage.initialize().await?;
    state_storage.initialize().await?;
    merkle_storage.initialize().await?;
    
    println!("   所有存储组件初始化完成 ✅");
    
    // 3. 创建创世区块
    println!("\n🏗️  3. 创建创世区块");
    let genesis_transaction = Transaction::new(vec![], vec![], 0);
    let genesis_block = Block::new(0, [0u8; 32], vec![genesis_transaction], get_current_timestamp());
    
    // 存储创世区块
    block_storage.store_block(0, genesis_block.clone()).await?;
    println!("   创世区块创建并存储完成 ✅");
    println!("   区块哈希: {}", hex::encode(genesis_block.hash()));
    
    // 4. 演示Merkle树
    println!("\n🌳 4. 演示Merkle树操作");
    let merkle_data = vec![
        [1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32]
    ];
    let merkle_tree = MerkleTree::new(merkle_data)?;
    let merkle_root = merkle_tree.root();
    println!("   Merkle根: {}", hex::encode(merkle_root));
    
    // 生成和验证证明
    let proof = merkle_tree.generate_proof(0)?;
    let is_proof_valid = MerkleTree::verify_proof(&proof);
    println!("   证明验证: {}", if is_proof_valid { "✅ 成功" } else { "❌ 失败" });
    
    // 存储Merkle树
    merkle_storage.store_merkle_tree("demo_tree".to_string(), merkle_tree).await?;
    
    // 5. 演示共识算法
    println!("\n⚡ 5. 演示共识算法");
    
    // PoW
    let pow = ProofOfWork::new(4);
    println!("   PoW难度: {}", pow.difficulty);
    
    // PoS
    let mut pos = ProofOfStake::new(1000);
    pos.add_validator("validator1".to_string(), 5000);
    pos.add_validator("validator2".to_string(), 3000);
    println!("   PoS验证者数量: {}", pos.validators.len());
    
    // DPoS
    let mut dpos = DelegatedProofOfStake::new(21);
    dpos.vote_for_delegate("delegate1".to_string(), 10000);
    dpos.vote_for_delegate("delegate2".to_string(), 8000);
    let top_delegates = dpos.get_top_delegates();
    println!("   DPoS顶级委托者: {}", top_delegates.len());
    
    // PBFT
    let pbft = PBFT::new(vec![
        "node1".to_string(),
        "node2".to_string(),
        "node3".to_string(),
        "node4".to_string()
    ]);
    println!("   PBFT验证者数量: {}", pbft.validators.len());
    
    // 6. 演示智能合约
    println!("\n🤖 6. 演示智能合约");
    let mut vm = VirtualMachine::new();
    let compiler = Compiler::new();
    let mut runtime = Runtime::new();
    
    let source_code = "contract HelloWorld { function greet() returns string { return 'Hello, World!'; } }";
    let bytecode = compiler.compile(source_code).await?;
    println!("   智能合约编译完成，字节码长度: {} bytes", bytecode.len());
    
    let contract_address = vm.deploy(&bytecode).await?;
    println!("   合约部署地址: {}", contract_address);
    
    // 7. 演示网络组件
    println!("\n🌐 7. 演示网络组件");
    let mut p2p_network = P2PNetwork::new();
    let mut message_router = MessageRouter::new();
    let mut peer_manager = PeerManager::new();
    
    // 模拟添加节点
    peer_manager.add_peer("127.0.0.1:8080".to_string()).await?;
    peer_manager.add_peer("127.0.0.1:8081".to_string()).await?;
    let active_peers = peer_manager.get_active_peers().await;
    println!("   活跃节点数量: {}", active_peers.len());
    
    // 8. 演示算法模块
    println!("\n🧮 8. 演示算法模块");
    let consensus_algorithms = ConsensusAlgorithms::new();
    let crypto_algorithms = CryptographicAlgorithms::new();
    let optimization_algorithms = OptimizationAlgorithms::new();
    
    let difficulty = consensus_algorithms.calculate_difficulty(100, 600, 550);
    println!("   难度计算: {}", difficulty);
    
    let random_bytes = crypto_algorithms.generate_random_bytes(32);
    println!("   随机字节生成: {} bytes", random_bytes.len());
    
    // 9. 演示应用层
    println!("\n📱 9. 演示应用层");
    let app_layer = ApplicationLayer::new();
    
    // 创建钱包
    let wallet_address = app_layer.wallet_service.create_wallet().await?;
    println!("   钱包地址: {}", wallet_address);
    
    // 10. 演示监控系统
    println!("\n📊 10. 演示监控系统");
    let mut monitor = Monitor::new();
    
    // 记录一些指标
    monitor.record_metric("blocks_mined", 1.0);
    monitor.record_metric("transactions_processed", 5.0);
    monitor.record_metric("network_peers", 2.0);
    
    let stats = monitor.get_statistics();
    println!("   监控统计: 区块={}, 交易={}, 节点={}", 
             stats.get("blocks_mined").unwrap_or(&0.0),
             stats.get("transactions_processed").unwrap_or(&0.0),
             stats.get("network_peers").unwrap_or(&0.0));
    
    // 11. 模拟区块链操作
    println!("\n⛏️  11. 模拟区块链挖矿");
    for i in 1..=5 {
        println!("   挖矿区块 #{}", i);
        
        // 创建新交易
        let transaction = Transaction::new(vec![], vec![], i);
        transaction_storage.store_transaction(transaction.clone()).await?;
        
        // 创建新区块
        let previous_hash = if i == 1 { genesis_block.hash() } else { [i as u8; 32] };
        let new_block = Block::new(i, previous_hash, vec![transaction], get_current_timestamp());
        
        // 存储区块
        block_storage.store_block(i, new_block.clone()).await?;
        
        // 更新状态
        let mut new_state = State::new();
        new_state.latest_block_height = i;
        new_state.latest_block_hash = new_block.hash();
        state_storage.store_state(i, new_state).await?;
        
        println!("     区块哈希: {}", hex::encode(new_block.hash()));
        
        // 模拟挖矿时间
        sleep(Duration::from_millis(100)).await;
    }
    
    // 12. 显示最终统计
    println!("\n📈 12. 最终统计信息");
    let block_stats = block_storage.get_stats().await?;
    let tx_stats = transaction_storage.get_transaction_stats().await;
    let state_stats = state_storage.get_state_stats().await;
    let merkle_stats = merkle_storage.get_merkle_stats().await;
    
    println!("   区块存储: {} 个区块", block_stats.total_blocks);
    println!("   交易存储: {} 个交易", tx_stats.total_transactions);
    println!("   状态存储: {} 个状态", state_stats.total_states);
    println!("   Merkle存储: {} 个树", merkle_stats.total_trees);
    
    // 13. 清理资源
    println!("\n🧹 13. 清理资源");
    block_storage.shutdown().await?;
    transaction_storage.shutdown().await?;
    state_storage.shutdown().await?;
    merkle_storage.shutdown().await?;
    
    println!("\n🎉 区块链演示完成！");
    println!("=" .repeat(50));
    
    Ok(())
}

fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
