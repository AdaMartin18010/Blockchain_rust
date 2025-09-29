//! API文档生成模块
//! 
//! 提供自动生成API文档的功能

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// API文档结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDocumentation {
    pub title: String,
    pub version: String,
    pub description: String,
    pub modules: Vec<ModuleDocumentation>,
}

/// 模块文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDocumentation {
    pub name: String,
    pub description: String,
    pub functions: Vec<FunctionDocumentation>,
    pub structs: Vec<StructDocumentation>,
    pub enums: Vec<EnumDocumentation>,
}

/// 函数文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDocumentation {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ParameterDocumentation>,
    pub return_type: String,
    pub return_description: String,
    pub is_async: bool,
    pub examples: Vec<String>,
}

/// 参数文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDocumentation {
    pub name: String,
    pub type_name: String,
    pub description: String,
    pub is_required: bool,
}

/// 结构体文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDocumentation {
    pub name: String,
    pub description: String,
    pub fields: Vec<FieldDocumentation>,
}

/// 字段文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDocumentation {
    pub name: String,
    pub type_name: String,
    pub description: String,
}

/// 枚举文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDocumentation {
    pub name: String,
    pub description: String,
    pub variants: Vec<VariantDocumentation>,
}

/// 变体文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantDocumentation {
    pub name: String,
    pub description: String,
}

/// 文档生成器
pub struct DocumentationGenerator {
    modules: HashMap<String, ModuleDocumentation>,
}

