//! # 共识算法演示程序
//! 
//! 展示不同共识算法的工作原理和性能对比
//! Demonstrates different consensus algorithms and their performance comparison

use blockchain::{
    simple_blockchain::Transaction,
    consensus::{
        ConsensusManager, ConsensusConfig, ConsensusType,
        Validator, Delegate,
    },
};
use std::time::{Duration, Instant};

fn main() {
    println!("🚀 共识算法演示程序");
    println!("🚀 Consensus Algorithm Demo");
    println!();

    // 1. PoW 共识演示
    println!("📋 1. 工作量证明 (Proof of Work) 演示");
    demo_proof_of_work();

    // 2. PoS 共识演示
    println!("\n📋 2. 权益证明 (Proof of Stake) 演示");
    demo_proof_of_stake();

    // 3. DPoS 共识演示
    println!("\n📋 3. 委托权益证明 (Delegated Proof of Stake) 演示");
    demo_delegated_proof_of_stake();

    // 4. PBFT 共识演示
    println!("\n📋 4. 实用拜占庭容错 (PBFT) 演示");
    demo_practical_byzantine_fault_tolerance();

    // 5. 性能对比
    println!("\n📋 5. 共识算法性能对比");
    performance_comparison();

    println!("\n🎉 共识算法演示完成！");
}

/// 工作量证明演示
fn demo_proof_of_work() {
    let config = ConsensusConfig {
        consensus_type: ConsensusType::ProofOfWork,
        difficulty: 1, // 低难度用于演示
        block_time: Duration::from_secs(10),
        stake_threshold: 1000,
        delegate_count: 21,
        byzantine_threshold: 1,
    };

    let mut manager = ConsensusManager::new(config);

    // 添加一些交易
    let transactions = vec![
        Transaction::new("alice".to_string(), "bob".to_string(), 100),
        Transaction::new("bob".to_string(), "charlie".to_string(), 50),
        Transaction::new("charlie".to_string(), "alice".to_string(), 25),
    ];

    for tx in transactions {
        if let Err(e) = manager.add_transaction(tx) {
            println!("❌ 添加交易失败: {}", e);
        }
    }

    println!("✅ PoW 交易添加完成");

    // 生成区块
    let start_time = Instant::now();
    match manager.generate_block() {
        Ok(block) => {
            let duration = start_time.elapsed();
            println!("✅ PoW 区块生成成功");
            println!("   - 区块索引: {}", block.index);
            println!("   - 区块哈希: {}", block.hash.to_string());
            println!("   - 挖矿时间: {:?}", duration);
            println!("   - 交易数量: {}", block.transactions.len());
        }
        Err(e) => println!("❌ PoW 区块生成失败: {}", e),
    }

    // 显示统计信息
    let stats = manager.get_consensus_stats();
    println!("📊 PoW 统计信息:");
    println!("   - 共识类型: {:?}", stats.consensus_type);
    println!("   - 链高度: {}", manager.get_blockchain().get_chain_length());
}

