//! # 安全审计演示程序
//! 
//! 展示区块链安全审计、漏洞检测和修复功能
//! Demonstrates blockchain security audit, vulnerability detection and fixes

use blockchain::*;
use blockchain::security::*;

fn main() {
    println!("🔒 区块链安全审计演示");
    println!("🔒 Blockchain Security Audit Demo");
    println!();

    // 1. 创建测试区块链
    println!("📋 1. 创建测试区块链");
    let blockchain = create_test_blockchain();

    // 2. 执行安全审计
    println!("\n📋 2. 执行安全审计");
    perform_security_audit(&blockchain);

    // 3. 模拟安全威胁
    println!("\n📋 3. 模拟安全威胁");
    simulate_security_threats();

    // 4. 实时安全监控
    println!("\n📋 4. 实时安全监控");
    demonstrate_real_time_monitoring();

    // 5. 安全报告生成
    println!("\n📋 5. 安全报告生成");
    generate_security_reports();

    println!("\n🎉 安全审计演示完成！");
}

/// 创建测试区块链
fn create_test_blockchain() -> Blockchain {
    println!("✅ 创建测试区块链");
    
    let mut blockchain = Blockchain::new(2);
    
    // 添加一些正常交易
    for i in 0..5 {
        let transaction = Transaction::new(
            "genesis".to_string(),
            format!("user_{}", i),
            100,
        );
        blockchain.add_transaction(transaction).unwrap();
    }
    
    // 挖矿
    blockchain.mine_pending_transactions("miner_1".to_string()).unwrap();
    
    println!("   - 区块链高度: {}", blockchain.get_chain_length());
    println!("   - 链有效性: {}", blockchain.is_valid_chain());
    
    blockchain
}

/// 执行安全审计
fn perform_security_audit(blockchain: &Blockchain) {
    println!("✅ 开始安全审计");
    
    let mut monitor = SecurityMonitor::new(blockchain.clone());
    
    // 执行完整审计
    let start_time = std::time::Instant::now();
    let report = monitor.perform_full_audit();
    let audit_time = start_time.elapsed();
    
    println!("   - 审计耗时: {:?}", audit_time);
    println!("   - 区块链高度: {}", report.blockchain_height);
    println!("   - 风险评分: {:.2}/100", report.risk_score);
    println!("   - 发现漏洞数量: {}", report.vulnerabilities.len());
    
    if report.vulnerabilities.is_empty() {
        println!("   ✅ 未发现安全漏洞");
    } else {
        println!("   ⚠️ 发现以下安全漏洞:");
        for (i, issue) in report.vulnerabilities.iter().enumerate() {
            println!("     {}. {:?} ({:?})", 
                i + 1, 
                issue.vulnerability, 
                issue.threat_level);
            println!("       描述: {}", issue.description);
            println!("       缓解措施: {}", issue.mitigation);
        }
    }
    
    if !report.recommendations.is_empty() {
        println!("   📋 安全建议:");
        for recommendation in &report.recommendations {
            println!("     - {}", recommendation);
        }
    }
}

/// 模拟安全威胁
fn simulate_security_threats() {
    println!("✅ 模拟安全威胁场景");
    
    // 创建包含各种威胁的区块链
    let blockchain = create_vulnerable_blockchain();
    let mut monitor = SecurityMonitor::new(blockchain);
    
    // 执行审计
    let report = monitor.perform_full_audit();
    
    println!("   - 风险评分: {:.2}/100", report.risk_score);
    println!("   - 威胁分析:");
    
    // 按威胁级别分组统计
    let mut threat_counts = std::collections::HashMap::new();
    for issue in &report.vulnerabilities {
        let count = threat_counts.entry(&issue.threat_level).or_insert(0);
        *count += 1;
    }
    
    for (level, count) in threat_counts {
        println!("     {:?}: {} 个", level, count);
    }
    
    // 显示最严重的威胁
    if let Some(critical_issue) = report.vulnerabilities.iter()
        .find(|issue| issue.threat_level == ThreatLevel::Critical) {
        println!("   🚨 最严重威胁: {:?}", critical_issue.vulnerability);
        println!("     描述: {}", critical_issue.description);
    }
}

/// 创建包含漏洞的区块链
fn create_vulnerable_blockchain() -> Blockchain {
    let mut blockchain = Blockchain::new(1); // 低难度以便快速挖矿
    
    // 添加一些交易
    for i in 0..3 {
        let transaction = Transaction::new(
            "genesis".to_string(),
            format!("user_{}", i),
            100,
        );
        blockchain.add_transaction(transaction).unwrap();
    }
    
    blockchain.mine_pending_transactions("attacker".to_string()).unwrap();
    
    blockchain
}

