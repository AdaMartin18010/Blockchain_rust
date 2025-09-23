//! # 性能优化演示程序
//! 
//! 展示区块链性能优化技术的效果
//! Demonstrates the effectiveness of blockchain performance optimization techniques

use blockchain::*;
use blockchain::performance::*;
//use blockchain::simple_blockchain::*;
use std::time::{Duration, Instant};

fn main() {
    println!("🚀 区块链性能优化演示");
    println!("🚀 Blockchain Performance Optimization Demo");
    println!();

    // 1. 基础性能测试
    println!("📋 1. 基础性能测试");
    basic_performance_test();

    // 2. 缓存优化效果
    println!("\n📋 2. 缓存优化效果");
    cache_optimization_demo();

    // 3. 内存池优化效果
    println!("\n📋 3. 内存池优化效果");
    memory_pool_demo();

    // 4. 性能监控演示
    println!("\n📋 4. 性能监控演示");
    performance_monitoring_demo();

    // 5. 优化前后对比
    println!("\n📋 5. 优化前后对比");
    optimization_comparison();

    println!("\n🎉 性能优化演示完成！");
}

/// 基础性能测试
fn basic_performance_test() {
    println!("✅ 开始基础性能测试");
    
    let start = Instant::now();
    let mut blockchain = Blockchain::new(2);
    
    // 添加大量交易（使用genesis账户）
    for i in 0..1000 {
        let transaction = Transaction::new(
            "genesis".to_string(), // 使用genesis账户，它有初始余额
            format!("receiver_{}", i % 100),
            1, // 减少金额以避免余额不足
        );
        blockchain.add_transaction(transaction).unwrap();
    }
    
    let add_time = start.elapsed();
    println!("   - 添加1000笔交易耗时: {:?}", add_time);
    
    // 挖矿测试
    let mine_start = Instant::now();
    blockchain.mine_pending_transactions("miner".to_string()).unwrap();
    let mine_time = mine_start.elapsed();
    println!("   - 挖矿耗时: {:?}", mine_time);
    
    // 验证测试
    let validate_start = Instant::now();
    let is_valid = blockchain.is_valid_chain();
    let validate_time = validate_start.elapsed();
    println!("   - 链验证耗时: {:?}", validate_time);
    println!("   - 链有效性: {}", is_valid);
    
    let total_time = start.elapsed();
    println!("   - 总耗时: {:?}", total_time);
}

/// 缓存优化演示
fn cache_optimization_demo() {
    println!("✅ 开始缓存优化演示");
    
    let cache = CacheManager::new(1000, Duration::from_secs(60));
    
    // 测试缓存性能
    let start = Instant::now();
    
    // 设置缓存
    for i in 0..100 {
        let key = format!("key_{}", i);
        let value = format!("value_{}", i).into_bytes();
        cache.set(key, value);
    }
    
    let set_time = start.elapsed();
    println!("   - 设置100个缓存项耗时: {:?}", set_time);
    
    // 读取缓存
    let read_start = Instant::now();
    let mut hits = 0;
    for i in 0..100 {
        let key = format!("key_{}", i);
        if cache.get(&key).is_some() {
            hits += 1;
        }
    }
    let read_time = read_start.elapsed();
    
    println!("   - 读取100个缓存项耗时: {:?}", read_time);
    println!("   - 缓存命中率: {}%", (hits * 100) / 100);
    println!("   - 缓存大小: {}", cache.size());
    println!("   - 缓存命中率: {:.2}%", cache.hit_rate() * 100.0);
}

/// 内存池演示
fn memory_pool_demo() {
    println!("✅ 开始内存池演示");
    
    let pool = MemoryPool::new(1000);
    
    // 测试内存池性能
    let start = Instant::now();
    
    // 添加交易到内存池
    for i in 0..500 {
        let transaction = Transaction::new(
            "genesis".to_string(),
            format!("receiver_{}", i),
            1,
        );
        pool.add_transaction(transaction).unwrap();
    }
    
    let add_time = start.elapsed();
    println!("   - 添加500笔交易到内存池耗时: {:?}", add_time);
    println!("   - 内存池大小: {}", pool.size());
    
    // 批量获取交易
    let get_start = Instant::now();
    let transactions = pool.get_transactions(100);
    let get_time = get_start.elapsed();
    
    println!("   - 获取100笔交易耗时: {:?}", get_time);
    println!("   - 获取的交易数量: {}", transactions.len());
    println!("   - 内存池剩余大小: {}", pool.size());
    
    // 显示性能统计
    let stats = pool.get_performance_stats();
    for (name, metric) in stats {
        println!("   - {}: 调用{}次, 平均耗时{:?}", 
                name, metric.count, metric.avg_time);
    }
}

