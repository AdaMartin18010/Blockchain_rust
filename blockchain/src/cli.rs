//! # 区块链命令行工具
//! 
//! 提供命令行界面用于区块链管理和操作
//! Provides command-line interface for blockchain management and operations

use std::path::PathBuf;
use clap::{Parser, Subcommand, ValueEnum};

use crate::simple_blockchain::{Blockchain, Transaction};
use crate::monitoring::BlockchainMonitor;

/// 区块链命令行工具
/// Blockchain CLI Tool
#[derive(Parser)]
#[command(name = "blockchain-cli")]
#[command(about = "区块链管理命令行工具")]
#[command(version = "1.0.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    /// 配置文件路径
    /// Configuration file path
    #[arg(short, long, default_value = "blockchain_config.json")]
    pub config: PathBuf,
    
    /// 数据目录路径
    /// Data directory path
    #[arg(short, long, default_value = "./data")]
    pub data_dir: PathBuf,
    
    /// 启用详细输出
    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

/// 命令枚举
/// Commands enum
#[derive(Subcommand)]
pub enum Commands {
    /// 初始化新的区块链
    /// Initialize new blockchain
    Init {
        /// 挖矿难度
        /// Mining difficulty
        #[arg(short, long, default_value = "2")]
        difficulty: usize,
        
        /// 创世账户余额
        /// Genesis account balance
        #[arg(short, long, default_value = "1000000")]
        genesis_balance: u64,
    },
    
    /// 显示区块链信息
    /// Show blockchain information
    Info {
        /// 显示详细信息
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },
    
    /// 挖矿操作
    /// Mining operations
    Mine {
        /// 矿工地址
        /// Miner address
        #[arg(short, long, default_value = "miner")]
        address: String,
        
        /// 挖矿数量
        /// Number of blocks to mine
        #[arg(short, long, default_value = "1")]
        count: u32,
    },
    
    /// 交易操作
    /// Transaction operations
    Transaction {
        #[command(subcommand)]
        tx_command: TransactionCommands,
    },
    
    /// 监控操作
    /// Monitoring operations
    Monitor {
        #[command(subcommand)]
        monitor_command: MonitorCommands,
    },
    
    /// 网络操作
    /// Network operations
    Network {
        #[command(subcommand)]
        network_command: NetworkCommands,
    },
    
    /// 验证区块链
    /// Validate blockchain
    Validate,
    
    /// 导出数据
    /// Export data
    Export {
        /// 导出格式
        /// Export format
        #[arg(short, long, value_enum, default_value = "json")]
        format: ExportFormat,
        
        /// 输出文件路径
        /// Output file path
        #[arg(short, long, default_value = "blockchain_export.json")]
        output: PathBuf,
    },
    
    /// 导入数据
    /// Import data
    Import {
        /// 输入文件路径
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,
    },
}

/// 交易子命令
/// Transaction subcommands
#[derive(Subcommand)]
pub enum TransactionCommands {
    /// 创建新交易
    /// Create new transaction
    Create {
        /// 发送者地址
        /// Sender address
        #[arg(short, long)]
        from: String,
        
        /// 接收者地址
        /// Receiver address
        #[arg(short, long)]
        to: String,
        
        /// 交易金额
        /// Transaction amount
        #[arg(short, long)]
        amount: u64,
    },
    
    /// 列出待处理交易
    /// List pending transactions
    Pending,
    
    /// 显示交易历史
    /// Show transaction history
    History {
        /// 地址过滤器
        /// Address filter
        #[arg(short, long)]
        address: Option<String>,
        
        /// 限制显示数量
        /// Limit number of results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
}

/// 监控子命令
/// Monitor subcommands
#[derive(Subcommand)]
pub enum MonitorCommands {
    /// 显示实时指标
    /// Show real-time metrics
    Metrics,
    
    /// 显示性能统计
    /// Show performance statistics
    Performance,
    
    /// 显示健康状态
    /// Show health status
    Health,
    
    /// 生成监控报告
    /// Generate monitoring report
    Report {
        /// 输出文件路径
        /// Output file path
        #[arg(short, long, default_value = "monitoring_report.json")]
        output: PathBuf,
    },
}

/// 网络子命令
/// Network subcommands
#[derive(Subcommand)]
pub enum NetworkCommands {
    /// 连接到对等节点
    /// Connect to peer node
    Connect {
        /// 节点地址
        /// Node address
        #[arg(short, long)]
        address: String,
    },
    
