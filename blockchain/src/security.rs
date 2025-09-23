//! # 安全审计模块
//! 
//! 提供区块链安全审计、漏洞检测和修复功能
//! Security audit module with vulnerability detection and fixes

use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use crate::simple_blockchain::*;

/// 安全漏洞类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum SecurityVulnerability {
    /// 双花攻击
    DoubleSpending,
    /// 51%攻击
    MajorityAttack,
    /// 重放攻击
    ReplayAttack,
    /// 时间戳攻击
    TimestampAttack,
    /// 难度操纵
    DifficultyManipulation,
    /// 内存池洪水攻击
    MemPoolFlooding,
    /// 无效交易
    InvalidTransaction,
    /// 链分叉
    ChainFork,
    /// 私钥泄露
    PrivateKeyLeak,
    /// 智能合约漏洞
    SmartContractVulnerability,
}

/// 安全威胁级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// 安全审计报告
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SecurityAuditReport {
    pub timestamp: u64,
    pub blockchain_height: u64,
    pub vulnerabilities: Vec<SecurityIssue>,
    pub risk_score: f64,
    pub recommendations: Vec<String>,
    pub audit_duration: Duration,
}

/// 安全问题详情
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SecurityIssue {
    pub vulnerability: SecurityVulnerability,
    pub threat_level: ThreatLevel,
    pub description: String,
    pub affected_blocks: Vec<u64>,
    pub affected_transactions: Vec<String>,
    pub mitigation: String,
    pub detected_at: u64,
}

/// 安全监控器
#[allow(dead_code)]
pub struct SecurityMonitor {
    blockchain: Blockchain,
    known_threats: HashMap<SecurityVulnerability, ThreatLevel>,
    audit_history: Vec<SecurityAuditReport>,
    real_time_alerts: Vec<SecurityAlert>,
}

/// 实时安全警报
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SecurityAlert {
    pub alert_id: String,
    pub threat_level: ThreatLevel,
    pub message: String,
    pub timestamp: u64,
    pub source: String,
    pub resolved: bool,
}

#[allow(dead_code)]
impl SecurityMonitor {
    pub fn new(blockchain: Blockchain) -> Self {
        let mut known_threats = HashMap::new();
        known_threats.insert(SecurityVulnerability::DoubleSpending, ThreatLevel::Critical);
        known_threats.insert(SecurityVulnerability::MajorityAttack, ThreatLevel::Critical);
        known_threats.insert(SecurityVulnerability::ReplayAttack, ThreatLevel::High);
        known_threats.insert(SecurityVulnerability::TimestampAttack, ThreatLevel::Medium);
        known_threats.insert(SecurityVulnerability::DifficultyManipulation, ThreatLevel::High);
        known_threats.insert(SecurityVulnerability::MemPoolFlooding, ThreatLevel::Medium);
        known_threats.insert(SecurityVulnerability::InvalidTransaction, ThreatLevel::High);
        known_threats.insert(SecurityVulnerability::ChainFork, ThreatLevel::High);
        known_threats.insert(SecurityVulnerability::PrivateKeyLeak, ThreatLevel::Critical);
        known_threats.insert(SecurityVulnerability::SmartContractVulnerability, ThreatLevel::High);

        Self {
            blockchain,
            known_threats,
            audit_history: Vec::new(),
            real_time_alerts: Vec::new(),
        }
    }

    /// 获取区块链的可变引用
    #[allow(dead_code)]
    pub fn get_blockchain_mut(&mut self) -> &mut Blockchain {
        &mut self.blockchain
    }

