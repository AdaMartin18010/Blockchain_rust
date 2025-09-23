//! # Web API 接口模块
//! 
//! 提供区块链的 REST API 接口

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

use crate::simple_blockchain::Blockchain;
use crate::monitoring::{BlockchainMonitor, PerformanceMetrics, BlockchainMetrics, HealthStatus};

/// API 错误类型
#[allow(dead_code)]
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ApiError {
    #[error("Invalid request")]
    InvalidRequest,
    #[error("Blockchain error: {0}")]
    BlockchainError(String),
    #[error("Serialization error")]
    SerializationError,
    #[error("Not found")]
    NotFound,
}

/// API 响应结构
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[allow(dead_code)]
impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// 区块链信息响应
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub chain_length: usize,
    pub difficulty: usize,
    pub pending_transactions: usize,
    pub latest_block_hash: Option<String>,
}

/// 交易信息
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub timestamp: u64,
}

/// 区块信息
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    pub index: u64,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<TransactionInfo>,
    pub nonce: u64,
}

/// 余额信息
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceInfo {
    pub address: String,
    pub balance: u64,
}

/// 挖矿请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningRequest {
    pub miner_address: String,
    pub count: Option<u32>,
}

/// 挖矿响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningResponse {
    pub success: bool,
    pub blocks_mined: u32,
    pub total_time: f64,
    pub hash_rate: f64,
}

/// 监控指标响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringResponse {
    pub timestamp: u64,
    pub uptime: u64,
    pub performance: PerformanceMetrics,
    pub blockchain: BlockchainMetrics,
    pub health: HealthStatus,
}

/// 节点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub node_id: String,
    pub version: String,
    pub uptime: u64,
    pub connected_peers: u32,
    pub sync_status: String,
}

/// 网络统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub total_connections: u32,
    pub active_connections: u32,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
}

/// 搜索请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub search_type: SearchType,
    pub limit: Option<usize>,
}

/// 搜索类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    Transaction,
    Block,
    Address,
    All,
}

/// 搜索响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total_count: usize,
}

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub result_type: SearchType,
    pub data: serde_json::Value,
    pub relevance_score: f64,
}

/// Web API 服务器
#[allow(dead_code)]
pub struct WebApiServer {
    blockchain: Arc<Mutex<Blockchain>>,
    monitor: Arc<BlockchainMonitor>,
    node_info: NodeInfo,
    network_stats: Arc<Mutex<NetworkStats>>,
}

#[allow(dead_code)]
impl WebApiServer {
    /// 创建新的 Web API 服务器
    pub fn new(blockchain: Blockchain) -> Self {
        let node_info = NodeInfo {
            node_id: uuid::Uuid::new_v4().to_string(),
            version: "1.0.0".to_string(),
            uptime: 0,
            connected_peers: 0,
            sync_status: "synced".to_string(),
        };

        let network_stats = NetworkStats {
            total_connections: 0,
            active_connections: 0,
            bytes_sent: 0,
            bytes_received: 0,
            messages_sent: 0,
            messages_received: 0,
        };

        Self {
            blockchain: Arc::new(Mutex::new(blockchain)),
            monitor: Arc::new(BlockchainMonitor::new()),
            node_info,
            network_stats: Arc::new(Mutex::new(network_stats)),
        }
    }
    
    /// 获取区块链信息
    pub fn get_blockchain_info(&self) -> Result<ApiResponse<BlockchainInfo>, ApiError> {
        let blockchain = self.blockchain.lock().unwrap();
        
        let info = BlockchainInfo {
            chain_length: blockchain.get_chain_length(),
            difficulty: blockchain.difficulty,
            pending_transactions: blockchain.pending_transactions.len(),
            latest_block_hash: blockchain.get_latest_block()
                .map(|block| block.hash.to_string()),
        };
        
        Ok(ApiResponse::success(info))
    }
    
    /// 获取区块信息
    pub fn get_block(&self, index: usize) -> Result<ApiResponse<BlockInfo>, ApiError> {
        let blockchain = self.blockchain.lock().unwrap();
        
        if let Some(block) = blockchain.chain.get(index) {
            let block_info = BlockInfo {
                index: block.index,
                hash: block.hash.to_string(),
                prev_hash: block.prev_hash.to_string(),
                timestamp: block.timestamp,
                transactions: block.transactions.iter().map(|tx| TransactionInfo {
                    sender: tx.sender.clone(),
                    receiver: tx.receiver.clone(),
                    amount: tx.amount,
                    timestamp: tx.timestamp,
                }).collect(),
                nonce: block.nonce,
            };
            
            Ok(ApiResponse::success(block_info))
        } else {
            Ok(ApiResponse::error("Block not found".to_string()))
        }
    }
    