/// 权益证明演示
fn demo_proof_of_stake() {
    let config = ConsensusConfig {
        consensus_type: ConsensusType::ProofOfStake,
        difficulty: 1,
        block_time: Duration::from_secs(10),
        stake_threshold: 1000,
        delegate_count: 21,
        byzantine_threshold: 1,
    };

    let mut manager = ConsensusManager::new(config);

    // 添加验证者
    let validators = vec![
        Validator {
            address: "validator1".to_string(),
            stake: 5000,
            voting_power: 5000,
            is_active: true,
            last_block_time: 0,
        },
        Validator {
            address: "validator2".to_string(),
            stake: 3000,
            voting_power: 3000,
            is_active: true,
            last_block_time: 0,
        },
        Validator {
            address: "validator3".to_string(),
            stake: 2000,
            voting_power: 2000,
            is_active: true,
            last_block_time: 0,
        },
    ];

    for validator in validators {
        manager.add_validator(validator);
    }

    println!("✅ PoS 验证者添加完成");

    // 添加交易
    let transactions = vec![
        Transaction::new("alice".to_string(), "bob".to_string(), 200),
        Transaction::new("bob".to_string(), "charlie".to_string(), 150),
    ];

    for tx in transactions {
        if let Err(e) = manager.add_transaction(tx) {
            println!("❌ 添加交易失败: {}", e);
        }
    }

    println!("✅ PoS 交易添加完成");

    // 生成区块
    let start_time = Instant::now();
    match manager.generate_block() {
        Ok(block) => {
            let duration = start_time.elapsed();
            println!("✅ PoS 区块生成成功");
            println!("   - 区块索引: {}", block.index);
            println!("   - 区块哈希: {}", block.hash.to_string());
            println!("   - 生成时间: {:?}", duration);
            println!("   - 交易数量: {}", block.transactions.len());
        }
        Err(e) => println!("❌ PoS 区块生成失败: {}", e),
    }

    // 显示统计信息
    let stats = manager.get_consensus_stats();
    println!("📊 PoS 统计信息:");
    println!("   - 共识类型: {:?}", stats.consensus_type);
    println!("   - 验证者数量: {}", stats.total_validators);
    println!("   - 活跃验证者: {}", stats.active_validators);
    println!("   - 总权益: {}", stats.total_stake);
}

/// 委托权益证明演示
fn demo_delegated_proof_of_stake() {
    let config = ConsensusConfig {
        consensus_type: ConsensusType::DelegatedProofOfStake,
        difficulty: 1,
        block_time: Duration::from_secs(10),
        stake_threshold: 1000,
        delegate_count: 5, // 5个委托者
        byzantine_threshold: 1,
    };

    let mut manager = ConsensusManager::new(config);

    // 添加委托者
    let delegates = vec![
        Delegate {
            address: "delegate1".to_string(),
            votes: 10000,
            productivity: 0.95,
            is_active: true,
            block_count: 100,
        },
        Delegate {
            address: "delegate2".to_string(),
            votes: 8000,
            productivity: 0.90,
            is_active: true,
            block_count: 85,
        },
        Delegate {
            address: "delegate3".to_string(),
            votes: 6000,
            productivity: 0.88,
            is_active: true,
            block_count: 70,
        },
        Delegate {
            address: "delegate4".to_string(),
            votes: 4000,
            productivity: 0.85,
            is_active: true,
            block_count: 60,
        },
        Delegate {
            address: "delegate5".to_string(),
            votes: 2000,
            productivity: 0.80,
            is_active: true,
            block_count: 50,
        },
    ];

    for delegate in delegates {
        manager.add_delegate(delegate);
    }

    println!("✅ DPoS 委托者添加完成");

    // 添加交易
    let transactions = vec![
        Transaction::new("alice".to_string(), "bob".to_string(), 300),
        Transaction::new("charlie".to_string(), "david".to_string(), 250),
    ];

    for tx in transactions {
        if let Err(e) = manager.add_transaction(tx) {
            println!("❌ 添加交易失败: {}", e);
        }
    }

    println!("✅ DPoS 交易添加完成");

    // 生成区块
    let start_time = Instant::now();
    match manager.generate_block() {
        Ok(block) => {
            let duration = start_time.elapsed();
            println!("✅ DPoS 区块生成成功");
            println!("   - 区块索引: {}", block.index);
            println!("   - 区块哈希: {}", block.hash.to_string());
            println!("   - 生成时间: {:?}", duration);
            println!("   - 交易数量: {}", block.transactions.len());
        }
        Err(e) => println!("❌ DPoS 区块生成失败: {}", e),
    }

    // 显示统计信息
    let stats = manager.get_consensus_stats();
    println!("📊 DPoS 统计信息:");
    println!("   - 共识类型: {:?}", stats.consensus_type);
    println!("   - 委托者数量: {}", stats.total_delegates);
    println!("   - 活跃委托者: {}", stats.active_delegates);
    println!("   - 总投票数: {}", stats.total_votes);
}