    /// 执行完整安全审计
    #[allow(dead_code)]
    pub fn perform_full_audit(&mut self) -> SecurityAuditReport {
        let start_time = SystemTime::now();
        let mut vulnerabilities = Vec::new();

        // 检查各种安全漏洞
        vulnerabilities.extend(self.check_double_spending());
        vulnerabilities.extend(self.check_majority_attack());
        vulnerabilities.extend(self.check_replay_attacks());
        vulnerabilities.extend(self.check_timestamp_attacks());
        vulnerabilities.extend(self.check_difficulty_manipulation());
        vulnerabilities.extend(self.check_invalid_transactions());
        vulnerabilities.extend(self.check_chain_fork());

        // 计算风险评分
        let risk_score = self.calculate_risk_score(&vulnerabilities);

        // 生成建议
        let recommendations = self.generate_recommendations(&vulnerabilities);

        let audit_duration = start_time.elapsed();
        let report = SecurityAuditReport {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            blockchain_height: self.blockchain.get_chain_length() as u64,
            vulnerabilities,
            risk_score,
            recommendations,
            audit_duration: audit_duration.unwrap_or(Duration::ZERO),
        };

        self.audit_history.push(report.clone());
        report
    }

    /// 检查双花攻击
    fn check_double_spending(&self) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();
        let mut spent_outputs = HashMap::new();

        for block in &self.blockchain.chain {
            for transaction in &block.transactions {
                if transaction.sender == "system" {
                    continue; // 跳过系统交易
                }

                let output_key = format!("{}:{}", transaction.sender, transaction.amount);
                
                if let Some(previous_block) = spent_outputs.get(&output_key) {
                    issues.push(SecurityIssue {
                        vulnerability: SecurityVulnerability::DoubleSpending,
                        threat_level: ThreatLevel::Critical,
                        description: format!(
                            "检测到双花攻击：地址 {} 在同一输出上花费了两次",
                            transaction.sender
                        ),
                        affected_blocks: vec![*previous_block, block.index],
                        affected_transactions: vec![transaction.to_bytes().iter().map(|b| format!("{:02x}", b)).collect()],
                        mitigation: "实现UTXO模型或增加确认时间".to_string(),
                        detected_at: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    });
                } else {
                    spent_outputs.insert(output_key, block.index);
                }
            }
        }