    /// 断开连接
    /// Disconnect
    Disconnect {
        /// 节点地址
        /// Node address
        #[arg(short, long)]
        address: String,
    },
    
    /// 列出连接的节点
    /// List connected nodes
    List,
    
    /// 广播消息
    /// Broadcast message
    Broadcast {
        /// 消息内容
        /// Message content
        #[arg(short, long)]
        message: String,
    },
}

/// 导出格式
/// Export formats
#[derive(ValueEnum, Clone)]
pub enum ExportFormat {
    Json,
    Csv,
    Xml,
    Binary,
}

/// CLI 处理器
/// CLI Handler
#[allow(dead_code)]
pub struct CliHandler {
    blockchain: Option<Blockchain>,
    monitor: BlockchainMonitor,
    data_dir: PathBuf,
    verbose: bool,
}

#[allow(dead_code)]
impl CliHandler {
    /// 创建新的 CLI 处理器
    /// Create new CLI handler
    pub fn new(data_dir: PathBuf, verbose: bool) -> Self {
        Self {
            blockchain: None,
            monitor: BlockchainMonitor::new(),
            data_dir,
            verbose,
        }
    }

    /// 加载区块链
    /// Load blockchain
    pub fn load_blockchain(&mut self, difficulty: usize) -> Result<(), String> {
        self.log("正在加载区块链...");
        self.blockchain = Some(Blockchain::new(difficulty));
        self.log("区块链加载完成");
        Ok(())
    }

    /// 处理命令
    /// Handle command
    pub fn handle_command(&mut self, command: Commands) -> Result<(), String> {
        match command {
            Commands::Init { difficulty, genesis_balance } => {
                self.handle_init(difficulty, genesis_balance)
            }
            Commands::Info { detailed } => {
                self.handle_info(detailed)
            }
            Commands::Mine { address, count } => {
                self.handle_mine(address, count)
            }
            Commands::Transaction { tx_command } => {
                self.handle_transaction(tx_command)
            }
            Commands::Monitor { monitor_command } => {
                self.handle_monitor(monitor_command)
            }
            Commands::Network { network_command } => {
                self.handle_network(network_command)
            }
            Commands::Validate => {
                self.handle_validate()
            }
            Commands::Export { format, output } => {
                self.handle_export(format, output)
            }
            Commands::Import { input } => {
                self.handle_import(input)
            }
        }
    }

    /// 处理初始化命令
    /// Handle init command
    fn handle_init(&mut self, difficulty: usize, genesis_balance: u64) -> Result<(), String> {
        self.log(&format!("初始化区块链，难度: {}, 创世余额: {}", difficulty, genesis_balance));
        
        let mut blockchain = Blockchain::new(difficulty);
        
        // 设置创世账户余额
        blockchain.balances.insert("genesis".to_string(), genesis_balance);
        
        self.blockchain = Some(blockchain);
        self.log("区块链初始化完成");
        Ok(())
    }

    /// 处理信息命令
    /// Handle info command
    fn handle_info(&mut self, detailed: bool) -> Result<(), String> {
        let blockchain = self.get_blockchain()?;
        
        println!("=== 区块链信息 ===");
        println!("链高度: {}", blockchain.get_chain_length());
        println!("难度: {}", blockchain.difficulty);
        println!("待处理交易数: {}", blockchain.pending_transactions.len());
        
        if detailed {
            println!("\n=== 详细统计 ===");
            println!("总交易数: {}", self.count_total_transactions(blockchain));
            println!("总余额: {}", self.calculate_total_balance(blockchain));
            
            if let Some(latest_block) = blockchain.get_latest_block() {
                println!("最新区块哈希: {}", latest_block.hash.to_string());
                println!("最新区块时间戳: {}", latest_block.timestamp);
            }
        }
        
        Ok(())
    }

    /// 处理挖矿命令
    /// Handle mine command
    fn handle_mine(&mut self, address: String, count: u32) -> Result<(), String> {
        self.log(&format!("开始挖矿，矿工地址: {}, 挖矿数量: {}", address, count));
        
        for i in 1..=count {
            let start_time = std::time::Instant::now();
            
            {
                let blockchain = self.get_blockchain_mut()?;
                blockchain.mine_pending_transactions(address.clone())?;
            }
            
            let mining_time = start_time.elapsed();
            self.monitor.record_block_mining_time(mining_time);
            
            self.log(&format!("第 {} 个区块挖矿完成，耗时: {:?}", i, mining_time));
        }
        
        Ok(())
    }