/// 实用拜占庭容错演示
fn demo_practical_byzantine_fault_tolerance() {
    let config = ConsensusConfig {
        consensus_type: ConsensusType::PracticalByzantineFaultTolerance,
        difficulty: 1,
        block_time: Duration::from_secs(10),
        stake_threshold: 1000,
        delegate_count: 21,
        byzantine_threshold: 1, // 容忍1个拜占庭节点
    };

    let mut manager = ConsensusManager::new(config);

    // 添加验证者（至少需要 3f+1 个节点，其中 f 是拜占庭节点数）
    let validators = vec![
        Validator {
            address: "validator1".to_string(),
            stake: 5000,
            voting_power: 5000,
            is_active: true,
            last_block_time: 0,
        },
        Validator {
            address: "validator2".to_string(),
            stake: 4000,
            voting_power: 4000,
            is_active: true,
            last_block_time: 0,
        },
        Validator {
            address: "validator3".to_string(),
            stake: 3000,
            voting_power: 3000,
            is_active: true,
            last_block_time: 0,
        },
        Validator {
            address: "validator4".to_string(),
            stake: 2000,
            voting_power: 2000,
            is_active: true,
            last_block_time: 0,
        },
    ];

    for validator in validators {
        manager.add_validator(validator);
    }

    println!("✅ PBFT 验证者添加完成");

    // 添加交易
    let transactions = vec![
        Transaction::new("alice".to_string(), "bob".to_string(), 400),
        Transaction::new("charlie".to_string(), "david".to_string(), 350),
        Transaction::new("eve".to_string(), "frank".to_string(), 300),
    ];

    for tx in transactions {
        if let Err(e) = manager.add_transaction(tx) {
            println!("❌ 添加交易失败: {}", e);
        }
    }

    println!("✅ PBFT 交易添加完成");

    // 生成区块
    let start_time = Instant::now();
    match manager.generate_block() {
        Ok(block) => {
            let duration = start_time.elapsed();
            println!("✅ PBFT 区块生成成功");
            println!("   - 区块索引: {}", block.index);
            println!("   - 区块哈希: {}", block.hash.to_string());
            println!("   - 生成时间: {:?}", duration);
            println!("   - 交易数量: {}", block.transactions.len());
        }
        Err(e) => println!("❌ PBFT 区块生成失败: {}", e),
    }

    // 显示统计信息
    let stats = manager.get_consensus_stats();
    println!("📊 PBFT 统计信息:");
    println!("   - 共识类型: {:?}", stats.consensus_type);
    println!("   - 验证者数量: {}", stats.total_validators);
    println!("   - 活跃验证者: {}", stats.active_validators);
    println!("   - 当前视图: {}", stats.current_view);
    println!("   - 序列号: {}", stats.sequence_number);
}

