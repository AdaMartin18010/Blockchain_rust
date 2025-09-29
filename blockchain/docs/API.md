# Blockchain Rust Library

**版本**: 0.1.0

A comprehensive blockchain implementation in Rust

## 目录

- [core](#core)
- [network](#network)
- [algorithms](#algorithms)
- [cryptography](#cryptography)
- [consensus](#consensus)
- [smart_contracts](#smart_contracts)
- [storage](#storage)

## core

Core blockchain data structures and types

### 结构体

#### Block

Represents a block in the blockchain

**字段**:

| 名称 | 类型 | 描述 |
|------|------|------|
| header | BlockHeader | Block header containing metadata |
| transactions | Vec<Transaction> | List of transactions in the block |

#### Transaction

Represents a blockchain transaction

**字段**:

| 名称 | 类型 | 描述 |
|------|------|------|
| version | u32 | Transaction version |
| inputs | Vec<TxInput> | Transaction inputs |
| outputs | Vec<TxOutput> | Transaction outputs |

#### MerkleTree

Merkle tree for efficient data verification

**字段**:

| 名称 | 类型 | 描述 |
|------|------|------|
| root | MerkleNode | Root node of the tree |
| leaves | Vec<MerkleNode> | Leaf nodes of the tree |

## network

P2P network communication components

### 结构体

#### P2PNetwork

Peer-to-peer network implementation

### 函数

#### start

Start the P2P network on specified port

**参数**:

| 名称 | 类型 | 描述 | 必需 |
|------|------|------|------|
| port | u16 | Port number to listen on | 是 |

**返回类型**: `Result<()>`

Result of network startup

**注意**: 这是一个异步函数

**示例**:

```rust
network.start(8080).await?;
```

## algorithms

Blockchain-specific algorithms and optimizations

### 结构体

#### ConsensusAlgorithms

Consensus-related algorithms

### 函数

#### calculate_difficulty

Calculate mining difficulty based on network conditions

**参数**:

| 名称 | 类型 | 描述 | 必需 |
|------|------|------|------|
| current_height | u64 | Current blockchain height | 是 |
| target_time | u64 | Target block time in seconds | 是 |
| actual_time | u64 | Actual block time in seconds | 是 |

**返回类型**: `u32`

Calculated difficulty value

**示例**:

```rust
let difficulty = algorithms.calculate_difficulty(100, 600, 550);
```

## cryptography

Cryptographic operations and algorithms

### 结构体

#### HashEngine

Hash function engine supporting multiple algorithms

#### SignatureEngine

Digital signature engine supporting multiple algorithms

### 函数

#### sha256

Compute SHA256 hash of input data

**参数**:

| 名称 | 类型 | 描述 | 必需 |
|------|------|------|------|
| data | &[u8] | Input data to hash | 是 |

**返回类型**: `[u8; 32]`

SHA256 hash as 32-byte array

**示例**:

```rust
let hash = engine.sha256(b"hello world");
```

#### blake2b

Compute Blake2b hash of input data

**参数**:

| 名称 | 类型 | 描述 | 必需 |
|------|------|------|------|
| data | &[u8] | Input data to hash | 是 |

**返回类型**: `[u8; 64]`

Blake2b hash as 64-byte array

**示例**:

```rust
let hash = engine.blake2b(b"hello world");
```

#### sign

Sign data with private key

**参数**:

| 名称 | 类型 | 描述 | 必需 |
|------|------|------|------|
| data | &[u8] | Data to sign | 是 |
| private_key | &[u8] | Private key for signing | 是 |
| algorithm | &str | Signature algorithm (ecdsa, ed25519) | 是 |

**返回类型**: `Result<Vec<u8>>`

Digital signature

**示例**:

```rust
let signature = engine.sign(data, &private_key, "ed25519")?;
```

## consensus

Blockchain consensus algorithms

### 结构体

#### ProofOfWork

Proof of Work consensus algorithm

**字段**:

| 名称 | 类型 | 描述 |
|------|------|------|
| difficulty | u32 | Mining difficulty level |

### 函数

#### mine_block

Mine a block using Proof of Work

**参数**:

| 名称 | 类型 | 描述 | 必需 |
|------|------|------|------|
| block | &mut Block | Block to mine | 是 |

**返回类型**: `Result<()>`

Result of mining operation

**注意**: 这是一个异步函数

**示例**:

```rust
pow.mine_block(&mut block).await?;
```

## smart_contracts

Smart contract execution environment

### 结构体

#### VirtualMachine

Smart contract virtual machine

### 函数

#### execute

Execute smart contract bytecode

**参数**:

| 名称 | 类型 | 描述 | 必需 |
|------|------|------|------|
| bytecode | &[u8] | Contract bytecode to execute | 是 |
| input | &[u8] | Input data for contract execution | 是 |

**返回类型**: `Result<Vec<u8>>`

Contract execution result

**注意**: 这是一个异步函数

**示例**:

```rust
let result = vm.execute(&bytecode, &input).await?;
```

## storage

Blockchain data storage components

### 结构体

#### BlockStorage

Block storage and retrieval system

### 函数

#### store_block

Store a block in the storage system

**参数**:

| 名称 | 类型 | 描述 | 必需 |
|------|------|------|------|
| height | u64 | Block height | 是 |
| block | Block | Block to store | 是 |

**返回类型**: `Result<()>`

Result of storage operation

**注意**: 这是一个异步函数

**示例**:

```rust
storage.store_block(1, block).await?;
```