    /// 处理交易命令
    /// Handle transaction command
    fn handle_transaction(&mut self, tx_command: TransactionCommands) -> Result<(), String> {
        match tx_command {
            TransactionCommands::Create { from, to, amount } => {
                self.handle_create_transaction(from, to, amount)
            }
            TransactionCommands::Pending => {
                self.handle_list_pending_transactions()
            }
            TransactionCommands::History { address, limit } => {
                self.handle_transaction_history(address, limit)
            }
        }
    }

    /// 处理创建交易
    /// Handle create transaction
    fn handle_create_transaction(&mut self, from: String, to: String, amount: u64) -> Result<(), String> {
        let blockchain = self.get_blockchain_mut()?;
        
        let start_time = std::time::Instant::now();
        let transaction = Transaction::new(from.clone(), to.clone(), amount);
        blockchain.add_transaction(transaction)?;
        
        let processing_time = start_time.elapsed();
        self.monitor.record_transaction_processing_time(processing_time);
        
        self.log(&format!("交易创建成功: {} -> {} ({})", from, to, amount));
        Ok(())
    }

    /// 处理列出待处理交易
    /// Handle list pending transactions
    fn handle_list_pending_transactions(&self) -> Result<(), String> {
        let blockchain = self.get_blockchain()?;
        
        println!("=== 待处理交易 ===");
        if blockchain.pending_transactions.is_empty() {
            println!("暂无待处理交易");
        } else {
            for (i, tx) in blockchain.pending_transactions.iter().enumerate() {
                println!("{}. {} -> {} ({})", i + 1, tx.sender, tx.receiver, tx.amount);
            }
        }
        
        Ok(())
    }

    /// 处理交易历史
    /// Handle transaction history
    fn handle_transaction_history(&self, address: Option<String>, limit: usize) -> Result<(), String> {
        let blockchain = self.get_blockchain()?;
        
        println!("=== 交易历史 ===");
        let mut count = 0;
        
        for block in &blockchain.chain {
            for tx in &block.transactions {
                if let Some(ref addr) = address {
                    if tx.sender != *addr && tx.receiver != *addr {
                        continue;
                    }
                }
                
                if count >= limit {
                    break;
                }
                
                println!("区块 {}: {} -> {} ({})", 
                    block.index, tx.sender, tx.receiver, tx.amount);
                count += 1;
            }
            
            if count >= limit {
                break;
            }
        }
        
        Ok(())
    }

    /// 处理监控命令
    /// Handle monitor command
    fn handle_monitor(&self, monitor_command: MonitorCommands) -> Result<(), String> {
        match monitor_command {
            MonitorCommands::Metrics => {
                self.handle_show_metrics()
            }
            MonitorCommands::Performance => {
                self.handle_show_performance()
            }
            MonitorCommands::Health => {
                self.handle_show_health()
            }
            MonitorCommands::Report { output } => {
                self.handle_generate_report(output)
            }
        }
    }

    /// 显示指标
    /// Show metrics
    fn handle_show_metrics(&self) -> Result<(), String> {
        let metrics = self.monitor.get_all_metrics();
        
        println!("=== 监控指标 ===");
        for (name, metric) in metrics {
            println!("{}: {} (时间戳: {})", name, metric.value, metric.timestamp);
        }
        
        Ok(())
    }

    /// 显示性能统计
    /// Show performance statistics
    fn handle_show_performance(&self) -> Result<(), String> {
        let perf_metrics = self.monitor.get_performance_metrics();
        
        println!("=== 性能统计 ===");
        println!("交易/秒: {:.2}", perf_metrics.transactions_per_second);
        println!("区块/分钟: {:.2}", perf_metrics.blocks_per_minute);
        println!("平均区块时间: {:.2}秒", perf_metrics.average_block_time);
        println!("平均交易大小: {:.2}字节", perf_metrics.average_transaction_size);
        println!("内存使用: {}字节", perf_metrics.memory_usage);
        println!("CPU使用率: {:.2}%", perf_metrics.cpu_usage);
        println!("网络吞吐量: {:.2}Mbps", perf_metrics.network_throughput);
        
        Ok(())
    }