/// 性能对比
fn performance_comparison() {
    println!("🔬 开始性能对比测试...");

    let consensus_types = vec![
        ConsensusType::ProofOfWork,
        ConsensusType::ProofOfStake,
        ConsensusType::DelegatedProofOfStake,
        ConsensusType::PracticalByzantineFaultTolerance,
    ];

    let mut results = Vec::new();

    for consensus_type in consensus_types {
        let config = ConsensusConfig {
            consensus_type: consensus_type.clone(),
            difficulty: 1,
            block_time: Duration::from_secs(10),
            stake_threshold: 1000,
            delegate_count: 5,
            byzantine_threshold: 1,
        };

        let mut manager = ConsensusManager::new(config);

        // 为 PoS 和 DPoS 添加验证者/委托者
        match consensus_type {
            ConsensusType::ProofOfStake => {
                for i in 1..=3 {
                    manager.add_validator(Validator {
                        address: format!("validator{}", i),
                        stake: 5000 - (i as u64 * 1000),
                        voting_power: 5000 - (i as u64 * 1000),
                        is_active: true,
                        last_block_time: 0,
                    });
                }
            }
            ConsensusType::DelegatedProofOfStake => {
                for i in 1..=5 {
                    manager.add_delegate(Delegate {
                        address: format!("delegate{}", i),
                        votes: 10000 - (i as u64 * 1000),
                        productivity: 0.95 - (i as f64 * 0.02),
                        is_active: true,
                        block_count: 100 - (i as u64 * 10),
                    });
                }
            }
            ConsensusType::PracticalByzantineFaultTolerance => {
                for i in 1..=4 {
                    manager.add_validator(Validator {
                        address: format!("validator{}", i),
                        stake: 5000 - (i as u64 * 1000),
                        voting_power: 5000 - (i as u64 * 1000),
                        is_active: true,
                        last_block_time: 0,
                    });
                }
            }
            _ => {}
        }

        // 添加相同的交易
        for i in 0..5 {
            let tx = Transaction::new(
                format!("sender{}", i),
                format!("receiver{}", i),
                100 + (i as u64 * 50),
            );
            let _ = manager.add_transaction(tx);
        }

        // 测试区块生成性能
        let start_time = Instant::now();
        let mut success_count = 0;
        let mut total_time = Duration::ZERO;

        for _ in 0..3 {
            match manager.generate_block() {
                Ok(_) => {
                    success_count += 1;
                    total_time += start_time.elapsed();
                }
                Err(_) => {}
            }
        }

        let average_time = if success_count > 0 {
            total_time / success_count
        } else {
            Duration::ZERO
        };

        results.push(PerformanceResult {
            consensus_type: consensus_type.clone(),
            success_rate: success_count as f64 / 3.0,
            average_time,
            throughput: success_count as f64 / total_time.as_secs_f64(),
        });
    }

    // 显示对比结果
    println!("\n📊 性能对比结果:");
    println!("{:<25} {:<15} {:<15} {:<15}", "共识算法", "成功率", "平均时间", "吞吐量");
    println!("{}", "-".repeat(70));

    for result in results {
        println!(
            "{:<25} {:<15.2} {:<15?} {:<15.2}",
            format!("{:?}", result.consensus_type),
            result.success_rate,
            result.average_time,
            result.throughput
        );
    }

    // 分析结果
    println!("\n📈 性能分析:");
    println!("• PoW: 需要计算密集型挖矿，时间较长但安全性高");
    println!("• PoS: 基于权益选择，速度较快，能耗低");
    println!("• DPoS: 委托机制，速度最快，但需要信任委托者");
    println!("• PBFT: 拜占庭容错，安全性最高，但需要更多节点通信");
}

/// 性能测试结果
#[allow(dead_code)]
struct PerformanceResult {
    consensus_type: ConsensusType,
    success_rate: f64,
    average_time: Duration,
    throughput: f64,
}

/// 共识算法特性对比
#[allow(dead_code)]
fn compare_consensus_features() {
    println!("\n🔍 共识算法特性对比:");
    
    let features = vec![
        ("能耗", vec![
            ("PoW", "高"),
            ("PoS", "低"),
            ("DPoS", "极低"),
            ("PBFT", "低"),
        ]),
        ("安全性", vec![
            ("PoW", "高"),
            ("PoS", "中"),
            ("DPoS", "中"),
            ("PBFT", "极高"),
        ]),
        ("去中心化程度", vec![
            ("PoW", "高"),
            ("PoS", "高"),
            ("DPoS", "中"),
            ("PBFT", "高"),
        ]),
        ("交易速度", vec![
            ("PoW", "慢"),
            ("PoS", "中"),
            ("DPoS", "快"),
            ("PBFT", "快"),
        ]),
        ("扩展性", vec![
            ("PoW", "低"),
            ("PoS", "中"),
            ("DPoS", "高"),
            ("PBFT", "中"),
        ]),
    ];

    for (feature, algorithms) in features {
        println!("\n{}:", feature);
        for (algorithm, rating) in algorithms {
            println!("  {}: {}", algorithm, rating);
        }
    }
}
