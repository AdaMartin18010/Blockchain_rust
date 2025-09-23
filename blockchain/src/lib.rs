//! # 区块链库
//!
//! 展示 Rust 1.89 特性在区块链开发中的应用
//! Demonstrates the application of Rust 1.89 features in blockchain development

pub mod cryptography;
pub mod network;
pub mod simple_blockchain;
pub mod smart_contract;
pub mod tools;
pub mod types;
pub mod monitoring;
pub mod cli;
pub mod consensus;
pub mod web_api;
pub mod performance;
pub mod security;

// 新增的高级模块
#[cfg(feature = "crypto-advanced")]
pub mod advanced_cryptography_simple;

#[cfg(feature = "smart-contracts")]
pub mod smart_contract_engine;

#[cfg(feature = "p2p")]
pub mod p2p_network;

#[cfg(feature = "database")]
pub mod database;

// 重新导出常用的类型和函数
pub use simple_blockchain::{
    Blockchain, Block, Transaction, BlockHash, ValidationResult,
};

