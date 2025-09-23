//! # 区块链监控和指标收集系统
//! 
//! 提供实时监控、指标收集和性能分析功能
//! Provides real-time monitoring, metrics collection and performance analysis

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// 监控指标类型
/// Types of monitoring metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum MetricType {
    Counter(String),
    Gauge(String),
    Histogram(String),
    Timer(String),
}

/// 监控指标数据
/// Monitoring metric data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub timestamp: u64,
    pub labels: HashMap<String, String>,
}

/// 性能指标
/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub transactions_per_second: f64,
    pub blocks_per_minute: f64,
    pub average_block_time: f64,
    pub average_transaction_size: f64,
    pub memory_usage: u64,
    pub cpu_usage: f64,
    pub network_throughput: f64,
}

/// 区块链状态指标
/// Blockchain status metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainMetrics {
    pub chain_height: u64,
    pub total_transactions: u64,
    pub pending_transactions: u64,
    pub average_difficulty: f64,
    pub total_hash_rate: f64,
    pub active_miners: u64,
    pub network_size: u64,
}

/// 系统健康状态
/// System health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String, // "healthy", "degraded", "unhealthy"
    pub last_block_time: u64,
    pub sync_status: String,
    pub error_count: u64,
    pub warnings: Vec<String>,
}

/// 监控器结构
/// Monitor structure
pub struct BlockchainMonitor {
    metrics: Arc<Mutex<HashMap<String, Metric>>>,
    performance_metrics: Arc<Mutex<PerformanceMetrics>>,
    blockchain_metrics: Arc<Mutex<BlockchainMetrics>>,
    health_status: Arc<Mutex<HealthStatus>>,
    start_time: Instant,
}

impl BlockchainMonitor {
    /// 创建新的监控器
    /// Create new monitor
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
            performance_metrics: Arc::new(Mutex::new(PerformanceMetrics {
                transactions_per_second: 0.0,
                blocks_per_minute: 0.0,
                average_block_time: 0.0,
                average_transaction_size: 0.0,
                memory_usage: 0,
                cpu_usage: 0.0,
                network_throughput: 0.0,
            })),
            blockchain_metrics: Arc::new(Mutex::new(BlockchainMetrics {
                chain_height: 0,
                total_transactions: 0,
                pending_transactions: 0,
                average_difficulty: 0.0,
                total_hash_rate: 0.0,
                active_miners: 0,
                network_size: 0,
            })),
            health_status: Arc::new(Mutex::new(HealthStatus {
                status: "healthy".to_string(),
                last_block_time: 0,
                sync_status: "synced".to_string(),
                error_count: 0,
                warnings: Vec::new(),
            })),
            start_time: Instant::now(),
        }
    }

    /// 记录指标
    /// Record metric
    pub fn record_metric(&self, name: String, value: f64, labels: Option<HashMap<String, String>>) {
        let metric = Metric {
            name: name.clone(),
            value,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            labels: labels.unwrap_or_default(),
        };

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.insert(name, metric);
        }
    }

    /// 增加计数器
    /// Increment counter
    #[allow(dead_code)]
    pub fn increment_counter(&self, name: String, value: f64) {
        if let Ok(mut metrics) = self.metrics.lock() {
            if let Some(metric) = metrics.get_mut(&name) {
                metric.value += value;
                metric.timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
            } else {
                self.record_metric(name.clone(), value, None);
            }
        }
    }

    /// 设置仪表值
    /// Set gauge value
    #[allow(dead_code)]
    pub fn set_gauge(&self, name: String, value: f64) {
        self.record_metric(name, value, None);
    }

    /// 更新性能指标
    /// Update performance metrics
    #[allow(dead_code)]
    pub fn update_performance_metrics(&self, metrics: PerformanceMetrics) {
        if let Ok(mut perf_metrics) = self.performance_metrics.lock() {
            *perf_metrics = metrics;
        }
    }

    /// 更新区块链指标
    /// Update blockchain metrics
    #[allow(dead_code)]
    pub fn update_blockchain_metrics(&self, metrics: BlockchainMetrics) {
        if let Ok(mut bc_metrics) = self.blockchain_metrics.lock() {
            *bc_metrics = metrics;
        }
    }

    /// 更新健康状态
    /// Update health status
    #[allow(dead_code)]
    pub fn update_health_status(&self, status: HealthStatus) {
        if let Ok(mut health) = self.health_status.lock() {
            *health = status;
        }
    }

    /// 获取所有指标
    /// Get all metrics
    #[allow(dead_code)]
    pub fn get_all_metrics(&self) -> HashMap<String, Metric> {
        self.metrics.lock().unwrap().clone()
    }

    /// 获取性能指标
    /// Get performance metrics
    #[allow(dead_code)]
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.performance_metrics.lock().unwrap().clone()
    }

    /// 获取区块链指标
    /// Get blockchain metrics
    #[allow(dead_code)]
    pub fn get_blockchain_metrics(&self) -> BlockchainMetrics {
        self.blockchain_metrics.lock().unwrap().clone()
    }

    /// 获取健康状态
    /// Get health status
    #[allow(dead_code)]
    pub fn get_health_status(&self) -> HealthStatus {
        self.health_status.lock().unwrap().clone()
    }

    /// 获取运行时间
    /// Get uptime
    pub fn get_uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// 计算平均TPS
    /// Calculate average TPS
    #[allow(dead_code)]
    pub fn calculate_tps(&self, transaction_count: u64, time_span: Duration) -> f64 {
        if time_span.as_secs() > 0 {
            transaction_count as f64 / time_span.as_secs() as f64
        } else {
            0.0
        }
    }

    /// 计算平均BPM（每分钟区块数）
    /// Calculate average BPM (blocks per minute)
    #[allow(dead_code)]
    pub fn calculate_bpm(&self, block_count: u64, time_span: Duration) -> f64 {
        if time_span.as_secs() > 0 {
            (block_count as f64 * 60.0) / time_span.as_secs() as f64
        } else {
            0.0
        }
    }

    /// 记录区块挖矿时间
    /// Record block mining time
    pub fn record_block_mining_time(&self, mining_time: Duration) {
        self.record_metric(
            "block_mining_time".to_string(),
            mining_time.as_secs_f64(),
            None,
        );
    }

    /// 记录交易处理时间
    /// Record transaction processing time
    pub fn record_transaction_processing_time(&self, processing_time: Duration) {
        self.record_metric(
            "transaction_processing_time".to_string(),
            processing_time.as_secs_f64(),
            None,
        );
    }

    /// 记录网络延迟
    /// Record network latency
    #[allow(dead_code)]
    pub fn record_network_latency(&self, latency: Duration, peer_id: String) {
        let mut labels = HashMap::new();
        labels.insert("peer_id".to_string(), peer_id);
        self.record_metric(
            "network_latency".to_string(),
            latency.as_millis() as f64,
            Some(labels),
        );
    }

    /// 记录内存使用情况
    /// Record memory usage
    #[allow(dead_code)]
    pub fn record_memory_usage(&self, memory_usage: u64) {
        self.record_metric("memory_usage".to_string(), memory_usage as f64, None);
    }

    /// 记录错误
    /// Record error
    #[allow(dead_code)]
    pub fn record_error(&self, error_type: String, error_message: String) {
        let mut labels = HashMap::new();
        labels.insert("error_type".to_string(), error_type);
        labels.insert("error_message".to_string(), error_message);
        self.record_metric("error_count".to_string(), 1.0, Some(labels));
        
        // 更新健康状态
        if let Ok(mut health) = self.health_status.lock() {
            health.error_count += 1;
            if health.error_count > 10 {
                health.status = "degraded".to_string();
            }
            if health.error_count > 50 {
                health.status = "unhealthy".to_string();
            }
        }
    }

    /// 添加警告
    /// Add warning
    #[allow(dead_code)]
    pub fn add_warning(&self, warning: String) {
        if let Ok(mut health) = self.health_status.lock() {
            health.warnings.push(warning);
            if health.warnings.len() > 5 {
                health.warnings.remove(0); // 保持最多5个警告
            }
            if health.warnings.len() > 3 {
                health.status = "degraded".to_string();
            }
        }
    }

    /// 重置指标
    /// Reset metrics
    #[allow(dead_code)]
    pub fn reset_metrics(&self) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.clear();
        }
    }

    /// 导出指标到JSON
    /// Export metrics to JSON
    #[allow(dead_code)]
    pub fn export_metrics_json(&self) -> Result<String, serde_json::Error> {
        let all_metrics = self.get_all_metrics();
        serde_json::to_string_pretty(&all_metrics)
    }

    /// 生成监控报告
    /// Generate monitoring report
    #[allow(dead_code)]
    pub fn generate_report(&self) -> MonitoringReport {
        let uptime = self.get_uptime();
        let performance = self.get_performance_metrics();
        let blockchain = self.get_blockchain_metrics();
        let health = self.get_health_status();

        MonitoringReport {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            uptime_seconds: uptime.as_secs(),
            performance_metrics: performance,
            blockchain_metrics: blockchain,
            health_status: health,
            total_metrics_count: self.metrics.lock().unwrap().len(),
        }
    }
}

