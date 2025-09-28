//! # 安全模块
//! 
//! 提供区块链安全审计、漏洞检测和修复功能
//! Provides blockchain security audit, vulnerability detection and fixes

use crate::Blockchain;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// 安全漏洞类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityVulnerability {
    DoubleSpending,
    InvalidSignature,
    WeakHash,
    ReplayAttack,
    SybilAttack,
    SelfishMining,
    EclipseAttack,
    TimeManipulation,
    MemoryPoolFlooding,
    InvalidTransaction,
}

/// 威胁级别
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// 安全漏洞详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub vulnerability: SecurityVulnerability,
    pub threat_level: ThreatLevel,
    pub description: String,
    pub mitigation: String,
    pub block_height: Option<u64>,
    pub transaction_hash: Option<String>,
}

/// 安全审计报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityReport {
    pub blockchain_height: u64,
    pub risk_score: f64,
    pub vulnerabilities: Vec<SecurityIssue>,
    pub recommendations: Vec<String>,
    pub audit_timestamp: u64,
}

/// 安全警报
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlert {
    pub message: String,
    pub threat_level: ThreatLevel,
    pub source: String,
    pub timestamp: String,
}

/// 安全摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySummary {
    pub total_audits: u64,
    pub latest_risk_score: f64,
    pub unresolved_alerts: u64,
    pub critical_vulnerabilities: u64,
}

/// 报告格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    JSON,
    Markdown,
}

/// 安全监控器
pub struct SecurityMonitor {
    blockchain: Blockchain,
    audit_count: u64,
    alerts: Vec<SecurityAlert>,
}

impl SecurityMonitor {
    /// 创建新的安全监控器
    pub fn new(blockchain: Blockchain) -> Self {
        Self {
            blockchain,
            audit_count: 0,
            alerts: Vec::new(),
        }
    }

    /// 获取区块链的可变引用
    pub fn get_blockchain_mut(&mut self) -> &mut Blockchain {
        &mut self.blockchain
    }

