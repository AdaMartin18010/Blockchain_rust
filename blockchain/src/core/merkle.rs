// Merkle树实现
use crate::core::{Result, BlockchainError};
use serde::{Serialize, Deserialize};

/// Merkle树
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleTree {
    /// 根节点
    pub root: MerkleNode,
    
    /// 叶子节点
    pub leaves: Vec<MerkleNode>,
    
    /// 树的高度
    pub height: usize,
}

/// Merkle节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleNode {
    /// 节点哈希
    pub hash: [u8; 32],
    
    /// 左子节点
    pub left: Option<Box<MerkleNode>>,
    
    /// 右子节点
    pub right: Option<Box<MerkleNode>>,
    
    /// 是否为叶子节点
    pub is_leaf: bool,
    
    /// 叶子节点的索引
    pub leaf_index: Option<usize>,
}

/// Merkle证明
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    /// 证明路径
    pub path: Vec<MerkleProofNode>,
    
    /// 叶子节点的哈希
    pub leaf_hash: [u8; 32],
    
    /// 根哈希
    pub root_hash: [u8; 32],
}

/// Merkle证明节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProofNode {
    /// 节点哈希
    pub hash: [u8; 32],
    
    /// 是否为左节点
    pub is_left: bool,
}

impl MerkleTree {
    /// 创建新的Merkle树
    pub fn new(data: Vec<[u8; 32]>) -> Result<Self> {
        if data.is_empty() {
            return Err(BlockchainError::InvalidState("Empty data for Merkle tree".to_string()));
        }
        
        let mut leaves = Vec::new();
        
        // 创建叶子节点
        for (index, hash) in data.iter().enumerate() {
            leaves.push(MerkleNode {
                hash: *hash,
                left: None,
                right: None,
                is_leaf: true,
                leaf_index: Some(index),
            });
        }
        
        // 如果叶子节点数量为奇数，复制最后一个节点
        if leaves.len() % 2 == 1 {
            let last_leaf = leaves.last().unwrap().clone();
            leaves.push(last_leaf);
        }
        
        // 构建树
        let root = Self::build_tree(&mut leaves)?;
        let height = Self::calculate_height(leaves.len());
        
        Ok(MerkleTree {
            root,
            leaves,
            height,
        })
    }
    
    /// 构建Merkle树
    fn build_tree(leaves: &mut Vec<MerkleNode>) -> Result<MerkleNode> {
        if leaves.is_empty() {
            return Err(BlockchainError::InvalidState("Empty leaves".to_string()));
        }
        
        if leaves.len() == 1 {
            return Ok(leaves[0].clone());
        }
        
        let mut current_level = leaves.clone();
        
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for i in (0..current_level.len()).step_by(2) {
                let left = current_level[i].clone();
                let right = if i + 1 < current_level.len() {
                    current_level[i + 1].clone()
                } else {
                    left.clone()
                };
                
                let parent_hash = Self::hash_pair(left.hash, right.hash);
                
                next_level.push(MerkleNode {
                    hash: parent_hash,
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                    is_leaf: false,
                    leaf_index: None,
                });
            }
            
            current_level = next_level;
        }
        
        Ok(current_level[0].clone())
    }
    
    /// 计算树的高度
    fn calculate_height(leaf_count: usize) -> usize {
        if leaf_count <= 1 {
            return 1;
        }
        
        (leaf_count as f64).log2().ceil() as usize
    }
    
    /// 获取根哈希
    pub fn root(&self) -> [u8; 32] {
        self.root.hash
    }
    
    /// 生成Merkle证明
    pub fn generate_proof(&self, leaf_index: usize) -> Result<MerkleProof> {
        if leaf_index >= self.leaves.len() {
            return Err(BlockchainError::InvalidState("Invalid leaf index".to_string()));
        }
        
        let leaf_hash = self.leaves[leaf_index].hash;
        let mut path = Vec::new();
        
        // 从叶子节点开始向上遍历
        let mut current_index = leaf_index;
        let mut current_level = self.leaves.clone();
        
        while current_level.len() > 1 {
            let is_left = current_index % 2 == 0;
            let sibling_index = if is_left {
                current_index + 1
            } else {
                current_index - 1
            };
            
            if sibling_index < current_level.len() {
                path.push(MerkleProofNode {
                    hash: current_level[sibling_index].hash,
                    is_left: !is_left,
                });
            }
            
            // 移动到上一层
            current_index /= 2;
            current_level = Self::build_next_level(&current_level)?;
        }
        
        Ok(MerkleProof {
            path,
            leaf_hash,
            root_hash: self.root.hash,
        })
    }
    
    /// 构建下一层节点
    fn build_next_level(level: &[MerkleNode]) -> Result<Vec<MerkleNode>> {
        let mut next_level = Vec::new();
        
        for i in (0..level.len()).step_by(2) {
            let left = &level[i];
            let right = if i + 1 < level.len() {
                &level[i + 1]
            } else {
                left
            };
            
            let parent_hash = Self::hash_pair(left.hash, right.hash);
            
            next_level.push(MerkleNode {
                hash: parent_hash,
                left: Some(Box::new(left.clone())),
                right: Some(Box::new(right.clone())),
                is_leaf: false,
                leaf_index: None,
            });
        }
        
        Ok(next_level)
    }
    
    /// 验证Merkle证明
    pub fn verify_proof(proof: &MerkleProof) -> bool {
        let mut current_hash = proof.leaf_hash;
        
        for proof_node in &proof.path {
            if proof_node.is_left {
                current_hash = Self::hash_pair(proof_node.hash, current_hash);
            } else {
                current_hash = Self::hash_pair(current_hash, proof_node.hash);
            }
        }
        
        current_hash == proof.root_hash
    }
    
    /// 哈希两个值
    fn hash_pair(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(&left);
        hasher.update(&right);
        hasher.finalize().into()
    }
    
    /// 获取叶子节点数量
    pub fn leaf_count(&self) -> usize {
        self.leaves.len()
    }
    
    /// 获取树的高度
    pub fn height(&self) -> usize {
        self.height
    }
    
    /// 序列化Merkle树
    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| BlockchainError::InvalidState(format!("Merkle tree serialization failed: {}", e)))
    }
    
    /// 反序列化Merkle树
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data)
            .map_err(|e| BlockchainError::InvalidState(format!("Merkle tree deserialization failed: {}", e)))
    }
}