/// 监控报告
/// Monitoring report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MonitoringReport {
    pub timestamp: u64,
    pub uptime_seconds: u64,
    pub performance_metrics: PerformanceMetrics,
    pub blockchain_metrics: BlockchainMetrics,
    pub health_status: HealthStatus,
    pub total_metrics_count: usize,
}

#[allow(dead_code)]
impl Default for BlockchainMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_monitor_creation() {
        let monitor = BlockchainMonitor::new();
        assert_eq!(monitor.get_uptime().as_secs(), 0);
    }

    #[test]
    fn test_metric_recording() {
        let monitor = BlockchainMonitor::new();
        monitor.record_metric("test_metric".to_string(), 42.0, None);
        
        let metrics = monitor.get_all_metrics();
        assert!(metrics.contains_key("test_metric"));
        assert_eq!(metrics["test_metric"].value, 42.0);
    }

    #[test]
    fn test_counter_increment() {
        let monitor = BlockchainMonitor::new();
        monitor.increment_counter("test_counter".to_string(), 10.0);
        monitor.increment_counter("test_counter".to_string(), 5.0);
        
        let metrics = monitor.get_all_metrics();
        assert_eq!(metrics["test_counter"].value, 15.0);
    }

    #[test]
    fn test_performance_calculations() {
        let monitor = BlockchainMonitor::new();
        
        let tps = monitor.calculate_tps(1000, Duration::from_secs(10));
        assert_eq!(tps, 100.0);
        
        let bpm = monitor.calculate_bpm(60, Duration::from_secs(60));
        assert_eq!(bpm, 60.0);
    }

    #[test]
    fn test_health_status_updates() {
        let monitor = BlockchainMonitor::new();
        
        // 添加一些错误
        for _ in 0..15 {
            monitor.record_error("test_error".to_string(), "test message".to_string());
        }
        
        let health = monitor.get_health_status();
        assert_eq!(health.status, "degraded");
        assert_eq!(health.error_count, 15);
    }

    #[test]
    fn test_report_generation() {
        let monitor = BlockchainMonitor::new();
        thread::sleep(Duration::from_millis(10));
        
        let report = monitor.generate_report();
        assert!(report.uptime_seconds > 0);
        assert_eq!(report.performance_metrics.transactions_per_second, 0.0);
    }
}
