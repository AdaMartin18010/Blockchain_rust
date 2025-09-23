//! # 智能合约演示程序
//! 
//! 展示智能合约引擎的各种功能
//! Demonstrates various smart contract engine features

use blockchain::{
    smart_contract_engine::{
        SmartContractEngine, ContractInterface, ContractMethod, ContractParameter,
        ContractStats, ContractTemplate, ContractCategory,
    },
};
use serde_json::json;

fn main() {
    println!("🚀 智能合约演示程序");
    println!("🚀 Smart Contract Demo");
    println!();

    // 1. 创建智能合约引擎
    println!("📋 1. 创建智能合约引擎");
    let mut engine = SmartContractEngine::new();
    println!("✅ 智能合约引擎创建成功");

    // 2. 部署 ERC20 Token 合约
    println!("\n📋 2. 部署 ERC20 Token 合约");
    demo_erc20_token(&mut engine);

    // 3. 部署存储合约
    println!("\n📋 3. 部署存储合约");
    demo_storage_contract(&mut engine);

    // 4. 合约调用和交互
    println!("\n📋 4. 合约调用和交互");
    demo_contract_interaction(&mut engine);

    // 5. 合约管理功能
    println!("\n📋 5. 合约管理功能");
    demo_contract_management(&mut engine);

    // 6. 合约模板系统
    println!("\n📋 6. 合约模板系统");
    demo_contract_templates(&mut engine);

    // 7. 批量操作
    println!("\n📋 7. 批量操作演示");
    demo_batch_operations(&mut engine);

    // 8. 统计信息
    println!("\n📋 8. 合约统计信息");
    display_contract_stats(&engine);

    println!("\n🎉 智能合约演示完成！");
}

/// ERC20 Token 合约演示
fn demo_erc20_token(engine: &mut SmartContractEngine) {
    // 创建 ERC20 合约接口
    let interface = ContractInterface {
        name: "ERC20Token".to_string(),
        methods: vec![
            ContractMethod {
                name: "transfer".to_string(),
                inputs: vec![
                    ContractParameter { name: "to".to_string(), param_type: "address".to_string() },
                    ContractParameter { name: "amount".to_string(), param_type: "uint256".to_string() },
                ],
                outputs: vec![ContractParameter { name: "success".to_string(), param_type: "bool".to_string() }],
                payable: false,
                constant: false,
            },
            ContractMethod {
                name: "balanceOf".to_string(),
                inputs: vec![ContractParameter { name: "owner".to_string(), param_type: "address".to_string() }],
                outputs: vec![ContractParameter { name: "balance".to_string(), param_type: "uint256".to_string() }],
                payable: false,
                constant: true,
            },
            ContractMethod {
                name: "totalSupply".to_string(),
                inputs: vec![],
                outputs: vec![ContractParameter { name: "supply".to_string(), param_type: "uint256".to_string() }],
                payable: false,
                constant: true,
            },
            ContractMethod {
                name: "mint".to_string(),
                inputs: vec![
                    ContractParameter { name: "to".to_string(), param_type: "address".to_string() },
                    ContractParameter { name: "amount".to_string(), param_type: "uint256".to_string() },
                ],
                outputs: vec![],
                payable: false,
                constant: false,
            },
        ],
        events: vec![],
    };

    // 模拟合约代码（实际应该是编译后的 WASM 字节码）
    let contract_code = b"ERC20 Token Contract Code".to_vec();

    // 部署合约
    match engine.deploy_contract(
        contract_code,
        "owner".to_string(),
        interface,
        1000000, // 初始余额
    ) {
        Ok(address) => {
            println!("✅ ERC20 Token 合约部署成功");
            println!("   - 合约地址: {}", address);
            println!("   - 初始余额: 1,000,000 tokens");
        }
        Err(e) => {
            println!("❌ ERC20 Token 合约部署失败: {}", e);
        }
    }
}

