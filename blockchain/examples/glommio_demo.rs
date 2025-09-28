// Glommio 高性能异步运行时演示
// 注意: 此示例仅在Linux系统上运行，需要内核版本5.8+

#[cfg(feature = "glommio-runtime")]
use glommio::{LocalExecutor, LocalExecutorBuilder, Timer};
#[cfg(feature = "glommio-runtime")]
use std::time::{Duration, Instant};

#[cfg(feature = "glommio-runtime")]
async fn high_performance_transaction_processing() {
    println!("🚀 Glommio 高性能交易处理演示");
    
    let start = Instant::now();
    let mut processed_count = 0;
    
    // 模拟处理10000个交易
    for i in 0..10000 {
        // 模拟交易处理
        let tx = create_mock_transaction(i);
        process_transaction(tx).await;
        processed_count += 1;
        
        // 每1000个交易显示一次进度
        if processed_count % 1000 == 0 {
            println!("已处理 {} 个交易", processed_count);
        }
    }
    
    let duration = start.elapsed();
    let tps = processed_count as f64 / duration.as_secs_f64();
    
    println!("✅ 处理完成!");
    println!("总交易数: {}", processed_count);
    println!("总耗时: {:?}", duration);
    println!("TPS: {:.2}", tps);
}

#[cfg(feature = "glommio-runtime")]
async fn process_transaction(tx: MockTransaction) {
    // 模拟交易验证
    validate_transaction(&tx).await;
    
    // 模拟交易执行
    execute_transaction(&tx).await;
    
    // 模拟状态更新
    update_state(&tx).await;
}

#[cfg(feature = "glommio-runtime")]
async fn validate_transaction(tx: &MockTransaction) {
    // 模拟验证延迟
    Timer::new(Duration::from_micros(10)).await;
}

#[cfg(feature = "glommio-runtime")]
async fn execute_transaction(tx: &MockTransaction) {
    // 模拟执行延迟
    Timer::new(Duration::from_micros(20)).await;
}

#[cfg(feature = "glommio-runtime")]
async fn update_state(tx: &MockTransaction) {
    // 模拟状态更新延迟
    Timer::new(Duration::from_micros(5)).await;
}

#[cfg(feature = "glommio-runtime")]
fn create_mock_transaction(id: u32) -> MockTransaction {
    MockTransaction {
        id,
        from: format!("address_{}", id % 100),
        to: format!("address_{}", (id + 1) % 100),
        amount: 1000 + (id % 10000),
        fee: 10 + (id % 100),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }
}

#[cfg(feature = "glommio-runtime")]
#[derive(Debug, Clone)]
struct MockTransaction {
    id: u32,
    from: String,
    to: String,
    amount: u64,
    fee: u64,
    timestamp: u64,
}

#[cfg(feature = "glommio-runtime")]
async fn cpu_bound_consensus_algorithm() {
    println!("🧠 Glommio CPU密集型共识算法演示");
    
    let start = Instant::now();
    
    // 模拟共识算法计算
    let mut block_hash = [0u8; 32];
    let mut nonce = 0u64;
    let target_difficulty = 0x0000FFFF; // 模拟挖矿难度
    
    loop {
        // 计算区块哈希
        let hash = calculate_block_hash(nonce, &block_hash);
        
        // 检查是否满足难度要求
        if u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]]) < target_difficulty {
            println!("✅ 找到有效nonce: {}", nonce);
            break;
        }
        
        nonce += 1;
        
        // 防止无限循环
        if nonce > 1000000 {
            println!("⏰ 达到最大尝试次数");
            break;
        }
    }
    
    let duration = start.elapsed();
    println!("挖矿耗时: {:?}", duration);
    println!("尝试次数: {}", nonce);
    println!("哈希率: {:.2} H/s", nonce as f64 / duration.as_secs_f64());
}

#[cfg(feature = "glommio-runtime")]
fn calculate_block_hash(nonce: u64, previous_hash: &[u8; 32]) -> [u8; 32] {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(previous_hash);
    hasher.update(&nonce.to_be_bytes());
    hasher.finalize().into()
}

#[cfg(feature = "glommio-runtime")]
async fn io_intensive_operations() {
    println!("💾 Glommio I/O密集型操作演示");
    
    let start = Instant::now();
    
    // 模拟大量I/O操作
    let mut futures = Vec::new();
    
    for i in 0..1000 {
        let future = async move {
            // 模拟数据库写入
            simulate_database_write(i).await;
            
            // 模拟网络请求
            simulate_network_request(i).await;
            
            // 模拟文件操作
            simulate_file_operation(i).await;
        };
        
        futures.push(future);
    }
    
    // 并发执行所有I/O操作
    futures::future::join_all(futures).await;
    
    let duration = start.elapsed();
    println!("I/O操作完成，耗时: {:?}", duration);
    println!("平均每个操作耗时: {:?}", duration / 1000);
}

#[cfg(feature = "glommio-runtime")]
async fn simulate_database_write(id: u32) {
    // 模拟数据库写入延迟
    Timer::new(Duration::from_millis(1)).await;
}

#[cfg(feature = "glommio-runtime")]
async fn simulate_network_request(id: u32) {
    // 模拟网络请求延迟
    Timer::new(Duration::from_millis(2)).await;
}

#[cfg(feature = "glommio-runtime")]
async fn simulate_file_operation(id: u32) {
    // 模拟文件操作延迟
    Timer::new(Duration::from_millis(1)).await;
}

#[cfg(feature = "glommio-runtime")]
fn main() {
    println!("🔧 Glommio 区块链性能演示");
    println!("注意: 此演示仅在Linux系统上运行");
    println!("需要启用 glommio-runtime 特性");
    println!();
    
    // 创建Glommio执行器
    let ex = LocalExecutorBuilder::new()
        .pin_to_cpu(0)  // 绑定到CPU核心0
        .spawn(|| async {
            println!("📍 执行器绑定到CPU核心0");
            println!();
            
            // 运行高性能交易处理演示
            high_performance_transaction_processing().await;
            println!();
            
            // 运行CPU密集型共识算法演示
            cpu_bound_consensus_algorithm().await;
            println!();
            
            // 运行I/O密集型操作演示
            io_intensive_operations().await;
            println!();
            
            println!("🎉 所有演示完成!");
        })
        .unwrap();
    
    ex.join().unwrap();
}

#[cfg(not(feature = "glommio-runtime"))]
fn main() {
    println!("❌ Glommio 演示需要启用 glommio-runtime 特性");
    println!("请使用以下命令运行:");
    println!("cargo run --example glommio_demo --features glommio-runtime");
    println!();
    println!("注意: Glommio 仅在 Linux 系统上支持");
    println!("需要内核版本 5.8 或更高");
}