impl MerkleProof {
    /// 验证证明
    pub fn verify(&self) -> bool {
        MerkleTree::verify_proof(self)
    }
    
    /// 序列化证明
    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| BlockchainError::InvalidState(format!("Merkle proof serialization failed: {}", e)))
    }
    
    /// 反序列化证明
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data)
            .map_err(|e| BlockchainError::InvalidState(format!("Merkle proof deserialization failed: {}", e)))
    }
}

impl MerkleNode {
    /// 创建叶子节点
    pub fn new_leaf(hash: [u8; 32], index: usize) -> Self {
        Self {
            hash,
            left: None,
            right: None,
            is_leaf: true,
            leaf_index: Some(index),
        }
    }
    
    /// 创建内部节点
    pub fn new_internal(hash: [u8; 32], left: MerkleNode, right: MerkleNode) -> Self {
        Self {
            hash,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            is_leaf: false,
            leaf_index: None,
        }
    }
    
    /// 获取节点哈希
    pub fn hash(&self) -> [u8; 32] {
        self.hash
    }
    
    /// 是否为叶子节点
    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }
    
    /// 获取叶子节点索引
    pub fn leaf_index(&self) -> Option<usize> {
        self.leaf_index
    }
    
    /// 获取左子节点
    pub fn left(&self) -> Option<&MerkleNode> {
        self.left.as_ref().map(|n| n.as_ref())
    }
    
    /// 获取右子节点
    pub fn right(&self) -> Option<&MerkleNode> {
        self.right.as_ref().map(|n| n.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_merkle_tree_creation() {
        let data = vec![
            [1u8; 32],
            [2u8; 32],
            [3u8; 32],
            [4u8; 32],
        ];
        
        let tree = MerkleTree::new(data).unwrap();
        assert_eq!(tree.leaf_count(), 4);
        assert_eq!(tree.height(), 2);
    }
    
    #[test]
    fn test_merkle_tree_odd_leaves() {
        let data = vec![
            [1u8; 32],
            [2u8; 32],
            [3u8; 32],
        ];
        
        let tree = MerkleTree::new(data).unwrap();
        assert_eq!(tree.leaf_count(), 4); // 应该复制最后一个节点
    }
    
    #[test]
    fn test_merkle_proof_generation() {
        let data = vec![
            [1u8; 32],
            [2u8; 32],
            [3u8; 32],
            [4u8; 32],
        ];
        
        let tree = MerkleTree::new(data).unwrap();
        let proof = tree.generate_proof(0).unwrap();
        
        assert_eq!(proof.leaf_hash, [1u8; 32]);
        assert_eq!(proof.root_hash, tree.root());
    }
    
    #[test]
    fn test_merkle_proof_verification() {
        let data = vec![
            [1u8; 32],
            [2u8; 32],
            [3u8; 32],
            [4u8; 32],
        ];
        
        let tree = MerkleTree::new(data).unwrap();
        let proof = tree.generate_proof(0).unwrap();
        
        assert!(proof.verify());
        assert!(MerkleTree::verify_proof(&proof));
    }
    
    #[test]
    fn test_merkle_tree_serialization() {
        let data = vec![
            [1u8; 32],
            [2u8; 32],
            [3u8; 32],
            [4u8; 32],
        ];
        
        let tree = MerkleTree::new(data).unwrap();
        let serialized = tree.serialize().unwrap();
        let deserialized = MerkleTree::deserialize(&serialized).unwrap();
        
        assert_eq!(tree.root(), deserialized.root());
        assert_eq!(tree.leaf_count(), deserialized.leaf_count());
    }
    
    #[test]
    fn test_merkle_proof_serialization() {
        let data = vec![
            [1u8; 32],
            [2u8; 32],
            [3u8; 32],
            [4u8; 32],
        ];
        
        let tree = MerkleTree::new(data).unwrap();
        let proof = tree.generate_proof(0).unwrap();
        let serialized = proof.serialize().unwrap();
        let deserialized = MerkleProof::deserialize(&serialized).unwrap();
        
        assert_eq!(proof.leaf_hash, deserialized.leaf_hash);
        assert_eq!(proof.root_hash, deserialized.root_hash);
        assert_eq!(proof.path.len(), deserialized.path.len());
    }
}
