//! # 区块链演示程序
//! 
//! 展示区块链的基本功能和 Rust 特性

use blockchain::simple_blockchain::*;

fn main() {
    println!("🚀 区块链演示程序");
    println!("🚀 Blockchain Demo Program");
    println!();

    // 创建区块链
    let mut blockchain = Blockchain::new(2);
    println!("✅ 创建区块链，难度: {}", blockchain.difficulty);

    // 添加交易
    let transactions = vec![
        Transaction::new("alice".to_string(), "bob".to_string(), 100),
        Transaction::new("bob".to_string(), "charlie".to_string(), 50),
        Transaction::new("charlie".to_string(), "david".to_string(), 25),
    ];

    for (i, tx) in transactions.iter().enumerate() {
        match blockchain.add_transaction(tx.clone()) {
            Ok(_) => println!("✅ 交易 {} 添加成功", i + 1),
            Err(e) => println!("❌ 交易 {} 添加失败: {}", i + 1, e),
        }
    }

    // 挖矿
    println!("\n⛏️ 开始挖矿...");
    match blockchain.mine_pending_transactions("miner".to_string()) {
        Ok(_) => {
            println!("✅ 挖矿成功！");
            println!("   链长度: {}", blockchain.get_chain_length());
            println!("   Alice 余额: {}", blockchain.get_balance("alice"));
            println!("   Bob 余额: {}", blockchain.get_balance("bob"));
            println!("   Charlie 余额: {}", blockchain.get_balance("charlie"));
            println!("   David 余额: {}", blockchain.get_balance("david"));
            println!("   Miner 余额: {}", blockchain.get_balance("miner"));
        }
        Err(e) => println!("❌ 挖矿失败: {}", e),
    }

    // 验证链
    println!("\n🔍 链验证结果: {}", 
        if blockchain.is_valid_chain() { "✅ 有效" } else { "❌ 无效" });

    // 显示链信息
    if let Some(latest_block) = blockchain.get_latest_block() {
        println!("\n📦 最新区块信息:");
        println!("   索引: {}", latest_block.index);
        println!("   哈希: {}", latest_block.hash.to_string());
        println!("   交易数: {}", latest_block.transactions.len());
        println!("   时间戳: {}", latest_block.timestamp);
    }

    println!("\n🎉 演示完成！");
}