/// 存储合约演示
fn demo_storage_contract(engine: &mut SmartContractEngine) {
    // 创建存储合约接口
    let interface = ContractInterface {
        name: "Storage".to_string(),
        methods: vec![
            ContractMethod {
                name: "set".to_string(),
                inputs: vec![
                    ContractParameter { name: "key".to_string(), param_type: "string".to_string() },
                    ContractParameter { name: "value".to_string(), param_type: "string".to_string() },
                ],
                outputs: vec![],
                payable: false,
                constant: false,
            },
            ContractMethod {
                name: "get".to_string(),
                inputs: vec![ContractParameter { name: "key".to_string(), param_type: "string".to_string() }],
                outputs: vec![ContractParameter { name: "value".to_string(), param_type: "string".to_string() }],
                payable: false,
                constant: true,
            },
            ContractMethod {
                name: "delete".to_string(),
                inputs: vec![ContractParameter { name: "key".to_string(), param_type: "string".to_string() }],
                outputs: vec![],
                payable: false,
                constant: false,
            },
            ContractMethod {
                name: "getAll".to_string(),
                inputs: vec![],
                outputs: vec![ContractParameter { name: "data".to_string(), param_type: "map".to_string() }],
                payable: false,
                constant: true,
            },
        ],
        events: vec![],
    };

    // 模拟合约代码
    let contract_code = b"Storage Contract Code".to_vec();

    // 部署合约
    match engine.deploy_contract(
        contract_code,
        "storage_owner".to_string(),
        interface,
        0,
    ) {
        Ok(address) => {
            println!("✅ 存储合约部署成功");
            println!("   - 合约地址: {}", address);
        }
        Err(e) => {
            println!("❌ 存储合约部署失败: {}", e);
        }
    }
}

/// 合约交互演示
fn demo_contract_interaction(engine: &mut SmartContractEngine) {
    // 获取已部署的合约
    let contracts = engine.get_contract_addresses();
    
    if contracts.is_empty() {
        println!("❌ 没有已部署的合约");
        return;
    }

    let token_address = &contracts[0];
    println!("✅ 开始与合约交互");

    // 调用合约方法
    let context = blockchain::smart_contract_engine::ExecutionContext {
        caller: "user1".to_string(),
        value: 0,
        gas_limit: 100000,
        gas_used: 0,
        block_height: 1,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        contract_address: token_address.clone(),
    };

    // 调用 balanceOf 方法
    match engine.call_contract(
        token_address,
        "balanceOf",
        b"owner".as_slice(),
        context.clone(),
    ) {
        Ok(result) => {
            println!("✅ 调用 balanceOf 成功");
            println!("   - 结果: {:?}", result);
        }
        Err(e) => {
            println!("❌ 调用 balanceOf 失败: {}", e);
        }
    }

    // 调用 transfer 方法
    match engine.call_contract(
        token_address,
        "transfer",
        b"user1,1000".as_slice(),
        context,
    ) {
        Ok(result) => {
            println!("✅ 调用 transfer 成功");
            println!("   - 结果: {:?}", result);
        }
        Err(e) => {
            println!("❌ 调用 transfer 失败: {}", e);
        }
    }
}

/// 合约管理功能演示
fn demo_contract_management(engine: &mut SmartContractEngine) {
    let contracts = engine.get_contract_addresses();
    
    if contracts.is_empty() {
        println!("❌ 没有已部署的合约");
        return;
    }

    let contract_address = &contracts[0];
    println!("✅ 开始合约管理操作");

    // 暂停合约
    match engine.pause_contract(contract_address, "owner") {
        Ok(_) => {
            println!("✅ 合约暂停成功");
        }
        Err(e) => {
            println!("❌ 合约暂停失败: {}", e);
        }
    }

    // 恢复合约
    match engine.resume_contract(contract_address, "owner") {
        Ok(_) => {
            println!("✅ 合约恢复成功");
        }
        Err(e) => {
            println!("❌ 合约恢复失败: {}", e);
        }
    }

    // 升级合约
    let new_code = b"Updated ERC20 Token Contract Code".to_vec();
    match engine.upgrade_contract(contract_address, new_code, "owner") {
        Ok(_) => {
            println!("✅ 合约升级成功");
        }
        Err(e) => {
            println!("❌ 合约升级失败: {}", e);
        }
    }
}

/// 合约模板系统演示
fn demo_contract_templates(engine: &mut SmartContractEngine) {
    println!("✅ 获取合约模板");
    
    let templates = engine.get_contract_templates();
    
    for template in &templates {
        println!("📄 模板: {}", template.name);
        println!("   - 描述: {}", template.description);
        println!("   - 分类: {:?}", template.category);
        println!("   - 版本: {}", template.version);
        println!("   - 作者: {}", template.author);
        println!("   - 方法数量: {}", template.abi.methods.len());
        println!();
    }

    println!("📊 模板统计:");
    println!("   - 总模板数: {}", templates.len());
    
    let categories: std::collections::HashMap<ContractCategory, usize> = 
        templates.iter().fold(std::collections::HashMap::new(), |mut acc, template| {
            *acc.entry(template.category.clone()).or_insert(0) += 1;
            acc
        });

    for (category, count) in categories {
        println!("   - {:?}: {} 个", category, count);
    }
}

