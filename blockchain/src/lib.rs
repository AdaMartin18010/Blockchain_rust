//! # 区块链库
//!
//! 展示 Rust 1.90 特性在区块链开发中的应用
//! 基于区块链基本知识架构、组件架构和原理设计重新构建
//! Demonstrates the application of Rust 1.90 features in blockchain development
//! Redesigned based on blockchain fundamental knowledge architecture, component architecture, and principle design

// 核心模块
pub mod core;

// 组件模块
pub mod components;

// 分层架构模块
pub mod layers;

// 算法模块
pub mod algorithms;

// 智能合约模块
pub mod smart_contracts;

// 工具模块
pub mod utils;

// 兼容性模块（保留原有接口）
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
// #[cfg(feature = "advanced")]
// pub mod performance;
#[cfg(feature = "advanced")]
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