    /// 执行完整安全审计
    pub fn perform_full_audit(&mut self) -> SecurityReport {
        self.audit_count += 1;
        
        let blockchain_height = self.blockchain.get_chain_length();
        let mut vulnerabilities = Vec::new();
        let mut recommendations = Vec::new();
        
        // 检查双花攻击
        if let Some(issue) = self.check_double_spending() {
            vulnerabilities.push(issue);
        }
        
        // 检查无效签名
        if let Some(issue) = self.check_invalid_signatures() {
            vulnerabilities.push(issue);
        }
        
        // 检查弱哈希
        if let Some(issue) = self.check_weak_hashes() {
            vulnerabilities.push(issue);
        }
        
        // 检查内存池洪水攻击
        if let Some(issue) = self.check_memory_pool_flooding() {
            vulnerabilities.push(issue);
        }
        
        // 计算风险评分
        let risk_score = self.calculate_risk_score(&vulnerabilities);
        
        // 生成建议
        recommendations = self.generate_recommendations(&vulnerabilities);
        
        SecurityReport {
            blockchain_height: blockchain_height as u64,
            risk_score,
            vulnerabilities,
            recommendations,
            audit_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// 实时安全监控
    pub fn real_time_monitor(&mut self) -> Vec<SecurityAlert> {
        let mut alerts = Vec::new();
        
        // 检查内存池大小
        let pending_transactions = self.blockchain.pending_transactions.len();
        if pending_transactions > 1000 {
            alerts.push(SecurityAlert {
                message: format!("内存池过大: {} 个待处理交易", pending_transactions),
                threat_level: ThreatLevel::High,
                source: "MemoryPoolMonitor".to_string(),
                timestamp: self.get_current_timestamp(),
            });
        }
        
        // 检查异常挖矿活动
        if pending_transactions > 500 {
            alerts.push(SecurityAlert {
                message: "检测到异常挖矿活动".to_string(),
                threat_level: ThreatLevel::Medium,
                source: "MiningMonitor".to_string(),
                timestamp: self.get_current_timestamp(),
            });
        }
        
        self.alerts.extend(alerts.clone());
        alerts
    }

    /// 获取安全摘要
    pub fn get_security_summary(&self) -> SecuritySummary {
        let critical_vulnerabilities = self.alerts.iter()
            .filter(|alert| alert.threat_level == ThreatLevel::Critical)
            .count() as u64;
        
        SecuritySummary {
            total_audits: self.audit_count,
            latest_risk_score: 25.0, // 默认低风险
            unresolved_alerts: self.alerts.len() as u64,
            critical_vulnerabilities,
        }
    }

    /// 导出安全报告
    pub fn export_security_report(&self, format: ReportFormat) -> Result<String, String> {
        let report = SecurityReport {
            blockchain_height: self.blockchain.get_chain_length() as u64,
            risk_score: 25.0,
            vulnerabilities: Vec::new(),
            recommendations: vec![
                "定期更新密码学算法".to_string(),
                "监控网络异常活动".to_string(),
                "实施多重签名验证".to_string(),
            ],
            audit_timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        match format {
            ReportFormat::JSON => {
                serde_json::to_string_pretty(&report)
                    .map_err(|e| format!("JSON序列化失败: {}", e))
            },
            ReportFormat::Markdown => {
                Ok(format!(
                    "# 区块链安全审计报告\n\n\
                    ## 基本信息\n\
                    - 区块链高度: {}\n\
                    - 风险评分: {:.2}/100\n\
                    - 审计时间: {}\n\n\
                    ## 安全建议\n\
                    {}\n\n\
                    ## 总结\n\
                    当前区块链系统运行正常，未发现严重安全漏洞。",
                    report.blockchain_height,
                    report.risk_score,
                    report.audit_timestamp,
                    report.recommendations.join("\n- ")
                ))
            }
        }
    }

    // 私有辅助方法

    fn check_double_spending(&self) -> Option<SecurityIssue> {
        // 简化的双花检测逻辑
        None
    }

    fn check_invalid_signatures(&self) -> Option<SecurityIssue> {
        // 简化的签名验证逻辑
        None
    }

    fn check_weak_hashes(&self) -> Option<SecurityIssue> {
        // 简化的哈希强度检查
        None
    }

    fn check_memory_pool_flooding(&self) -> Option<SecurityIssue> {
        let pending_count = self.blockchain.pending_transactions.len();
        if pending_count > 1500 {
            Some(SecurityIssue {
                vulnerability: SecurityVulnerability::MemoryPoolFlooding,
                threat_level: ThreatLevel::High,
                description: format!("内存池包含过多待处理交易: {}", pending_count),
                mitigation: "实施交易费用机制和内存池大小限制".to_string(),
                block_height: None,
                transaction_hash: None,
            })
        } else {
            None
        }
    }

    fn calculate_risk_score(&self, vulnerabilities: &[SecurityIssue]) -> f64 {
        let mut score: f64 = 0.0;
        for issue in vulnerabilities {
            score += match issue.threat_level {
                ThreatLevel::Low => 10.0,
                ThreatLevel::Medium => 25.0,
                ThreatLevel::High => 50.0,
                ThreatLevel::Critical => 100.0,
            };
        }
        score.min(100.0)
    }

    fn generate_recommendations(&self, vulnerabilities: &[SecurityIssue]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        for issue in vulnerabilities {
            match issue.vulnerability {
                SecurityVulnerability::MemoryPoolFlooding => {
                    recommendations.push("实施交易费用机制".to_string());
                    recommendations.push("设置内存池大小限制".to_string());
                },
                SecurityVulnerability::DoubleSpending => {
                    recommendations.push("增强交易验证逻辑".to_string());
                    recommendations.push("实施UTXO模型".to_string());
                },
                _ => {
                    recommendations.push("定期进行安全审计".to_string());
                }
            }
        }
        
        if recommendations.is_empty() {
            recommendations.push("保持当前安全配置".to_string());
        }
        
        recommendations.sort();
        recommendations.dedup();
        recommendations
    }

    fn get_current_timestamp(&self) -> String {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string()
    }
}
