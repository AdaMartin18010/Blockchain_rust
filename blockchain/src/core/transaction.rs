// 交易结构定义
use serde::{Serialize, Deserialize};
use crate::core::{Result, BlockchainError};
// use std::collections::HashMap;

/// 交易结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// 版本号
    pub version: u32,
    
    /// 输入列表
    pub inputs: Vec<TxInput>,
    
    /// 输出列表
    pub outputs: Vec<TxOutput>,
    
    /// 锁定时间
    pub locktime: u32,
    
    /// 见证数据（可选）
    pub witness: Option<Witness>,
}

/// 交易输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInput {
    /// 前一个输出的引用
    pub previous_output: OutPoint,
    
    /// 脚本签名
    pub script_sig: Vec<u8>,
    
    /// 序列号
    pub sequence: u32,
    
    /// 地址
    pub address: String,
    
    /// 金额
    pub amount: u64,
    
    /// 公钥
    pub public_key: Vec<u8>,
    
    /// 签名
    pub signature: Vec<u8>,
}

/// 交易输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOutput {
    /// 金额
    pub amount: u64,
    
    /// 脚本公钥
    pub script_pubkey: Vec<u8>,
    
    /// 地址
    pub address: String,
}

/// 输出点（引用前一个输出）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutPoint {
    /// 交易哈希
    pub tx_hash: [u8; 32],
    
    /// 输出索引
    pub output_index: u32,
}

/// 见证数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Witness {
    /// 见证数据列表
    pub data: Vec<Vec<u8>>,
}

impl Transaction {
    /// 创建新交易
    pub fn new(inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        Self {
            version: 1,
            inputs,
            outputs,
            locktime: 0,
            witness: None,
        }
    }
    
    /// 创建转账交易
    pub fn create_transfer(
        from_address: String,
        to_address: String,
        amount: u64,
        fee: u64,
        private_key: &[u8],
    ) -> Result<Self> {
        // 创建输入
        let input = TxInput {
            previous_output: OutPoint {
                tx_hash: [0u8; 32], // 这里应该从UTXO中获取
                output_index: 0,
            },
            script_sig: Vec::new(),
            sequence: 0xFFFFFFFF,
            address: from_address.clone(),
            amount: amount + fee,
            public_key: Vec::new(), // 这里应该从私钥推导
            signature: Vec::new(),
        };
        
        // 创建输出
        let mut outputs = vec![
            TxOutput {
                amount,
                script_pubkey: Vec::new(),
                address: to_address,
            }
        ];
        
        // 如果有找零，添加找零输出
        if fee > 0 {
            outputs.push(TxOutput {
                amount: fee,
                script_pubkey: Vec::new(),
                address: from_address,
            });
        }
        
        let mut tx = Self::new(vec![input], outputs);
        
        // 签名交易
        tx.sign(private_key)?;
        
        Ok(tx)
    }
    
    /// 签名交易
    pub fn sign(&mut self, _private_key: &[u8]) -> Result<()> {
        // 计算交易哈希
        let _tx_hash = self.hash();
        
        // 为每个输入签名
        for input in &mut self.inputs {
            // 这里应该实现具体的签名逻辑
            // 暂时使用占位符
            input.signature = vec![0u8; 64]; // 64字节签名
            input.public_key = vec![0u8; 33]; // 33字节公钥
        }
        
        Ok(())
    }
    
    /// 验证交易
    pub fn validate(&self) -> Result<()> {
        // 1. 验证基本格式
        if self.inputs.is_empty() {
            return Err(BlockchainError::InvalidTransaction("No inputs".to_string()));
        }
        
        if self.outputs.is_empty() {
            return Err(BlockchainError::InvalidTransaction("No outputs".to_string()));
        }
        
        // 2. 验证输入输出金额
        let input_total: u64 = self.inputs.iter().map(|i| i.amount).sum();
        let output_total: u64 = self.outputs.iter().map(|o| o.amount).sum();
        
        if input_total < output_total {
            return Err(BlockchainError::InvalidTransaction("Insufficient input amount".to_string()));
        }
        
        // 3. 验证签名
        for input in &self.inputs {
            if !self.verify_signature(input)? {
                return Err(BlockchainError::InvalidTransaction("Invalid signature".to_string()));
            }
        }
        
        // 4. 验证锁定时间
        if self.locktime > 0 {
            // 这里应该检查锁定时间是否已到
        }
        
        Ok(())
    }
    
    /// 验证签名
    fn verify_signature(&self, _input: &TxInput) -> Result<bool> {
        // 这里应该实现具体的签名验证逻辑
        // 暂时返回true作为占位符
        Ok(true)
    }
    