    /// 显示健康状态
    /// Show health status
    fn handle_show_health(&self) -> Result<(), String> {
        let health = self.monitor.get_health_status();
        
        println!("=== 健康状态 ===");
        println!("状态: {}", health.status);
        println!("错误计数: {}", health.error_count);
        println!("同步状态: {}", health.sync_status);
        
        if !health.warnings.is_empty() {
            println!("警告:");
            for warning in &health.warnings {
                println!("  - {}", warning);
            }
        }
        
        Ok(())
    }

    /// 生成监控报告
    /// Generate monitoring report
    fn handle_generate_report(&self, output: PathBuf) -> Result<(), String> {
        let report = self.monitor.generate_report();
        let json = serde_json::to_string_pretty(&report)
            .map_err(|e| format!("序列化报告失败: {}", e))?;
        
        std::fs::write(&output, json)
            .map_err(|e| format!("写入报告文件失败: {}", e))?;
        
        println!("监控报告已生成: {}", output.display());
        Ok(())
    }

    /// 处理网络命令
    /// Handle network command
    fn handle_network(&self, _network_command: NetworkCommands) -> Result<(), String> {
        // 网络功能需要 P2P 模块支持
        self.log("网络功能需要启用 P2P 特性");
        Ok(())
    }

    /// 处理验证命令
    /// Handle validate command
    fn handle_validate(&self) -> Result<(), String> {
        let blockchain = self.get_blockchain()?;
        
        self.log("开始验证区块链...");
        let is_valid = blockchain.is_valid_chain();
        
        if is_valid {
            println!("✅ 区块链验证通过");
        } else {
            println!("❌ 区块链验证失败");
            return Err("区块链验证失败".to_string());
        }
        
        Ok(())
    }

    /// 处理导出命令
    /// Handle export command
    fn handle_export(&self, format: ExportFormat, output: PathBuf) -> Result<(), String> {
        let blockchain = self.get_blockchain()?;
        
        match format {
            ExportFormat::Json => {
                let json = serde_json::to_string_pretty(blockchain)
                    .map_err(|e| format!("序列化失败: {}", e))?;
                std::fs::write(&output, json)
                    .map_err(|e| format!("写入文件失败: {}", e))?;
            }
            _ => {
                return Err("暂不支持该导出格式".to_string());
            }
        }
        
        println!("数据已导出到: {}", output.display());
        Ok(())
    }

    /// 处理导入命令
    /// Handle import command
    fn handle_import(&mut self, _input: PathBuf) -> Result<(), String> {
        // 导入功能实现
        self.log("导入功能开发中...");
        Ok(())
    }

    /// 获取区块链引用
    /// Get blockchain reference
    fn get_blockchain(&self) -> Result<&Blockchain, String> {
        self.blockchain.as_ref()
            .ok_or_else(|| "区块链未初始化".to_string())
    }

    /// 获取区块链可变引用
    /// Get mutable blockchain reference
    fn get_blockchain_mut(&mut self) -> Result<&mut Blockchain, String> {
        self.blockchain.as_mut()
            .ok_or_else(|| "区块链未初始化".to_string())
    }

    /// 统计总交易数
    /// Count total transactions
    fn count_total_transactions(&self, blockchain: &Blockchain) -> u64 {
        blockchain.chain.iter()
            .map(|block| block.transactions.len() as u64)
            .sum()
    }

    /// 计算总余额
    /// Calculate total balance
    fn calculate_total_balance(&self, blockchain: &Blockchain) -> u64 {
        blockchain.balances.values().sum()
    }

    /// 日志输出
    /// Log output
    fn log(&self, message: &str) {
        if self.verbose {
            println!("[LOG] {}", message);
        }
    }
}

/// 运行 CLI
/// Run CLI
#[allow(dead_code)]
pub fn run_cli() -> Result<(), String> {
    let cli = <Cli as clap::Parser>::parse();
    
    let mut handler = CliHandler::new(cli.data_dir, cli.verbose);
    
    // 如果命令不是初始化，尝试加载现有区块链
    if !matches!(cli.command, Commands::Init { .. }) {
        handler.load_blockchain(2)?;
    }
    
    handler.handle_command(cli.command)
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_handler_creation() {
        let handler = CliHandler::new(PathBuf::from("./test_data"), false);
        assert_eq!(handler.verbose, false);
    }

    #[test]
    fn test_blockchain_loading() {
        let mut handler = CliHandler::new(PathBuf::from("./test_data"), false);
        let result = handler.load_blockchain(2);
        assert!(result.is_ok());
        assert!(handler.blockchain.is_some());
    }
}
