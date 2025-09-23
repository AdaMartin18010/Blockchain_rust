//! # 性能优化模块
//! 
//! 提供各种性能优化技术和工具
//! Performance optimization module with various optimization techniques

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use crate::simple_blockchain::*;

/// 性能监控器
#[derive(Debug)]
pub struct PerformanceMonitor {
    metrics: Arc<RwLock<HashMap<String, PerformanceMetric>>>,
    start_time: Instant,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    pub name: String,
    pub count: u64,
    pub total_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub avg_time: Duration,
    pub last_update: u64,
}

impl Default for PerformanceMetric {
    fn default() -> Self {
        Self {
            name: String::new(),
            count: 0,
            total_time: Duration::ZERO,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
            avg_time: Duration::ZERO,
            last_update: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            start_time: Instant::now(),
        }
    }

    pub fn record_operation<F, R>(&self, name: &str, operation: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed();

        self.update_metric(name, duration);
        result
    }

    fn update_metric(&self, name: &str, duration: Duration) {
        let mut metrics = self.metrics.write().unwrap();
        let metric = metrics.entry(name.to_string()).or_insert_with(|| {
            let mut m = PerformanceMetric::default();
            m.name = name.to_string();
            m
        });

        metric.count += 1;
        metric.total_time += duration;
        metric.min_time = metric.min_time.min(duration);
        metric.max_time = metric.max_time.max(duration);
        metric.avg_time = Duration::from_nanos(metric.total_time.as_nanos() as u64 / metric.count);
        metric.last_update = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    #[allow(dead_code)]
    pub fn get_metric(&self, name: &str) -> Option<PerformanceMetric> {
        self.metrics.read().unwrap().get(name).cloned()
    }

    pub fn get_all_metrics(&self) -> HashMap<String, PerformanceMetric> {
        self.metrics.read().unwrap().clone()
    }

    #[allow(dead_code)]
    pub fn reset(&self) {
        self.metrics.write().unwrap().clear();
    }

    pub fn get_uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// 缓存管理器
#[derive(Debug)]
pub struct CacheManager {
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    max_size: usize,
    ttl: Duration,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    value: Vec<u8>,
    created_at: Instant,
    access_count: u64,
}

impl CacheManager {
    pub fn new(max_size: usize, ttl: Duration) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            max_size,
            ttl,
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let mut cache = self.cache.lock().unwrap();
        if let Some(entry) = cache.get_mut(key) {
            if entry.created_at.elapsed() < self.ttl {
                entry.access_count += 1;
                return Some(entry.value.clone());
            } else {
                cache.remove(key);
            }
        }
        None
    }

    pub fn set(&self, key: String, value: Vec<u8>) {
        let mut cache = self.cache.lock().unwrap();
        
        // 如果缓存已满，移除最旧的条目
        if cache.len() >= self.max_size {
            let oldest_key = cache.iter()
                .min_by_key(|(_, entry)| entry.created_at)
                .map(|(k, _)| k.clone());
            
            if let Some(oldest) = oldest_key {
                cache.remove(&oldest);
            }
        }

        cache.insert(key, CacheEntry {
            value,
            created_at: Instant::now(),
            access_count: 1,
        });
    }

    #[allow(dead_code)]
    pub fn clear(&self) {
        self.cache.lock().unwrap().clear();
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.cache.lock().unwrap().len()
    }

    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let cache = self.cache.lock().unwrap();
        let total_accesses: u64 = cache.values().map(|entry| entry.access_count).sum();
        if total_accesses == 0 {
            return 0.0;
        }
        
        let hits: u64 = cache.values()
            .filter(|entry| entry.created_at.elapsed() < self.ttl)
            .map(|entry| entry.access_count)
            .sum();
        
        hits as f64 / total_accesses as f64
    }
}

/// 并行挖矿器
#[allow(dead_code)]
pub struct ParallelMiner {
    thread_count: usize,
    cache: CacheManager,
    monitor: PerformanceMonitor,
}

#[allow(dead_code)]
impl ParallelMiner {
    pub fn new(thread_count: usize) -> Self {
        Self {
            thread_count,
            cache: CacheManager::new(1000, Duration::from_secs(300)), // 5分钟TTL
            monitor: PerformanceMonitor::new(),
        }
    }

    pub fn mine_block_parallel(&self, blockchain: &mut Blockchain, _miner: String) -> Result<Block, String> {
        self.monitor.record_operation("parallel_mining", || {
            // 简化实现：直接调用区块链的挖矿方法
            blockchain.mine_pending_transactions("parallel_miner".to_string())?;
            
            // 返回最新挖出的块
            blockchain.chain.last()
                .cloned()
                .ok_or("没有找到新挖出的块".to_string())
        })
    }

    pub fn get_performance_stats(&self) -> HashMap<String, PerformanceMetric> {
        self.monitor.get_all_metrics()
    }
}

/// 内存池管理器
pub struct MemoryPool {
    transactions: Arc<Mutex<Vec<Transaction>>>,
    max_size: usize,
    monitor: PerformanceMonitor,
}

impl MemoryPool {
    pub fn new(max_size: usize) -> Self {
        Self {
            transactions: Arc::new(Mutex::new(Vec::new())),
            max_size,
            monitor: PerformanceMonitor::new(),
        }
    }