    /// 获取余额
    pub fn get_balance(&self, address: &str) -> Result<ApiResponse<BalanceInfo>, ApiError> {
        let blockchain = self.blockchain.lock().unwrap();
        let balance = blockchain.get_balance(address);
        
        let balance_info = BalanceInfo {
            address: address.to_string(),
            balance,
        };
        
        Ok(ApiResponse::success(balance_info))
    }
    
    /// 添加交易
    pub fn add_transaction(&self, sender: String, receiver: String, amount: u64) -> Result<ApiResponse<String>, ApiError> {
        let mut blockchain = self.blockchain.lock().unwrap();
        
        let transaction = super::simple_blockchain::Transaction::new(sender.clone(), receiver.clone(), amount);
        
        match blockchain.add_transaction(transaction) {
            Ok(_) => Ok(ApiResponse::success("Transaction added successfully".to_string())),
            Err(e) => Ok(ApiResponse::error(format!("Failed to add transaction: {}", e))),
        }
    }
    
    /// 挖矿
    pub fn mine(&self, reward_address: String) -> Result<ApiResponse<String>, ApiError> {
        let mut blockchain = self.blockchain.lock().unwrap();
        
        match blockchain.mine_pending_transactions(reward_address) {
            Ok(_) => Ok(ApiResponse::success("Mining completed successfully".to_string())),
            Err(e) => Ok(ApiResponse::error(format!("Mining failed: {}", e))),
        }
    }
    
    /// 获取所有地址的余额
    pub fn get_all_balances(&self) -> Result<ApiResponse<Vec<BalanceInfo>>, ApiError> {
        let blockchain = self.blockchain.lock().unwrap();
        
        let mut balances = Vec::new();
        for (address, balance) in &blockchain.balances {
            balances.push(BalanceInfo {
                address: address.clone(),
                balance: *balance,
            });
        }
        
        Ok(ApiResponse::success(balances))
    }
    
    /// 获取链验证状态
    pub fn validate_chain(&self) -> Result<ApiResponse<bool>, ApiError> {
        let blockchain = self.blockchain.lock().unwrap();
        let is_valid = blockchain.is_valid_chain();
        Ok(ApiResponse::success(is_valid))
    }

    /// 开始挖矿
    pub fn start_mining(&self, request: MiningRequest) -> Result<ApiResponse<MiningResponse>, ApiError> {
        let start_time = SystemTime::now();
        let mut blockchain = self.blockchain.lock().unwrap();
        
        let count = request.count.unwrap_or(1);
        let mut blocks_mined = 0;
        
        for _ in 0..count {
            if let Err(e) = blockchain.mine_pending_transactions(request.miner_address.clone()) {
                return Ok(ApiResponse::error(format!("挖矿失败: {}", e)));
            }
            blocks_mined += 1;
        }
        
        let total_time = start_time.elapsed().unwrap_or_default().as_secs_f64();
        let hash_rate = if total_time > 0.0 {
            blocks_mined as f64 / total_time
        } else {
            0.0
        };
        
        let response = MiningResponse {
            success: true,
            blocks_mined,
            total_time,
            hash_rate,
        };
        
        Ok(ApiResponse::success(response))
    }

    /// 获取监控指标
    pub fn get_monitoring_metrics(&self) -> Result<ApiResponse<MonitoringResponse>, ApiError> {
        let performance = self.monitor.get_performance_metrics();
        let blockchain = self.monitor.get_blockchain_metrics();
        let health = self.monitor.get_health_status();
        let uptime = self.monitor.get_uptime().as_secs();
        
        let response = MonitoringResponse {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            uptime,
            performance,
            blockchain,
            health,
        };
        
        Ok(ApiResponse::success(response))
    }

    /// 获取节点信息
    pub fn get_node_info(&self) -> Result<ApiResponse<NodeInfo>, ApiError> {
        let mut node_info = self.node_info.clone();
        node_info.uptime = self.monitor.get_uptime().as_secs();
        
        Ok(ApiResponse::success(node_info))
    }

    /// 获取网络统计
    pub fn get_network_stats(&self) -> Result<ApiResponse<NetworkStats>, ApiError> {
        let stats = self.network_stats.lock().unwrap().clone();
        Ok(ApiResponse::success(stats))
    }

