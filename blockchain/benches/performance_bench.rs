//! # 区块链性能基准测试
//! 
//! 测试区块链各项操作的性能

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use blockchain::*;

fn benchmark_blockchain_creation(c: &mut Criterion) {
    c.bench_function("blockchain_creation", |b| {
        b.iter(|| {
            let blockchain = Blockchain::new(black_box(2));
            black_box(blockchain)
        })
    });
}

fn benchmark_transaction_creation(c: &mut Criterion) {
    c.bench_function("transaction_creation", |b| {
        b.iter(|| {
            let tx = Transaction::new(
                black_box("alice".to_string()),
                black_box("bob".to_string()),
                black_box(100),
            );
            black_box(tx)
        })
    });
}

fn benchmark_hash_calculation(c: &mut Criterion) {
    c.bench_function("hash_calculation", |b| {
        b.iter(|| {
            let data = black_box(b"test data for hashing");
            let hash = BlockHash::<32>::from_data(data);
            black_box(hash)
        })
    });
}

fn benchmark_block_mining(c: &mut Criterion) {
    c.bench_function("block_mining", |b| {
        let mut blockchain = Blockchain::new(1); // 低难度用于基准测试
        let tx = Transaction::new("alice".to_string(), "bob".to_string(), 100);
        blockchain.add_transaction(tx).unwrap();
        
        b.iter(|| {
            let mut test_blockchain = blockchain.clone();
            test_blockchain.mine_pending_transactions(black_box("miner".to_string()))
                .unwrap();
            black_box(test_blockchain)
        })
    });
}

fn benchmark_chain_validation(c: &mut Criterion) {
    c.bench_function("chain_validation", |b| {
        let mut blockchain = Blockchain::new(1);
        for i in 0..10 {
            let tx = Transaction::new(
                format!("sender_{}", i),
                format!("receiver_{}", i),
                100,
            );
            blockchain.add_transaction(tx).unwrap();
            blockchain.mine_pending_transactions("miner".to_string()).unwrap();
        }
        
        b.iter(|| {
            black_box(blockchain.is_valid_chain())
        })
    });
}

fn benchmark_transaction_processing(c: &mut Criterion) {
    c.bench_function("transaction_processing", |b| {
        let mut blockchain = Blockchain::new(1);
        
        b.iter(|| {
            let tx = Transaction::new(
                black_box("alice".to_string()),
                black_box("bob".to_string()),
                black_box(100),
            );
            let result = blockchain.add_transaction(tx);
            black_box(result)
        })
    });
}

criterion_group!(
    benches,
    benchmark_blockchain_creation,
    benchmark_transaction_creation,
    benchmark_hash_calculation,
    benchmark_block_mining,
    benchmark_chain_validation,
    benchmark_transaction_processing
);

criterion_main!(benches);