impl DocumentationGenerator {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }
    
    /// 生成完整的API文档
    pub fn generate_api_docs(&mut self) -> ApiDocumentation {
        self.add_core_module();
        self.add_cryptography_module();
        self.add_network_module();
        self.add_storage_module();
        self.add_consensus_module();
        self.add_smart_contracts_module();
        self.add_algorithms_module();
        
        ApiDocumentation {
            title: "Blockchain Rust Library".to_string(),
            version: "0.1.0".to_string(),
            description: "A comprehensive blockchain implementation in Rust".to_string(),
            modules: self.modules.values().cloned().collect(),
        }
    }
    
    fn add_core_module(&mut self) {
        let mut module = ModuleDocumentation {
            name: "core".to_string(),
            description: "Core blockchain data structures and types".to_string(),
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
        };
        
        // Block struct
        module.structs.push(StructDocumentation {
            name: "Block".to_string(),
            description: "Represents a block in the blockchain".to_string(),
            fields: vec![
                FieldDocumentation {
                    name: "header".to_string(),
                    type_name: "BlockHeader".to_string(),
                    description: "Block header containing metadata".to_string(),
                },
                FieldDocumentation {
                    name: "transactions".to_string(),
                    type_name: "Vec<Transaction>".to_string(),
                    description: "List of transactions in the block".to_string(),
                },
            ],
        });
        
        // Transaction struct
        module.structs.push(StructDocumentation {
            name: "Transaction".to_string(),
            description: "Represents a blockchain transaction".to_string(),
            fields: vec![
                FieldDocumentation {
                    name: "version".to_string(),
                    type_name: "u32".to_string(),
                    description: "Transaction version".to_string(),
                },
                FieldDocumentation {
                    name: "inputs".to_string(),
                    type_name: "Vec<TxInput>".to_string(),
                    description: "Transaction inputs".to_string(),
                },
                FieldDocumentation {
                    name: "outputs".to_string(),
                    type_name: "Vec<TxOutput>".to_string(),
                    description: "Transaction outputs".to_string(),
                },
            ],
        });
        
        // MerkleTree struct
        module.structs.push(StructDocumentation {
            name: "MerkleTree".to_string(),
            description: "Merkle tree for efficient data verification".to_string(),
            fields: vec![
                FieldDocumentation {
                    name: "root".to_string(),
                    type_name: "MerkleNode".to_string(),
                    description: "Root node of the tree".to_string(),
                },
                FieldDocumentation {
                    name: "leaves".to_string(),
                    type_name: "Vec<MerkleNode>".to_string(),
                    description: "Leaf nodes of the tree".to_string(),
                },
            ],
        });
        
        self.modules.insert("core".to_string(), module);
    }
    
    fn add_cryptography_module(&mut self) {
        let mut module = ModuleDocumentation {
            name: "cryptography".to_string(),
            description: "Cryptographic operations and algorithms".to_string(),
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
        };
        
        // HashEngine struct
        module.structs.push(StructDocumentation {
            name: "HashEngine".to_string(),
            description: "Hash function engine supporting multiple algorithms".to_string(),
            fields: vec![],
        });
        
        // HashEngine functions
        module.functions.push(FunctionDocumentation {
            name: "sha256".to_string(),
            description: "Compute SHA256 hash of input data".to_string(),
            parameters: vec![
                ParameterDocumentation {
                    name: "data".to_string(),
                    type_name: "&[u8]".to_string(),
                    description: "Input data to hash".to_string(),
                    is_required: true,
                },
            ],
            return_type: "[u8; 32]".to_string(),
            return_description: "SHA256 hash as 32-byte array".to_string(),
            is_async: false,
            examples: vec![
                "let hash = engine.sha256(b\"hello world\");".to_string(),
            ],
        });
        
        module.functions.push(FunctionDocumentation {
            name: "blake2b".to_string(),
            description: "Compute Blake2b hash of input data".to_string(),
            parameters: vec![
                ParameterDocumentation {
                    name: "data".to_string(),
                    type_name: "&[u8]".to_string(),
                    description: "Input data to hash".to_string(),
                    is_required: true,
                },
            ],
            return_type: "[u8; 64]".to_string(),
            return_description: "Blake2b hash as 64-byte array".to_string(),
            is_async: false,
            examples: vec![
                "let hash = engine.blake2b(b\"hello world\");".to_string(),
            ],
        });
        
        // SignatureEngine struct
        module.structs.push(StructDocumentation {
            name: "SignatureEngine".to_string(),
            description: "Digital signature engine supporting multiple algorithms".to_string(),
            fields: vec![],
        });
        
        module.functions.push(FunctionDocumentation {
            name: "sign".to_string(),
            description: "Sign data with private key".to_string(),
            parameters: vec![
                ParameterDocumentation {
                    name: "data".to_string(),
                    type_name: "&[u8]".to_string(),
                    description: "Data to sign".to_string(),
                    is_required: true,
                },
                ParameterDocumentation {
                    name: "private_key".to_string(),
                    type_name: "&[u8]".to_string(),
                    description: "Private key for signing".to_string(),
                    is_required: true,
                },
                ParameterDocumentation {
                    name: "algorithm".to_string(),
                    type_name: "&str".to_string(),
                    description: "Signature algorithm (ecdsa, ed25519)".to_string(),
                    is_required: true,
                },
            ],
            return_type: "Result<Vec<u8>>".to_string(),
            return_description: "Digital signature".to_string(),
            is_async: false,
            examples: vec![
                "let signature = engine.sign(data, &private_key, \"ed25519\")?;".to_string(),
            ],
        });
        
        self.modules.insert("cryptography".to_string(), module);
    }
    
    fn add_network_module(&mut self) {
        let mut module = ModuleDocumentation {
            name: "network".to_string(),
            description: "P2P network communication components".to_string(),
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
        };
        
        // P2PNetwork struct
        module.structs.push(StructDocumentation {
            name: "P2PNetwork".to_string(),
            description: "Peer-to-peer network implementation".to_string(),
            fields: vec![],
        });
        
        module.functions.push(FunctionDocumentation {
            name: "start".to_string(),
            description: "Start the P2P network on specified port".to_string(),
            parameters: vec![
                ParameterDocumentation {
                    name: "port".to_string(),
                    type_name: "u16".to_string(),
                    description: "Port number to listen on".to_string(),
                    is_required: true,
                },
            ],
            return_type: "Result<()>".to_string(),
            return_description: "Result of network startup".to_string(),
            is_async: true,
            examples: vec![
                "network.start(8080).await?;".to_string(),
            ],
        });
        
        self.modules.insert("network".to_string(), module);
    }
    
    fn add_storage_module(&mut self) {
        let mut module = ModuleDocumentation {
            name: "storage".to_string(),
            description: "Blockchain data storage components".to_string(),
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
        };
        
        // BlockStorage struct
        module.structs.push(StructDocumentation {
            name: "BlockStorage".to_string(),
            description: "Block storage and retrieval system".to_string(),
            fields: vec![],
        });
        
        module.functions.push(FunctionDocumentation {
            name: "store_block".to_string(),
            description: "Store a block in the storage system".to_string(),
            parameters: vec![
                ParameterDocumentation {
                    name: "height".to_string(),
                    type_name: "u64".to_string(),
                    description: "Block height".to_string(),
                    is_required: true,
                },
                ParameterDocumentation {
                    name: "block".to_string(),
                    type_name: "Block".to_string(),
                    description: "Block to store".to_string(),
                    is_required: true,
                },
            ],
            return_type: "Result<()>".to_string(),
            return_description: "Result of storage operation".to_string(),
            is_async: true,
            examples: vec![
                "storage.store_block(1, block).await?;".to_string(),
            ],
        });
        
        self.modules.insert("storage".to_string(), module);
    }
    
    fn add_consensus_module(&mut self) {
        let mut module = ModuleDocumentation {
            name: "consensus".to_string(),
            description: "Blockchain consensus algorithms".to_string(),
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
        };
        
        // ProofOfWork struct
        module.structs.push(StructDocumentation {
            name: "ProofOfWork".to_string(),
            description: "Proof of Work consensus algorithm".to_string(),
            fields: vec![
                FieldDocumentation {
                    name: "difficulty".to_string(),
                    type_name: "u32".to_string(),
                    description: "Mining difficulty level".to_string(),
                },
            ],
        });
        
        module.functions.push(FunctionDocumentation {
            name: "mine_block".to_string(),
            description: "Mine a block using Proof of Work".to_string(),
            parameters: vec![
                ParameterDocumentation {
                    name: "block".to_string(),
                    type_name: "&mut Block".to_string(),
                    description: "Block to mine".to_string(),
                    is_required: true,
                },
            ],
            return_type: "Result<()>".to_string(),
            return_description: "Result of mining operation".to_string(),
            is_async: true,
            examples: vec![
                "pow.mine_block(&mut block).await?;".to_string(),
            ],
        });
        
        self.modules.insert("consensus".to_string(), module);
    }
    
    fn add_smart_contracts_module(&mut self) {
        let mut module = ModuleDocumentation {
            name: "smart_contracts".to_string(),
            description: "Smart contract execution environment".to_string(),
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
        };
        
        // VirtualMachine struct
        module.structs.push(StructDocumentation {
            name: "VirtualMachine".to_string(),
            description: "Smart contract virtual machine".to_string(),
            fields: vec![],
        });
        
        module.functions.push(FunctionDocumentation {
            name: "execute".to_string(),
            description: "Execute smart contract bytecode".to_string(),
            parameters: vec![
                ParameterDocumentation {
                    name: "bytecode".to_string(),
                    type_name: "&[u8]".to_string(),
                    description: "Contract bytecode to execute".to_string(),
                    is_required: true,
                },
                ParameterDocumentation {
                    name: "input".to_string(),
                    type_name: "&[u8]".to_string(),
                    description: "Input data for contract execution".to_string(),
                    is_required: true,
                },
            ],
            return_type: "Result<Vec<u8>>".to_string(),
            return_description: "Contract execution result".to_string(),
            is_async: true,
            examples: vec![
                "let result = vm.execute(&bytecode, &input).await?;".to_string(),
            ],
        });
        
        self.modules.insert("smart_contracts".to_string(), module);
    }
    
    fn add_algorithms_module(&mut self) {
        let mut module = ModuleDocumentation {
            name: "algorithms".to_string(),
            description: "Blockchain-specific algorithms and optimizations".to_string(),
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
        };
        
        // ConsensusAlgorithms struct
        module.structs.push(StructDocumentation {
            name: "ConsensusAlgorithms".to_string(),
            description: "Consensus-related algorithms".to_string(),
            fields: vec![],
        });
        
        module.functions.push(FunctionDocumentation {
            name: "calculate_difficulty".to_string(),
            description: "Calculate mining difficulty based on network conditions".to_string(),
            parameters: vec![
                ParameterDocumentation {
                    name: "current_height".to_string(),
                    type_name: "u64".to_string(),
                    description: "Current blockchain height".to_string(),
                    is_required: true,
                },
                ParameterDocumentation {
                    name: "target_time".to_string(),
                    type_name: "u64".to_string(),
                    description: "Target block time in seconds".to_string(),
                    is_required: true,
                },
                ParameterDocumentation {
                    name: "actual_time".to_string(),
                    type_name: "u64".to_string(),
                    description: "Actual block time in seconds".to_string(),
                    is_required: true,
                },
            ],
            return_type: "u32".to_string(),
            return_description: "Calculated difficulty value".to_string(),
            is_async: false,
            examples: vec![
                "let difficulty = algorithms.calculate_difficulty(100, 600, 550);".to_string(),
            ],
        });
        
        self.modules.insert("algorithms".to_string(), module);
    }
}