/// 性能监控演示
fn performance_monitoring_demo() {
    println!("✅ 开始性能监控演示");
    
    let monitor = PerformanceMonitor::new();
    
    // 模拟各种操作
    for i in 0..10 {
        let _result = monitor.record_operation("test_operation", || {
            // 模拟一些工作
            std::thread::sleep(Duration::from_millis(10));
            i * 2
        });
        
        if i % 3 == 0 {
            monitor.record_operation("special_operation", || {
                std::thread::sleep(Duration::from_millis(5));
                "special"
            });
        }
    }
    
    // 显示性能统计
    let metrics = monitor.get_all_metrics();
    println!("   - 监控运行时间: {:?}", monitor.get_uptime());
    
    for (name, metric) in metrics {
        println!("   - 操作 '{}':", name);
        println!("     * 调用次数: {}", metric.count);
        println!("     * 总耗时: {:?}", metric.total_time);
        println!("     * 平均耗时: {:?}", metric.avg_time);
        println!("     * 最小耗时: {:?}", metric.min_time);
        println!("     * 最大耗时: {:?}", metric.max_time);
    }
}

/// 优化前后对比
fn optimization_comparison() {
    println!("✅ 开始优化前后对比");
    
    // 普通区块链测试
    println!("   📊 普通区块链性能:");
    let normal_start = Instant::now();
    let mut normal_blockchain = Blockchain::new(2);
    
    // 添加交易
    for i in 0..100 {
        let transaction = Transaction::new(
            "genesis".to_string(),
            format!("receiver_{}", i),
            1,
        );
        normal_blockchain.add_transaction(transaction).unwrap();
    }
    
    // 挖矿
    normal_blockchain.mine_pending_transactions("miner".to_string()).unwrap();
    
    // 重复查询余额
    for _ in 0..1000 {
        normal_blockchain.get_balance("sender_50");
    }
    
    let normal_time = normal_start.elapsed();
    println!("     - 普通区块链总耗时: {:?}", normal_time);
    
    // 优化区块链测试
    println!("   📊 优化区块链性能:");
    let optimized_start = Instant::now();
    let mut optimized_blockchain = OptimizedBlockchain::new(2, 1000, 4);
    
    // 添加交易
    for i in 0..100 {
        let transaction = Transaction::new(
            "genesis".to_string(),
            format!("receiver_{}", i),
            1,
        );
        optimized_blockchain.add_transaction(transaction).unwrap();
    }
    
    // 挖矿
    optimized_blockchain.mine_block_optimized("miner".to_string()).unwrap();
    
    // 重复查询余额（使用缓存）
    for _ in 0..1000 {
        optimized_blockchain.get_balance_cached("sender_50");
    }
    
    let optimized_time = optimized_start.elapsed();
    println!("     - 优化区块链总耗时: {:?}", optimized_time);
    
    // 性能提升计算
    let improvement = if optimized_time < normal_time {
        ((normal_time.as_nanos() - optimized_time.as_nanos()) as f64 / normal_time.as_nanos() as f64) * 100.0
    } else {
        0.0
    };
    
    println!("   📈 性能提升: {:.2}%", improvement);
    
    // 显示详细性能报告
    let report = optimized_blockchain.get_performance_report();
    println!("   📋 详细性能报告:");
    println!("     - 运行时间: {:?}", report.uptime);
    println!("     - 缓存统计: 大小={}, 命中率={:.2}%", 
             report.cache_stats.size, 
             report.cache_stats.hit_rate * 100.0);
    println!("     - 内存池统计: 大小={}", report.memory_pool_stats.size);
    
    println!("     - 操作统计:");
    for (name, metric) in report.operations {
        println!("       * {}: {}次调用, 平均{:?}", 
                name, metric.count, metric.avg_time);
    }
}

/// 压力测试
#[allow(dead_code)]
fn stress_test() {
    println!("✅ 开始压力测试");
    
    let start = Instant::now();
    let mut blockchain = Blockchain::new(1); // 降低难度以加快测试
    
    // 大量交易测试
    let tx_start = Instant::now();
    for i in 0..10000 {
        let transaction = Transaction::new(
            format!("sender_{}", i % 1000),
            format!("receiver_{}", i % 1000),
            1,
        );
        blockchain.add_transaction(transaction).unwrap();
    }
    let tx_time = tx_start.elapsed();
    
    println!("   - 添加10000笔交易耗时: {:?}", tx_time);
    println!("   - 平均每笔交易: {:?}", tx_time / 10000);
    
    // 批量挖矿测试
    let mine_start = Instant::now();
    for _ in 0..10 {
        blockchain.mine_pending_transactions("stress_miner".to_string()).unwrap();
    }
    let mine_time = mine_start.elapsed();
    
    println!("   - 挖矿10个区块耗时: {:?}", mine_time);
    println!("   - 平均每个区块: {:?}", mine_time / 10);
    
    // 链验证测试
    let validate_start = Instant::now();
    for _ in 0..100 {
        blockchain.is_valid_chain();
    }
    let validate_time = validate_start.elapsed();
    
    println!("   - 验证链100次耗时: {:?}", validate_time);
    println!("   - 平均每次验证: {:?}", validate_time / 100);
    
    let total_time = start.elapsed();
    println!("   - 压力测试总耗时: {:?}", total_time);
    println!("   - 链长度: {}", blockchain.get_chain_length());
}