    /// 搜索功能
    pub fn search(&self, request: SearchRequest) -> Result<ApiResponse<SearchResponse>, ApiError> {
        let blockchain = self.blockchain.lock().unwrap();
        let mut results = Vec::new();
        let limit = request.limit.unwrap_or(50);
        
        match request.search_type {
            SearchType::Transaction => {
                self.search_transactions(&blockchain, &request.query, &mut results, limit);
            }
            SearchType::Block => {
                self.search_blocks(&blockchain, &request.query, &mut results, limit);
            }
            SearchType::Address => {
                self.search_addresses(&blockchain, &request.query, &mut results, limit);
            }
            SearchType::All => {
                let tx_limit = limit / 3;
                let block_limit = limit / 3;
                let addr_limit = limit / 3;
                
                self.search_transactions(&blockchain, &request.query, &mut results, tx_limit);
                self.search_blocks(&blockchain, &request.query, &mut results, block_limit);
                self.search_addresses(&blockchain, &request.query, &mut results, addr_limit);
            }
        }
        
        let total_count = results.len();
        let response = SearchResponse {
            results,
            total_count,
        };
        
        Ok(ApiResponse::success(response))
    }

    /// 搜索交易
    fn search_transactions(&self, blockchain: &Blockchain, query: &str, results: &mut Vec<SearchResult>, limit: usize) {
        let mut count = 0;
        for block in &blockchain.chain {
            for tx in &block.transactions {
                if count >= limit {
                    break;
                }
                
                if tx.sender.contains(query) || tx.receiver.contains(query) || 
                   tx.amount.to_string().contains(query) {
                    let relevance = self.calculate_relevance(&tx.sender, query) + 
                                   self.calculate_relevance(&tx.receiver, query);
                    
                    results.push(SearchResult {
                        result_type: SearchType::Transaction,
                        data: serde_json::to_value(tx).unwrap_or_default(),
                        relevance_score: relevance,
                    });
                    count += 1;
                }
            }
            if count >= limit {
                break;
            }
        }
    }

    /// 搜索区块
    fn search_blocks(&self, blockchain: &Blockchain, query: &str, results: &mut Vec<SearchResult>, limit: usize) {
        let mut count = 0;
        for block in &blockchain.chain {
            if count >= limit {
                break;
            }
            
            if block.hash.to_string().contains(query) || 
               block.index.to_string().contains(query) {
                let relevance = if block.hash.to_string().contains(query) { 1.0 } else { 0.5 };
                
                results.push(SearchResult {
                    result_type: SearchType::Block,
                    data: serde_json::to_value(block).unwrap_or_default(),
                    relevance_score: relevance,
                });
                count += 1;
            }
        }
    }

    /// 搜索地址
    fn search_addresses(&self, blockchain: &Blockchain, query: &str, results: &mut Vec<SearchResult>, limit: usize) {
        let mut count = 0;
        for (address, balance) in &blockchain.balances {
            if count >= limit {
                break;
            }
            
            if address.contains(query) {
                let relevance = self.calculate_relevance(address, query);
                
                let balance_info = BalanceInfo {
                    address: address.clone(),
                    balance: *balance,
                };
                
                results.push(SearchResult {
                    result_type: SearchType::Address,
                    data: serde_json::to_value(balance_info).unwrap_or_default(),
                    relevance_score: relevance,
                });
                count += 1;
            }
        }
    }

    /// 计算相关性分数
    fn calculate_relevance(&self, text: &str, query: &str) -> f64 {
        if text == query {
            1.0
        } else if text.starts_with(query) {
            0.8
        } else if text.contains(query) {
            0.6
        } else {
            0.0
        }
    }

    /// 更新网络统计
    pub fn update_network_stats(&self, bytes_sent: u64, bytes_received: u64) {
        if let Ok(mut stats) = self.network_stats.lock() {
            stats.bytes_sent += bytes_sent;
            stats.bytes_received += bytes_received;
            stats.messages_sent += 1;
            stats.messages_received += 1;
        }
    }

    /// 获取所有账户余额
    pub fn get_all_account_balances(&self) -> Result<ApiResponse<Vec<BalanceInfo>>, ApiError> {
        let blockchain = self.blockchain.lock().unwrap();
        let balances: Vec<BalanceInfo> = blockchain.balances
            .iter()
            .map(|(address, balance)| BalanceInfo {
                address: address.clone(),
                balance: *balance,
            })
            .collect();
        
        Ok(ApiResponse::success(balances))
    }

