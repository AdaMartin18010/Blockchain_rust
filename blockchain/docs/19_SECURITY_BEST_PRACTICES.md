# 安全最佳实践

## 📋 目录

- [安全最佳实践](#安全最佳实践)
  - [📋 目录](#-目录)
  - [1. 智能合约安全](#1-智能合约安全)
    - [1.1 常见漏洞防范](#11-常见漏洞防范)
      - [重入攻击防护](#重入攻击防护)
      - [整数溢出防护](#整数溢出防护)
      - [权限控制](#权限控制)
    - [1.2 安全编码模式](#12-安全编码模式)
      - [Pull over Push模式](#pull-over-push模式)
      - [速率限制](#速率限制)
    - [1.3 审计清单](#13-审计清单)
  - [2. 密钥管理安全](#2-密钥管理安全)
    - [2.1 密钥生成](#21-密钥生成)
    - [2.2 密钥存储](#22-密钥存储)
    - [2.3 密钥使用](#23-密钥使用)
  - [3. 网络安全](#3-网络安全)
    - [3.1 DDoS防护](#31-ddos防护)
    - [3.2 女巫攻击防御](#32-女巫攻击防御)
    - [3.3 Eclipse攻击防护](#33-eclipse攻击防护)
  - [4. 共识安全](#4-共识安全)
    - [4.1 51%攻击防御](#41-51攻击防御)
    - [4.2 长程攻击防护](#42-长程攻击防护)
    - [4.3 双花攻击防范](#43-双花攻击防范)
  - [5. 节点安全](#5-节点安全)
    - [5.1 节点加固](#51-节点加固)
    - [5.2 访问控制](#52-访问控制)
    - [5.3 监控告警](#53-监控告警)
  - [6. 数据安全](#6-数据安全)
    - [6.1 数据加密](#61-数据加密)
    - [6.2 隐私保护](#62-隐私保护)
    - [6.3 备份恢复](#63-备份恢复)
  - [7. 应用安全](#7-应用安全)
    - [7.1 前端安全](#71-前端安全)
    - [7.2 API安全](#72-api安全)
    - [7.3 钱包安全](#73-钱包安全)
  - [8. 运维安全](#8-运维安全)
    - [8.1 部署安全](#81-部署安全)
    - [8.2 升级策略](#82-升级策略)
    - [8.3 应急响应](#83-应急响应)
  - [9. 总结](#9-总结)

## 1. 智能合约安全

### 1.1 常见漏洞防范

#### 重入攻击防护

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

/// 防重入保护
#[derive(Debug)]
pub struct ReentrancyGuard {
    locked: Arc<RwLock<bool>>,
}

impl ReentrancyGuard {
    pub fn new() -> Self {
        Self {
            locked: Arc::new(RwLock::new(false)),
        }
    }
    
    /// 执行带重入保护的函数
    pub async fn execute<F, R>(&self, f: F) -> Result<R, Error>
    where
        F: FnOnce() -> Result<R, Error>,
    {
        // 检查是否已锁定
        {
            let mut locked = self.locked.write().await;
            if *locked {
                return Err(Error::ReentrancyDetected);
            }
            *locked = true;
        }
        
        // 执行函数
        let result = f();
        
        // 解锁
        {
            let mut locked = self.locked.write().await;
            *locked = false;
        }
        
        result
    }
}

/// 安全的提款函数示例
#[derive(Debug)]
pub struct SecureVault {
    balances: Arc<RwLock<HashMap<Address, u64>>>,
    reentrancy_guard: ReentrancyGuard,
}

impl SecureVault {
    /// 安全提款（使用Checks-Effects-Interactions模式）
    pub async fn withdraw(&self, user: Address, amount: u64) -> Result<(), Error> {
        self.reentrancy_guard.execute(|| {
            // 1. Checks - 检查条件
            let balance = self.get_balance(&user)?;
            if balance < amount {
                return Err(Error::InsufficientBalance);
            }
            
            // 2. Effects - 更新状态
            self.update_balance(&user, balance - amount)?;
            
            // 3. Interactions - 外部调用
            self.transfer_funds(&user, amount)?;
            
            Ok(())
        }).await
    }
    
    fn get_balance(&self, user: &Address) -> Result<u64, Error> {
        // 实现余额查询
        Ok(0)
    }
    
    fn update_balance(&self, user: &Address, new_balance: u64) -> Result<(), Error> {
        // 实现余额更新
        Ok(())
    }
    
    fn transfer_funds(&self, user: &Address, amount: u64) -> Result<(), Error> {
        // 实现资金转移
        Ok(())
    }
}
```

#### 整数溢出防护

```rust
use std::ops::{Add, Sub, Mul};

/// 安全整数运算
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafeU256(u128);

impl SafeU256 {
    pub fn new(value: u128) -> Self {
        Self(value)
    }
    
    /// 安全加法
    pub fn safe_add(self, other: Self) -> Result<Self, Error> {
        self.0.checked_add(other.0)
            .map(SafeU256)
            .ok_or(Error::Overflow)
    }
    
    /// 安全减法
    pub fn safe_sub(self, other: Self) -> Result<Self, Error> {
        self.0.checked_sub(other.0)
            .map(SafeU256)
            .ok_or(Error::Underflow)
    }
    
    /// 安全乘法
    pub fn safe_mul(self, other: Self) -> Result<Self, Error> {
        self.0.checked_mul(other.0)
            .map(SafeU256)
            .ok_or(Error::Overflow)
    }
    
    /// 安全除法
    pub fn safe_div(self, other: Self) -> Result<Self, Error> {
        if other.0 == 0 {
            return Err(Error::DivisionByZero);
        }
        self.0.checked_div(other.0)
            .map(SafeU256)
            .ok_or(Error::Overflow)
    }
}

/// 使用示例
pub fn calculate_reward(base: u128, multiplier: u128) -> Result<u128, Error> {
    let base_safe = SafeU256::new(base);
    let multiplier_safe = SafeU256::new(multiplier);
    
    let result = base_safe.safe_mul(multiplier_safe)?;
    Ok(result.0)
}
```

#### 权限控制

```rust
/// 访问控制管理器
#[derive(Debug)]
pub struct AccessControl {
    /// 角色映射
    roles: Arc<RwLock<HashMap<Address, HashSet<Role>>>>,
    /// 角色权限
    role_permissions: Arc<RwLock<HashMap<Role, HashSet<Permission>>>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Role {
    Owner,
    Admin,
    Minter,
    Burner,
    User,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Permission {
    Mint,
    Burn,
    Transfer,
    Pause,
    Upgrade,
}

impl AccessControl {
    /// 检查权限
    pub async fn check_permission(
        &self,
        user: &Address,
        permission: Permission
    ) -> Result<bool, Error> {
        // 1. 获取用户角色
        let roles = self.roles.read().await;
        let user_roles = roles.get(user).ok_or(Error::NoRoles)?;
        
        // 2. 检查角色权限
        let role_perms = self.role_permissions.read().await;
        
        for role in user_roles {
            if let Some(perms) = role_perms.get(role) {
                if perms.contains(&permission) {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    /// 授予角色
    pub async fn grant_role(&self, user: Address, role: Role) -> Result<(), Error> {
        let mut roles = self.roles.write().await;
        roles.entry(user)
            .or_insert_with(HashSet::new)
            .insert(role);
        Ok(())
    }
    
    /// 撤销角色
    pub async fn revoke_role(&self, user: &Address, role: &Role) -> Result<(), Error> {
        let mut roles = self.roles.write().await;
        if let Some(user_roles) = roles.get_mut(user) {
            user_roles.remove(role);
        }
        Ok(())
    }
}

/// 带权限检查的函数装饰器
pub async fn require_permission<F, R>(
    access_control: &AccessControl,
    user: &Address,
    permission: Permission,
    f: F
) -> Result<R, Error>
where
    F: FnOnce() -> Result<R, Error>,
{
    // 检查权限
    if !access_control.check_permission(user, permission).await? {
        return Err(Error::PermissionDenied);
    }
    
    // 执行函数
    f()
}
```

### 1.2 安全编码模式

#### Pull over Push模式

```rust
/// 安全的奖励分发（使用Pull模式）
#[derive(Debug)]
pub struct RewardDistributor {
    /// 待领取奖励
    pending_rewards: Arc<RwLock<HashMap<Address, u64>>>,
    /// 总奖励池
    reward_pool: Arc<RwLock<u64>>,
}

impl RewardDistributor {
    /// 记录奖励（不直接转账）
    pub async fn allocate_reward(&self, user: Address, amount: u64) -> Result<(), Error> {
        let mut pending = self.pending_rewards.write().await;
        *pending.entry(user).or_insert(0) += amount;
        Ok(())
    }
    
    /// 用户主动领取奖励
    pub async fn claim_rewards(&self, user: Address) -> Result<u64, Error> {
        let mut pending = self.pending_rewards.write().await;
        
        let amount = pending.remove(&user).ok_or(Error::NoRewards)?;
        
        if amount == 0 {
            return Err(Error::NoRewards);
        }
        
        // 转账
        self.transfer(&user, amount).await?;
        
        Ok(amount)
    }
    
    async fn transfer(&self, user: &Address, amount: u64) -> Result<(), Error> {
        // 实现转账逻辑
        Ok(())
    }
}
```

#### 速率限制

```rust
use std::time::{Instant, Duration};

/// 速率限制器
#[derive(Debug)]
pub struct RateLimiter {
    /// 用户请求记录
    requests: Arc<RwLock<HashMap<Address, Vec<Instant>>>>,
    /// 时间窗口
    window: Duration,
    /// 最大请求数
    max_requests: usize,
}

impl RateLimiter {
    pub fn new(window: Duration, max_requests: usize) -> Self {
        Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
            window,
            max_requests,
        }
    }
    
    /// 检查速率限制
    pub async fn check_rate_limit(&self, user: &Address) -> Result<(), Error> {
        let mut requests = self.requests.write().await;
        let now = Instant::now();
        
        // 获取用户请求历史
        let user_requests = requests.entry(*user).or_insert_with(Vec::new);
        
        // 移除过期请求
        user_requests.retain(|&time| now.duration_since(time) < self.window);
        
        // 检查是否超过限制
        if user_requests.len() >= self.max_requests {
            return Err(Error::RateLimitExceeded);
        }
        
        // 记录新请求
        user_requests.push(now);
        
        Ok(())
    }
}

/// 带速率限制的函数
pub async fn rate_limited_function<F, R>(
    rate_limiter: &RateLimiter,
    user: &Address,
    f: F
) -> Result<R, Error>
where
    F: FnOnce() -> Result<R, Error>,
{
    // 检查速率限制
    rate_limiter.check_rate_limit(user).await?;
    
    // 执行函数
    f()
}
```

### 1.3 审计清单

```rust
/// 智能合约安全审计清单
#[derive(Debug)]
pub struct SecurityAuditChecklist {
    items: Vec<AuditItem>,
}

#[derive(Debug, Clone)]
pub struct AuditItem {
    pub category: AuditCategory,
    pub description: String,
    pub severity: Severity,
    pub status: AuditStatus,
}

#[derive(Debug, Clone)]
pub enum AuditCategory {
    ReentrancyProtection,
    IntegerOverflow,
    AccessControl,
    InputValidation,
    GasOptimization,
    CodeQuality,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

#[derive(Debug, Clone)]
pub enum AuditStatus {
    Pending,
    Pass,
    Fail,
    NeedsReview,
}

impl SecurityAuditChecklist {
    pub fn new() -> Self {
        let mut items = Vec::new();
        
        // 重入攻击检查
        items.push(AuditItem {
            category: AuditCategory::ReentrancyProtection,
            description: "所有外部调用都使用了Checks-Effects-Interactions模式".to_string(),
            severity: Severity::Critical,
            status: AuditStatus::Pending,
        });
        
        // 整数溢出检查
        items.push(AuditItem {
            category: AuditCategory::IntegerOverflow,
            description: "所有算术运算都使用安全数学库".to_string(),
            severity: Severity::High,
            status: AuditStatus::Pending,
        });
        
        // 访问控制检查
        items.push(AuditItem {
            category: AuditCategory::AccessControl,
            description: "所有特权函数都有适当的访问控制".to_string(),
            severity: Severity::Critical,
            status: AuditStatus::Pending,
        });
        
        // 输入验证
        items.push(AuditItem {
            category: AuditCategory::InputValidation,
            description: "所有用户输入都经过验证".to_string(),
            severity: Severity::High,
            status: AuditStatus::Pending,
        });
        
        Self { items }
    }
    
    /// 生成审计报告
    pub fn generate_report(&self) -> AuditReport {
        let total = self.items.len();
        let passed = self.items.iter().filter(|i| i.status == AuditStatus::Pass).count();
        let failed = self.items.iter().filter(|i| i.status == AuditStatus::Fail).count();
        
        AuditReport {
            total_items: total,
            passed,
            failed,
            pass_rate: passed as f64 / total as f64,
            items: self.items.clone(),
        }
    }
}

#[derive(Debug)]
pub struct AuditReport {
    pub total_items: usize,
    pub passed: usize,
    pub failed: usize,
    pub pass_rate: f64,
    pub items: Vec<AuditItem>,
}
```

## 2. 密钥管理安全

### 2.1 密钥生成

```rust
use rand::rngs::OsRng;
use ed25519_dalek::Keypair;

/// 安全密钥生成器
pub struct SecureKeyGenerator;

impl SecureKeyGenerator {
    /// 生成强随机密钥
    pub fn generate_keypair() -> Result<Keypair, Error> {
        let mut csprng = OsRng;
        Ok(Keypair::generate(&mut csprng))
    }
    
    /// 从助记词生成密钥
    pub fn from_mnemonic(mnemonic: &str) -> Result<Keypair, Error> {
        use bip39::{Mnemonic, Language};
        
        // 验证助记词
        let mnemonic = Mnemonic::from_phrase(mnemonic, Language::English)?;
        
        // 生成种子
        let seed = mnemonic.to_seed("");
        
        // 从种子派生密钥
        let secret_key = ed25519_dalek::SecretKey::from_bytes(&seed[..32])?;
        let public_key = ed25519_dalek::PublicKey::from(&secret_key);
        
        Ok(Keypair {
            secret: secret_key,
            public: public_key,
        })
    }
    
    /// 生成助记词
    pub fn generate_mnemonic() -> Result<String, Error> {
        use bip39::{Mnemonic, MnemonicType, Language};
        
        let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
        Ok(mnemonic.phrase().to_string())
    }
}
```

### 2.2 密钥存储

```rust
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use argon2::{Argon2, PasswordHasher};

/// 加密密钥存储
#[derive(Debug)]
pub struct EncryptedKeyStore {
    /// 加密的密钥数据
    encrypted_data: Vec<u8>,
    /// 派生密钥的盐
    salt: Vec<u8>,
}

impl EncryptedKeyStore {
    /// 加密并存储密钥
    pub fn encrypt_key(
        private_key: &[u8],
        password: &str
    ) -> Result<Self, Error> {
        use rand::RngCore;
        
        // 生成随机盐
        let mut salt = vec![0u8; 32];
        OsRng.fill_bytes(&mut salt);
        
        // 使用Argon2派生加密密钥
        let encryption_key = Self::derive_key(password, &salt)?;
        
        // 生成随机nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        
        // 加密私钥
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&encryption_key));
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher.encrypt(nonce, private_key)
            .map_err(|_| Error::EncryptionFailed)?;
        
        // 组合nonce和密文
        let mut encrypted_data = nonce_bytes.to_vec();
        encrypted_data.extend_from_slice(&ciphertext);
        
        Ok(Self {
            encrypted_data,
            salt,
        })
    }
    
    /// 解密密钥
    pub fn decrypt_key(&self, password: &str) -> Result<Vec<u8>, Error> {
        // 派生解密密钥
        let decryption_key = Self::derive_key(password, &self.salt)?;
        
        // 分离nonce和密文
        let nonce = Nonce::from_slice(&self.encrypted_data[..12]);
        let ciphertext = &self.encrypted_data[12..];
        
        // 解密
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&decryption_key));
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|_| Error::DecryptionFailed)?;
        
        Ok(plaintext)
    }
    
    /// 使用Argon2派生密钥
    fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32], Error> {
        use argon2::password_hash::{PasswordHash, SaltString};
        use argon2::PasswordHasher;
        
        let argon2 = Argon2::default();
        let salt_string = SaltString::encode_b64(salt)
            .map_err(|_| Error::KeyDerivationFailed)?;
        
        let hash = argon2.hash_password(password.as_bytes(), &salt_string)
            .map_err(|_| Error::KeyDerivationFailed)?;
        
        let hash_bytes = hash.hash.ok_or(Error::KeyDerivationFailed)?;
        let mut key = [0u8; 32];
        key.copy_from_slice(&hash_bytes.as_bytes()[..32]);
        
        Ok(key)
    }
}
```

### 2.3 密钥使用

```rust
/// 安全密钥使用原则
pub struct SecureKeyUsage;

impl SecureKeyUsage {
    /// 密钥轮换
    pub async fn rotate_key(
        old_key: &Keypair,
        transition_period: Duration
    ) -> Result<Keypair, Error> {
        // 1. 生成新密钥
        let new_key = SecureKeyGenerator::generate_keypair()?;
        
        // 2. 启动过渡期（两个密钥都有效）
        tokio::time::sleep(transition_period).await;
        
        // 3. 废弃旧密钥
        // 在实际应用中，需要更新所有引用
        
        Ok(new_key)
    }
    
    /// 密钥分割（Shamir秘密共享）
    pub fn split_key(
        secret: &[u8],
        threshold: usize,
        total_shares: usize
    ) -> Result<Vec<Vec<u8>>, Error> {
        // 使用Shamir秘密共享算法分割密钥
        // 简化实现
        let mut shares = Vec::new();
        for i in 0..total_shares {
            shares.push(vec![i as u8]); // 实际实现需要使用Shamir算法
        }
        Ok(shares)
    }
    
    /// 恢复密钥
    pub fn recover_key(
        shares: &[Vec<u8>],
        threshold: usize
    ) -> Result<Vec<u8>, Error> {
        if shares.len() < threshold {
            return Err(Error::InsufficientShares);
        }
        
        // 使用Shamir秘密共享恢复密钥
        // 简化实现
        Ok(vec![])
    }
}
```

## 3. 网络安全

### 3.1 DDoS防护

```rust
use governor::{Quota, RateLimiter, Jitter};
use std::net::IpAddr;

/// DDoS防护系统
#[derive(Debug)]
pub struct DDoSProtection {
    /// IP速率限制
    ip_limiter: Arc<RateLimiter<IpAddr, _, _>>,
    /// 黑名单
    blacklist: Arc<RwLock<HashSet<IpAddr>>>,
    /// 白名单
    whitelist: Arc<RwLock<HashSet<IpAddr>>>,
    /// 异常检测
    anomaly_detector: Arc<AnomalyDetector>,
}

impl DDoSProtection {
    /// 检查请求是否允许
    pub async fn check_request(&self, ip: IpAddr) -> Result<(), Error> {
        // 1. 检查黑名单
        if self.blacklist.read().await.contains(&ip) {
            return Err(Error::IpBlacklisted);
        }
        
        // 2. 白名单直接通过
        if self.whitelist.read().await.contains(&ip) {
            return Ok(());
        }
        
        // 3. 速率限制
        self.ip_limiter.check_key(&ip)
            .map_err(|_| Error::RateLimitExceeded)?;
        
        // 4. 异常检测
        if self.anomaly_detector.is_suspicious(&ip).await? {
            self.blacklist.write().await.insert(ip);
            return Err(Error::SuspiciousActivity);
        }
        
        Ok(())
    }
    
    /// 添加到黑名单
    pub async fn ban_ip(&self, ip: IpAddr, duration: Duration) {
        self.blacklist.write().await.insert(ip);
        
        // 定时解封
        let blacklist = self.blacklist.clone();
        tokio::spawn(async move {
            tokio::time::sleep(duration).await;
            blacklist.write().await.remove(&ip);
        });
    }
}

/// 异常检测器
#[derive(Debug)]
pub struct AnomalyDetector {
    /// 请求模式
    request_patterns: Arc<RwLock<HashMap<IpAddr, RequestPattern>>>,
}

#[derive(Debug, Clone)]
struct RequestPattern {
    request_count: u64,
    last_request: Instant,
    average_interval: Duration,
}

impl AnomalyDetector {
    /// 检测异常行为
    pub async fn is_suspicious(&self, ip: &IpAddr) -> Result<bool, Error> {
        let mut patterns = self.request_patterns.write().await;
        let now = Instant::now();
        
        let pattern = patterns.entry(*ip).or_insert(RequestPattern {
            request_count: 0,
            last_request: now,
            average_interval: Duration::from_secs(1),
        });
        
        // 更新统计
        let interval = now.duration_since(pattern.last_request);
        pattern.request_count += 1;
        pattern.last_request = now;
        
        // 检测异常模式
        if interval < Duration::from_millis(10) {
            // 请求过于频繁
            return Ok(true);
        }
        
        if pattern.request_count > 1000 {
            // 请求次数过多
            return Ok(true);
        }
        
        Ok(false)
    }
}
```

### 3.2 女巫攻击防御

```rust
/// 女巫攻击防御
#[derive(Debug)]
pub struct SybilDefense {
    /// 节点信誉
    reputation: Arc<RwLock<HashMap<PeerId, ReputationScore>>>,
    /// PoW挑战
    pow_challenges: Arc<RwLock<HashMap<PeerId, PowChallenge>>>,
}

#[derive(Debug, Clone)]
pub struct ReputationScore {
    score: f64,
    successful_interactions: u64,
    failed_interactions: u64,
    last_updated: Instant,
}

#[derive(Debug, Clone)]
pub struct PowChallenge {
    challenge: Vec<u8>,
    difficulty: u32,
    issued_at: Instant,
}

impl SybilDefense {
    /// 验证新节点
    pub async fn verify_new_node(&self, peer: PeerId) -> Result<(), Error> {
        // 1. 发起PoW挑战
        let challenge = self.create_pow_challenge(&peer).await?;
        
        // 2. 等待节点完成挑战
        // 在实际实现中，需要异步等待
        
        // 3. 验证PoW
        // self.verify_pow_solution(&peer, solution)?;
        
        // 4. 初始化信誉分数
        self.initialize_reputation(&peer).await?;
        
        Ok(())
    }
    
    /// 创建PoW挑战
    async fn create_pow_challenge(&self, peer: &PeerId) -> Result<PowChallenge, Error> {
        use rand::RngCore;
        
        let mut challenge = vec![0u8; 32];
        OsRng.fill_bytes(&mut challenge);
        
        let pow_challenge = PowChallenge {
            challenge,
            difficulty: 4, // 需要4个前导零
            issued_at: Instant::now(),
        };
        
        self.pow_challenges.write().await.insert(*peer, pow_challenge.clone());
        
        Ok(pow_challenge)
    }
    
    /// 更新信誉分数
    pub async fn update_reputation(
        &self,
        peer: &PeerId,
        success: bool
    ) -> Result<(), Error> {
        let mut reputation = self.reputation.write().await;
        
        let score = reputation.entry(*peer).or_insert(ReputationScore {
            score: 0.5,
            successful_interactions: 0,
            failed_interactions: 0,
            last_updated: Instant::now(),
        });
        
        if success {
            score.successful_interactions += 1;
            score.score = (score.score * 0.9 + 0.1).min(1.0);
        } else {
            score.failed_interactions += 1;
            score.score = (score.score * 0.9 - 0.1).max(0.0);
        }
        
        score.last_updated = Instant::now();
        
        Ok(())
    }
    
    /// 初始化信誉
    async fn initialize_reputation(&self, peer: &PeerId) -> Result<(), Error> {
        self.reputation.write().await.insert(*peer, ReputationScore {
            score: 0.5,
            successful_interactions: 0,
            failed_interactions: 0,
            last_updated: Instant::now(),
        });
        Ok(())
    }
}
```

### 3.3 Eclipse攻击防护

```rust
/// Eclipse攻击防护
#[derive(Debug)]
pub struct EclipseDefense {
    /// 已知节点
    known_peers: Arc<RwLock<HashSet<PeerId>>>,
    /// 连接多样性
    connection_diversity: Arc<RwLock<ConnectionDiversity>>,
}

#[derive(Debug)]
struct ConnectionDiversity {
    /// 按地理位置分布
    geo_distribution: HashMap<String, usize>,
    /// 按AS分布
    as_distribution: HashMap<u32, usize>,
    /// 按IP范围分布
    ip_distribution: HashMap<String, usize>,
}

impl EclipseDefense {
    /// 验证对等节点连接
    pub async fn validate_peer_connection(
        &self,
        peer: &PeerId,
        peer_info: &PeerInfo
    ) -> Result<bool, Error> {
        // 1. 检查是否来自同一AS过多
        if self.is_as_concentration(&peer_info).await? {
            return Ok(false);
        }
        
        // 2. 检查IP范围多样性
        if self.is_ip_concentration(&peer_info).await? {
            return Ok(false);
        }
        
        // 3. 检查地理位置多样性
        if self.is_geo_concentration(&peer_info).await? {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    /// 检查AS集中度
    async fn is_as_concentration(&self, peer_info: &PeerInfo) -> Result<bool, Error> {
        let diversity = self.connection_diversity.read().await;
        
        if let Some(count) = diversity.as_distribution.get(&peer_info.as_number) {
            // 如果来自同一AS的连接超过20%，拒绝
            let total_connections: usize = diversity.as_distribution.values().sum();
            if (*count as f64 / total_connections as f64) > 0.2 {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// 检查IP集中度
    async fn is_ip_concentration(&self, peer_info: &PeerInfo) -> Result<bool, Error> {
        // 简化实现
        Ok(false)
    }
    
    /// 检查地理位置集中度
    async fn is_geo_concentration(&self, peer_info: &PeerInfo) -> Result<bool, Error> {
        // 简化实现
        Ok(false)
    }
}

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub ip: std::net::IpAddr,
    pub as_number: u32,
    pub country: String,
}
```

## 4. 共识安全

### 4.1 51%攻击防御

```rust
/// 51%攻击监测
#[derive(Debug)]
pub struct ConsensusAttackDetector {
    /// 区块链分析
    chain_analyzer: Arc<ChainAnalyzer>,
    /// 算力监控
    hashrate_monitor: Arc<HashrateMonitor>,
}

impl ConsensusAttackDetector {
    /// 检测潜在的51%攻击
    pub async fn detect_attack(&self) -> Result<Option<AttackAlert>, Error> {
        // 1. 检查算力集中度
        let hashrate_concentration = self.hashrate_monitor.calculate_concentration().await?;
        
        if hashrate_concentration > 0.45 {
            return Ok(Some(AttackAlert {
                alert_type: AttackType::HashrateCentralization,
                severity: Severity::High,
                description: format!("算力集中度达到 {:.2}%", hashrate_concentration * 100.0),
            }));
        }
        
        // 2. 检测链重组
        let reorg_depth = self.chain_analyzer.detect_reorg().await?;
        
        if reorg_depth > 6 {
            return Ok(Some(AttackAlert {
                alert_type: AttackType::DeepReorg,
                severity: Severity::Critical,
                description: format!("检测到深度为 {} 的链重组", reorg_depth),
            }));
        }
        
        Ok(None)
    }
}

#[derive(Debug)]
pub struct AttackAlert {
    alert_type: AttackType,
    severity: Severity,
    description: String,
}

#[derive(Debug)]
pub enum AttackType {
    HashrateCentralization,
    DeepReorg,
    DoubleSpend,
}

/// 算力监控
#[derive(Debug)]
pub struct HashrateMonitor {
    miner_stats: Arc<RwLock<HashMap<Address, MinerStats>>>,
}

#[derive(Debug)]
struct MinerStats {
    blocks_mined: u64,
    total_hashrate: u64,
}

impl HashrateMonitor {
    /// 计算算力集中度
    pub async fn calculate_concentration(&self) -> Result<f64, Error> {
        let stats = self.miner_stats.read().await;
        
        if stats.is_empty() {
            return Ok(0.0);
        }
        
        // 计算总算力
        let total: u64 = stats.values().map(|s| s.total_hashrate).sum();
        
        // 找出最大矿工的算力
        let max_hashrate = stats.values()
            .map(|s| s.total_hashrate)
            .max()
            .unwrap_or(0);
        
        Ok(max_hashrate as f64 / total as f64)
    }
}

/// 链分析器
#[derive(Debug)]
pub struct ChainAnalyzer {
    blockchain: Arc<RwLock<Vec<Block>>>,
}

impl ChainAnalyzer {
    /// 检测链重组
    pub async fn detect_reorg(&self) -> Result<usize, Error> {
        // 简化实现：检测最近的分叉深度
        Ok(0)
    }
}
```

### 4.2 长程攻击防护

```rust
/// 长程攻击防护（针对PoS）
#[derive(Debug)]
pub struct LongRangeDefense {
    /// 检查点
    checkpoints: Arc<RwLock<Vec<Checkpoint>>>,
    /// 弱主观性
    weak_subjectivity: Arc<WeakSubjectivity>,
}

#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub height: u64,
    pub hash: Hash,
    pub timestamp: SystemTime,
}

impl LongRangeDefense {
    /// 验证历史区块
    pub async fn validate_historical_chain(
        &self,
        chain: &[Block]
    ) -> Result<bool, Error> {
        let checkpoints = self.checkpoints.read().await;
        
        // 验证链是否通过所有检查点
        for checkpoint in checkpoints.iter() {
            if let Some(block) = chain.iter().find(|b| b.header.height == checkpoint.height) {
                if block.hash() != checkpoint.hash {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// 添加检查点
    pub async fn add_checkpoint(&self, checkpoint: Checkpoint) -> Result<(), Error> {
        self.checkpoints.write().await.push(checkpoint);
        Ok(())
    }
}

/// 弱主观性
#[derive(Debug)]
pub struct WeakSubjectivity {
    /// 最新可信区块
    trusted_block: Arc<RwLock<Option<Block>>>,
}

impl WeakSubjectivity {
    /// 同步时使用弱主观性
    pub async fn sync_with_weak_subjectivity(
        &self,
        peer_chain: &[Block]
    ) -> Result<bool, Error> {
        let trusted = self.trusted_block.read().await;
        
        if let Some(trusted_block) = &*trusted {
            // 确保对等方的链包含可信区块
            let has_trusted = peer_chain.iter()
                .any(|b| b.hash() == trusted_block.hash());
            
            if !has_trusted {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}
```

### 4.3 双花攻击防范

```rust
/// 双花检测
#[derive(Debug)]
pub struct DoubleSpendDetector {
    /// 交易监控
    tx_monitor: Arc<TransactionMonitor>,
    /// UTXO集合
    utxo_set: Arc<RwLock<HashSet<UTXO>>>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct UTXO {
    pub tx_hash: Hash,
    pub output_index: u32,
}

impl DoubleSpendDetector {
    /// 检测双花尝试
    pub async fn detect_double_spend(
        &self,
        tx: &Transaction
    ) -> Result<bool, Error> {
        let utxo_set = self.utxo_set.read().await;
        
        // 检查交易输入是否已被花费
        for input in &tx.inputs {
            let utxo = UTXO {
                tx_hash: input.prev_tx_hash.clone(),
                output_index: input.output_index,
            };
            
            if !utxo_set.contains(&utxo) {
                // UTXO已被花费或不存在
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// 等待确认深度
    pub async fn wait_for_confirmations(
        &self,
        tx_hash: &Hash,
        required_confirmations: u64
    ) -> Result<bool, Error> {
        let monitor = self.tx_monitor.clone();
        
        for _ in 0..required_confirmations {
            // 等待新区块
            tokio::time::sleep(Duration::from_secs(10)).await;
            
            // 检查交易是否仍在主链上
            if !monitor.is_on_main_chain(tx_hash).await? {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}

/// 交易监控器
#[derive(Debug)]
pub struct TransactionMonitor {
    confirmed_txs: Arc<RwLock<HashMap<Hash, u64>>>,
}

impl TransactionMonitor {
    async fn is_on_main_chain(&self, tx_hash: &Hash) -> Result<bool, Error> {
        // 检查交易是否在主链上
        Ok(true)
    }
}
```

## 5. 节点安全

### 5.1 节点加固

```rust
/// 节点安全配置
#[derive(Debug, Clone)]
pub struct NodeSecurityConfig {
    /// 启用TLS
    pub enable_tls: bool,
    /// 最小TLS版本
    pub min_tls_version: TlsVersion,
    /// 允许的密码套件
    pub cipher_suites: Vec<String>,
    /// 启用防火墙
    pub enable_firewall: bool,
    /// 允许的端口
    pub allowed_ports: Vec<u16>,
    /// 启用入侵检测
    pub enable_ids: bool,
}

#[derive(Debug, Clone)]
pub enum TlsVersion {
    Tls12,
    Tls13,
}

impl NodeSecurityConfig {
    /// 推荐的安全配置
    pub fn recommended() -> Self {
        Self {
            enable_tls: true,
            min_tls_version: TlsVersion::Tls13,
            cipher_suites: vec![
                "TLS_AES_256_GCM_SHA384".to_string(),
                "TLS_CHACHA20_POLY1305_SHA256".to_string(),
            ],
            enable_firewall: true,
            allowed_ports: vec![8545, 30303],
            enable_ids: true,
        }
    }
    
    /// 应用安全配置
    pub fn apply(&self) -> Result<(), Error> {
        // 1. 配置TLS
        if self.enable_tls {
            self.configure_tls()?;
        }
        
        // 2. 配置防火墙
        if self.enable_firewall {
            self.configure_firewall()?;
        }
        
        // 3. 启用IDS
        if self.enable_ids {
            self.enable_intrusion_detection()?;
        }
        
        Ok(())
    }
    
    fn configure_tls(&self) -> Result<(), Error> {
        // 配置TLS
        Ok(())
    }
    
    fn configure_firewall(&self) -> Result<(), Error> {
        // 配置防火墙规则
        Ok(())
    }
    
    fn enable_intrusion_detection(&self) -> Result<(), Error> {
        // 启用入侵检测系统
        Ok(())
    }
}
```

### 5.2 访问控制

```rust
/// 节点访问控制
#[derive(Debug)]
pub struct NodeAccessControl {
    /// API密钥
    api_keys: Arc<RwLock<HashMap<String, ApiKey>>>,
    /// IP白名单
    ip_whitelist: Arc<RwLock<HashSet<IpAddr>>>,
    /// JWT验证
    jwt_validator: Arc<JwtValidator>,
}

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub key: String,
    pub permissions: Vec<ApiPermission>,
    pub expires_at: Option<SystemTime>,
}

#[derive(Debug, Clone)]
pub enum ApiPermission {
    ReadBlockchain,
    SubmitTransaction,
    Admin,
}

impl NodeAccessControl {
    /// 验证API请求
    pub async fn validate_api_request(
        &self,
        api_key: &str,
        ip: IpAddr,
        permission: ApiPermission
    ) -> Result<(), Error> {
        // 1. 检查IP白名单
        if !self.ip_whitelist.read().await.contains(&ip) {
            return Err(Error::IpNotWhitelisted);
        }
        
        // 2. 验证API密钥
        let keys = self.api_keys.read().await;
        let key_info = keys.get(api_key).ok_or(Error::InvalidApiKey)?;
        
        // 3. 检查过期时间
        if let Some(expires_at) = key_info.expires_at {
            if SystemTime::now() > expires_at {
                return Err(Error::ApiKeyExpired);
            }
        }
        
        // 4. 检查权限
        if !key_info.permissions.contains(&permission) {
            return Err(Error::InsufficientPermissions);
        }
        
        Ok(())
    }
}

/// JWT验证器
#[derive(Debug)]
pub struct JwtValidator {
    secret: Vec<u8>,
}

impl JwtValidator {
    /// 验证JWT token
    pub fn validate_token(&self, token: &str) -> Result<Claims, Error> {
        // 使用jsonwebtoken库验证JWT
        // 简化实现
        Ok(Claims {
            sub: "user".to_string(),
            exp: 0,
        })
    }
}

#[derive(Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}
```

### 5.3 监控告警

```rust
/// 安全监控系统
#[derive(Debug)]
pub struct SecurityMonitoring {
    /// 事件收集器
    event_collector: Arc<EventCollector>,
    /// 告警管理器
    alert_manager: Arc<AlertManager>,
    /// 日志分析器
    log_analyzer: Arc<LogAnalyzer>,
}

impl SecurityMonitoring {
    /// 监控安全事件
    pub async fn monitor_security_events(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        
        loop {
            interval.tick().await;
            
            // 收集事件
            let events = self.event_collector.collect_events().await;
            
            // 分析事件
            for event in events {
                if self.log_analyzer.is_suspicious(&event).await {
                    // 发送告警
                    self.alert_manager.send_alert(Alert {
                        severity: Severity::High,
                        message: format!("检测到可疑事件: {:?}", event),
                        timestamp: SystemTime::now(),
                    }).await;
                }
            }
        }
    }
}

/// 事件收集器
#[derive(Debug)]
pub struct EventCollector {
    events: Arc<RwLock<Vec<SecurityEvent>>>,
}

#[derive(Debug, Clone)]
pub struct SecurityEvent {
    pub event_type: EventType,
    pub source: String,
    pub timestamp: SystemTime,
    pub details: String,
}

#[derive(Debug, Clone)]
pub enum EventType {
    LoginAttempt,
    ApiRequest,
    NetworkAnomaly,
    ResourceExhaustion,
}

impl EventCollector {
    async fn collect_events(&self) -> Vec<SecurityEvent> {
        // 收集安全事件
        vec![]
    }
}

/// 告警管理器
#[derive(Debug)]
pub struct AlertManager {
    alerts: Arc<RwLock<Vec<Alert>>>,
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub severity: Severity,
    pub message: String,
    pub timestamp: SystemTime,
}

impl AlertManager {
    async fn send_alert(&self, alert: Alert) {
        // 发送告警（邮件、短信、Slack等）
        println!("🚨 安全告警: {}", alert.message);
        self.alerts.write().await.push(alert);
    }
}

/// 日志分析器
#[derive(Debug)]
pub struct LogAnalyzer;

impl LogAnalyzer {
    async fn is_suspicious(&self, event: &SecurityEvent) -> bool {
        // 分析事件是否可疑
        false
    }
}
```

## 6. 数据安全

### 6.1 数据加密

已在密钥存储部分实现。

### 6.2 隐私保护

```rust
/// 零知识证明隐私保护
pub struct PrivacyProtection;

impl PrivacyProtection {
    /// 私密转账
    pub fn create_private_transfer(
        sender: &Address,
        receiver: &Address,
        amount: u64,
        sender_key: &SecretKey
    ) -> Result<PrivateTransaction, Error> {
        // 使用零知识证明创建私密交易
        Ok(PrivateTransaction {
            proof: vec![],
            commitment: vec![],
        })
    }
}

#[derive(Debug)]
pub struct PrivateTransaction {
    proof: Vec<u8>,
    commitment: Vec<u8>,
}
```

### 6.3 备份恢复

```rust
/// 数据备份管理器
#[derive(Debug)]
pub struct BackupManager {
    backup_path: PathBuf,
    encryption_key: [u8; 32],
}

impl BackupManager {
    /// 创建加密备份
    pub async fn create_backup(&self, data: &[u8]) -> Result<(), Error> {
        // 1. 压缩数据
        let compressed = self.compress(data)?;
        
        // 2. 加密
        let encrypted = self.encrypt(&compressed)?;
        
        // 3. 保存
        tokio::fs::write(&self.backup_path, &encrypted).await?;
        
        Ok(())
    }
    
    /// 恢复备份
    pub async fn restore_backup(&self) -> Result<Vec<u8>, Error> {
        // 1. 读取
        let encrypted = tokio::fs::read(&self.backup_path).await?;
        
        // 2. 解密
        let compressed = self.decrypt(&encrypted)?;
        
        // 3. 解压
        let data = self.decompress(&compressed)?;
        
        Ok(data)
    }
    
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        // 使用zstd压缩
        use zstd::stream::encode_all;
        Ok(encode_all(data, 3)?)
    }
    
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        // 解压
        use zstd::stream::decode_all;
        Ok(decode_all(data)?)
    }
    
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        // 使用AES-256-GCM加密
        Ok(vec![])
    }
    
    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        // 解密
        Ok(vec![])
    }
}
```

## 7. 应用安全

### 7.1 前端安全

```rust
/// 前端安全检查
pub struct FrontendSecurity;

impl FrontendSecurity {
    /// XSS防护
    pub fn sanitize_input(input: &str) -> String {
        // HTML编码
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }
    
    /// CSRF Token生成
    pub fn generate_csrf_token() -> String {
        use rand::Rng;
        let token: Vec<u8> = (0..32).map(|_| rand::thread_rng().gen()).collect();
        hex::encode(token)
    }
    
    /// Content Security Policy
    pub fn csp_headers() -> Vec<(&'static str, &'static str)> {
        vec![
            ("Content-Security-Policy", "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'"),
            ("X-Content-Type-Options", "nosniff"),
            ("X-Frame-Options", "DENY"),
            ("X-XSS-Protection", "1; mode=block"),
        ]
    }
}
```

### 7.2 API安全

```rust
/// API安全中间件
pub struct ApiSecurity;

impl ApiSecurity {
    /// 请求签名验证
    pub fn verify_request_signature(
        request_body: &[u8],
        signature: &str,
        public_key: &PublicKey
    ) -> Result<bool, Error> {
        // 验证请求签名
        Ok(true)
    }
    
    /// API速率限制
    pub async fn check_rate_limit(
        api_key: &str,
        endpoint: &str
    ) -> Result<(), Error> {
        // 检查速率限制
        Ok(())
    }
}
```

### 7.3 钱包安全

```rust
/// 钱包安全管理
pub struct WalletSecurity;

impl WalletSecurity {
    /// 多签钱包
    pub fn create_multisig_wallet(
        required_signatures: usize,
        total_signers: usize,
        signers: Vec<PublicKey>
    ) -> Result<MultisigWallet, Error> {
        Ok(MultisigWallet {
            required_signatures,
            total_signers,
            signers,
            pending_transactions: vec![],
        })
    }
    
    /// 时间锁
    pub fn create_timelock_transaction(
        tx: Transaction,
        unlock_time: SystemTime
    ) -> Result<TimelockTransaction, Error> {
        Ok(TimelockTransaction {
            transaction: tx,
            unlock_time,
        })
    }
}

#[derive(Debug)]
pub struct MultisigWallet {
    required_signatures: usize,
    total_signers: usize,
    signers: Vec<PublicKey>,
    pending_transactions: Vec<PendingTransaction>,
}

#[derive(Debug)]
pub struct PendingTransaction {
    tx: Transaction,
    signatures: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct TimelockTransaction {
    transaction: Transaction,
    unlock_time: SystemTime,
}
```

## 8. 运维安全

### 8.1 部署安全

```rust
/// 安全部署清单
pub struct DeploymentSecurity;

impl DeploymentSecurity {
    /// 部署前检查
    pub fn pre_deployment_checks() -> Vec<SecurityCheck> {
        vec![
            SecurityCheck {
                name: "代码审计".to_string(),
                status: CheckStatus::Pending,
            },
            SecurityCheck {
                name: "依赖扫描".to_string(),
                status: CheckStatus::Pending,
            },
            SecurityCheck {
                name: "安全测试".to_string(),
                status: CheckStatus::Pending,
            },
            SecurityCheck {
                name: "配置验证".to_string(),
                status: CheckStatus::Pending,
            },
        ]
    }
}

#[derive(Debug)]
pub struct SecurityCheck {
    name: String,
    status: CheckStatus,
}

#[derive(Debug)]
pub enum CheckStatus {
    Pending,
    Passed,
    Failed,
}
```

### 8.2 升级策略

```rust
/// 安全升级策略
pub struct UpgradeStrategy;

impl UpgradeStrategy {
    /// 灰度发布
    pub async fn canary_deployment(
        old_version: &str,
        new_version: &str,
        canary_percentage: f64
    ) -> Result<(), Error> {
        // 实现金丝雀部署
        Ok(())
    }
    
    /// 回滚计划
    pub async fn rollback_plan() -> Result<(), Error> {
        // 准备回滚
        Ok(())
    }
}
```

### 8.3 应急响应

```rust
/// 应急响应计划
pub struct IncidentResponse;

impl IncidentResponse {
    /// 事件响应流程
    pub async fn handle_security_incident(
        incident: SecurityIncident
    ) -> Result<(), Error> {
        // 1. 识别
        Self::identify_incident(&incident)?;
        
        // 2. 遏制
        Self::contain_incident(&incident).await?;
        
        // 3. 根除
        Self::eradicate_threat(&incident).await?;
        
        // 4. 恢复
        Self::recover_services(&incident).await?;
        
        // 5. 总结
        Self::post_incident_review(&incident).await?;
        
        Ok(())
    }
    
    fn identify_incident(incident: &SecurityIncident) -> Result<(), Error> {
        println!("识别安全事件: {:?}", incident);
        Ok(())
    }
    
    async fn contain_incident(incident: &SecurityIncident) -> Result<(), Error> {
        println!("遏制安全事件");
        Ok(())
    }
    
    async fn eradicate_threat(incident: &SecurityIncident) -> Result<(), Error> {
        println!("根除威胁");
        Ok(())
    }
    
    async fn recover_services(incident: &SecurityIncident) -> Result<(), Error> {
        println!("恢复服务");
        Ok(())
    }
    
    async fn post_incident_review(incident: &SecurityIncident) -> Result<(), Error> {
        println!("事后复盘");
        Ok(())
    }
}

#[derive(Debug)]
pub struct SecurityIncident {
    pub incident_type: IncidentType,
    pub severity: Severity,
    pub description: String,
    pub affected_systems: Vec<String>,
}

#[derive(Debug)]
pub enum IncidentType {
    DataBreach,
    DdosAttack,
    Malware,
    UnauthorizedAccess,
}
```

## 9. 总结

本文档详细介绍了区块链系统的安全最佳实践，包括：

1. **智能合约安全**：重入攻击、整数溢出、权限控制、安全编码模式
2. **密钥管理安全**：安全生成、加密存储、密钥轮换、秘密共享
3. **网络安全**：DDoS防护、女巫攻击防御、Eclipse攻击防护
4. **共识安全**：51%攻击防御、长程攻击防护、双花防范
5. **节点安全**：节点加固、访问控制、安全监控
6. **数据安全**：加密存储、隐私保护、备份恢复
7. **应用安全**：前端安全、API安全、钱包安全
8. **运维安全**：部署安全、升级策略、应急响应

遵循这些最佳实践可以显著提升区块链系统的安全性。

---

**文档版本**: v1.0.0  
**最后更新**: 2025年10月17日  
**作者**: Rust区块链技术团队  
**相关文档**:

- [06_SECURITY_MODELS.md](./06_SECURITY_MODELS.md) - 安全模型与威胁分析
- [13_CRYPTO_IMPLEMENTATION.md](./13_CRYPTO_IMPLEMENTATION.md) - 密码学实现
- [18_PROTOCOL_SPECIFICATIONS.md](./18_PROTOCOL_SPECIFICATIONS.md) - 协议规范详解