        issues
    }

    /// 检查51%攻击
    fn check_majority_attack(&self) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();
        
        if self.blockchain.get_chain_length() > 6 {
            // 检查最近的6个区块是否有异常的时间戳或难度
            let recent_blocks: Vec<&Block> = self.blockchain.chain.iter().rev().take(6).collect();
            
            for (i, block) in recent_blocks.iter().enumerate() {
                if i > 0 {
                    let prev_block = recent_blocks[i - 1];
                    let time_diff = block.timestamp - prev_block.timestamp;
                    
                    // 检查异常快的挖矿时间（可能是攻击）
                    if time_diff < 10 && block.difficulty > 2 {
                        issues.push(SecurityIssue {
                            vulnerability: SecurityVulnerability::MajorityAttack,
                            threat_level: ThreatLevel::Critical,
                            description: format!(
                                "检测到可能的51%攻击：区块 {} 挖矿时间异常短 ({})",
                                block.index, time_diff
                            ),
                            affected_blocks: vec![block.index],
                            affected_transactions: vec![],
                            mitigation: "增加确认区块数量，监控网络算力分布".to_string(),
                            detected_at: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        });
                    }
                }
            }
        }

        issues
    }

    /// 检查重放攻击
    fn check_replay_attacks(&self) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();
        let mut transaction_hashes = HashMap::new();

        for block in &self.blockchain.chain {
            for transaction in &block.transactions {
                let tx_hash = transaction.to_bytes().iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>();
                
                if let Some(previous_occurrence) = transaction_hashes.get(&tx_hash) {
                    issues.push(SecurityIssue {
                        vulnerability: SecurityVulnerability::ReplayAttack,
                        threat_level: ThreatLevel::High,
                        description: format!(
                            "检测到重放攻击：交易在区块 {} 和 {} 中重复出现",
                            previous_occurrence, block.index
                        ),
                        affected_blocks: vec![*previous_occurrence, block.index],
                        affected_transactions: vec![tx_hash],
                        mitigation: "实现交易nonce机制或唯一交易ID".to_string(),
                        detected_at: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    });
                } else {
                    transaction_hashes.insert(tx_hash, block.index);
                }
            }
        }

        issues
    }

    /// 检查时间戳攻击
    fn check_timestamp_attacks(&self) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        for block in &self.blockchain.chain {
            // 检查未来时间戳
            if block.timestamp > current_time + 7200 { // 2小时容差
                issues.push(SecurityIssue {
                    vulnerability: SecurityVulnerability::TimestampAttack,
                    threat_level: ThreatLevel::Medium,
                    description: format!(
                        "检测到时间戳攻击：区块 {} 的时间戳 {} 比当前时间超前过多",
                        block.index, block.timestamp
                    ),
                    affected_blocks: vec![block.index],
                    affected_transactions: vec![],
                    mitigation: "严格验证时间戳范围，拒绝未来时间戳".to_string(),
                    detected_at: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                });
            }

            // 检查时间戳倒退
            if block.index > 0 {
                if let Some(prev_block) = self.blockchain.chain.get(block.index as usize - 1) {
                if block.timestamp < prev_block.timestamp {
                    issues.push(SecurityIssue {
                        vulnerability: SecurityVulnerability::TimestampAttack,
                        threat_level: ThreatLevel::Medium,
                        description: format!(
                            "检测到时间戳倒退：区块 {} 的时间戳 {} 比前一个区块 {} 早",
                            block.index, block.timestamp, prev_block.timestamp
                        ),
                        affected_blocks: vec![block.index],
                        affected_transactions: vec![],
                        mitigation: "确保时间戳单调递增".to_string(),
                        detected_at: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    });
                }
            }
            }
        }

        issues
    }

    /// 检查难度操纵
    fn check_difficulty_manipulation(&self) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();

        if self.blockchain.get_chain_length() > 10 {
            // 分析最近10个区块的挖矿时间
            let recent_blocks: Vec<&Block> = self.blockchain.chain.iter().rev().take(10).collect();
            let mut total_time = 0u64;
            let mut count = 0;

            for (i, block) in recent_blocks.iter().enumerate() {
                if i > 0 {
                    let time_diff = block.timestamp - recent_blocks[i - 1].timestamp;
                    total_time += time_diff;
                    count += 1;
                }
            }

            if count > 0 {
                let avg_time = total_time / count;
                
                // 如果平均挖矿时间异常短，可能是难度被操纵
                if avg_time < 30 && self.blockchain.difficulty > 1 {
                    issues.push(SecurityIssue {
                        vulnerability: SecurityVulnerability::DifficultyManipulation,
                        threat_level: ThreatLevel::High,
                        description: format!(
                            "检测到难度操纵：最近10个区块平均挖矿时间 {} 秒，但难度为 {}",
                            avg_time, self.blockchain.difficulty
                        ),
                        affected_blocks: (self.blockchain.get_chain_length() - 10..self.blockchain.get_chain_length())
                            .map(|i| i as u64)
                            .collect(),
                        affected_transactions: vec![],
                        mitigation: "实现动态难度调整算法".to_string(),
                        detected_at: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    });
                }
            }
        }

        issues
    }

    /// 检查无效交易
    fn check_invalid_transactions(&self) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();

        for block in &self.blockchain.chain {
            for transaction in &block.transactions {
                let validation = transaction.validate();
                
                if !validation.is_valid {
                    issues.push(SecurityIssue {
                        vulnerability: SecurityVulnerability::InvalidTransaction,
                        threat_level: ThreatLevel::High,
                        description: format!(
                            "检测到无效交易：{}",
                            validation.errors.join(", ")
                        ),
                        affected_blocks: vec![block.index],
                        affected_transactions: vec![transaction.to_bytes().iter().map(|b| format!("{:02x}", b)).collect()],
                        mitigation: "严格验证所有交易格式和内容".to_string(),
                        detected_at: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    });
                }

                // 检查余额不足
                if transaction.sender != "genesis" && transaction.sender != "system" {
                    let balance = self.blockchain.get_balance(&transaction.sender);
                    if balance < transaction.amount {
                        issues.push(SecurityIssue {
                            vulnerability: SecurityVulnerability::InvalidTransaction,
                            threat_level: ThreatLevel::High,
                            description: format!(
                                "检测到余额不足交易：地址 {} 余额 {} 不足支付 {}",
                                transaction.sender, balance, transaction.amount
                            ),
                            affected_blocks: vec![block.index],
                            affected_transactions: vec![transaction.to_bytes().iter().map(|b| format!("{:02x}", b)).collect()],
                            mitigation: "实现UTXO模型或严格余额检查".to_string(),
                            detected_at: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        });
                    }
                }
            }
        }

        issues
    }

    /// 检查链分叉
    fn check_chain_fork(&self) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();

        // 检查链的完整性
        if !self.blockchain.is_valid_chain() {
            issues.push(SecurityIssue {
                vulnerability: SecurityVulnerability::ChainFork,
                threat_level: ThreatLevel::High,
                description: "检测到链分叉或链完整性破坏".to_string(),
                affected_blocks: (0..self.blockchain.get_chain_length() as u64).collect(),
                affected_transactions: vec![],
                mitigation: "重新同步区块链或回滚到最后一个有效区块".to_string(),
                detected_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            });
        }

        issues
    }

    /// 计算风险评分
    fn calculate_risk_score(&self, vulnerabilities: &[SecurityIssue]) -> f64 {
        let mut score = 0.0;
        
        for issue in vulnerabilities {
            let weight = match issue.threat_level {
                ThreatLevel::Low => 1.0,
                ThreatLevel::Medium => 3.0,
                ThreatLevel::High => 7.0,
                ThreatLevel::Critical => 15.0,
            };
            score += weight;
        }

        // 归一化到0-100
        (score / 100.0_f64).min(100.0_f64)
    }

    /// 生成安全建议
    fn generate_recommendations(&self, vulnerabilities: &[SecurityIssue]) -> Vec<String> {
        let mut recommendations = Vec::new();

        let mut has_critical = false;
        let mut has_high = false;
        let mut has_medium = false;

        for issue in vulnerabilities {
            match issue.threat_level {
                ThreatLevel::Critical => has_critical = true,
                ThreatLevel::High => has_high = true,
                ThreatLevel::Medium => has_medium = true,
                ThreatLevel::Low => {},
            }
        }

        if has_critical {
            recommendations.push("🚨 发现严重安全漏洞，建议立即停止网络并修复".to_string());
            recommendations.push("实施紧急安全补丁和网络升级".to_string());
        }

        if has_high {
            recommendations.push("⚠️ 发现高危漏洞，建议尽快修复".to_string());
            recommendations.push("增加额外的安全验证机制".to_string());
        }

        if has_medium {
            recommendations.push("📋 发现中等风险问题，建议在下次升级中修复".to_string());
        }

        recommendations.push("定期进行安全审计和渗透测试".to_string());
        recommendations.push("实施多重签名和访问控制".to_string());
        recommendations.push("建立安全事件响应机制".to_string());

        recommendations
    }

    /// 实时监控
    pub fn real_time_monitor(&mut self) -> Vec<SecurityAlert> {
        let mut new_alerts = Vec::new();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // 检查内存池洪水攻击
        if self.blockchain.pending_transactions.len() > 1000 {
            new_alerts.push(SecurityAlert {
                alert_id: format!("mem_pool_flood_{}", current_time),
                threat_level: ThreatLevel::Medium,
                message: format!(
                    "内存池洪水攻击：待处理交易数量 {} 超过正常范围",
                    self.blockchain.pending_transactions.len()
                ),
                timestamp: current_time,
                source: "memory_pool_monitor".to_string(),
                resolved: false,
            });
        }

        // 检查异常挖矿活动
        if let Some(last_block) = self.blockchain.chain.last() {
            let time_since_last_block = current_time - last_block.timestamp;
            if time_since_last_block > 3600 { // 1小时无新区块
                new_alerts.push(SecurityAlert {
                    alert_id: format!("mining_halt_{}", current_time),
                    threat_level: ThreatLevel::High,
                    message: "挖矿活动异常：超过1小时未产生新区块".to_string(),
                    timestamp: current_time,
                    source: "mining_monitor".to_string(),
                    resolved: false,
                });
            }
        }

        self.real_time_alerts.extend(new_alerts.clone());
        new_alerts
    }

    /// 获取安全报告摘要
    pub fn get_security_summary(&self) -> SecuritySummary {
        let latest_audit = self.audit_history.last();
        let unresolved_alerts = self.real_time_alerts.iter()
            .filter(|alert| !alert.resolved)
            .count();

        SecuritySummary {
            total_audits: self.audit_history.len(),
            latest_risk_score: latest_audit.map(|audit| audit.risk_score).unwrap_or(0.0),
            unresolved_alerts,
            critical_vulnerabilities: self.audit_history.iter()
                .flat_map(|audit| &audit.vulnerabilities)
                .filter(|issue| issue.threat_level == ThreatLevel::Critical)
                .count(),
            last_audit_time: latest_audit.map(|audit| audit.timestamp).unwrap_or(0),
        }
    }

    /// 导出安全报告
    pub fn export_security_report(&self, format: ReportFormat) -> Result<String, String> {
        match format {
            ReportFormat::JSON => {
                let latest_audit = self.audit_history.last()
                    .ok_or("没有可用的审计报告")?;
                serde_json::to_string_pretty(latest_audit)
                    .map_err(|e| format!("JSON序列化失败: {}", e))
            },
            ReportFormat::Markdown => {
                self.generate_markdown_report()
            },
        }
    }

    /// 生成Markdown格式报告
    fn generate_markdown_report(&self) -> Result<String, String> {
        let latest_audit = self.audit_history.last()
            .ok_or("没有可用的审计报告")?;

        let mut report = String::new();
        report.push_str("# 区块链安全审计报告\n\n");
        report.push_str(&format!("**审计时间**: {}\n", latest_audit.timestamp));
        report.push_str(&format!("**区块链高度**: {}\n", latest_audit.blockchain_height));
        report.push_str(&format!("**风险评分**: {:.2}/100\n\n", latest_audit.risk_score));

        if latest_audit.vulnerabilities.is_empty() {
            report.push_str("✅ **未发现安全漏洞**\n\n");
        } else {
            report.push_str("## 发现的安全漏洞\n\n");
            for (i, issue) in latest_audit.vulnerabilities.iter().enumerate() {
                report.push_str(&format!("### {}. {} ({:?})\n", 
                    i + 1, 
                    format!("{:?}", issue.vulnerability), 
                    issue.threat_level));
                report.push_str(&format!("**描述**: {}\n\n", issue.description));
                report.push_str(&format!("**缓解措施**: {}\n\n", issue.mitigation));
            }
        }

        if !latest_audit.recommendations.is_empty() {
            report.push_str("## 安全建议\n\n");
            for recommendation in &latest_audit.recommendations {
                report.push_str(&format!("- {}\n", recommendation));
            }
        }

        Ok(report)
    }
}

/// 安全摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySummary {
    pub total_audits: usize,
    pub latest_risk_score: f64,
    pub unresolved_alerts: usize,
    pub critical_vulnerabilities: usize,
    pub last_audit_time: u64,
}

/// 报告格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    JSON,
    Markdown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_monitor_creation() {
        let blockchain = Blockchain::new(2);
        let monitor = SecurityMonitor::new(blockchain);
        
        assert_eq!(monitor.audit_history.len(), 0);
        assert_eq!(monitor.real_time_alerts.len(), 0);
    }

    #[test]
    fn test_risk_score_calculation() {
        let blockchain = Blockchain::new(2);
        let mut monitor = SecurityMonitor::new(blockchain);
        
        let report = monitor.perform_full_audit();
        assert!(report.risk_score >= 0.0 && report.risk_score <= 100.0);
    }

    #[test]
    fn test_security_summary() {
        let blockchain = Blockchain::new(2);
        let mut monitor = SecurityMonitor::new(blockchain);
        
        monitor.perform_full_audit();
        let summary = monitor.get_security_summary();
        
        assert_eq!(summary.total_audits, 1);
        assert!(summary.latest_risk_score >= 0.0);
    }
}
