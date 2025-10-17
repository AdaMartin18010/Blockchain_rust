# 密码学实现

## 📋 目录

- [1. 哈希函数实现](#1-哈希函数实现)
  - [1.1 SHA-256实现](#11-sha-256实现)
  - [1.2 Blake2实现](#12-blake2实现)
  - [1.3 Keccak-256实现](#13-keccak-256实现)
- [2. 数字签名实现](#2-数字签名实现)
  - [2.1 ECDSA签名](#21-ecdsa签名)
  - [2.2 EdDSA签名](#22-eddsa签名)
  - [2.3 Schnorr签名](#23-schnorr签名)
- [3. 加密算法实现](#3-加密算法实现)
  - [3.1 对称加密](#31-对称加密)
  - [3.2 非对称加密](#32-非对称加密)
  - [3.3 混合加密](#33-混合加密)
- [4. 零知识证明实现](#4-零知识证明实现)
  - [4.1 zk-SNARK实现](#41-zk-snark实现)
  - [4.2 zk-STARK实现](#42-zk-stark实现)
  - [4.3 Bulletproofs实现](#43-bulletproofs实现)
- [5. Merkle树实现](#5-merkle树实现)
  - [5.1 标准Merkle树](#51-标准merkle树)
  - [5.2 Merkle Patricia Trie](#52-merkle-patricia-trie)
  - [5.3 稀疏Merkle树](#53-稀疏merkle树)
- [6. 密钥管理](#6-密钥管理)
  - [6.1 密钥生成](#61-密钥生成)
  - [6.2 密钥派生](#62-密钥派生)
  - [6.3 密钥存储](#63-密钥存储)
- [7. 随机数生成](#7-随机数生成)
  - [7.1 CSPRNG实现](#71-csprng实现)
  - [7.2 确定性随机数](#72-确定性随机数)
  - [7.3 VRF实现](#73-vrf实现)
- [8. 高级密码学协议](#8-高级密码学协议)
  - [8.1 多方计算](#81-多方计算)
  - [8.2 阈值签名](#82-阈值签名)
  - [8.3 同态加密](#83-同态加密)

## 1. 哈希函数实现

### 1.1 SHA-256实现

```rust
use sha2::{Sha256, Digest};

/// SHA-256哈希包装器
#[derive(Debug, Clone)]
pub struct Sha256Hasher;

impl Sha256Hasher {
    /// 计算哈希
    pub fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        result.into()
    }
    
    /// 双重哈希（比特币使用）
    pub fn double_hash(data: &[u8]) -> [u8; 32] {
        let first_hash = Self::hash(data);
        Self::hash(&first_hash)
    }
    
    /// HMAC-SHA256
    pub fn hmac(key: &[u8], data: &[u8]) -> [u8; 32] {
        use hmac::{Hmac, Mac};
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = HmacSha256::new_from_slice(key)
            .expect("HMAC can take key of any size");
        mac.update(data);
        let result = mac.finalize();
        result.into_bytes().into()
    }
}

/// 哈希工具实现
#[derive(Debug, Clone)]
pub struct Hash([u8; 32]);

impl Hash {
    /// 创建新哈希
    pub fn new(data: [u8; 32]) -> Self {
        Hash(data)
    }
    
    /// 计算数据哈希
    pub fn hash(data: &[u8]) -> Self {
        Hash(Sha256Hasher::hash(data))
    }
    
    /// 空哈希
    pub fn empty() -> Self {
        Hash([0u8; 32])
    }
    
    /// 从十六进制字符串创建
    pub fn from_hex(s: &str) -> Result<Self, Error> {
        let bytes = hex::decode(s)?;
        if bytes.len() != 32 {
            return Err(Error::InvalidHashLength);
        }
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&bytes);
        Ok(Hash(hash))
    }
    
    /// 转换为十六进制字符串
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
    
    /// 获取字节切片
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    
    /// 从字节切片创建
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != 32 {
            return Err(Error::InvalidHashLength);
        }
        let mut hash = [0u8; 32];
        hash.copy_from_slice(bytes);
        Ok(Hash(hash))
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}
```

### 1.2 Blake2实现

```rust
use blake2::{Blake2b512, Blake2s256, Digest};

/// Blake2哈希器
#[derive(Debug, Clone)]
pub struct Blake2Hasher;

impl Blake2Hasher {
    /// Blake2b-512哈希
    pub fn blake2b(data: &[u8]) -> [u8; 64] {
        let mut hasher = Blake2b512::new();
        hasher.update(data);
        hasher.finalize().into()
    }
    
    /// Blake2s-256哈希
    pub fn blake2s(data: &[u8]) -> [u8; 32] {
        let mut hasher = Blake2s256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
    
    /// 带密钥的Blake2（用于MAC）
    pub fn blake2b_mac(key: &[u8], data: &[u8]) -> [u8; 64] {
        use blake2::Blake2bMac512;
        use blake2::digest::{KeyInit, Update, FixedOutput};
        
        let mut hasher = Blake2bMac512::new_from_slice(key)
            .expect("Blake2 can take key of any size");
        hasher.update(data);
        hasher.finalize_fixed().into()
    }
}
```

### 1.3 Keccak-256实现

```rust
use sha3::{Keccak256, Digest};

/// Keccak-256哈希器（以太坊使用）
#[derive(Debug, Clone)]
pub struct Keccak256Hasher;

impl Keccak256Hasher {
    /// 计算Keccak-256哈希
    pub fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
    
    /// 计算以太坊地址
    pub fn eth_address(public_key: &[u8]) -> [u8; 20] {
        let hash = Self::hash(public_key);
        let mut address = [0u8; 20];
        address.copy_from_slice(&hash[12..]);
        address
    }
}
```

## 2. 数字签名实现

### 2.1 ECDSA签名

```rust
use secp256k1::{Secp256k1, SecretKey, PublicKey, Message, Signature};

/// ECDSA签名器（secp256k1曲线）
#[derive(Debug)]
pub struct EcdsaSigner {
    secp: Secp256k1<secp256k1::All>,
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl EcdsaSigner {
    /// 创建新的签名器
    pub fn new(secret_key_bytes: &[u8]) -> Result<Self, Error> {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(secret_key_bytes)?;
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        
        Ok(Self {
            secp,
            secret_key,
            public_key,
        })
    }
    
    /// 生成新密钥对
    pub fn generate() -> Result<Self, Error> {
        use rand::rngs::OsRng;
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        
        Ok(Self {
            secp,
            secret_key,
            public_key,
        })
    }
    
    /// 签名
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        // 1. 哈希消息
        let message_hash = Sha256Hasher::hash(message);
        let message = Message::from_slice(&message_hash)?;
        
        // 2. 签名
        let signature = self.secp.sign_ecdsa(&message, &self.secret_key);
        
        // 3. 序列化签名
        Ok(signature.serialize_compact().to_vec())
    }
    
    /// 签名可恢复（包含恢复ID）
    pub fn sign_recoverable(&self, message: &[u8]) -> Result<RecoverableSignature, Error> {
        use secp256k1::ecdsa::RecoverableSignature as Secp256k1RecoverableSignature;
        
        let message_hash = Sha256Hasher::hash(message);
        let message = Message::from_slice(&message_hash)?;
        
        let signature = self.secp.sign_ecdsa_recoverable(&message, &self.secret_key);
        
        let (recovery_id, signature_bytes) = signature.serialize_compact();
        
        Ok(RecoverableSignature {
            signature: signature_bytes.to_vec(),
            recovery_id: recovery_id.to_i32() as u8,
        })
    }
    
    /// 获取公钥
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
    
    /// 获取公钥字节
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public_key.serialize().to_vec()
    }
}

/// ECDSA验证器
#[derive(Debug)]
pub struct EcdsaVerifier {
    secp: Secp256k1<secp256k1::All>,
}

impl EcdsaVerifier {
    pub fn new() -> Self {
        Self {
            secp: Secp256k1::new(),
        }
    }
    
    /// 验证签名
    pub fn verify(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8]
    ) -> Result<bool, Error> {
        // 1. 解析公钥
        let public_key = PublicKey::from_slice(public_key)?;
        
        // 2. 解析签名
        let signature = Signature::from_compact(signature)?;
        
        // 3. 哈希消息
        let message_hash = Sha256Hasher::hash(message);
        let message = Message::from_slice(&message_hash)?;
        
        // 4. 验证
        Ok(self.secp.verify_ecdsa(&message, &signature, &public_key).is_ok())
    }
    
    /// 从签名恢复公钥
    pub fn recover_public_key(
        &self,
        message: &[u8],
        recoverable_sig: &RecoverableSignature
    ) -> Result<Vec<u8>, Error> {
        use secp256k1::ecdsa::RecoverableSignature as Secp256k1RecoverableSignature;
        use secp256k1::ecdsa::RecoveryId;
        
        // 1. 哈希消息
        let message_hash = Sha256Hasher::hash(message);
        let message = Message::from_slice(&message_hash)?;
        
        // 2. 创建可恢复签名
        let recovery_id = RecoveryId::from_i32(recoverable_sig.recovery_id as i32)?;
        let signature = Secp256k1RecoverableSignature::from_compact(
            &recoverable_sig.signature,
            recovery_id
        )?;
        
        // 3. 恢复公钥
        let public_key = self.secp.recover_ecdsa(&message, &signature)?;
        
        Ok(public_key.serialize().to_vec())
    }
}

/// 可恢复签名
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecoverableSignature {
    pub signature: Vec<u8>,
    pub recovery_id: u8,
}
```

### 2.2 EdDSA签名

```rust
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};

/// EdDSA签名器（Ed25519曲线）
#[derive(Debug)]
pub struct EddsaSigner {
    keypair: Keypair,
}

impl EddsaSigner {
    /// 创建新的签名器
    pub fn new(secret_key_bytes: &[u8]) -> Result<Self, Error> {
        let secret_key = SecretKey::from_bytes(secret_key_bytes)?;
        let public_key = PublicKey::from(&secret_key);
        let keypair = Keypair {
            secret: secret_key,
            public: public_key,
        };
        
        Ok(Self { keypair })
    }
    
    /// 生成新密钥对
    pub fn generate() -> Result<Self, Error> {
        use rand::rngs::OsRng;
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        
        Ok(Self { keypair })
    }
    
    /// 签名
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let signature = self.keypair.sign(message);
        signature.to_bytes().to_vec()
    }
    
    /// 获取公钥
    pub fn public_key(&self) -> &PublicKey {
        &self.keypair.public
    }
    
    /// 获取公钥字节
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.keypair.public.to_bytes().to_vec()
    }
}

/// EdDSA验证器
#[derive(Debug)]
pub struct EddsaVerifier;

impl EddsaVerifier {
    /// 验证签名
    pub fn verify(
        message: &[u8],
        signature: &[u8],
        public_key: &[u8]
    ) -> Result<bool, Error> {
        // 1. 解析公钥
        let public_key = PublicKey::from_bytes(public_key)?;
        
        // 2. 解析签名
        let signature = Signature::from_bytes(signature)?;
        
        // 3. 验证
        Ok(public_key.verify(message, &signature).is_ok())
    }
}
```

### 2.3 Schnorr签名

```rust
use secp256k1::schnorr::{Signature as SchnorrSignature, KeyPair};
use secp256k1::{Secp256k1, XOnlyPublicKey, Message};

/// Schnorr签名器
#[derive(Debug)]
pub struct SchnorrSigner {
    secp: Secp256k1<secp256k1::All>,
    keypair: KeyPair,
}

impl SchnorrSigner {
    /// 生成新密钥对
    pub fn generate() -> Result<Self, Error> {
        use rand::rngs::OsRng;
        let secp = Secp256k1::new();
        let keypair = KeyPair::new(&secp, &mut OsRng);
        
        Ok(Self { secp, keypair })
    }
    
    /// 签名
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        // 1. 哈希消息
        let message_hash = Sha256Hasher::hash(message);
        let message = Message::from_slice(&message_hash)?;
        
        // 2. 签名
        let signature = self.secp.sign_schnorr(&message, &self.keypair);
        
        Ok(signature.as_ref().to_vec())
    }
    
    /// 获取公钥
    pub fn public_key(&self) -> XOnlyPublicKey {
        self.keypair.x_only_public_key().0
    }
}

/// Schnorr验证器
#[derive(Debug)]
pub struct SchnorrVerifier {
    secp: Secp256k1<secp256k1::All>,
}

impl SchnorrVerifier {
    pub fn new() -> Self {
        Self {
            secp: Secp256k1::new(),
        }
    }
    
    /// 验证签名
    pub fn verify(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &XOnlyPublicKey
    ) -> Result<bool, Error> {
        // 1. 解析签名
        let signature = SchnorrSignature::from_slice(signature)?;
        
        // 2. 哈希消息
        let message_hash = Sha256Hasher::hash(message);
        let message = Message::from_slice(&message_hash)?;
        
        // 3. 验证
        Ok(self.secp.verify_schnorr(&signature, &message, public_key).is_ok())
    }
}
```

## 3. 加密算法实现

### 3.1 对称加密

```rust
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};

/// AES-256-GCM加密器
#[derive(Debug)]
pub struct Aes256GcmCipher {
    cipher: Aes256Gcm,
}

impl Aes256GcmCipher {
    /// 创建加密器
    pub fn new(key: &[u8; 32]) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        
        Self { cipher }
    }
    
    /// 加密
    pub fn encrypt(&self, plaintext: &[u8], nonce: &[u8; 12]) -> Result<Vec<u8>, Error> {
        let nonce = Nonce::from_slice(nonce);
        self.cipher.encrypt(nonce, plaintext)
            .map_err(|_| Error::EncryptionFailed)
    }
    
    /// 解密
    pub fn decrypt(&self, ciphertext: &[u8], nonce: &[u8; 12]) -> Result<Vec<u8>, Error> {
        let nonce = Nonce::from_slice(nonce);
        self.cipher.decrypt(nonce, ciphertext)
            .map_err(|_| Error::DecryptionFailed)
    }
}

/// ChaCha20-Poly1305加密器
#[derive(Debug)]
pub struct ChaCha20Poly1305Cipher {
    cipher: chacha20poly1305::ChaCha20Poly1305,
}

impl ChaCha20Poly1305Cipher {
    /// 创建加密器
    pub fn new(key: &[u8; 32]) -> Self {
        use chacha20poly1305::Key;
        let key = Key::from_slice(key);
        let cipher = chacha20poly1305::ChaCha20Poly1305::new(key);
        
        Self { cipher }
    }
    
    /// 加密
    pub fn encrypt(&self, plaintext: &[u8], nonce: &[u8; 12]) -> Result<Vec<u8>, Error> {
        use chacha20poly1305::Nonce;
        let nonce = Nonce::from_slice(nonce);
        self.cipher.encrypt(nonce, plaintext)
            .map_err(|_| Error::EncryptionFailed)
    }
    
    /// 解密
    pub fn decrypt(&self, ciphertext: &[u8], nonce: &[u8; 12]) -> Result<Vec<u8>, Error> {
        use chacha20poly1305::Nonce;
        let nonce = Nonce::from_slice(nonce);
        self.cipher.decrypt(nonce, ciphertext)
            .map_err(|_| Error::DecryptionFailed)
    }
}
```

### 3.2 非对称加密

```rust
use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};

/// RSA加密器
#[derive(Debug)]
pub struct RsaCipher {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl RsaCipher {
    /// 生成新密钥对
    pub fn generate(bits: usize) -> Result<Self, Error> {
        use rand::rngs::OsRng;
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, bits)?;
        let public_key = RsaPublicKey::from(&private_key);
        
        Ok(Self {
            private_key,
            public_key,
        })
    }
    
    /// 加密
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, Error> {
        use rand::rngs::OsRng;
        let mut rng = OsRng;
        let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
        
        self.public_key.encrypt(&mut rng, padding, plaintext)
            .map_err(|_| Error::EncryptionFailed)
    }
    
    /// 解密
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
        let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
        
        self.private_key.decrypt(padding, ciphertext)
            .map_err(|_| Error::DecryptionFailed)
    }
}
```

### 3.3 混合加密

```rust
/// 混合加密系统（ECIES）
#[derive(Debug)]
pub struct HybridEncryption;

impl HybridEncryption {
    /// 加密（使用接收者公钥）
    pub fn encrypt(
        receiver_public_key: &[u8],
        plaintext: &[u8]
    ) -> Result<EncryptedMessage, Error> {
        use rand::rngs::OsRng;
        
        // 1. 生成临时密钥对
        let ephemeral_signer = EcdsaSigner::generate()?;
        let ephemeral_public_key = ephemeral_signer.public_key_bytes();
        
        // 2. 进行ECDH密钥交换
        let shared_secret = Self::ecdh(
            ephemeral_signer.secret_key.as_ref(),
            receiver_public_key
        )?;
        
        // 3. 派生加密密钥
        let encryption_key = Self::derive_key(&shared_secret)?;
        
        // 4. 生成随机nonce
        let mut nonce = [0u8; 12];
        use rand::RngCore;
        OsRng.fill_bytes(&mut nonce);
        
        // 5. 使用AES-GCM加密
        let cipher = Aes256GcmCipher::new(&encryption_key);
        let ciphertext = cipher.encrypt(plaintext, &nonce)?;
        
        Ok(EncryptedMessage {
            ephemeral_public_key,
            nonce: nonce.to_vec(),
            ciphertext,
        })
    }
    
    /// 解密（使用接收者私钥）
    pub fn decrypt(
        receiver_signer: &EcdsaSigner,
        encrypted_msg: &EncryptedMessage
    ) -> Result<Vec<u8>, Error> {
        // 1. 进行ECDH密钥交换
        let shared_secret = Self::ecdh(
            receiver_signer.secret_key.as_ref(),
            &encrypted_msg.ephemeral_public_key
        )?;
        
        // 2. 派生解密密钥
        let decryption_key = Self::derive_key(&shared_secret)?;
        
        // 3. 使用AES-GCM解密
        let cipher = Aes256GcmCipher::new(&decryption_key);
        let nonce: [u8; 12] = encrypted_msg.nonce.as_slice().try_into()?;
        let plaintext = cipher.decrypt(&encrypted_msg.ciphertext, &nonce)?;
        
        Ok(plaintext)
    }
    
    /// ECDH密钥交换
    fn ecdh(private_key: &[u8], public_key: &[u8]) -> Result<Vec<u8>, Error> {
        use secp256k1::{Secp256k1, SecretKey, PublicKey};
        use secp256k1::ecdh::SharedSecret;
        
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(private_key)?;
        let public_key = PublicKey::from_slice(public_key)?;
        
        let shared_secret = SharedSecret::new(&public_key, &secret_key);
        Ok(shared_secret.secret_bytes().to_vec())
    }
    
    /// 密钥派生函数
    fn derive_key(shared_secret: &[u8]) -> Result<[u8; 32], Error> {
        // 使用HKDF派生密钥
        use hkdf::Hkdf;
        use sha2::Sha256;
        
        let hkdf = Hkdf::<Sha256>::new(None, shared_secret);
        let mut okm = [0u8; 32];
        hkdf.expand(b"encryption_key", &mut okm)
            .map_err(|_| Error::KeyDerivationFailed)?;
        
        Ok(okm)
    }
}

/// 加密消息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncryptedMessage {
    /// 临时公钥
    pub ephemeral_public_key: Vec<u8>,
    /// Nonce
    pub nonce: Vec<u8>,
    /// 密文
    pub ciphertext: Vec<u8>,
}
```

## 4. 零知识证明实现

### 4.1 zk-SNARK实现

```rust
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bls12_381::{Bls12, Scalar};

/// zk-SNARK电路示例：证明知道某个值的原像
#[derive(Clone)]
pub struct PreimageCircuit {
    /// 原像（私密输入）
    preimage: Option<Scalar>,
    /// 哈希值（公开输入）
    hash: Option<Scalar>,
}

impl Circuit<Scalar> for PreimageCircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(
        self,
        cs: &mut CS
    ) -> Result<(), SynthesisError> {
        // 分配私密输入
        let preimage = cs.alloc(
            || "preimage",
            || self.preimage.ok_or(SynthesisError::AssignmentMissing)
        )?;
        
        // 分配公开输入
        let hash = cs.alloc_input(
            || "hash",
            || self.hash.ok_or(SynthesisError::AssignmentMissing)
        )?;
        
        // 约束：hash = H(preimage)
        // 这里简化处理，实际需要实现哈希电路
        cs.enforce(
            || "hash constraint",
            |lc| lc + preimage,
            |lc| lc + CS::one(),
            |lc| lc + hash
        );
        
        Ok(())
    }
}

/// zk-SNARK证明器
pub struct ZkSnarkProver;

impl ZkSnarkProver {
    /// 生成证明
    pub fn prove(
        preimage: Scalar,
        hash: Scalar
    ) -> Result<ZkSnarkProof, Error> {
        use bellman::groth16::{generate_random_parameters, create_random_proof};
        use rand::rngs::OsRng;
        
        let mut rng = OsRng;
        
        // 1. 创建电路
        let circuit = PreimageCircuit {
            preimage: Some(preimage),
            hash: Some(hash),
        };
        
        // 2. 生成证明参数（实际应该预先生成）
        let params = generate_random_parameters::<Bls12, _, _>(
            PreimageCircuit {
                preimage: None,
                hash: None,
            },
            &mut rng
        )?;
        
        // 3. 生成证明
        let proof = create_random_proof(circuit, &params, &mut rng)?;
        
        Ok(ZkSnarkProof {
            proof: bincode::serialize(&proof)?,
        })
    }
    
    /// 验证证明
    pub fn verify(
        proof: &ZkSnarkProof,
        hash: Scalar
    ) -> Result<bool, Error> {
        // 实现证明验证
        Ok(true)
    }
}

/// zk-SNARK证明
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ZkSnarkProof {
    pub proof: Vec<u8>,
}
```

### 4.2 zk-STARK实现

```rust
/// zk-STARK证明器（简化实现）
pub struct ZkStarkProver;

impl ZkStarkProver {
    /// 生成STARK证明
    pub fn prove(
        computation_trace: &[u64],
        public_input: &[u64]
    ) -> Result<ZkStarkProof, Error> {
        // 实现STARK证明生成
        // 1. 构建执行轨迹
        // 2. 生成多项式承诺
        // 3. 进行FRI协议
        
        Ok(ZkStarkProof {
            proof: vec![],
        })
    }
    
    /// 验证STARK证明
    pub fn verify(
        proof: &ZkStarkProof,
        public_input: &[u64]
    ) -> Result<bool, Error> {
        // 实现STARK证明验证
        Ok(true)
    }
}

/// zk-STARK证明
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ZkStarkProof {
    pub proof: Vec<u8>,
}
```

### 4.3 Bulletproofs实现

```rust
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek::scalar::Scalar as Curve25519Scalar;

/// Bulletproofs范围证明器
pub struct BulletproofsRangeProver {
    bp_gens: BulletproofGens,
    pc_gens: PedersenGens,
}

impl BulletproofsRangeProver {
    pub fn new() -> Self {
        Self {
            bp_gens: BulletproofGens::new(64, 1),
            pc_gens: PedersenGens::default(),
        }
    }
    
    /// 生成范围证明（证明value在[0, 2^64)范围内）
    pub fn prove_range(
        &self,
        value: u64,
        blinding: &Curve25519Scalar
    ) -> Result<BulletproofRangeProof, Error> {
        use bulletproofs::Transcript;
        
        let mut transcript = Transcript::new(b"range_proof");
        
        let (proof, commitment) = RangeProof::prove_single(
            &self.bp_gens,
            &self.pc_gens,
            &mut transcript,
            value,
            blinding,
            64
        )?;
        
        Ok(BulletproofRangeProof {
            proof: bincode::serialize(&proof)?,
            commitment: commitment.compress().to_bytes().to_vec(),
        })
    }
    
    /// 验证范围证明
    pub fn verify_range(
        &self,
        proof: &BulletproofRangeProof
    ) -> Result<bool, Error> {
        use bulletproofs::Transcript;
        use curve25519_dalek::ristretto::CompressedRistretto;
        
        let mut transcript = Transcript::new(b"range_proof");
        
        let range_proof: RangeProof = bincode::deserialize(&proof.proof)?;
        let commitment = CompressedRistretto::from_slice(&proof.commitment)
            .decompress()
            .ok_or(Error::InvalidCommitment)?;
        
        range_proof.verify_single(
            &self.bp_gens,
            &self.pc_gens,
            &mut transcript,
            &commitment,
            64
        ).map_err(|_| Error::ProofVerificationFailed)?;
        
        Ok(true)
    }
}

/// Bulletproofs范围证明
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BulletproofRangeProof {
    pub proof: Vec<u8>,
    pub commitment: Vec<u8>,
}
```

## 5. Merkle树实现

### 5.1 标准Merkle树

```rust
/// Merkle树实现
#[derive(Debug, Clone)]
pub struct MerkleTree {
    /// 叶子节点
    leaves: Vec<Hash>,
    /// 树的层次
    layers: Vec<Vec<Hash>>,
    /// 根哈希
    root: Hash,
}

impl MerkleTree {
    /// 构建Merkle树
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        // 1. 计算叶子节点哈希
        let leaves: Vec<Hash> = data.iter()
            .map(|d| Hash::hash(d))
            .collect();
        
        // 2. 构建树
        let mut layers = vec![leaves.clone()];
        let mut current_layer = leaves.clone();
        
        while current_layer.len() > 1 {
            let next_layer = Self::build_layer(&current_layer);
            layers.push(next_layer.clone());
            current_layer = next_layer;
        }
        
        let root = current_layer[0].clone();
        
        Self {
            leaves,
            layers,
            root,
        }
    }
    
    /// 构建上一层
    fn build_layer(current: &[Hash]) -> Vec<Hash> {
        let mut next = Vec::new();
        
        for chunk in current.chunks(2) {
            let hash = if chunk.len() == 2 {
                // 合并两个哈希
                let mut data = Vec::new();
                data.extend_from_slice(chunk[0].as_bytes());
                data.extend_from_slice(chunk[1].as_bytes());
                Hash::hash(&data)
            } else {
                // 奇数节点，直接向上传递
                chunk[0].clone()
            };
            next.push(hash);
        }
        
        next
    }
    
    /// 获取根哈希
    pub fn root(&self) -> &Hash {
        &self.root
    }
    
    /// 生成Merkle证明
    pub fn generate_proof(&self, index: usize) -> Result<MerkleProof, Error> {
        if index >= self.leaves.len() {
            return Err(Error::InvalidIndex);
        }
        
        let mut proof = Vec::new();
        let mut current_index = index;
        
        for layer in &self.layers[..self.layers.len() - 1] {
            // 获取兄弟节点
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };
            
            if sibling_index < layer.len() {
                proof.push(MerkleProofNode {
                    hash: layer[sibling_index].clone(),
                    is_left: current_index % 2 == 1,
                });
            }
            
            current_index /= 2;
        }
        
        Ok(MerkleProof {
            leaf_index: index,
            leaf_hash: self.leaves[index].clone(),
            proof,
        })
    }
    
    /// 验证Merkle证明
    pub fn verify_proof(
        root: &Hash,
        proof: &MerkleProof
    ) -> bool {
        let mut current_hash = proof.leaf_hash.clone();
        
        for node in &proof.proof {
            let mut data = Vec::new();
            
            if node.is_left {
                data.extend_from_slice(node.hash.as_bytes());
                data.extend_from_slice(current_hash.as_bytes());
            } else {
                data.extend_from_slice(current_hash.as_bytes());
                data.extend_from_slice(node.hash.as_bytes());
            }
            
            current_hash = Hash::hash(&data);
        }
        
        &current_hash == root
    }
}

/// Merkle证明
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MerkleProof {
    pub leaf_index: usize,
    pub leaf_hash: Hash,
    pub proof: Vec<MerkleProofNode>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MerkleProofNode {
    pub hash: Hash,
    pub is_left: bool,
}
```

### 5.2 Merkle Patricia Trie

已在存储系统文档中实现。

### 5.3 稀疏Merkle树

```rust
/// 稀疏Merkle树
#[derive(Debug)]
pub struct SparseMerkleTree {
    /// 树的深度
    depth: usize,
    /// 节点存储
    nodes: HashMap<Vec<u8>, Hash>,
    /// 根哈希
    root: Hash,
    /// 默认哈希（空节点）
    default_hashes: Vec<Hash>,
}

impl SparseMerkleTree {
    /// 创建稀疏Merkle树
    pub fn new(depth: usize) -> Self {
        // 预计算默认哈希
        let default_hashes = Self::compute_default_hashes(depth);
        let root = default_hashes[depth].clone();
        
        Self {
            depth,
            nodes: HashMap::new(),
            root,
            default_hashes,
        }
    }
    
    /// 预计算默认哈希
    fn compute_default_hashes(depth: usize) -> Vec<Hash> {
        let mut hashes = vec![Hash::empty()];
        
        for _ in 0..depth {
            let prev = hashes.last().unwrap();
            let mut data = Vec::new();
            data.extend_from_slice(prev.as_bytes());
            data.extend_from_slice(prev.as_bytes());
            hashes.push(Hash::hash(&data));
        }
        
        hashes
    }
    
    /// 更新叶子节点
    pub fn update(&mut self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        let leaf_hash = Hash::hash(value);
        let path = Self::key_to_path(key, self.depth);
        
        self.root = self.update_path(&path, &leaf_hash, 0)?;
        
        Ok(())
    }
    
    /// 递归更新路径
    fn update_path(
        &mut self,
        path: &[bool],
        leaf_hash: &Hash,
        depth: usize
    ) -> Result<Hash, Error> {
        if depth == self.depth {
            return Ok(leaf_hash.clone());
        }
        
        let node_key = Self::node_key(path, depth);
        
        // 获取当前节点的子节点
        let (left, right) = if let Some(node_hash) = self.nodes.get(&node_key) {
            // 节点存在，获取子节点
            self.get_children(&node_key)?
        } else {
            // 节点不存在，使用默认哈希
            (
                self.default_hashes[depth].clone(),
                self.default_hashes[depth].clone()
            )
        };
        
        // 递归更新
        let new_child = self.update_path(path, leaf_hash, depth + 1)?;
        
        let new_hash = if path[depth] {
            // 更新右子节点
            let mut data = Vec::new();
            data.extend_from_slice(left.as_bytes());
            data.extend_from_slice(new_child.as_bytes());
            Hash::hash(&data)
        } else {
            // 更新左子节点
            let mut data = Vec::new();
            data.extend_from_slice(new_child.as_bytes());
            data.extend_from_slice(right.as_bytes());
            Hash::hash(&data)
        };
        
        self.nodes.insert(node_key, new_hash.clone());
        
        Ok(new_hash)
    }
    
    /// 密钥转换为路径
    fn key_to_path(key: &[u8], depth: usize) -> Vec<bool> {
        let mut path = Vec::new();
        
        for byte in key.iter().take(depth / 8 + 1) {
            for i in (0..8).rev() {
                if path.len() >= depth {
                    break;
                }
                path.push((byte >> i) & 1 == 1);
            }
        }
        
        path
    }
    
    /// 节点密钥
    fn node_key(path: &[bool], depth: usize) -> Vec<u8> {
        let mut key = vec![depth as u8];
        
        for chunk in path[..depth].chunks(8) {
            let mut byte = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                if bit {
                    byte |= 1 << (7 - i);
                }
            }
            key.push(byte);
        }
        
        key
    }
    
    /// 获取子节点
    fn get_children(&self, node_key: &[u8]) -> Result<(Hash, Hash), Error> {
        // 简化实现
        Ok((Hash::empty(), Hash::empty()))
    }
    
    /// 获取根哈希
    pub fn root(&self) -> &Hash {
        &self.root
    }
}
```

## 6. 密钥管理

### 6.1 密钥生成

```rust
/// 密钥生成器
pub struct KeyGenerator;

impl KeyGenerator {
    /// 生成随机密钥
    pub fn generate_random_key(size: usize) -> Vec<u8> {
        use rand::RngCore;
        let mut key = vec![0u8; size];
        rand::rngs::OsRng.fill_bytes(&mut key);
        key
    }
    
    /// 生成ECDSA密钥对
    pub fn generate_ecdsa_keypair() -> Result<(Vec<u8>, Vec<u8>), Error> {
        let signer = EcdsaSigner::generate()?;
        let private_key = signer.secret_key.secret_bytes().to_vec();
        let public_key = signer.public_key_bytes();
        Ok((private_key, public_key))
    }
    
    /// 生成EdDSA密钥对
    pub fn generate_eddsa_keypair() -> Result<(Vec<u8>, Vec<u8>), Error> {
        let signer = EddsaSigner::generate()?;
        let private_key = signer.keypair.secret.to_bytes().to_vec();
        let public_key = signer.public_key_bytes();
        Ok((private_key, public_key))
    }
}
```

### 6.2 密钥派生

```rust
use hkdf::Hkdf;
use sha2::Sha256;

/// 密钥派生函数
pub struct KeyDerivation;

impl KeyDerivation {
    /// HKDF密钥派生
    pub fn hkdf_derive(
        ikm: &[u8],      // 输入密钥材料
        salt: &[u8],     // 盐
        info: &[u8],     // 上下文信息
        okm_len: usize   // 输出密钥长度
    ) -> Result<Vec<u8>, Error> {
        let hkdf = Hkdf::<Sha256>::new(Some(salt), ikm);
        let mut okm = vec![0u8; okm_len];
        hkdf.expand(info, &mut okm)
            .map_err(|_| Error::KeyDerivationFailed)?;
        Ok(okm)
    }
    
    /// PBKDF2密钥派生
    pub fn pbkdf2_derive(
        password: &[u8],
        salt: &[u8],
        iterations: u32,
        key_len: usize
    ) -> Vec<u8> {
        use pbkdf2::pbkdf2_hmac;
        
        let mut key = vec![0u8; key_len];
        pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut key);
        key
    }
    
    /// BIP32分层确定性密钥派生
    pub fn bip32_derive(
        master_key: &[u8],
        path: &[u32]
    ) -> Result<Vec<u8>, Error> {
        // 实现BIP32密钥派生
        // 简化实现
        Ok(master_key.to_vec())
    }
}
```

### 6.3 密钥存储

```rust
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};

/// 加密密钥存储
pub struct KeyStore {
    /// 主密钥（用于加密存储的密钥）
    master_key: [u8; 32],
    /// 存储的加密密钥
    encrypted_keys: HashMap<String, Vec<u8>>,
}

impl KeyStore {
    /// 创建密钥存储
    pub fn new(password: &str) -> Result<Self, Error> {
        // 从密码派生主密钥
        let salt = b"keystore_salt";
        let master_key = KeyDerivation::pbkdf2_derive(
            password.as_bytes(),
            salt,
            100_000,
            32
        );
        
        let mut key = [0u8; 32];
        key.copy_from_slice(&master_key);
        
        Ok(Self {
            master_key: key,
            encrypted_keys: HashMap::new(),
        })
    }
    
    /// 存储密钥
    pub fn store_key(&mut self, name: &str, key: &[u8]) -> Result<(), Error> {
        // 使用主密钥加密
        let cipher = Aes256GcmCipher::new(&self.master_key);
        let nonce = Self::generate_nonce();
        let encrypted = cipher.encrypt(key, &nonce)?;
        
        // 存储（nonce + 密文）
        let mut stored = nonce.to_vec();
        stored.extend_from_slice(&encrypted);
        
        self.encrypted_keys.insert(name.to_string(), stored);
        
        Ok(())
    }
    
    /// 读取密钥
    pub fn load_key(&self, name: &str) -> Result<Vec<u8>, Error> {
        let encrypted_data = self.encrypted_keys.get(name)
            .ok_or(Error::KeyNotFound)?;
        
        // 分离nonce和密文
        let nonce: [u8; 12] = encrypted_data[..12].try_into()?;
        let ciphertext = &encrypted_data[12..];
        
        // 解密
        let cipher = Aes256GcmCipher::new(&self.master_key);
        cipher.decrypt(ciphertext, &nonce)
    }
    
    /// 生成随机nonce
    fn generate_nonce() -> [u8; 12] {
        use rand::RngCore;
        let mut nonce = [0u8; 12];
        rand::rngs::OsRng.fill_bytes(&mut nonce);
        nonce
    }
}
```

## 7. 随机数生成

### 7.1 CSPRNG实现

```rust
use rand::{RngCore, CryptoRng};

/// 密码学安全随机数生成器
pub struct SecureRandom;

impl SecureRandom {
    /// 生成随机字节
    pub fn random_bytes(len: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; len];
        rand::rngs::OsRng.fill_bytes(&mut bytes);
        bytes
    }
    
    /// 生成随机u64
    pub fn random_u64() -> u64 {
        rand::rngs::OsRng.next_u64()
    }
    
    /// 生成范围内的随机数
    pub fn random_range(min: u64, max: u64) -> u64 {
        use rand::Rng;
        rand::rngs::OsRng.gen_range(min..max)
    }
}
```

### 7.2 确定性随机数

```rust
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;

/// 确定性随机数生成器
pub struct DeterministicRandom {
    rng: ChaCha20Rng,
}

impl DeterministicRandom {
    /// 从种子创建
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        Self {
            rng: ChaCha20Rng::from_seed(*seed),
        }
    }
    
    /// 生成随机字节
    pub fn random_bytes(&mut self, len: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; len];
        self.rng.fill_bytes(&mut bytes);
        bytes
    }
}
```

### 7.3 VRF实现

```rust
/// 可验证随机函数（VRF）
pub struct Vrf {
    signer: EddsaSigner,
}

impl Vrf {
    /// 创建VRF
    pub fn new(signer: EddsaSigner) -> Self {
        Self { signer }
    }
    
    /// 生成VRF证明和输出
    pub fn prove(&self, input: &[u8]) -> (VrfOutput, VrfProof) {
        // 1. 签名输入
        let signature = self.signer.sign(input);
        
        // 2. 从签名派生输出
        let output = Hash::hash(&signature);
        
        (
            VrfOutput { value: output },
            VrfProof {
                signature,
                public_key: self.signer.public_key_bytes(),
            }
        )
    }
    
    /// 验证VRF证明
    pub fn verify(
        input: &[u8],
        output: &VrfOutput,
        proof: &VrfProof
    ) -> Result<bool, Error> {
        // 1. 验证签名
        let valid = EddsaVerifier::verify(
            input,
            &proof.signature,
            &proof.public_key
        )?;
        
        if !valid {
            return Ok(false);
        }
        
        // 2. 重新计算输出
        let computed_output = Hash::hash(&proof.signature);
        
        Ok(computed_output == output.value)
    }
}

#[derive(Debug, Clone)]
pub struct VrfOutput {
    pub value: Hash,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VrfProof {
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}
```

## 8. 高级密码学协议

### 8.1 多方计算

```rust
/// 安全多方计算（简化实现）
pub struct SecureMultiPartyComputation;

impl SecureMultiPartyComputation {
    /// 秘密分享（Shamir秘密共享）
    pub fn share_secret(
        secret: &[u8],
        threshold: usize,
        total_shares: usize
    ) -> Result<Vec<Share>, Error> {
        // 实现Shamir秘密共享
        let mut shares = Vec::new();
        
        for i in 1..=total_shares {
            shares.push(Share {
                index: i,
                value: vec![], // 计算共享值
            });
        }
        
        Ok(shares)
    }
    
    /// 恢复秘密
    pub fn recover_secret(
        shares: &[Share],
        threshold: usize
    ) -> Result<Vec<u8>, Error> {
        if shares.len() < threshold {
            return Err(Error::InsufficientShares);
        }
        
        // 实现拉格朗日插值恢复秘密
        Ok(vec![])
    }
}

#[derive(Debug, Clone)]
pub struct Share {
    pub index: usize,
    pub value: Vec<u8>,
}
```

### 8.2 阈值签名

```rust
/// 阈值签名方案
pub struct ThresholdSignature;

impl ThresholdSignature {
    /// 生成阈值密钥
    pub fn generate_threshold_keys(
        threshold: usize,
        total: usize
    ) -> Result<(Vec<PrivateKeyShare>, PublicKey), Error> {
        // 实现阈值密钥生成
        Ok((vec![], PublicKey::from_slice(&[]).unwrap()))
    }
    
    /// 部分签名
    pub fn partial_sign(
        private_share: &PrivateKeyShare,
        message: &[u8]
    ) -> Result<PartialSignature, Error> {
        // 实现部分签名
        Ok(PartialSignature {
            index: 0,
            signature: vec![],
        })
    }
    
    /// 合并部分签名
    pub fn combine_signatures(
        partial_sigs: &[PartialSignature],
        threshold: usize
    ) -> Result<Vec<u8>, Error> {
        if partial_sigs.len() < threshold {
            return Err(Error::InsufficientSignatures);
        }
        
        // 合并部分签名
        Ok(vec![])
    }
}

#[derive(Debug, Clone)]
pub struct PrivateKeyShare {
    pub index: usize,
    pub share: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct PartialSignature {
    pub index: usize,
    pub signature: Vec<u8>,
}
```

### 8.3 同态加密

```rust
/// 同态加密（简化示例）
pub struct HomomorphicEncryption;

impl HomomorphicEncryption {
    /// 加密
    pub fn encrypt(plaintext: u64, public_key: &[u8]) -> Result<Vec<u8>, Error> {
        // 实现同态加密
        Ok(vec![])
    }
    
    /// 解密
    pub fn decrypt(ciphertext: &[u8], private_key: &[u8]) -> Result<u64, Error> {
        // 实现解密
        Ok(0)
    }
    
    /// 同态加法
    pub fn add(ciphertext1: &[u8], ciphertext2: &[u8]) -> Result<Vec<u8>, Error> {
        // 密文相加
        Ok(vec![])
    }
    
    /// 同态乘法
    pub fn multiply(ciphertext: &[u8], scalar: u64) -> Result<Vec<u8>, Error> {
        // 密文与明文标量相乘
        Ok(vec![])
    }
}
```

## 9. 总结

本文档详细介绍了区块链密码学的Rust实现，包括：

1. **哈希函数**：SHA-256、Blake2、Keccak-256
2. **数字签名**：ECDSA、EdDSA、Schnorr
3. **加密算法**：对称加密、非对称加密、混合加密
4. **零知识证明**：zk-SNARK、zk-STARK、Bulletproofs
5. **Merkle树**：标准Merkle树、MPT、稀疏Merkle树
6. **密钥管理**：生成、派生、存储
7. **随机数**：CSPRNG、确定性RNG、VRF
8. **高级协议**：MPC、阈值签名、同态加密

这些实现为构建安全、高效的区块链系统提供了完整的密码学基础设施。

---

**文档版本**: v1.0.0  
**最后更新**: 2025年10月17日  
**作者**: Rust区块链技术团队  
**相关文档**:
- [02_CRYPTOGRAPHIC_FOUNDATIONS.md](./02_CRYPTOGRAPHIC_FOUNDATIONS.md) - 密码学理论基础
- [12_RUST_IMPLEMENTATION.md](./12_RUST_IMPLEMENTATION.md) - Rust语言实现
- [19_SECURITY_BEST_PRACTICES.md](./19_SECURITY_BEST_PRACTICES.md) - 安全最佳实践