    /// 计算交易哈希
    pub fn hash(&self) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        
        let serialized = self.serialize_for_hash();
        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        hasher.finalize().into()
    }
    
    /// 序列化用于哈希计算
    fn serialize_for_hash(&self) -> Vec<u8> {
        let mut data = Vec::new();
        
        // 版本号
        data.extend_from_slice(&self.version.to_be_bytes());
        
        // 输入数量
        data.extend_from_slice(&(self.inputs.len() as u32).to_be_bytes());
        
        // 输入
        for input in &self.inputs {
            data.extend_from_slice(&input.previous_output.tx_hash);
            data.extend_from_slice(&input.previous_output.output_index.to_be_bytes());
            data.extend_from_slice(&(input.script_sig.len() as u32).to_be_bytes());
            data.extend_from_slice(&input.script_sig);
            data.extend_from_slice(&input.sequence.to_be_bytes());
        }
        
        // 输出数量
        data.extend_from_slice(&(self.outputs.len() as u32).to_be_bytes());
        
        // 输出
        for output in &self.outputs {
            data.extend_from_slice(&output.amount.to_be_bytes());
            data.extend_from_slice(&(output.script_pubkey.len() as u32).to_be_bytes());
            data.extend_from_slice(&output.script_pubkey);
        }
        
        // 锁定时间
        data.extend_from_slice(&self.locktime.to_be_bytes());
        
        data
    }
    
    /// 序列化交易
    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| BlockchainError::InvalidTransaction(format!("Serialization failed: {}", e)))
    }
    
    /// 反序列化交易
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data)
            .map_err(|e| BlockchainError::InvalidTransaction(format!("Deserialization failed: {}", e)))
    }
    
    /// 获取交易大小
    pub fn size(&self) -> usize {
        self.serialize().unwrap_or_default().len()
    }
    
    /// 获取输入总金额
    pub fn input_total(&self) -> u64 {
        self.inputs.iter().map(|i| i.amount).sum()
    }
    
    /// 获取输出总金额
    pub fn output_total(&self) -> u64 {
        self.outputs.iter().map(|o| o.amount).sum()
    }
    
    /// 获取手续费
    pub fn fee(&self) -> u64 {
        self.input_total() - self.output_total()
    }
    
    /// 检查是否涉及指定地址
    pub fn involves_address(&self, address: &str) -> bool {
        self.inputs.iter().any(|i| i.address == address) ||
        self.outputs.iter().any(|o| o.address == address)
    }
    
    /// 获取涉及的地址列表
    pub fn get_addresses(&self) -> Vec<String> {
        let mut addresses = Vec::new();
        
        for input in &self.inputs {
            if !addresses.contains(&input.address) {
                addresses.push(input.address.clone());
            }
        }
        
        for output in &self.outputs {
            if !addresses.contains(&output.address) {
                addresses.push(output.address.clone());
            }
        }
        
        addresses
    }
}

impl TxInput {
    /// 创建新的交易输入
    pub fn new(previous_output: OutPoint, amount: u64, address: String) -> Self {
        Self {
            previous_output,
            script_sig: Vec::new(),
            sequence: 0xFFFFFFFF,
            address,
            amount,
            public_key: Vec::new(),
            signature: Vec::new(),
        }
    }
    
    /// 验证输入
    pub fn validate(&self) -> Result<()> {
        if self.amount == 0 {
            return Err(BlockchainError::InvalidTransaction("Zero input amount".to_string()));
        }
        
        if self.address.is_empty() {
            return Err(BlockchainError::InvalidTransaction("Empty input address".to_string()));
        }
        
        Ok(())
    }
}

impl TxOutput {
    /// 创建新的交易输出
    pub fn new(amount: u64, address: String) -> Self {
        Self {
            amount,
            script_pubkey: Vec::new(),
            address,
        }
    }
    
    /// 验证输出
    pub fn validate(&self) -> Result<()> {
        if self.amount == 0 {
            return Err(BlockchainError::InvalidTransaction("Zero output amount".to_string()));
        }
        
        if self.address.is_empty() {
            return Err(BlockchainError::InvalidTransaction("Empty output address".to_string()));
        }
        
        Ok(())
    }
}

impl OutPoint {
    /// 创建新的输出点
    pub fn new(tx_hash: [u8; 32], output_index: u32) -> Self {
        Self {
            tx_hash,
            output_index,
        }
    }
    
    /// 验证输出点
    pub fn validate(&self) -> Result<()> {
        if self.tx_hash == [0u8; 32] {
            return Err(BlockchainError::InvalidTransaction("Invalid transaction hash".to_string()));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transaction_creation() {
        let input = TxInput::new(
            OutPoint::new([1u8; 32], 0),
            1000,
            "address1".to_string(),
        );
        let output = TxOutput::new(900, "address2".to_string());
        
        let tx = Transaction::new(vec![input], vec![output]);
        
        assert_eq!(tx.input_total(), 1000);
        assert_eq!(tx.output_total(), 900);
        assert_eq!(tx.fee(), 100);
    }
    
    #[test]
    fn test_transaction_validation() {
        let input = TxInput::new(
            OutPoint::new([1u8; 32], 0),
            1000,
            "address1".to_string(),
        );
        let output = TxOutput::new(900, "address2".to_string());
        
        let tx = Transaction::new(vec![input], vec![output]);
        assert!(tx.validate().is_ok());
    }
    
    #[test]
    fn test_transaction_hash() {
        let input = TxInput::new(
            OutPoint::new([1u8; 32], 0),
            1000,
            "address1".to_string(),
        );
        let output = TxOutput::new(900, "address2".to_string());
        
        let tx = Transaction::new(vec![input], vec![output]);
        let hash1 = tx.hash();
        let hash2 = tx.hash();
        
        assert_eq!(hash1, hash2);
    }
    
    #[test]
    fn test_transaction_serialization() {
        let input = TxInput::new(
            OutPoint::new([1u8; 32], 0),
            1000,
            "address1".to_string(),
        );
        let output = TxOutput::new(900, "address2".to_string());
        
        let tx = Transaction::new(vec![input], vec![output]);
        let serialized = tx.serialize().unwrap();
        let deserialized = Transaction::deserialize(&serialized).unwrap();
        
        assert_eq!(tx.input_total(), deserialized.input_total());
        assert_eq!(tx.output_total(), deserialized.output_total());
    }
}