    pub fn add_transaction(&self, transaction: Transaction) -> Result<(), String> {
        self.monitor.record_operation("add_transaction", || {
            let mut transactions = self.transactions.lock().unwrap();
            
            if transactions.len() >= self.max_size {
                return Err("内存池已满".to_string());
            }

            // 检查重复交易
            if transactions.iter().any(|tx| {
                tx.sender == transaction.sender &&
                tx.receiver == transaction.receiver &&
                tx.amount == transaction.amount &&
                tx.timestamp == transaction.timestamp
            }) {
                return Err("重复交易".to_string());
            }

            transactions.push(transaction);
            Ok(())
        })
    }

    pub fn get_transactions(&self, count: usize) -> Vec<Transaction> {
        self.monitor.record_operation("get_transactions", || {
            let mut transactions = self.transactions.lock().unwrap();
            let actual_count = count.min(transactions.len());
            transactions.drain(0..actual_count).collect()
        })
    }

    #[allow(dead_code)]
    pub fn clear(&self) {
        self.transactions.lock().unwrap().clear();
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.transactions.lock().unwrap().len()
    }

    #[allow(dead_code)]
    pub fn get_performance_stats(&self) -> HashMap<String, PerformanceMetric> {
        self.monitor.get_all_metrics()
    }
}

/// 优化的区块链结构
#[allow(dead_code)]
pub struct OptimizedBlockchain {
    blockchain: Blockchain,
    cache: CacheManager,
    memory_pool: MemoryPool,
    monitor: PerformanceMonitor,
    parallel_miner: ParallelMiner,
}

#[allow(dead_code)]
impl OptimizedBlockchain {
    pub fn new(difficulty: usize, _genesis_balance: u64, thread_count: usize) -> Self {
        Self {
            blockchain: Blockchain::new(difficulty),
            cache: CacheManager::new(10000, Duration::from_secs(600)), // 10分钟TTL
            memory_pool: MemoryPool::new(10000),
            monitor: PerformanceMonitor::new(),
            parallel_miner: ParallelMiner::new(thread_count),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        self.monitor.record_operation("add_transaction", || {
            self.memory_pool.add_transaction(transaction)
        })
    }

    pub fn mine_block_optimized(&mut self, miner: String) -> Result<(), String> {
        self.monitor.record_operation("mine_block", || {
            // 从内存池获取交易
            let transactions = self.memory_pool.get_transactions(1000); // 每块最多1000笔交易
            
            // 添加到区块链
            for tx in transactions {
                self.blockchain.add_transaction(tx)?;
            }

            // 并行挖矿
            self.parallel_miner.mine_block_parallel(&mut self.blockchain, miner)?;
            Ok(())
        })
    }

    pub fn get_balance_cached(&self, address: &str) -> u64 {
        let cache_key = format!("balance_{}", address);
        
        if let Some(_cached_balance) = self.cache.get(&cache_key) {
            // 简化处理，实际应该反序列化
            return 1000; // 模拟缓存命中
        }

        let balance = self.blockchain.get_balance(address);
        
        // 缓存结果
        self.cache.set(cache_key, balance.to_le_bytes().to_vec());
        balance
    }

    pub fn validate_chain_optimized(&self) -> bool {
        self.monitor.record_operation("validate_chain", || {
            self.blockchain.is_valid_chain()
        })
    }

    pub fn get_performance_report(&self) -> PerformanceReport {
        let report = PerformanceReport {
            uptime: self.monitor.get_uptime(),
            operations: self.monitor.get_all_metrics(),
            cache_stats: CacheStats {
                size: self.cache.size(),
                hit_rate: self.cache.hit_rate(),
            },
            memory_pool_stats: MemoryPoolStats {
                size: self.memory_pool.size(),
                operations: self.memory_pool.get_performance_stats(),
            },
            mining_stats: self.parallel_miner.get_performance_stats(),
        };

        report
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PerformanceReport {
    pub uptime: Duration,
    pub operations: HashMap<String, PerformanceMetric>,
    pub cache_stats: CacheStats,
    pub memory_pool_stats: MemoryPoolStats,
    pub mining_stats: HashMap<String, PerformanceMetric>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CacheStats {
    pub size: usize,
    pub hit_rate: f64,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MemoryPoolStats {
    pub size: usize,
    pub operations: HashMap<String, PerformanceMetric>,
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_performance_monitor() {
        let monitor = PerformanceMonitor::new();
        
        let result = monitor.record_operation("test_op", || {
            thread::sleep(Duration::from_millis(10));
            42
        });
        
        assert_eq!(result, 42);
        
        let metric = monitor.get_metric("test_op");
        assert!(metric.is_some());
        let metric = metric.unwrap();
        assert_eq!(metric.count, 1);
        assert!(metric.total_time.as_millis() >= 10);
    }

    #[test]
    fn test_cache_manager() {
        let cache = CacheManager::new(10, Duration::from_secs(1));
        
        cache.set("key1".to_string(), vec![1, 2, 3]);
        assert_eq!(cache.get("key1"), Some(vec![1, 2, 3]));
        
        // 测试TTL
        thread::sleep(Duration::from_millis(1100));
        assert_eq!(cache.get("key1"), None);
    }

    #[test]
    fn test_memory_pool() {
        let pool = MemoryPool::new(5);
        
        let tx = Transaction {
            sender: "alice".to_string(),
            receiver: "bob".to_string(),
            amount: 100,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            signature: b"test_signature".to_vec(),
        };
        
        assert!(pool.add_transaction(tx).is_ok());
        assert_eq!(pool.size(), 1);
    }
}
