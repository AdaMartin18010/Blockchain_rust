//! # 性能优化演示程序
//! 
//! 展示区块链性能优化技术的效果
//! Demonstrates the effectiveness of blockchain performance optimization techniques

use blockchain::*;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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

// ===== 性能优化相关的结构体和实现 =====

/// 缓存管理器
pub struct CacheManager {
    cache: Arc<Mutex<HashMap<String, (Vec<u8>, Instant)>>>,
    max_size: usize,
    ttl: Duration,
    hits: Arc<Mutex<u64>>,
    misses: Arc<Mutex<u64>>,
}

impl CacheManager {
    pub fn new(max_size: usize, ttl: Duration) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            max_size,
            ttl,
            hits: Arc::new(Mutex::new(0)),
            misses: Arc::new(Mutex::new(0)),
        }
    }

    pub fn set(&self, key: String, value: Vec<u8>) {
        let mut cache = self.cache.lock().unwrap();
        if cache.len() >= self.max_size {
            // 简单的LRU实现：移除最旧的项
            if let Some(oldest_key) = cache.keys().next().cloned() {
                cache.remove(&oldest_key);
            }
        }
        cache.insert(key, (value, Instant::now()));
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let mut cache = self.cache.lock().unwrap();
        if let Some((value, timestamp)) = cache.get(key) {
            if timestamp.elapsed() < self.ttl {
                *self.hits.lock().unwrap() += 1;
                return Some(value.clone());
            } else {
                cache.remove(key);
            }
        }
        *self.misses.lock().unwrap() += 1;
        None
    }

    pub fn size(&self) -> usize {
        self.cache.lock().unwrap().len()
    }

    pub fn hit_rate(&self) -> f64 {
        let hits = *self.hits.lock().unwrap();
        let misses = *self.misses.lock().unwrap();
        if hits + misses == 0 {
            0.0
        } else {
            hits as f64 / (hits + misses) as f64
        }
    }
}

/// 内存池
pub struct MemoryPool {
    transactions: Arc<Mutex<Vec<Transaction>>>,
    max_size: usize,
    performance_stats: Arc<Mutex<HashMap<String, PerformanceMetric>>>,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    pub count: u64,
    pub total_time: Duration,
    pub avg_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
}

impl MemoryPool {
    pub fn new(max_size: usize) -> Self {
        Self {
            transactions: Arc::new(Mutex::new(Vec::new())),
            max_size,
            performance_stats: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_transaction(&self, transaction: Transaction) -> Result<(), String> {
        let start = Instant::now();
        let mut transactions = self.transactions.lock().unwrap();
        
        if transactions.len() >= self.max_size {
            return Err("内存池已满".to_string());
        }
        
        transactions.push(transaction);
        let elapsed = start.elapsed();
        
        // 记录性能指标
        self.record_metric("add_transaction", elapsed);
        
        Ok(())
    }

    pub fn get_transactions(&self, count: usize) -> Vec<Transaction> {
        let start = Instant::now();
        let mut transactions = self.transactions.lock().unwrap();
        
        let len = transactions.len();
        let result = transactions.drain(..count.min(len)).collect();
        let elapsed = start.elapsed();
        
        // 记录性能指标
        self.record_metric("get_transactions", elapsed);
        
        result
    }

    pub fn size(&self) -> usize {
        self.transactions.lock().unwrap().len()
    }

    fn record_metric(&self, operation: &str, duration: Duration) {
        let mut stats = self.performance_stats.lock().unwrap();
        let metric = stats.entry(operation.to_string()).or_insert(PerformanceMetric {
            count: 0,
            total_time: Duration::ZERO,
            avg_time: Duration::ZERO,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
        });
        
        metric.count += 1;
        metric.total_time += duration;
        metric.avg_time = metric.total_time / metric.count as u32;
        metric.min_time = metric.min_time.min(duration);
        metric.max_time = metric.max_time.max(duration);
    }

    pub fn get_performance_stats(&self) -> HashMap<String, PerformanceMetric> {
        self.performance_stats.lock().unwrap().clone()
    }
}

/// 性能监控器
pub struct PerformanceMonitor {
    metrics: Arc<Mutex<HashMap<String, PerformanceMetric>>>,
    start_time: Instant,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
            start_time: Instant::now(),
        }
    }

    pub fn record_operation<F, R>(&self, operation: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        
        self.record_metric(operation, duration);
        result
    }

    fn record_metric(&self, operation: &str, duration: Duration) {
        let mut metrics = self.metrics.lock().unwrap();
        let metric = metrics.entry(operation.to_string()).or_insert(PerformanceMetric {
            count: 0,
            total_time: Duration::ZERO,
            avg_time: Duration::ZERO,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
        });
        
        metric.count += 1;
        metric.total_time += duration;
        metric.avg_time = metric.total_time / metric.count as u32;
        metric.min_time = metric.min_time.min(duration);
        metric.max_time = metric.max_time.max(duration);
    }

    pub fn get_all_metrics(&self) -> HashMap<String, PerformanceMetric> {
        self.metrics.lock().unwrap().clone()
    }

    pub fn get_uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// 优化的区块链
pub struct OptimizedBlockchain {
    blockchain: Blockchain,
    cache: CacheManager,
    memory_pool: MemoryPool,
    monitor: PerformanceMonitor,
}

impl OptimizedBlockchain {
    pub fn new(difficulty: usize, cache_size: usize, pool_size: usize) -> Self {
        Self {
            blockchain: Blockchain::new(difficulty),
            cache: CacheManager::new(cache_size, Duration::from_secs(60)),
            memory_pool: MemoryPool::new(pool_size),
            monitor: PerformanceMonitor::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        self.monitor.record_operation("add_transaction", || {
            self.blockchain.add_transaction(transaction)
        })
    }

    pub fn mine_block_optimized(&mut self, miner_address: String) -> Result<(), String> {
        self.monitor.record_operation("mine_block", || {
            self.blockchain.mine_pending_transactions(miner_address)
        })
    }

    pub fn get_balance_cached(&self, address: &str) -> u64 {
        let cache_key = format!("balance_{}", address);
        
        if let Some(cached_balance) = self.cache.get(&cache_key) {
            if let Ok(balance) = String::from_utf8(cached_balance) {
                if let Ok(balance_num) = balance.parse::<u64>() {
                    return balance_num;
                }
            }
        }
        
        let balance = self.blockchain.get_balance(address);
        self.cache.set(cache_key, balance.to_string().into_bytes());
        balance
    }

    pub fn get_performance_report(&self) -> PerformanceReport {
        PerformanceReport {
            uptime: self.monitor.get_uptime(),
            cache_stats: CacheStats {
                size: self.cache.size(),
                hit_rate: self.cache.hit_rate(),
            },
            memory_pool_stats: MemoryPoolStats {
                size: self.memory_pool.size(),
            },
            operations: self.monitor.get_all_metrics(),
        }
    }
}

#[derive(Debug)]
pub struct PerformanceReport {
    pub uptime: Duration,
    pub cache_stats: CacheStats,
    pub memory_pool_stats: MemoryPoolStats,
    pub operations: HashMap<String, PerformanceMetric>,
}

#[derive(Debug)]
pub struct CacheStats {
    pub size: usize,
    pub hit_rate: f64,
}

#[derive(Debug)]
pub struct MemoryPoolStats {
    pub size: usize,
}