/// 生成Markdown格式的API文档
pub fn generate_markdown_docs() -> String {
    let mut generator = DocumentationGenerator::new();
    let docs = generator.generate_api_docs();
    
    let mut markdown = String::new();
    
    // 标题
    markdown.push_str(&format!("# {}\n\n", docs.title));
    markdown.push_str(&format!("**版本**: {}\n\n", docs.version));
    markdown.push_str(&format!("{}\n\n", docs.description));
    
    // 目录
    markdown.push_str("## 目录\n\n");
    for module in &docs.modules {
        markdown.push_str(&format!("- [{}](#{})\n", module.name, module.name));
    }
    markdown.push_str("\n");
    
    // 模块文档
    for module in &docs.modules {
        markdown.push_str(&format!("## {}\n\n", module.name));
        markdown.push_str(&format!("{}\n\n", module.description));
        
        // 结构体
        if !module.structs.is_empty() {
            markdown.push_str("### 结构体\n\n");
            for struct_doc in &module.structs {
                markdown.push_str(&format!("#### {}\n\n", struct_doc.name));
                markdown.push_str(&format!("{}\n\n", struct_doc.description));
                
                if !struct_doc.fields.is_empty() {
                    markdown.push_str("**字段**:\n\n");
                    markdown.push_str("| 名称 | 类型 | 描述 |\n");
                    markdown.push_str("|------|------|------|\n");
                    for field in &struct_doc.fields {
                        markdown.push_str(&format!("| {} | {} | {} |\n", field.name, field.type_name, field.description));
                    }
                    markdown.push_str("\n");
                }
            }
        }
        
        // 函数
        if !module.functions.is_empty() {
            markdown.push_str("### 函数\n\n");
            for func_doc in &module.functions {
                markdown.push_str(&format!("#### {}\n\n", func_doc.name));
                markdown.push_str(&format!("{}\n\n", func_doc.description));
                
                if !func_doc.parameters.is_empty() {
                    markdown.push_str("**参数**:\n\n");
                    markdown.push_str("| 名称 | 类型 | 描述 | 必需 |\n");
                    markdown.push_str("|------|------|------|------|\n");
                    for param in &func_doc.parameters {
                        markdown.push_str(&format!("| {} | {} | {} | {} |\n", 
                            param.name, param.type_name, param.description, 
                            if param.is_required { "是" } else { "否" }));
                    }
                    markdown.push_str("\n");
                }
                
                markdown.push_str(&format!("**返回类型**: `{}`\n\n", func_doc.return_type));
                markdown.push_str(&format!("{}\n\n", func_doc.return_description));
                
                if func_doc.is_async {
                    markdown.push_str("**注意**: 这是一个异步函数\n\n");
                }
                
                if !func_doc.examples.is_empty() {
                    markdown.push_str("**示例**:\n\n");
                    markdown.push_str("```rust\n");
                    for example in &func_doc.examples {
                        markdown.push_str(&format!("{}\n", example));
                    }
                    markdown.push_str("```\n\n");
                }
            }
        }
    }
    
    markdown
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_documentation_generation() {
        let mut generator = DocumentationGenerator::new();
        let docs = generator.generate_api_docs();
        
        assert_eq!(docs.title, "Blockchain Rust Library");
        assert_eq!(docs.version, "0.1.0");
        assert!(!docs.modules.is_empty());
    }
    
    #[test]
    fn test_markdown_generation() {
        let markdown = generate_markdown_docs();
        assert!(markdown.contains("# Blockchain Rust Library"));
        assert!(markdown.contains("## core"));
        assert!(markdown.contains("## cryptography"));
    }
}