/// 批量操作演示
fn demo_batch_operations(engine: &mut SmartContractEngine) {
    println!("✅ 开始批量部署合约");

    // 创建多个合约部署请求
    let deployments = vec![
        blockchain::smart_contract_engine::ContractDeployment {
            code: b"Contract 1 Code".to_vec(),
            owner: "owner1".to_string(),
            interface: ContractInterface {
                name: "Contract1".to_string(),
                methods: vec![],
                events: vec![],
            },
            initial_value: 1000,
        },
        blockchain::smart_contract_engine::ContractDeployment {
            code: b"Contract 2 Code".to_vec(),
            owner: "owner2".to_string(),
            interface: ContractInterface {
                name: "Contract2".to_string(),
                methods: vec![],
                events: vec![],
            },
            initial_value: 2000,
        },
        blockchain::smart_contract_engine::ContractDeployment {
            code: b"Contract 3 Code".to_vec(),
            owner: "owner3".to_string(),
            interface: ContractInterface {
                name: "Contract3".to_string(),
                methods: vec![],
                events: vec![],
            },
            initial_value: 3000,
        },
    ];

    // 批量部署
    let results = engine.batch_deploy(deployments);
    
    let mut success_count = 0;
    let mut failure_count = 0;
    
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(address) => {
                println!("✅ 合约 {} 部署成功: {}", i + 1, address);
                success_count += 1;
            }
            Err(e) => {
                println!("❌ 合约 {} 部署失败: {}", i + 1, e);
                failure_count += 1;
            }
        }
    }

    println!("📊 批量部署结果:");
    println!("   - 成功: {}", success_count);
    println!("   - 失败: {}", failure_count);
    println!("   - 成功率: {:.1}%", (success_count as f64 / (success_count + failure_count) as f64) * 100.0);
}

/// 显示合约统计信息
fn display_contract_stats(engine: &SmartContractEngine) {
    let stats = engine.get_contract_stats();
    
    println!("📊 合约统计信息:");
    println!("   - 总合约数: {}", stats.total_contracts);
    println!("   - 活跃合约数: {}", stats.active_contracts);
    println!("   - 总调用次数: {}", stats.total_calls);
    println!("   - 总 Gas 消耗: {}", stats.total_gas_used);
    println!("   - 平均每次调用 Gas: {}", stats.average_gas_per_call);
    
    // 显示已部署的合约
    let contracts = engine.get_contract_addresses();
    println!("   - 已部署合约:");
    for (i, address) in contracts.iter().enumerate() {
        println!("     {}. {}", i + 1, address);
    }

    // 显示合约状态
    println!("   - 合约状态详情:");
    for address in &contracts {
        if let Ok(state) = engine.get_contract_state(address) {
            println!("     📄 合约: {}", address);
            println!("       - 所有者: {}", state.owner);
            println!("       - 余额: {}", state.balance);
            println!("       - 版本: {}", state.version);
            println!("       - 状态: {}", if state.is_active { "活跃" } else { "暂停" });
            println!("       - 调用次数: {}", state.call_count);
            println!("       - Gas 消耗: {}", state.total_gas_used);
            println!("       - 存储项数: {}", state.storage.len());
        }
    }
}

/// 性能测试
fn performance_test() {
    println!("\n⚡ 智能合约性能测试");
    
    let mut engine = SmartContractEngine::new();
    let start_time = std::time::Instant::now();
    
    // 部署多个合约
    for i in 0..10 {
        let interface = ContractInterface {
            name: format!("TestContract{}", i),
            methods: vec![],
            events: vec![],
        };
        
        let _ = engine.deploy_contract(
            b"test contract code".to_vec(),
            format!("owner{}", i),
            interface,
            1000,
        );
    }
    
    let deployment_time = start_time.elapsed();
    println!("✅ 部署 10 个合约耗时: {:?}", deployment_time);
    
    // 调用测试
    let contracts = engine.get_contract_addresses();
    let call_start = std::time::Instant::now();
    
    for _ in 0..100 {
        if let Some(address) = contracts.first() {
            let context = blockchain::smart_contract_engine::ExecutionContext {
                caller: "tester".to_string(),
                value: 0,
                gas_limit: 10000,
                gas_used: 0,
                block_height: 1,
                contract_address: address.clone(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            
            let _ = engine.call_contract(address, "testMethod", b"test".as_slice(), context);
        }
    }
    
    let call_time = call_start.elapsed();
    println!("✅ 100 次合约调用耗时: {:?}", call_time);
    
    let stats = engine.get_contract_stats();
    println!("📊 性能统计:");
    println!("   - 平均部署时间: {:?}", deployment_time / 10);
    println!("   - 平均调用时间: {:?}", call_time / 100);
    println!("   - 总 Gas 消耗: {}", stats.total_gas_used);
}