    /// 获取交易统计
    pub fn get_transaction_stats(&self) -> Result<ApiResponse<HashMap<String, u64>>, ApiError> {
        let blockchain = self.blockchain.lock().unwrap();
        let mut stats = HashMap::new();
        
        let mut total_transactions = 0;
        let mut total_amount = 0;
        let mut unique_addresses = std::collections::HashSet::new();
        
        for block in &blockchain.chain {
            for tx in &block.transactions {
                total_transactions += 1;
                total_amount += tx.amount;
                unique_addresses.insert(tx.sender.clone());
                unique_addresses.insert(tx.receiver.clone());
            }
        }
        
        stats.insert("total_transactions".to_string(), total_transactions);
        stats.insert("total_amount".to_string(), total_amount);
        stats.insert("unique_addresses".to_string(), unique_addresses.len() as u64);
        stats.insert("pending_transactions".to_string(), blockchain.pending_transactions.len() as u64);
        
        Ok(ApiResponse::success(stats))
    }
}

/// 简化的 HTTP 请求处理器 (模拟)
#[allow(dead_code)]
pub struct HttpHandler {
    api_server: WebApiServer,
}

#[allow(dead_code)]
impl HttpHandler {
    /// 创建新的 HTTP 处理器
    pub fn new(api_server: WebApiServer) -> Self {
        Self { api_server }
    }
    
    /// 处理 GET 请求
    pub fn handle_get(&self, path: &str) -> Result<String, ApiError> {
        match path {
            "/info" => {
                let response = self.api_server.get_blockchain_info()?;
                Ok(serde_json::to_string(&response)
                    .map_err(|_| ApiError::SerializationError)?)
            }
            "/balances" => {
                let response = self.api_server.get_all_balances()?;
                Ok(serde_json::to_string(&response)
                    .map_err(|_| ApiError::SerializationError)?)
            }
            "/validate" => {
                let response = self.api_server.validate_chain()?;
                Ok(serde_json::to_string(&response)
                    .map_err(|_| ApiError::SerializationError)?)
            }
            _ => {
                // 尝试解析区块请求 /block/{index}
                if path.starts_with("/block/") {
                    let index_str = path.strip_prefix("/block/").unwrap();
                    if let Ok(index) = index_str.parse::<usize>() {
                        let response = self.api_server.get_block(index)?;
                        return Ok(serde_json::to_string(&response)
                            .map_err(|_| ApiError::SerializationError)?);
                    }
                }
                
                // 尝试解析余额请求 /balance/{address}
                if path.starts_with("/balance/") {
                    let address = path.strip_prefix("/balance/").unwrap();
                    let response = self.api_server.get_balance(address)?;
                    return Ok(serde_json::to_string(&response)
                        .map_err(|_| ApiError::SerializationError)?);
                }
                
                Ok(serde_json::to_string(&ApiResponse::<()>::error("Not found".to_string()))
                    .map_err(|_| ApiError::SerializationError)?)
            }
        }
    }
    
    /// 处理 POST 请求
    pub fn handle_post(&self, path: &str, body: &str) -> Result<String, ApiError> {
        match path {
            "/transaction" => {
                let data: HashMap<String, String> = serde_json::from_str(body)
                    .map_err(|_| ApiError::SerializationError)?;
                
                let sender = data.get("sender").ok_or(ApiError::InvalidRequest)?.clone();
                let receiver = data.get("receiver").ok_or(ApiError::InvalidRequest)?.clone();
                let amount = data.get("amount").ok_or(ApiError::InvalidRequest)?
                    .parse::<u64>().map_err(|_| ApiError::InvalidRequest)?;
                
                let response = self.api_server.add_transaction(sender, receiver, amount)?;
                Ok(serde_json::to_string(&response)
                    .map_err(|_| ApiError::SerializationError)?)
            }
            "/mine" => {
                let data: HashMap<String, String> = serde_json::from_str(body)
                    .map_err(|_| ApiError::SerializationError)?;
                
                let reward_address = data.get("reward_address").ok_or(ApiError::InvalidRequest)?.clone();
                
                let response = self.api_server.mine(reward_address)?;
                Ok(serde_json::to_string(&response)
                    .map_err(|_| ApiError::SerializationError)?)
            }
            _ => Ok(serde_json::to_string(&ApiResponse::<()>::error("Not found".to_string()))
                .map_err(|_| ApiError::SerializationError)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response() {
        let success_response = ApiResponse::success("test data");
        assert!(success_response.success);
        assert_eq!(success_response.data, Some("test data"));
        
        let error_response = ApiResponse::<()>::error("test error".to_string());
        assert!(!error_response.success);
        assert_eq!(error_response.error, Some("test error".to_string()));
    }

    #[test]
    fn test_web_api_server() {
        let blockchain = super::super::simple_blockchain::Blockchain::new(2);
        let api_server = WebApiServer::new(blockchain);
        
        let info_response = api_server.get_blockchain_info().unwrap();
        assert!(info_response.success);
        assert!(info_response.data.is_some());
    }
}
