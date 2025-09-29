use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use blockchain::core::{Block, Transaction, MerkleTree, State};
use blockchain::core::transaction::{TxInput, TxOutput, OutPoint};
use blockchain::components::cryptography::{HashEngine, SignatureEngine, EncryptionEngine};
use blockchain::components::storage::{BlockStorage, TransactionStorage, StateStorage};
use std::time::Duration;

fn bench_hash_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_operations");
    group.measurement_time(Duration::from_secs(10));
    
    let hash_engine = HashEngine::new();
    let data = vec![1u8; 1024]; // 1KB data
    
    group.bench_function("sha256_1kb", |b| {
        b.iter(|| {
            black_box(hash_engine.sha256(&data))
        })
    });
    
    group.bench_function("blake2b_1kb", |b| {
        b.iter(|| {
            black_box(hash_engine.blake2b(&data))
        })
    });
    
    group.bench_function("double_sha256_1kb", |b| {
        b.iter(|| {
            black_box(hash_engine.double_sha256(&data))
        })
    });
    
    group.finish();
}

fn bench_signature_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("signature_operations");
    group.measurement_time(Duration::from_secs(10));
    
    let signature_engine = SignatureEngine::new();
    let data = vec![1u8; 256]; // 256 bytes data
    
    group.bench_function("ecdsa_key_generation", |b| {
        b.iter(|| {
            black_box(signature_engine.generate_keypair("ecdsa").unwrap())
        })
    });
    
    group.bench_function("ed25519_key_generation", |b| {
        b.iter(|| {
            black_box(signature_engine.generate_keypair("ed25519").unwrap())
        })
    });
    
    // Generate keys once for signing/verification benchmarks
    let (ecdsa_private, ecdsa_public) = signature_engine.generate_keypair("ecdsa").unwrap();
    let (ed25519_private, ed25519_public) = signature_engine.generate_keypair("ed25519").unwrap();
    
    group.bench_function("ecdsa_sign", |b| {
        b.iter(|| {
            black_box(signature_engine.sign(&data, &ecdsa_private, "ecdsa").unwrap())
        })
    });
    
    group.bench_function("ed25519_sign", |b| {
        b.iter(|| {
            black_box(signature_engine.sign(&data, &ed25519_private, "ed25519").unwrap())
        })
    });
    
    let ecdsa_signature = signature_engine.sign(&data, &ecdsa_private, "ecdsa").unwrap();
    let ed25519_signature = signature_engine.sign(&data, &ed25519_private, "ed25519").unwrap();
    
    group.bench_function("ecdsa_verify", |b| {
        b.iter(|| {
            black_box(signature_engine.verify(&data, &ecdsa_signature, &ecdsa_public, "ecdsa").unwrap())
        })
    });
    
    group.bench_function("ed25519_verify", |b| {
        b.iter(|| {
            black_box(signature_engine.verify(&data, &ed25519_signature, &ed25519_public, "ed25519").unwrap())
        })
    });
    
    group.finish();
}

fn bench_merkle_tree_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("merkle_tree_operations");
    group.measurement_time(Duration::from_secs(10));
    
    // Create test data
    let mut test_data = Vec::new();
    for i in 0..1000 {
        test_data.push([i as u8; 32]);
    }
    
    group.bench_function("merkle_tree_creation_1000_leaves", |b| {
        b.iter(|| {
            black_box(MerkleTree::new(test_data.clone()).unwrap())
        })
    });
    
    let tree = MerkleTree::new(test_data).unwrap();
    
    group.bench_function("merkle_proof_generation", |b| {
        b.iter(|| {
            black_box(tree.generate_proof(0).unwrap())
        })
    });
    
    let proof = tree.generate_proof(0).unwrap();
    
    group.bench_function("merkle_proof_verification", |b| {
        b.iter(|| {
            black_box(MerkleTree::verify_proof(&proof))
        })
    });
    
    group.finish();
}

