//! 编码工具实现

use super::{UtilsResult, UtilsError};
// use crate::core::{Result, BlockchainError};

/// 编码工具
#[derive(Debug)]
pub struct EncodingUtils {
    // 编码工具相关状态
}

impl EncodingUtils {
    pub fn new() -> Self {
        Self {}
    }

    pub fn encode_hex(&self, data: &[u8]) -> String {
        hex::encode(data)
    }

    pub fn decode_hex(&self, hex_str: &str) -> UtilsResult<Vec<u8>> {
        hex::decode(hex_str)
            .map_err(|e| UtilsError::EncodingError(e.to_string()).into())
    }
}