/// 演示实时安全监控
fn demonstrate_real_time_monitoring() {
    println!("✅ 开始实时安全监控演示");
    
    let blockchain = create_test_blockchain();
    let mut monitor = SecurityMonitor::new(blockchain);
    
    // 模拟一些异常活动
    println!("   - 模拟异常挖矿活动...");
    
    // 添加大量交易到内存池（模拟洪水攻击）
    for i in 0..1500 {
        let transaction = Transaction::new(
            "genesis".to_string(),
            format!("spam_user_{}", i),
            1,
        );
        monitor.get_blockchain_mut().add_transaction(transaction).unwrap();
    }
    
    // 执行实时监控
    let alerts = monitor.real_time_monitor();
    
    println!("   - 检测到 {} 个安全警报", alerts.len());
    
    for alert in alerts {
        println!("     🚨 警报: {} ({:?})", alert.message, alert.threat_level);
        println!("       来源: {}", alert.source);
        println!("       时间: {}", alert.timestamp);
    }
    
    // 显示安全摘要
    let summary = monitor.get_security_summary();
    println!("   📊 安全摘要:");
    println!("     - 总审计次数: {}", summary.total_audits);
    println!("     - 最新风险评分: {:.2}", summary.latest_risk_score);
    println!("     - 未解决警报: {}", summary.unresolved_alerts);
    println!("     - 严重漏洞数量: {}", summary.critical_vulnerabilities);
}

/// 生成安全报告
fn generate_security_reports() {
    println!("✅ 生成安全报告");
    
    let blockchain = create_test_blockchain();
    let mut monitor = SecurityMonitor::new(blockchain);
    
    // 执行审计
    monitor.perform_full_audit();
    
    // 生成JSON报告
    match monitor.export_security_report(ReportFormat::JSON) {
        Ok(json_report) => {
            println!("   📄 JSON报告生成成功");
            println!("   - 报告长度: {} 字符", json_report.len());
            // 显示报告摘要
            if json_report.len() > 200 {
                // 安全地截取字符串，避免UTF-8字符边界问题
                let preview = json_report.chars().take(200).collect::<String>();
                println!("   - 报告预览: {}...", preview);
            } else {
                println!("   - 报告内容: {}", json_report);
            }
        },
        Err(e) => {
            println!("   ❌ JSON报告生成失败: {}", e);
        }
    }
    
    // 生成Markdown报告
    match monitor.export_security_report(ReportFormat::Markdown) {
        Ok(markdown_report) => {
            println!("   📄 Markdown报告生成成功");
            println!("   - 报告长度: {} 字符", markdown_report.len());
            // 显示报告摘要
            if markdown_report.len() > 300 {
                println!("   - 报告预览:");
                for line in markdown_report.lines().take(10) {
                    println!("     {}", line);
                }
                println!("     ...");
            } else {
                println!("   - 报告内容:");
                for line in markdown_report.lines() {
                    println!("     {}", line);
                }
            }
        },
        Err(e) => {
            println!("   ❌ Markdown报告生成失败: {}", e);
        }
    }
}

/// 安全威胁检测演示
#[allow(dead_code)]
fn demonstrate_threat_detection() {
    println!("✅ 安全威胁检测演示");
    
    // 双花攻击检测
    println!("   🔍 双花攻击检测:");
    let mut blockchain = Blockchain::new(2);
    
    // 模拟双花攻击
    let transaction1 = Transaction::new(
        "genesis".to_string(),
        "victim".to_string(),
        1000,
    );
    blockchain.add_transaction(transaction1).unwrap();
    blockchain.mine_pending_transactions("miner_1".to_string()).unwrap();
    
    let transaction2 = Transaction::new(
        "genesis".to_string(),
        "attacker".to_string(),
        1000, // 相同的发送者和金额
    );
    blockchain.add_transaction(transaction2).unwrap();
    blockchain.mine_pending_transactions("miner_2".to_string()).unwrap();
    
    let mut monitor = SecurityMonitor::new(blockchain);
    let report = monitor.perform_full_audit();
    
    let double_spending_issues: Vec<_> = report.vulnerabilities.iter()
        .filter(|issue| issue.vulnerability == SecurityVulnerability::DoubleSpending)
        .collect();
    
    if !double_spending_issues.is_empty() {
        println!("     ✅ 成功检测到双花攻击");
        for issue in double_spending_issues {
            println!("       - {}", issue.description);
        }
    } else {
        println!("     ❌ 未能检测到双花攻击");
    }
}

/// 安全修复建议演示
#[allow(dead_code)]
fn demonstrate_security_fixes() {
    println!("✅ 安全修复建议演示");
    
    let blockchain = create_vulnerable_blockchain();
    let mut monitor = SecurityMonitor::new(blockchain);
    
    let report = monitor.perform_full_audit();
    
    println!("   🔧 修复建议:");
    
    // 按漏洞类型分组建议
    let mut fix_suggestions = std::collections::HashMap::new();
    
    for issue in &report.vulnerabilities {
        let suggestions = fix_suggestions.entry(&issue.vulnerability)
            .or_insert_with(Vec::new);
        suggestions.push(&issue.mitigation);
    }
    
    for (vulnerability, suggestions) in fix_suggestions {
        println!("     {:?}:", vulnerability);
        for suggestion in suggestions {
            println!("       - {}", suggestion);
        }
    }
    
    // 显示通用安全建议
    if !report.recommendations.is_empty() {
        println!("   📋 通用安全建议:");
        for recommendation in &report.recommendations {
            println!("     - {}", recommendation);
        }
    }
}