fn bench_storage_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage_operations");
    group.measurement_time(Duration::from_secs(10));
    
    // Benchmark block storage
    group.bench_function("block_storage_store", |b| {
        let mut storage = BlockStorage::new();
        let block = Block::new([0u8; 32], vec![], 0, 1).unwrap();
        
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                black_box(storage.store_block(block.clone()).await.unwrap())
            })
        })
    });
    
    // Benchmark transaction storage
    group.bench_function("transaction_storage_store", |b| {
        let mut storage = TransactionStorage::new();
        let input = TxInput::new(
            OutPoint::new([1u8; 32], 0),
            1000,
            "test_address".to_string(),
        );
        let output = TxOutput::new(900, "output_address".to_string());
        let transaction = Transaction::new(vec![input], vec![output]);
        
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                black_box(storage.store_transaction(transaction.clone()).await.unwrap())
            })
        })
    });
    
    // Benchmark state storage
    group.bench_function("state_storage_store", |b| {
        let mut storage = StateStorage::new();
        let state = State::new();
        
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                black_box(storage.store_state(1, state.clone()).await.unwrap())
            })
        })
    });
    
    group.finish();
}

fn bench_encryption_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("encryption_operations");
    group.measurement_time(Duration::from_secs(10));
    
    let encryption_engine = EncryptionEngine::new();
    let data = vec![1u8; 1024]; // 1KB data
    let key = encryption_engine.generate_key("aes-gcm").unwrap();
    
    group.bench_function("aes_gcm_encrypt_1kb", |b| {
        b.iter(|| {
            black_box(encryption_engine.encrypt(&data, &key, "aes-gcm").unwrap())
        })
    });
    
    let encrypted = encryption_engine.encrypt(&data, &key, "aes-gcm").unwrap();
    
    group.bench_function("aes_gcm_decrypt_1kb", |b| {
        b.iter(|| {
            black_box(encryption_engine.decrypt(&encrypted, &key, "aes-gcm").unwrap())
        })
    });
    
    let chacha20_key = encryption_engine.generate_key("chacha20-poly1305").unwrap();
    
    group.bench_function("chacha20_poly1305_encrypt_1kb", |b| {
        b.iter(|| {
            black_box(encryption_engine.encrypt(&data, &chacha20_key, "chacha20-poly1305").unwrap())
        })
    });
    
    let chacha20_encrypted = encryption_engine.encrypt(&data, &chacha20_key, "chacha20-poly1305").unwrap();
    
    group.bench_function("chacha20_poly1305_decrypt_1kb", |b| {
        b.iter(|| {
            black_box(encryption_engine.decrypt(&chacha20_encrypted, &chacha20_key, "chacha20-poly1305").unwrap())
        })
    });
    
    group.finish();
}

fn bench_block_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("block_operations");
    group.measurement_time(Duration::from_secs(10));
    
    // Create test transactions
    let mut transactions = Vec::new();
    for i in 0..100 {
        let input = TxInput::new(
            OutPoint::new([i as u8; 32], 0),
            1000,
            format!("address_{}", i),
        );
        let output = TxOutput::new(900, format!("output_address_{}", i));
        transactions.push(Transaction::new(vec![input], vec![output]));
    }
    
    group.bench_function("block_creation_100_txs", |b| {
        b.iter(|| {
            black_box(Block::new([0u8; 32], transactions.clone(), 1, 1).unwrap())
        })
    });
    
    let block = Block::new([0u8; 32], transactions, 1, 1).unwrap();
    
    group.bench_function("block_validation", |b| {
        b.iter(|| {
            black_box(block.validate().unwrap())
        })
    });
    
    group.bench_function("block_serialization", |b| {
        b.iter(|| {
            black_box(block.serialize().unwrap())
        })
    });
    
    let serialized = block.serialize().unwrap();
    
    group.bench_function("block_deserialization", |b| {
        b.iter(|| {
            black_box(Block::deserialize(&serialized).unwrap())
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_hash_operations,
    bench_signature_operations,
    bench_merkle_tree_operations,
    bench_storage_operations,
    bench_encryption_operations,
    bench_block_operations
);
criterion_main!(benches);
