# 问题诊断与解决

## 📋 目录

- [问题诊断与解决](#问题诊断与解决)
  - [📋 目录](#-目录)
  - [1. 诊断方法论](#1-诊断方法论)
    - [1.1 系统化诊断流程](#11-系统化诊断流程)
    - [1.2 日志分析](#12-日志分析)
    - [1.3 性能监控](#13-性能监控)
  - [2. 节点问题](#2-节点问题)
    - [2.1 节点无法启动](#21-节点无法启动)
    - [2.2 节点同步失败](#22-节点同步失败)
    - [2.3 节点频繁崩溃](#23-节点频繁崩溃)
  - [3. 共识问题](#3-共识问题)
    - [3.1 共识失败](#31-共识失败)
    - [3.2 分叉问题](#32-分叉问题)
    - [3.3 网络分区](#33-网络分区)
  - [4. 交易问题](#4-交易问题)
    - [4.1 交易pending](#41-交易pending)
    - [4.2 交易失败](#42-交易失败)
    - [4.3 Gas估算错误](#43-gas估算错误)
  - [5. 智能合约问题](#5-智能合约问题)
    - [5.1 合约部署失败](#51-合约部署失败)
    - [5.2 合约调用错误](#52-合约调用错误)
    - [5.3 合约安全漏洞](#53-合约安全漏洞)
  - [6. 网络问题](#6-网络问题)
    - [6.1 P2P连接问题](#61-p2p连接问题)
    - [6.2 网络延迟高](#62-网络延迟高)
    - [6.3 DDoS攻击](#63-ddos攻击)
  - [7. 存储问题](#7-存储问题)
    - [7.1 数据库损坏](#71-数据库损坏)
    - [7.2 磁盘空间不足](#72-磁盘空间不足)
    - [7.3 数据同步慢](#73-数据同步慢)
  - [8. 性能问题](#8-性能问题)
    - [8.1 TPS低](#81-tps低)
    - [8.2 内存泄漏](#82-内存泄漏)
    - [8.3 CPU占用高](#83-cpu占用高)
  - [9. 安全问题](#9-安全问题)
    - [9.1 私钥泄露](#91-私钥泄露)
    - [9.2 51%攻击](#92-51攻击)
    - [9.3 女巫攻击](#93-女巫攻击)
  - [10. 工具与最佳实践](#10-工具与最佳实践)
    - [10.1 诊断工具](#101-诊断工具)
    - [10.2 监控工具](#102-监控工具)
    - [10.3 最佳实践](#103-最佳实践)
  - [总结](#总结)

## 1. 诊断方法论

### 1.1 系统化诊断流程

```rust
/// 系统化故障诊断框架
pub struct TroubleshootingFramework {
    diagnostic_steps: Vec<DiagnosticStep>,
    troubleshooting_tools: Vec<Tool>,
}

#[derive(Debug)]
pub struct DiagnosticStep {
    step_number: u32,
    name: &'static str,
    actions: Vec<&'static str>,
    expected_outcome: &'static str,
}

#[derive(Debug)]
pub struct Tool {
    name: &'static str,
    purpose: &'static str,
    usage: &'static str,
}

impl TroubleshootingFramework {
    pub fn new() -> Self {
        Self {
            diagnostic_steps: vec![
                DiagnosticStep {
                    step_number: 1,
                    name: "问题识别",
                    actions: vec![
                        "收集症状描述",
                        "确定影响范围",
                        "检查错误日志",
                        "记录问题发生时间",
                    ],
                    expected_outcome: "清楚定义问题",
                },
                DiagnosticStep {
                    step_number: 2,
                    name: "信息收集",
                    actions: vec![
                        "查看系统日志",
                        "检查资源使用情况",
                        "获取网络状态",
                        "导出配置文件",
                    ],
                    expected_outcome: "完整的诊断数据",
                },
                DiagnosticStep {
                    step_number: 3,
                    name: "假设提出",
                    actions: vec![
                        "分析可能原因",
                        "根据经验排序",
                        "参考知识库",
                    ],
                    expected_outcome: "优先级排序的假设列表",
                },
                DiagnosticStep {
                    step_number: 4,
                    name: "测试验证",
                    actions: vec![
                        "逐一测试假设",
                        "记录测试结果",
                        "排除不可能原因",
                    ],
                    expected_outcome: "确认根本原因",
                },
                DiagnosticStep {
                    step_number: 5,
                    name: "解决方案",
                    actions: vec![
                        "制定修复方案",
                        "评估风险",
                        "准备回滚计划",
                        "执行修复",
                    ],
                    expected_outcome: "问题解决",
                },
                DiagnosticStep {
                    step_number: 6,
                    name: "验证与文档",
                    actions: vec![
                        "验证修复效果",
                        "监控系统运行",
                        "更新知识库",
                        "总结经验教训",
                    ],
                    expected_outcome: "系统恢复正常，文档更新",
                },
            ],
            troubleshooting_tools: vec![
                Tool {
                    name: "日志分析工具",
                    purpose: "分析系统和应用日志",
                    usage: "tail -f /var/log/blockchain/node.log",
                },
                Tool {
                    name: "网络诊断工具",
                    purpose: "检查网络连接",
                    usage: "netstat -an | grep ESTABLISHED",
                },
                Tool {
                    name: "性能监控工具",
                    purpose: "监控资源使用",
                    usage: "htop, iostat, vmstat",
                },
            ],
        }
    }
    
    /// 诊断检查清单
    pub fn diagnostic_checklist(&self) -> DiagnosticChecklist {
        DiagnosticChecklist {
            system_level: vec![
                "[ ] CPU使用率正常（<80%）",
                "[ ] 内存充足（>20%可用）",
                "[ ] 磁盘空间足够（>20%可用）",
                "[ ] 网络连接正常",
                "[ ] 时间同步正确（NTP）",
            ],
            application_level: vec![
                "[ ] 节点进程运行中",
                "[ ] 配置文件正确",
                "[ ] 日志无error级别错误",
                "[ ] 数据库连接正常",
                "[ ] API响应正常",
            ],
            blockchain_level: vec![
                "[ ] 区块高度同步",
                "[ ] 交易池正常",
                "[ ] 共识参与正常",
                "[ ] P2P连接数正常（>3）",
                "[ ] 区块验证通过",
            ],
        }
    }
}

#[derive(Debug)]
pub struct DiagnosticChecklist {
    system_level: Vec<&'static str>,
    application_level: Vec<&'static str>,
    blockchain_level: Vec<&'static str>,
}
```

### 1.2 日志分析

```rust
/// 日志分析工具
pub struct LogAnalyzer {
    log_patterns: Vec<LogPattern>,
}

#[derive(Debug)]
pub struct LogPattern {
    pattern: &'static str,
    severity: Severity,
    possible_causes: Vec<&'static str>,
    recommended_actions: Vec<&'static str>,
}

#[derive(Debug)]
pub enum Severity {
    Critical,
    Error,
    Warning,
    Info,
}

impl LogAnalyzer {
    pub fn new() -> Self {
        Self {
            log_patterns: vec![
                LogPattern {
                    pattern: "panic: runtime error",
                    severity: Severity::Critical,
                    possible_causes: vec![
                        "空指针引用",
                        "数组越界",
                        "类型断言失败",
                    ],
                    recommended_actions: vec![
                        "查看panic调用栈",
                        "检查代码逻辑",
                        "添加边界检查",
                    ],
                },
                LogPattern {
                    pattern: "connection refused",
                    severity: Severity::Error,
                    possible_causes: vec![
                        "服务未启动",
                        "端口被占用",
                        "防火墙阻止",
                        "网络不通",
                    ],
                    recommended_actions: vec![
                        "检查服务状态",
                        "验证端口配置",
                        "检查防火墙规则",
                        "测试网络连通性",
                    ],
                },
                LogPattern {
                    pattern: "database locked",
                    severity: Severity::Warning,
                    possible_causes: vec![
                        "并发写入冲突",
                        "事务未提交",
                        "死锁",
                    ],
                    recommended_actions: vec![
                        "检查事务管理",
                        "减少并发写入",
                        "优化数据库锁策略",
                    ],
                },
            ],
        }
    }
    
    /// 分析日志文件
    pub async fn analyze_logs(&self, log_path: &str) -> Result<AnalysisReport, Error> {
        // 读取日志文件
        let logs = self.read_logs(log_path).await?;
        
        // 统计错误模式
        let error_stats = self.count_error_patterns(&logs)?;
        
        // 时间序列分析
        let timeline = self.build_timeline(&logs)?;
        
        // 生成报告
        Ok(AnalysisReport {
            total_lines: logs.len(),
            error_count: error_stats.errors,
            warning_count: error_stats.warnings,
            critical_count: error_stats.critical,
            timeline,
            recommendations: self.generate_recommendations(&error_stats),
        })
    }
    
    async fn read_logs(&self, path: &str) -> Result<Vec<LogEntry>, Error> {
        // 读取日志
        Ok(vec![])
    }
    
    fn count_error_patterns(&self, logs: &[LogEntry]) -> Result<ErrorStats, Error> {
        Ok(ErrorStats {
            errors: 0,
            warnings: 0,
            critical: 0,
        })
    }
    
    fn build_timeline(&self, logs: &[LogEntry]) -> Result<Vec<TimelineEvent>, Error> {
        Ok(vec![])
    }
    
    fn generate_recommendations(&self, stats: &ErrorStats) -> Vec<String> {
        vec![]
    }
}

#[derive(Debug)]
pub struct LogEntry {
    timestamp: SystemTime,
    level: String,
    message: String,
}

#[derive(Debug)]
pub struct ErrorStats {
    errors: u32,
    warnings: u32,
    critical: u32,
}

#[derive(Debug)]
pub struct TimelineEvent {
    time: SystemTime,
    event: String,
}

#[derive(Debug)]
pub struct AnalysisReport {
    total_lines: usize,
    error_count: u32,
    warning_count: u32,
    critical_count: u32,
    timeline: Vec<TimelineEvent>,
    recommendations: Vec<String>,
}
```

### 1.3 性能监控

```rust
/// 性能监控系统
pub struct PerformanceMonitor {
    metrics: Vec<Metric>,
    thresholds: Thresholds,
}

#[derive(Debug)]
pub struct Metric {
    name: &'static str,
    current_value: f64,
    unit: &'static str,
}

#[derive(Debug)]
pub struct Thresholds {
    cpu_warning: f64,
    cpu_critical: f64,
    memory_warning: f64,
    memory_critical: f64,
    disk_warning: f64,
    disk_critical: f64,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: vec![],
            thresholds: Thresholds {
                cpu_warning: 75.0,
                cpu_critical: 90.0,
                memory_warning: 80.0,
                memory_critical: 95.0,
                disk_warning: 80.0,
                disk_critical: 90.0,
            },
        }
    }
    
    /// 收集系统指标
    pub async fn collect_metrics(&mut self) -> Result<(), Error> {
        self.metrics = vec![
            self.get_cpu_usage().await?,
            self.get_memory_usage().await?,
            self.get_disk_usage().await?,
            self.get_network_throughput().await?,
        ];
        Ok(())
    }
    
    /// 检查阈值
    pub fn check_thresholds(&self) -> Vec<Alert> {
        let mut alerts = Vec::new();
        
        for metric in &self.metrics {
            match metric.name {
                "cpu_usage" => {
                    if metric.current_value >= self.thresholds.cpu_critical {
                        alerts.push(Alert {
                            severity: AlertSeverity::Critical,
                            metric: metric.name,
                            value: metric.current_value,
                            threshold: self.thresholds.cpu_critical,
                            message: "CPU使用率超过临界值",
                        });
                    } else if metric.current_value >= self.thresholds.cpu_warning {
                        alerts.push(Alert {
                            severity: AlertSeverity::Warning,
                            metric: metric.name,
                            value: metric.current_value,
                            threshold: self.thresholds.cpu_warning,
                            message: "CPU使用率偏高",
                        });
                    }
                },
                _ => {}
            }
        }
        
        alerts
    }
    
    async fn get_cpu_usage(&self) -> Result<Metric, Error> {
        Ok(Metric {
            name: "cpu_usage",
            current_value: 45.0,
            unit: "%",
        })
    }
    
    async fn get_memory_usage(&self) -> Result<Metric, Error> {
        Ok(Metric {
            name: "memory_usage",
            current_value: 60.0,
            unit: "%",
        })
    }
    
    async fn get_disk_usage(&self) -> Result<Metric, Error> {
        Ok(Metric {
            name: "disk_usage",
            current_value: 70.0,
            unit: "%",
        })
    }
    
    async fn get_network_throughput(&self) -> Result<Metric, Error> {
        Ok(Metric {
            name: "network_throughput",
            current_value: 125.5,
            unit: "Mbps",
        })
    }
}

#[derive(Debug)]
pub struct Alert {
    severity: AlertSeverity,
    metric: &'static str,
    value: f64,
    threshold: f64,
    message: &'static str,
}

#[derive(Debug)]
pub enum AlertSeverity {
    Warning,
    Critical,
}
```

## 2. 节点问题

### 2.1 节点无法启动

```rust
/// 节点启动失败诊断
pub struct NodeStartupTroubleshooting;

impl NodeStartupTroubleshooting {
    /// 常见启动失败原因
    pub fn common_issues() -> Vec<StartupIssue> {
        vec![
            StartupIssue {
                symptom: "端口already in use",
                cause: "端口被其他进程占用",
                diagnosis: vec![
                    "lsof -i :8545",
                    "netstat -tulpn | grep 8545",
                ],
                solution: vec![
                    "kill占用端口的进程",
                    "修改配置使用其他端口",
                    "systemctl stop blockchain-node",
                ],
            },
            StartupIssue {
                symptom: "permission denied",
                cause: "权限不足",
                diagnosis: vec![
                    "ls -la /var/lib/blockchain",
                    "whoami",
                    "groups",
                ],
                solution: vec![
                    "sudo chown -R user:group /var/lib/blockchain",
                    "chmod 755 /var/lib/blockchain",
                    "以正确用户身份运行",
                ],
            },
            StartupIssue {
                symptom: "database corrupted",
                cause: "数据库文件损坏",
                diagnosis: vec![
                    "检查磁盘空间",
                    "查看数据库日志",
                    "尝试数据库修复工具",
                ],
                solution: vec![
                    "备份数据",
                    "使用repair工具修复",
                    "最坏情况：删除数据库重新同步",
                ],
            },
            StartupIssue {
                symptom: "config parse error",
                cause: "配置文件格式错误",
                diagnosis: vec![
                    "cat config.toml",
                    "验证JSON/TOML格式",
                ],
                solution: vec![
                    "检查语法错误（逗号、引号）",
                    "使用在线验证工具",
                    "恢复默认配置",
                ],
            },
        ]
    }
    
    /// 启动前检查清单
    pub fn pre_startup_checklist() -> Vec<&'static str> {
        vec![
            "[ ] 配置文件存在且格式正确",
            "[ ] 所需端口未被占用",
            "[ ] 文件权限正确",
            "[ ] 磁盘空间充足（>100GB推荐）",
            "[ ] 依赖服务运行（数据库等）",
            "[ ] 防火墙规则配置",
            "[ ] 网络连通性测试",
        ]
    }
}

#[derive(Debug)]
pub struct StartupIssue {
    symptom: &'static str,
    cause: &'static str,
    diagnosis: Vec<&'static str>,
    solution: Vec<&'static str>,
}
```

### 2.2 节点同步失败

```rust
/// 节点同步问题诊断
pub struct SyncTroubleshooting;

impl SyncTroubleshooting {
    pub fn common_sync_issues() -> Vec<SyncIssue> {
        vec![
            SyncIssue {
                symptom: "同步卡住不动",
                possible_causes: vec![
                    "网络问题",
                    "对等节点不足",
                    "数据库性能瓶颈",
                    "区块验证失败",
                ],
                diagnostic_steps: vec![
                    "检查对等节点数量：blockchain-cli net peer-count",
                    "查看同步状态：blockchain-cli sync status",
                    "检查网络连接：ping peer-node-ip",
                    "查看日志错误",
                ],
                solutions: vec![
                    "增加对等节点",
                    "检查防火墙配置",
                    "优化数据库配置",
                    "使用快照同步",
                ],
            },
            SyncIssue {
                symptom: "同步速度慢",
                possible_causes: vec![
                    "网络带宽限制",
                    "磁盘I/O慢",
                    "CPU计算慢",
                ],
                diagnostic_steps: vec![
                    "iftop -i eth0（监控网络）",
                    "iostat -x 1（监控磁盘）",
                    "top（监控CPU）",
                ],
                solutions: vec![
                    "使用SSD",
                    "增加peers数量",
                    "调整cache大小",
                    "启用快速同步模式",
                ],
            },
        ]
    }
}

#[derive(Debug)]
pub struct SyncIssue {
    symptom: &'static str,
    possible_causes: Vec<&'static str>,
    diagnostic_steps: Vec<&'static str>,
    solutions: Vec<&'static str>,
}
```

### 2.3 节点频繁崩溃

```rust
/// 节点崩溃问题诊断
pub struct NodeCrashTroubleshooting;

impl NodeCrashTroubleshooting {
    pub fn analyze_crash() -> CrashAnalysis {
        CrashAnalysis {
            data_collection: vec![
                "收集coredump文件",
                "获取panic堆栈信息",
                "查看系统日志：journalctl -u blockchain-node",
                "检查OOM killer日志：dmesg | grep -i kill",
            ],
            common_causes: vec![
                CrashCause {
                    cause: "内存不足（OOM）",
                    indicators: vec![
                        "日志中出现 'Out of memory'",
                        "dmesg中OOM killer记录",
                        "进程RSS持续增长",
                    ],
                    solutions: vec![
                        "增加系统内存",
                        "减小cache配置",
                        "启用swap（不推荐生产环境）",
                        "排查内存泄漏",
                    ],
                },
                CrashCause {
                    cause: "未处理的panic",
                    indicators: vec![
                        "堆栈跟踪显示panic",
                        "特定操作触发崩溃",
                    ],
                    solutions: vec![
                        "更新到最新版本",
                        "提交bug报告",
                        "添加错误处理",
                    ],
                },
                CrashCause {
                    cause: "数据库损坏",
                    indicators: vec![
                        "访问数据库时崩溃",
                        "checksum错误",
                    ],
                    solutions: vec![
                        "运行数据库修复工具",
                        "从备份恢复",
                        "重新同步",
                    ],
                },
            ],
        }
    }
}

#[derive(Debug)]
pub struct CrashAnalysis {
    data_collection: Vec<&'static str>,
    common_causes: Vec<CrashCause>,
}

#[derive(Debug)]
pub struct CrashCause {
    cause: &'static str,
    indicators: Vec<&'static str>,
    solutions: Vec<&'static str>,
}
```

## 3. 共识问题

### 3.1 共识失败

```rust
/// 共识失败诊断
pub struct ConsensusTroubleshooting;

impl ConsensusTroubleshooting {
    pub fn diagnose_consensus_failure() -> ConsensusAnalysis {
        ConsensusAnalysis {
            common_scenarios: vec![
                ConsensusScenario {
                    scenario: "无法达成共识",
                    indicators: vec![
                        "区块提议超时",
                        "投票不足",
                        "日志显示consensus timeout",
                    ],
                    possible_causes: vec![
                        "验证者节点数量不足",
                        "网络分区",
                        "时钟不同步",
                        "节点性能差异大",
                    ],
                    diagnostic_commands: vec![
                        "查看活跃验证者：blockchain-cli validators list",
                        "检查网络连通性：blockchain-cli net peers",
                        "检查时间同步：timedatectl status",
                    ],
                    solutions: vec![
                        "确保2f+1个验证者在线（f为拜占庭节点数）",
                        "修复网络分区",
                        "同步所有节点时钟（NTP）",
                        "升级慢节点硬件",
                    ],
                },
                ConsensusScenario {
                    scenario: "共识性能下降",
                    indicators: vec![
                        "出块时间延长",
                        "共识轮次增加",
                    ],
                    possible_causes: vec![
                        "验证者负载高",
                        "网络延迟大",
                        "交易验证慢",
                    ],
                    diagnostic_commands: vec![
                        "监控验证者资源：top, iostat",
                        "测量网络延迟：ping, mtr",
                        "分析交易池：blockchain-cli txpool status",
                    ],
                    solutions: vec![
                        "优化验证者配置",
                        "增加带宽",
                        "优化交易验证逻辑",
                    ],
                },
            ],
        }
    }
}

#[derive(Debug)]
pub struct ConsensusAnalysis {
    common_scenarios: Vec<ConsensusScenario>,
}

#[derive(Debug)]
pub struct ConsensusScenario {
    scenario: &'static str,
    indicators: Vec<&'static str>,
    possible_causes: Vec<&'static str>,
    diagnostic_commands: Vec<&'static str>,
    solutions: Vec<&'static str>,
}
```

### 3.2 分叉问题

```rust
/// 区块链分叉问题诊断
pub struct ForkTroubleshooting;

impl ForkTroubleshooting {
    pub fn analyze_fork() -> ForkAnalysis {
        ForkAnalysis {
            fork_types: vec![
                ForkType {
                    name: "短暂分叉",
                    description: "正常现象，通常自动解决",
                    duration: "几个区块",
                    handling: vec![
                        "无需人工干预",
                        "等待网络自动收敛",
                        "监控是否持续",
                    ],
                },
                ForkType {
                    name: "持久分叉",
                    description: "网络分区或共识bug导致",
                    duration: "数十个区块以上",
                    handling: vec![
                        "识别主链和副链",
                        "检查网络分区",
                        "验证共识规则",
                        "必要时协调人工干预",
                    ],
                },
                ForkType {
                    name: "恶意分叉",
                    description: "51%攻击或长程攻击",
                    duration: "可能很长",
                    handling: vec![
                        "立即通知社区",
                        "暂停服务",
                        "分析攻击来源",
                        "社区投票决定应对策略",
                    ],
                },
            ],
            detection_methods: vec![
                "监控孤块率",
                "比较不同节点的链头",
                "分析共识日志",
                "使用区块浏览器对比",
            ],
            prevention: vec![
                "确保网络健康",
                "及时同步时钟",
                "保持软件版本一致",
                "设置检查点（checkpoint）",
            ],
        }
    }
}

#[derive(Debug)]
pub struct ForkAnalysis {
    fork_types: Vec<ForkType>,
    detection_methods: Vec<&'static str>,
    prevention: Vec<&'static str>,
}

#[derive(Debug)]
pub struct ForkType {
    name: &'static str,
    description: &'static str,
    duration: &'static str,
    handling: Vec<&'static str>,
}
```

### 3.3 网络分区

```rust
/// 网络分区问题诊断
pub struct NetworkPartitionTroubleshooting;

impl NetworkPartitionTroubleshooting {
    pub fn diagnose_partition() -> PartitionAnalysis {
        PartitionAnalysis {
            detection: vec![
                "节点无法连接到部分对等节点",
                "共识超时频繁",
                "区块高度差异大",
                "不同节点报告不同的链头",
            ],
            diagnosis_steps: vec![
                "1. 绘制网络拓扑图",
                "2. 测试节点间连通性（ping, traceroute）",
                "3. 检查防火墙/安全组规则",
                "4. 查看BGP路由（如果跨ISP）",
                "5. 分析网络日志",
            ],
            resolution: vec![
                "修复网络连接",
                "调整防火墙规则",
                "添加中继节点",
                "等待网络自愈",
                "必要时手动协调",
            ],
            prevention: vec![
                "多区域部署",
                "冗余网络链路",
                "监控网络健康",
                "定期演练故障恢复",
            ],
        }
    }
}

#[derive(Debug)]
pub struct PartitionAnalysis {
    detection: Vec<&'static str>,
    diagnosis_steps: Vec<&'static str>,
    resolution: Vec<&'static str>,
    prevention: Vec<&'static str>,
}
```

## 4. 交易问题

### 4.1 交易pending

```rust
/// 交易pending问题诊断
pub struct TransactionPendingTroubleshooting;

impl TransactionPendingTroubleshooting {
    pub fn analyze_pending_tx() -> PendingTxAnalysis {
        PendingTxAnalysis {
            common_causes: vec![
                PendingCause {
                    cause: "Gas price过低",
                    how_to_check: "比较交易gas price与当前网络gas price",
                    solution: vec![
                        "提高gas price重新发送",
                        "使用gas price预言机",
                        "等待网络拥堵缓解",
                    ],
                },
                PendingCause {
                    cause: "Nonce错误",
                    how_to_check: "检查账户nonce与交易nonce",
                    solution: vec![
                        "使用正确的nonce",
                        "等待前序交易确认",
                        "取消卡住的交易（发送nonce相同、gas price更高的交易）",
                    ],
                },
                PendingCause {
                    cause: "交易池满",
                    how_to_check: "查询节点交易池状态",
                    solution: vec![
                        "等待交易池处理",
                        "提高gas price以优先处理",
                        "向其他节点提交",
                    ],
                },
                PendingCause {
                    cause: "余额不足",
                    how_to_check: "查询账户余额是否 >= value + gas",
                    solution: vec![
                        "充值账户",
                        "降低交易金额",
                        "降低gas limit",
                    ],
                },
            ],
            查询命令: vec![
                "eth_getTransactionByHash - 获取交易详情",
                "eth_getTransactionReceipt - 查看是否已确认",
                "txpool_status - 查看交易池状态",
                "eth_gasPrice - 获取当前gas price",
                "eth_getTransactionCount - 获取账户nonce",
            ],
        }
    }
}

#[derive(Debug)]
pub struct PendingTxAnalysis {
    common_causes: Vec<PendingCause>,
    查询命令: Vec<&'static str>,
}

#[derive(Debug)]
pub struct PendingCause {
    cause: &'static str,
    how_to_check: &'static str,
    solution: Vec<&'static str>,
}
```

### 4.2 交易失败

```rust
/// 交易失败问题诊断
pub struct TransactionFailureTroubleshooting;

impl TransactionFailureTroubleshooting {
    pub fn common_failures() -> Vec<TxFailure> {
        vec![
            TxFailure {
                error_message: "out of gas",
                meaning: "Gas limit不足",
                diagnosis: "检查交易的gas used vs gas limit",
                solution: vec![
                    "增加gas limit",
                    "优化合约代码减少gas消耗",
                    "分批处理数据",
                ],
            },
            TxFailure {
                error_message: "revert",
                meaning: "智能合约执行被回滚",
                diagnosis: "查看revert原因（如果有）",
                solution: vec![
                    "检查合约require/assert条件",
                    "验证输入参数",
                    "检查合约状态",
                ],
            },
            TxFailure {
                error_message: "invalid signature",
                meaning: "签名无效",
                diagnosis: "验证私钥、签名算法",
                solution: vec![
                    "确认使用正确私钥",
                    "检查签名格式（v,r,s）",
                    "验证chain ID",
                ],
            },
            TxFailure {
                error_message: "nonce too low",
                meaning: "Nonce已被使用",
                diagnosis: "对比交易nonce和账户current nonce",
                solution: vec![
                    "使用更新的nonce",
                    "等待pending交易确认",
                ],
            },
        ]
    }
}

#[derive(Debug)]
pub struct TxFailure {
    error_message: &'static str,
    meaning: &'static str,
    diagnosis: &'static str,
    solution: Vec<&'static str>,
}
```

### 4.3 Gas估算错误

```rust
/// Gas估算问题
pub struct GasEstimationTroubleshooting;

impl GasEstimationTroubleshooting {
    pub fn common_issues() -> Vec<GasIssue> {
        vec![
            GasIssue {
                issue: "估算失败",
                causes: vec![
                    "合约调用会revert",
                    "状态依赖导致模拟失败",
                ],
                solutions: vec![
                    "手动设置gas limit",
                    "检查合约逻辑",
                    "使用更高的gas limit（1.5x估算值）",
                ],
            },
            GasIssue {
                issue: "估算不准确",
                causes: vec![
                    "状态变化影响gas消耗",
                    "估算时间和执行时间不同",
                ],
                solutions: vec![
                    "添加安全系数（20-50%）",
                    "使用历史数据校准",
                ],
            },
        ]
    }
}

#[derive(Debug)]
pub struct GasIssue {
    issue: &'static str,
    causes: Vec<&'static str>,
    solutions: Vec<&'static str>,
}
```

## 5. 智能合约问题

### 5.1 合约部署失败

```rust
/// 合约部署问题诊断
pub struct ContractDeploymentTroubleshooting;

impl ContractDeploymentTroubleshooting {
    pub fn deployment_issues() -> Vec<DeploymentIssue> {
        vec![
            DeploymentIssue {
                error: "Contract code size exceeds maximum",
                explanation: "合约字节码超过24KB限制（EIP-170）",
                solutions: vec![
                    "拆分合约为多个子合约",
                    "使用库（library）",
                    "启用优化器：solc --optimize",
                    "移除未使用的代码",
                ],
            },
            DeploymentIssue {
                error: "Constructor revert",
                explanation: "构造函数执行失败",
                solutions: vec![
                    "检查构造函数参数",
                    "验证初始化逻辑",
                    "确保依赖合约已部署",
                ],
            },
            DeploymentIssue {
                error: "Insufficient funds",
                explanation: "部署账户余额不足",
                solutions: vec![
                    "充值部署账户",
                    "降低gas price",
                    "优化合约减少部署成本",
                ],
            },
        ]
    }
}

#[derive(Debug)]
pub struct DeploymentIssue {
    error: &'static str,
    explanation: &'static str,
    solutions: Vec<&'static str>,
}
```

### 5.2 合约调用错误

```rust
/// 合约调用问题诊断
pub struct ContractCallTroubleshooting;

impl ContractCallTroubleshooting {
    pub fn call_errors() -> Vec<CallError> {
        vec![
            CallError {
                error: "execution reverted",
                possible_reasons: vec![
                    "require条件不满足",
                    "assert失败",
                    "调用不存在的函数",
                    "访问权限不足",
                ],
                debugging: vec![
                    "使用Hardhat/Foundry调试",
                    "添加事件日志",
                    "使用Tenderly模拟",
                    "查看revert原因字符串",
                ],
            },
            CallError {
                error: "invalid opcode",
                possible_reasons: vec![
                    "访问未初始化的变量",
                    "除以零",
                    "数组越界（旧版本）",
                ],
                debugging: vec![
                    "检查变量初始化",
                    "添加边界检查",
                    "使用SafeMath",
                ],
            },
        ]
    }
}

#[derive(Debug)]
pub struct CallError {
    error: &'static str,
    possible_reasons: Vec<&'static str>,
    debugging: Vec<&'static str>,
}
```

### 5.3 合约安全漏洞

已在安全最佳实践文档中详细说明。

## 6. 网络问题

### 6.1 P2P连接问题

```rust
/// P2P网络连接问题诊断
pub struct P2PConnectionTroubleshooting;

impl P2PConnectionTroubleshooting {
    pub fn connection_issues() -> Vec<P2PIs sue> {
        vec![
            P2PIssue {
                symptom: "无法连接到任何peer",
                checks: vec![
                    "防火墙是否阻止P2P端口",
                    "NAT穿透是否配置",
                    "bootnode地址是否正确",
                    "网络是否可达",
                ],
                solutions: vec![
                    "开放P2P端口（默认30303）",
                    "配置端口转发/UPnP",
                    "使用公网IP的bootnode",
                    "检查网络连通性",
                ],
            },
            P2PIssue {
                symptom: "peer数量少",
                checks: vec![
                    "maxpeers配置",
                    "网络拓扑",
                    "节点声誉",
                ],
                solutions: vec![
                    "增加maxpeers值",
                    "添加静态节点",
                    "改善节点网络环境",
                ],
            },
        ]
    }
}

#[derive(Debug)]
pub struct P2PIssue {
    symptom: &'static str,
    checks: Vec<&'static str>,
    solutions: Vec<&'static str>,
}
```

### 6.2 网络延迟高

```rust
/// 网络延迟问题诊断
pub struct NetworkLatencyTroubleshooting;

impl NetworkLatencyTroubleshooting {
    pub fn diagnose_latency() -> LatencyAnalysis {
        LatencyAnalysis {
            measurement_tools: vec![
                "ping - ICMP延迟",
                "mtr - 路由跟踪",
                "iperf3 - 带宽测试",
                "blockchain-cli net latency - 应用层延迟",
            ],
            common_causes: vec![
                "地理距离远",
                "网络拥塞",
                "路由问题",
                "ISP限速",
            ],
            solutions: vec![
                "选择地理位置近的peer",
                "使用CDN/专线",
                "优化路由",
                "增加带宽",
            ],
        }
    }
}

#[derive(Debug)]
pub struct LatencyAnalysis {
    measurement_tools: Vec<&'static str>,
    common_causes: Vec<&'static str>,
    solutions: Vec<&'static str>,
}
```

### 6.3 DDoS攻击

```rust
/// DDoS攻击防护
pub struct DDoSTroubleshooting;

impl DDoSTroubleshooting {
    pub fn detection_and_mitigation() -> DDoSResponse {
        DDoSResponse {
            detection_indicators: vec![
                "异常高的连接请求",
                "CPU/带宽突然飙升",
                "大量来自同一IP/网段的请求",
                "服务响应缓慢或超时",
            ],
            immediate_actions: vec![
                "启用速率限制",
                "封禁恶意IP",
                "启用DDoS防护服务（Cloudflare等）",
                "限制新连接速率",
            ],
            long_term_solutions: vec![
                "部署DDoS防护系统",
                "使用负载均衡",
                "实施IP白名单机制",
                "增加基础设施冗余",
            ],
        }
    }
}

#[derive(Debug)]
pub struct DDoSResponse {
    detection_indicators: Vec<&'static str>,
    immediate_actions: Vec<&'static str>,
    long_term_solutions: Vec<&'static str>,
}
```

## 7. 存储问题

### 7.1 数据库损坏

```rust
/// 数据库损坏问题
pub struct DatabaseCorruptionTroubleshooting;

impl DatabaseCorruptionTroubleshooting {
    pub fn handle_corruption() -> CorruptionHandling {
        CorruptionHandling {
            detection: vec![
                "启动时报checksum error",
                "查询时崩溃",
                "数据不一致",
            ],
            recovery_steps: vec![
                "1. 停止节点",
                "2. 备份当前数据（即使已损坏）",
                "3. 尝试数据库修复工具",
                "4. 如果修复失败，从备份恢复",
                "5. 最后手段：删除数据库重新同步",
            ],
            prevention: vec![
                "定期备份",
                "使用RAID磁盘阵列",
                "启用数据库校验",
                "优雅关闭节点（避免强杀）",
            ],
        }
    }
}

#[derive(Debug)]
pub struct CorruptionHandling {
    detection: Vec<&'static str>,
    recovery_steps: Vec<&'static str>,
    prevention: Vec<&'static str>,
}
```

### 7.2 磁盘空间不足

```rust
/// 磁盘空间问题
pub struct DiskSpaceTroubleshooting;

impl DiskSpaceTroubleshooting {
    pub fn manage_disk_space() -> DiskManagement {
        DiskManagement {
            monitoring: vec![
                "df -h - 查看磁盘使用",
                "du -sh /var/lib/blockchain/* - 查看各目录大小",
                "设置监控告警（<20%剩余空间）",
            ],
            immediate_solutions: vec![
                "删除旧日志文件",
                "清理临时文件",
                "压缩归档数据",
            ],
            long_term_solutions: vec![
                "增加磁盘容量",
                "启用pruning模式",
                "使用外部存储",
                "实施数据生命周期管理",
            ],
        }
    }
}

#[derive(Debug)]
pub struct DiskManagement {
    monitoring: Vec<&'static str>,
    immediate_solutions: Vec<&'static str>,
    long_term_solutions: Vec<&'static str>,
}
```

### 7.3 数据同步慢

已在节点同步问题中说明。

## 8. 性能问题

### 8.1 TPS低

已在性能优化文档中详细说明。

### 8.2 内存泄漏

```rust
/// 内存泄漏诊断
pub struct MemoryLeakTroubleshooting;

impl MemoryLeakTroubleshooting {
    pub fn diagnose_memory_leak() -> MemoryLeakAnalysis {
        MemoryLeakAnalysis {
            detection: vec![
                "内存使用持续增长",
                "最终导致OOM",
                "GC频繁但内存不释放",
            ],
            profiling_tools: vec![
                "valgrind - C/C++内存分析",
                "pprof - Go内存分析",
                "heaptrack - 堆内存分析",
                "/proc/{pid}/smaps - 内存映射",
            ],
            common_causes: vec![
                "未关闭的连接/文件句柄",
                "缓存无界增长",
                "循环引用",
                "事件监听器未移除",
            ],
            solutions: vec![
                "修复代码泄漏点",
                "设置缓存上限",
                "使用弱引用",
                "定期重启（临时方案）",
            ],
        }
    }
}

#[derive(Debug)]
pub struct MemoryLeakAnalysis {
    detection: Vec<&'static str>,
    profiling_tools: Vec<&'static str>,
    common_causes: Vec<&'static str>,
    solutions: Vec<&'static str>,
}
```

### 8.3 CPU占用高

```rust
/// CPU占用高问题诊断
pub struct HighCPUTroubleshooting;

impl HighCPUTroubleshooting {
    pub fn analyze_high_cpu() -> CPUAnalysis {
        CPUAnalysis {
            profiling: vec![
                "top/htop - 实时监控",
                "perf top - 热点函数",
                "flamegraph - 火焰图分析",
                "pprof - Go CPU profile",
            ],
            common_causes: vec![
                "密集的加密运算",
                "区块验证",
                "P2P消息处理",
                "数据库查询",
                "无限循环/死锁",
            ],
            optimization: vec![
                "并行化处理",
                "缓存计算结果",
                "优化算法复杂度",
                "使用硬件加速",
            ],
        }
    }
}

#[derive(Debug)]
pub struct CPUAnalysis {
    profiling: Vec<&'static str>,
    common_causes: Vec<&'static str>,
    optimization: Vec<&'static str>,
}
```

## 9. 安全问题

### 9.1 私钥泄露

```rust
/// 私钥泄露应急响应
pub struct PrivateKeyCompromise;

impl PrivateKeyCompromise {
    pub fn incident_response() -> IncidentResponse {
        IncidentResponse {
            immediate_actions: vec![
                "1. 立即停止使用该密钥",
                "2. 转移资产到新地址",
                "3. 撤销相关权限",
                "4. 通知相关方",
            ],
            investigation: vec![
                "确定泄露范围",
                "追踪泄露来源",
                "评估损失",
                "记录事件",
            ],
            remediation: vec![
                "生成新密钥对",
                "更新所有相关配置",
                "加强密钥管理",
                "实施多签机制",
            ],
            prevention: vec![
                "使用硬件钱包/HSM",
                "密钥加密存储",
                "最小权限原则",
                "定期轮换密钥",
            ],
        }
    }
}

#[derive(Debug)]
pub struct IncidentResponse {
    immediate_actions: Vec<&'static str>,
    investigation: Vec<&'static str>,
    remediation: Vec<&'static str>,
    prevention: Vec<&'static str>,
}
```

### 9.2 51%攻击

```rust
/// 51%攻击检测与应对
pub struct FiftyOnePercentAttack;

impl FiftyOnePercentAttack {
    pub fn detection_and_response() -> AttackResponse {
        AttackResponse {
            detection_indicators: vec![
                "单一矿工/矿池算力超过51%",
                "出现深度区块重组",
                "双花交易",
                "审查交易",
            ],
            immediate_response: vec![
                "暂停大额交易",
                "增加确认数要求",
                "通知社区和交易所",
                "监控攻击者行为",
            ],
            long_term_solutions: vec![
                "切换共识算法（PoS）",
                "增加网络哈希率",
                "实施检查点机制",
                "联盟抵制攻击者",
            ],
        }
    }
}

#[derive(Debug)]
pub struct AttackResponse {
    detection_indicators: Vec<&'static str>,
    immediate_response: Vec<&'static str>,
    long_term_solutions: Vec<&'static str>,
}
```

### 9.3 女巫攻击

```rust
/// 女巫攻击防御
pub struct SybilAttackDefense;

impl SybilAttackDefense {
    pub fn defense_mechanisms() -> Vec<DefenseMechanism> {
        vec![
            DefenseMechanism {
                mechanism: "资源证明（PoW/PoS）",
                description: "创建身份需要成本",
                effectiveness: "高",
            },
            DefenseMechanism {
                mechanism: "声誉系统",
                description: "基于历史行为评分",
                effectiveness: "中",
            },
            DefenseMechanism {
                mechanism: "身份验证",
                description: "KYC/实名认证",
                effectiveness: "高（但牺牲匿名性）",
            },
            DefenseMechanism {
                mechanism: "社交图谱",
                description: "基于信任网络",
                effectiveness: "中",
            },
        ]
    }
}

#[derive(Debug)]
pub struct DefenseMechanism {
    mechanism: &'static str,
    description: &'static str,
    effectiveness: &'static str,
}
```

## 10. 工具与最佳实践

### 10.1 诊断工具

```rust
/// 诊断工具集
pub struct DiagnosticToolkit;

impl DiagnosticToolkit {
    pub fn essential_tools() -> Vec<DiagnosticTool> {
        vec![
            DiagnosticTool {
                category: "日志分析",
                tools: vec![
                    ("tail", "实时查看日志"),
                    ("grep", "搜索日志内容"),
                    ("awk/sed", "日志处理"),
                    ("logrotate", "日志轮转"),
                ],
            },
            DiagnosticTool {
                category: "网络诊断",
                tools: vec![
                    ("ping", "测试连通性"),
                    ("traceroute/mtr", "路由跟踪"),
                    ("netstat", "网络连接"),
                    ("tcpdump", "抓包分析"),
                    ("iftop", "实时流量监控"),
                ],
            },
            DiagnosticTool {
                category: "性能分析",
                tools: vec![
                    ("top/htop", "进程监控"),
                    ("iostat", "I/O统计"),
                    ("vmstat", "虚拟内存"),
                    ("perf", "性能分析"),
                    ("flamegraph", "火焰图"),
                ],
            },
            DiagnosticTool {
                category: "区块链专用",
                tools: vec![
                    ("blockchain-cli", "节点管理"),
                    ("etherscan", "区块浏览器"),
                    ("Tenderly", "交易模拟"),
                    ("Hardhat", "本地测试"),
                ],
            },
        ]
    }
}

#[derive(Debug)]
pub struct DiagnosticTool {
    category: &'static str,
    tools: Vec<(&'static str, &'static str)>,
}
```

### 10.2 监控工具

```rust
/// 监控工具推荐
pub struct MonitoringTools;

impl MonitoringTools {
    pub fn recommended_stack() -> MonitoringStack {
        MonitoringStack {
            metrics_collection: vec![
                ("Prometheus", "指标采集"),
                ("Node Exporter", "系统指标"),
                ("Process Exporter", "进程指标"),
            ],
            visualization: vec![
                ("Grafana", "指标可视化"),
                ("Kibana", "日志可视化"),
            ],
            alerting: vec![
                ("Alertmanager", "告警路由"),
                ("PagerDuty", "值班通知"),
            ],
            log_management: vec![
                ("ELK Stack", "日志聚合"),
                ("Loki", "轻量级日志"),
            ],
            apm: vec![
                ("Jaeger", "分布式追踪"),
                ("Zipkin", "追踪分析"),
            ],
        }
    }
}

#[derive(Debug)]
pub struct MonitoringStack {
    metrics_collection: Vec<(&'static str, &'static str)>,
    visualization: Vec<(&'static str, &'static str)>,
    alerting: Vec<(&'static str, &'static str)>,
    log_management: Vec<(&'static str, &'static str)>,
    apm: Vec<(&'static str, &'static str)>,
}
```

### 10.3 最佳实践

```rust
/// 故障排除最佳实践
pub struct TroubleshootingBestPractices;

impl TroubleshootingBestPractices {
    pub fn best_practices() -> Vec<BestPractice> {
        vec![
            BestPractice {
                category: "预防",
                practices: vec![
                    "定期备份数据",
                    "监控关键指标",
                    "保持软件更新",
                    "文档化配置",
                    "定期演练故障恢复",
                ],
            },
            BestPractice {
                category: "诊断",
                practices: vec![
                    "系统化诊断流程",
                    "保留现场（日志、配置）",
                    "逐步排除法",
                    "记录诊断过程",
                    "参考知识库",
                ],
            },
            BestPractice {
                category: "恢复",
                practices: vec![
                    "先备份再操作",
                    "准备回滚计划",
                    "小步快跑",
                    "验证修复效果",
                    "更新文档",
                ],
            },
            BestPractice {
                category: "事后",
                practices: vec![
                    "根因分析（Root Cause Analysis）",
                    "总结经验教训",
                    "改进预防措施",
                    "更新知识库",
                    "团队分享",
                ],
            },
        ]
    }
    
    /// 故障排除检查清单模板
    pub fn checklist_template() -> String {
        r#"
故障排除检查清单
==================

问题描述：
- 症状：
- 发生时间：
- 影响范围：
- 错误信息：

诊断步骤：
□ 收集日志
□ 检查系统资源
□ 验证网络连接
□ 查看配置文件
□ 对比正常状态

已尝试方案：
1. 
2. 
3. 

当前状态：
□ 问题已解决
□ 问题部分解决
□ 问题未解决

根本原因：

解决方案：

预防措施：

文档更新：
□ 更新知识库
□ 更新操作手册
□ 团队分享
        "#.to_string()
    }
}

#[derive(Debug)]
pub struct BestPractice {
    category: &'static str,
    practices: Vec<&'static str>,
}
```

## 总结

本文档提供了区块链系统常见问题的诊断与解决方案，涵盖：

1. **诊断方法论**：系统化流程、日志分析、性能监控
2. **节点问题**：启动失败、同步问题、频繁崩溃
3. **共识问题**：共识失败、分叉、网络分区
4. **交易问题**：pending、失败、gas估算
5. **智能合约**：部署、调用、安全漏洞
6. **网络问题**：P2P连接、延迟、DDoS
7. **存储问题**：数据库损坏、空间不足
8. **性能问题**：TPS低、内存泄漏、CPU高
9. **安全问题**：私钥泄露、51%攻击、女巫攻击
10. **工具与实践**：诊断工具、监控工具、最佳实践

**关键要点**：

- 系统化诊断流程
- 保留现场证据
- 参考知识库
- 事后总结改进

---

**文档版本**: v1.0.0  
**最后更新**: 2025年10月17日  
**作者**: Rust区块链技术团队  
**相关文档**:

- [11_PERFORMANCE_OPTIMIZATION.md](./11_PERFORMANCE_OPTIMIZATION.md) - 性能优化
- [19_SECURITY_BEST_PRACTICES.md](./19_SECURITY_BEST_PRACTICES.md) - 安全最佳实践
- [27_CASE_STUDIES.md](./27_CASE_STUDIES.md) - 案例分析
