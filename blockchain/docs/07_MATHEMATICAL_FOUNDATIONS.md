# 数学基础与算法分析

## 📋 目录

- [数学基础与算法分析](#数学基础与算法分析)
  - [📋 目录](#-目录)
  - [1. 数学基础](#1-数学基础)
    - [1.1 集合论](#11-集合论)
    - [1.2 关系与函数](#12-关系与函数)
  - [2. 数论基础](#2-数论基础)
    - [2.1 整数运算](#21-整数运算)
    - [2.2 素性测试](#22-素性测试)
  - [3. 抽象代数](#3-抽象代数)
    - [3.1 群论](#31-群论)
    - [3.2 环与域](#32-环与域)
  - [4. 概率论与统计学](#4-概率论与统计学)
    - [4.1 概率分布](#41-概率分布)
    - [4.2 统计检验](#42-统计检验)
  - [5. 图论](#5-图论)
    - [5.1 图的基本概念](#51-图的基本概念)
    - [5.2 图的算法](#52-图的算法)
  - [6. 复杂度理论](#6-复杂度理论)
    - [6.1 时间复杂度分析](#61-时间复杂度分析)
  - [7. 密码学数学基础](#7-密码学数学基础)
    - [7.1 椭圆曲线](#71-椭圆曲线)
    - [7.2 离散对数](#72-离散对数)
  - [8. 区块链数学原理](#8-区块链数学原理)
    - [8.1 哈希函数数学性质](#81-哈希函数数学性质)
    - [8.2 共识算法数学分析](#82-共识算法数学分析)
  - [9. 总结](#9-总结)

## 1. 数学基础

### 1.1 集合论

```rust
// 集合论基础实现
use std::collections::HashSet;
use std::hash::Hash;

// 集合定义
#[derive(Debug, Clone, PartialEq)]
struct Set<T: Hash + Eq + Clone> {
    elements: HashSet<T>,
}

impl<T: Hash + Eq + Clone> Set<T> {
    fn new() -> Self {
        Self {
            elements: HashSet::new(),
        }
    }
    
    fn from_vec(elements: Vec<T>) -> Self {
        Self {
            elements: elements.into_iter().collect(),
        }
    }
    
    // 并集
    fn union(&self, other: &Set<T>) -> Set<T> {
        let mut result = self.elements.clone();
        result.extend(other.elements.iter().cloned());
        Set { elements: result }
    }
    
    // 交集
    fn intersection(&self, other: &Set<T>) -> Set<T> {
        let result: HashSet<T> = self.elements
            .intersection(&other.elements)
            .cloned()
            .collect();
        Set { elements: result }
    }
    
    // 差集
    fn difference(&self, other: &Set<T>) -> Set<T> {
        let result: HashSet<T> = self.elements
            .difference(&other.elements)
            .cloned()
            .collect();
        Set { elements: result }
    }
    
    // 子集检查
    fn is_subset(&self, other: &Set<T>) -> bool {
        self.elements.is_subset(&other.elements)
    }
    
    // 幂集
    fn power_set(&self) -> Set<Set<T>> {
        let elements: Vec<T> = self.elements.iter().cloned().collect();
        let mut power_set = HashSet::new();
        
        // 生成所有子集
        for i in 0..(1 << elements.len()) {
            let mut subset = HashSet::new();
            for j in 0..elements.len() {
                if (i >> j) & 1 == 1 {
                    subset.insert(elements[j].clone());
                }
            }
            power_set.insert(Set { elements: subset });
        }
        
        Set { elements: power_set }
    }
}
```

### 1.2 关系与函数

```rust
// 关系定义
#[derive(Debug, Clone, PartialEq)]
struct Relation<T: Hash + Eq + Clone> {
    pairs: HashSet<(T, T)>,
}

impl<T: Hash + Eq + Clone> Relation<T> {
    fn new() -> Self {
        Self {
            pairs: HashSet::new(),
        }
    }
    
    // 添加关系对
    fn add_pair(&mut self, a: T, b: T) {
        self.pairs.insert((a, b));
    }
    
    // 自反性检查
    fn is_reflexive(&self, domain: &Set<T>) -> bool {
        for element in &domain.elements {
            if !self.pairs.contains(&(element.clone(), element.clone())) {
                return false;
            }
        }
        true
    }
    
    // 对称性检查
    fn is_symmetric(&self) -> bool {
        for (a, b) in &self.pairs {
            if !self.pairs.contains(&(b.clone(), a.clone())) {
                return false;
            }
        }
        true
    }
    
    // 传递性检查
    fn is_transitive(&self) -> bool {
        for (a, b) in &self.pairs {
            for (c, d) in &self.pairs {
                if b == c && !self.pairs.contains(&(a.clone(), d.clone())) {
                    return false;
                }
            }
        }
        true
    }
    
    // 等价关系检查
    fn is_equivalence(&self, domain: &Set<T>) -> bool {
        self.is_reflexive(domain) && self.is_symmetric() && self.is_transitive()
    }
}

// 函数定义
#[derive(Debug, Clone, PartialEq)]
struct Function<D: Hash + Eq + Clone, R: Hash + Eq + Clone> {
    mapping: HashMap<D, R>,
    domain: Set<D>,
    codomain: Set<R>,
}

impl<D: Hash + Eq + Clone, R: Hash + Eq + Clone> Function<D, R> {
    fn new(domain: Set<D>, codomain: Set<R>) -> Self {
        Self {
            mapping: HashMap::new(),
            domain,
            codomain,
        }
    }
    
    // 定义函数值
    fn define(&mut self, input: D, output: R) -> Result<(), FunctionError> {
        if !self.domain.elements.contains(&input) {
            return Err(FunctionError::InputNotInDomain);
        }
        if !self.codomain.elements.contains(&output) {
            return Err(FunctionError::OutputNotInCodomain);
        }
        self.mapping.insert(input, output);
        Ok(())
    }
    
    // 计算函数值
    fn apply(&self, input: &D) -> Option<&R> {
        self.mapping.get(input)
    }
    
    // 单射检查
    fn is_injective(&self) -> bool {
        let mut seen = HashSet::new();
        for output in self.mapping.values() {
            if seen.contains(output) {
                return false;
            }
            seen.insert(output);
        }
        true
    }
    
    // 满射检查
    fn is_surjective(&self) -> bool {
        let range: HashSet<&R> = self.mapping.values().collect();
        range.is_subset(&self.codomain.elements.iter().collect())
    }
    
    // 双射检查
    fn is_bijective(&self) -> bool {
        self.is_injective() && self.is_surjective()
    }
}
```

## 2. 数论基础

### 2.1 整数运算

```rust
// 数论基础实现
use num_bigint::BigInt;
use num_traits::{Zero, One};

// 最大公约数
fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    if b.is_zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

// 扩展欧几里得算法
fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        (a.clone(), BigInt::one(), BigInt::zero())
    } else {
        let (g, x1, y1) = extended_gcd(b, &(a % b));
        let x = y1.clone();
        let y = x1 - (a / b) * y1;
        (g, x, y)
    }
}

// 模逆
fn mod_inverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (g, x, _) = extended_gcd(a, m);
    if g == BigInt::one() {
        Some((x % m + m) % m)
    } else {
        None
    }
}

// 中国剩余定理
fn chinese_remainder_theorem(remainders: &[BigInt], moduli: &[BigInt]) -> Option<BigInt> {
    let n: BigInt = moduli.iter().product();
    let mut result = BigInt::zero();
    
    for (i, (a_i, m_i)) in remainders.iter().zip(moduli.iter()).enumerate() {
        let n_i = &n / m_i;
        let inv = mod_inverse(&n_i, m_i)?;
        result += a_i * &n_i * inv;
    }
    
    Some(result % n)
}
```

### 2.2 素性测试

```rust
// 素性测试
struct PrimalityTester;

impl PrimalityTester {
    // 试除法
    fn trial_division(n: &BigInt) -> bool {
        if n <= &BigInt::one() {
            return false;
        }
        if n <= &BigInt::from(3) {
            return true;
        }
        if n % 2 == BigInt::zero() || n % 3 == BigInt::zero() {
            return false;
        }
        
        let mut i = BigInt::from(5);
        while &i * &i <= *n {
            if n % &i == BigInt::zero() || n % (&i + 2) == BigInt::zero() {
                return false;
            }
            i += 6;
        }
        true
    }
    
    // Miller-Rabin素性测试
    fn miller_rabin(n: &BigInt, k: usize) -> bool {
        if n <= &BigInt::one() {
            return false;
        }
        if n <= &BigInt::from(3) {
            return true;
        }
        if n % 2 == BigInt::zero() {
            return false;
        }
        
        // 将 n-1 写成 d * 2^r 的形式
        let mut d = n - 1;
        let mut r = 0;
        while &d % 2 == BigInt::zero() {
            d /= 2;
            r += 1;
        }
        
        for _ in 0..k {
            let a = Self::random_bigint(&BigInt::from(2), &(n - 2));
            let mut x = Self::mod_pow(&a, &d, n);
            
            if x == BigInt::one() || x == n - 1 {
                continue;
            }
            
            let mut composite = true;
            for _ in 0..r - 1 {
                x = (&x * &x) % n;
                if x == n - 1 {
                    composite = false;
                    break;
                }
            }
            
            if composite {
                return false;
            }
        }
        true
    }
    
    // 模幂运算
    fn mod_pow(base: &BigInt, exp: &BigInt, modulus: &BigInt) -> BigInt {
        let mut result = BigInt::one();
        let mut base = base % modulus;
        let mut exp = exp.clone();
        
        while exp > BigInt::zero() {
            if &exp % 2 == BigInt::one() {
                result = (result * &base) % modulus;
            }
            exp /= 2;
            base = (&base * &base) % modulus;
        }
        
        result
    }
    
    // 生成随机大整数
    fn random_bigint(min: &BigInt, max: &BigInt) -> BigInt {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let range = max - min;
        let bytes = range.to_bytes_be().1;
        let mut result = BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes);
        result = result % range + min;
        result
    }
}
```

## 3. 抽象代数

### 3.1 群论

```rust
// 群论实现
use std::collections::HashMap;

// 群定义
trait Group<T: Clone + Eq> {
    fn identity(&self) -> T;
    fn operation(&self, a: &T, b: &T) -> T;
    fn inverse(&self, a: &T) -> T;
    fn is_associative(&self, elements: &[T]) -> bool;
    fn is_commutative(&self, elements: &[T]) -> bool;
}

// 有限群
struct FiniteGroup<T: Clone + Eq + std::hash::Hash> {
    elements: Vec<T>,
    operation_table: HashMap<(T, T), T>,
    identity: T,
    inverses: HashMap<T, T>,
}

impl<T: Clone + Eq + std::hash::Hash> FiniteGroup<T> {
    fn new(elements: Vec<T>, operation_table: HashMap<(T, T), T>, identity: T) -> Self {
        let mut inverses = HashMap::new();
        
        // 计算逆元
        for element in &elements {
            for potential_inverse in &elements {
                if operation_table.get(&(element.clone(), potential_inverse.clone())) == Some(&identity) {
                    inverses.insert(element.clone(), potential_inverse.clone());
                    break;
                }
            }
        }
        
        Self {
            elements,
            operation_table,
            identity,
            inverses,
        }
    }
}

impl<T: Clone + Eq + std::hash::Hash> Group<T> for FiniteGroup<T> {
    fn identity(&self) -> T {
        self.identity.clone()
    }
    
    fn operation(&self, a: &T, b: &T) -> T {
        self.operation_table.get(&(a.clone(), b.clone()))
            .unwrap_or_else(|| panic!("Operation not defined for ({:?}, {:?})", a, b))
            .clone()
    }
    
    fn inverse(&self, a: &T) -> T {
        self.inverses.get(a)
            .unwrap_or_else(|| panic!("Inverse not found for {:?}", a))
            .clone()
    }
    
    fn is_associative(&self, elements: &[T]) -> bool {
        for a in elements {
            for b in elements {
                for c in elements {
                    let ab = self.operation(a, b);
                    let bc = self.operation(b, c);
                    let ab_c = self.operation(&ab, c);
                    let a_bc = self.operation(a, &bc);
                    if ab_c != a_bc {
                        return false;
                    }
                }
            }
        }
        true
    }
    
    fn is_commutative(&self, elements: &[T]) -> bool {
        for a in elements {
            for b in elements {
                if self.operation(a, b) != self.operation(b, a) {
                    return false;
                }
            }
        }
        true
    }
}
```

### 3.2 环与域

```rust
// 环定义
trait Ring<T: Clone + Eq> {
    fn zero(&self) -> T;
    fn one(&self) -> T;
    fn add(&self, a: &T, b: &T) -> T;
    fn multiply(&self, a: &T, b: &T) -> T;
    fn additive_inverse(&self, a: &T) -> T;
}

// 有限域
struct FiniteField {
    p: BigInt,  // 素数
    elements: Vec<BigInt>,
}

impl FiniteField {
    fn new(p: BigInt) -> Self {
        let elements: Vec<BigInt> = (0..p.to_string().parse().unwrap())
            .map(BigInt::from)
            .collect();
        
        Self { p, elements }
    }
    
    // 域运算
    fn add(&self, a: &BigInt, b: &BigInt) -> BigInt {
        (a + b) % &self.p
    }
    
    fn multiply(&self, a: &BigInt, b: &BigInt) -> BigInt {
        (a * b) % &self.p
    }
    
    fn additive_inverse(&self, a: &BigInt) -> BigInt {
        (&self.p - a) % &self.p
    }
    
    fn multiplicative_inverse(&self, a: &BigInt) -> Option<BigInt> {
        if a.is_zero() {
            None
        } else {
            mod_inverse(a, &self.p)
        }
    }
    
    // 域元素阶
    fn order(&self, a: &BigInt) -> BigInt {
        if a.is_zero() {
            return BigInt::zero();
        }
        
        let mut result = BigInt::one();
        let mut power = a.clone();
        
        while power != BigInt::one() {
            power = self.multiply(&power, a);
            result += 1;
            if result > &self.p {
                break;
            }
        }
        
        result
    }
    
    // 本原根
    fn primitive_root(&self) -> Option<BigInt> {
        let phi = &self.p - 1;
        for candidate in &self.elements {
            if !candidate.is_zero() && self.order(candidate) == phi {
                return Some(candidate.clone());
            }
        }
        None
    }
}
```

## 4. 概率论与统计学

### 4.1 概率分布

```rust
// 概率分布实现
use rand::Rng;
use std::f64::consts::PI;

// 概率分布特征
trait ProbabilityDistribution {
    fn sample(&self) -> f64;
    fn pdf(&self, x: f64) -> f64;
    fn cdf(&self, x: f64) -> f64;
    fn mean(&self) -> f64;
    fn variance(&self) -> f64;
}

// 均匀分布
struct UniformDistribution {
    a: f64,
    b: f64,
}

impl UniformDistribution {
    fn new(a: f64, b: f64) -> Self {
        Self { a, b }
    }
}

impl ProbabilityDistribution for UniformDistribution {
    fn sample(&self) -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(self.a..=self.b)
    }
    
    fn pdf(&self, x: f64) -> f64 {
        if x >= self.a && x <= self.b {
            1.0 / (self.b - self.a)
        } else {
            0.0
        }
    }
    
    fn cdf(&self, x: f64) -> f64 {
        if x < self.a {
            0.0
        } else if x > self.b {
            1.0
        } else {
            (x - self.a) / (self.b - self.a)
        }
    }
    
    fn mean(&self) -> f64 {
        (self.a + self.b) / 2.0
    }
    
    fn variance(&self) -> f64 {
        let diff = self.b - self.a;
        diff * diff / 12.0
    }
}

// 正态分布
struct NormalDistribution {
    mu: f64,
    sigma: f64,
}

impl NormalDistribution {
    fn new(mu: f64, sigma: f64) -> Self {
        Self { mu, sigma }
    }
}

impl ProbabilityDistribution for NormalDistribution {
    fn sample(&self) -> f64 {
        // Box-Muller变换
        let mut rng = rand::thread_rng();
        let u1: f64 = rng.gen();
        let u2: f64 = rng.gen();
        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
        self.mu + self.sigma * z0
    }
    
    fn pdf(&self, x: f64) -> f64 {
        let coefficient = 1.0 / (self.sigma * (2.0 * PI).sqrt());
        let exponent = -0.5 * ((x - self.mu) / self.sigma).powi(2);
        coefficient * exponent.exp()
    }
    
    fn cdf(&self, x: f64) -> f64 {
        // 使用误差函数近似
        let z = (x - self.mu) / (self.sigma * 2.0_f64.sqrt());
        0.5 * (1.0 + erf(z))
    }
    
    fn mean(&self) -> f64 {
        self.mu
    }
    
    fn variance(&self) -> f64 {
        self.sigma * self.sigma
    }
}

// 误差函数近似
fn erf(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;
    
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
    
    sign * y
}
```

### 4.2 统计检验

```rust
// 统计检验
struct StatisticalTest;

impl StatisticalTest {
    // 卡方检验
    fn chi_square_test(observed: &[f64], expected: &[f64]) -> (f64, f64) {
        let mut chi_square = 0.0;
        let degrees_of_freedom = observed.len() - 1;
        
        for (obs, exp) in observed.iter().zip(expected.iter()) {
            if *exp > 0.0 {
                chi_square += (obs - exp).powi(2) / exp;
            }
        }
        
        let p_value = Self::chi_square_p_value(chi_square, degrees_of_freedom);
        (chi_square, p_value)
    }
    
    // 卡方分布p值
    fn chi_square_p_value(chi_square: f64, df: usize) -> f64 {
        // 使用不完全伽马函数近似
        Self::incomplete_gamma(df as f64 / 2.0, chi_square / 2.0)
    }
    
    // 不完全伽马函数
    fn incomplete_gamma(a: f64, x: f64) -> f64 {
        if x < 0.0 || a <= 0.0 {
            return 0.0;
        }
        
        if x < a + 1.0 {
            Self::series_incomplete_gamma(a, x)
        } else {
            1.0 - Self::continued_fraction_incomplete_gamma(a, x)
        }
    }
    
    // 级数展开
    fn series_incomplete_gamma(a: f64, x: f64) -> f64 {
        let mut sum = 1.0;
        let mut term = 1.0;
        
        for n in 1..100 {
            term *= x / (a + n as f64 - 1.0);
            sum += term;
            if term.abs() < 1e-10 {
                break;
            }
        }
        
        sum * x.powf(a) * (-x).exp() / Self::gamma(a)
    }
    
    // 连分数展开
    fn continued_fraction_incomplete_gamma(a: f64, x: f64) -> f64 {
        let mut b = x + 1.0 - a;
        let mut c = 1.0 / 1e-30;
        let mut d = 1.0 / b;
        let mut h = d;
        
        for i in 1..100 {
            let an = -i as f64 * (i as f64 - a);
            b += 2.0;
            d = an * d + b;
            if d.abs() < 1e-30 {
                d = 1e-30;
            }
            c = b + an / c;
            if c.abs() < 1e-30 {
                c = 1e-30;
            }
            d = 1.0 / d;
            let del = d * c;
            h *= del;
            if (del - 1.0).abs() < 1e-10 {
                break;
            }
        }
        
        h * x.powf(a) * (-x).exp() / Self::gamma(a)
    }
    
    // 伽马函数
    fn gamma(z: f64) -> f64 {
        if z < 0.5 {
            PI / ((PI * z).sin() * Self::gamma(1.0 - z))
        } else {
            let z = z - 1.0;
            let mut x = 0.99999999999980993;
            let coefficients = [
                676.5203681218851, -1259.1392167224028, 771.32342877765313,
                -176.61502916214059, 12.507343278686905, -0.13857109526572012,
                9.9843695780195716e-6, 1.5056327351493116e-7,
            ];
            
            for (i, &coeff) in coefficients.iter().enumerate() {
                x += coeff / (z + i as f64 + 1.0);
            }
            
            let t = z + coefficients.len() as f64 - 0.5;
            (2.0 * PI).sqrt() * t.powf(z + 0.5) * (-t).exp() * x
        }
    }
}
```

## 5. 图论

### 5.1 图的基本概念

```rust
// 图论实现
use std::collections::{HashMap, HashSet, VecDeque};

// 图定义
#[derive(Debug, Clone)]
struct Graph<T: Clone + Eq + std::hash::Hash> {
    vertices: HashSet<T>,
    edges: HashMap<T, HashSet<T>>,
    directed: bool,
}

impl<T: Clone + Eq + std::hash::Hash> Graph<T> {
    fn new(directed: bool) -> Self {
        Self {
            vertices: HashSet::new(),
            edges: HashMap::new(),
            directed,
        }
    }
    
    fn add_vertex(&mut self, vertex: T) {
        self.vertices.insert(vertex.clone());
        self.edges.entry(vertex).or_insert_with(HashSet::new);
    }
    
    fn add_edge(&mut self, from: T, to: T) {
        self.add_vertex(from.clone());
        self.add_vertex(to.clone());
        
        self.edges.entry(from.clone()).or_insert_with(HashSet::new).insert(to.clone());
        
        if !self.directed {
            self.edges.entry(to).or_insert_with(HashSet::new).insert(from);
        }
    }
    
    fn has_edge(&self, from: &T, to: &T) -> bool {
        self.edges.get(from).map_or(false, |neighbors| neighbors.contains(to))
    }
    
    fn neighbors(&self, vertex: &T) -> Option<&HashSet<T>> {
        self.edges.get(vertex)
    }
    
    fn degree(&self, vertex: &T) -> usize {
        self.edges.get(vertex).map_or(0, |neighbors| neighbors.len())
    }
    
    // 深度优先搜索
    fn dfs(&self, start: &T) -> Vec<T> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut stack = Vec::new();
        
        stack.push(start.clone());
        
        while let Some(vertex) = stack.pop() {
            if visited.insert(vertex.clone()) {
                result.push(vertex.clone());
                
                if let Some(neighbors) = self.neighbors(&vertex) {
                    for neighbor in neighbors {
                        if !visited.contains(neighbor) {
                            stack.push(neighbor.clone());
                        }
                    }
                }
            }
        }
        
        result
    }
    
    // 广度优先搜索
    fn bfs(&self, start: &T) -> Vec<T> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(start.clone());
        visited.insert(start.clone());
        
        while let Some(vertex) = queue.pop_front() {
            result.push(vertex.clone());
            
            if let Some(neighbors) = self.neighbors(&vertex) {
                for neighbor in neighbors {
                    if visited.insert(neighbor.clone()) {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        
        result
    }
    
    // 最短路径 (Dijkstra算法)
    fn shortest_path(&self, start: &T, end: &T) -> Option<Vec<T>> {
        let mut distances = HashMap::new();
        let mut previous = HashMap::new();
        let mut unvisited = self.vertices.clone();
        
        for vertex in &self.vertices {
            distances.insert(vertex.clone(), usize::MAX);
        }
        distances.insert(start.clone(), 0);
        
        while !unvisited.is_empty() {
            let current = unvisited.iter()
                .min_by_key(|v| distances.get(*v).unwrap_or(&usize::MAX))
                .cloned()?;
            
            unvisited.remove(&current);
            
            if current == *end {
                let mut path = Vec::new();
                let mut node = Some(end.clone());
                
                while let Some(n) = node {
                    path.push(n.clone());
                    node = previous.get(&n).cloned();
                }
                
                path.reverse();
                return Some(path);
            }
            
            if let Some(neighbors) = self.neighbors(&current) {
                for neighbor in neighbors {
                    if unvisited.contains(neighbor) {
                        let alt = distances.get(&current).unwrap_or(&usize::MAX) + 1;
                        if alt < *distances.get(neighbor).unwrap_or(&usize::MAX) {
                            distances.insert(neighbor.clone(), alt);
                            previous.insert(neighbor.clone(), current.clone());
                        }
                    }
                }
            }
        }
        
        None
    }
}
```

### 5.2 图的算法

```rust
// 图算法实现
impl<T: Clone + Eq + std::hash::Hash> Graph<T> {
    // 拓扑排序
    fn topological_sort(&self) -> Option<Vec<T>> {
        if !self.directed {
            return None;
        }
        
        let mut in_degree = HashMap::new();
        for vertex in &self.vertices {
            in_degree.insert(vertex.clone(), 0);
        }
        
        for (_, neighbors) in &self.edges {
            for neighbor in neighbors {
                *in_degree.entry(neighbor.clone()).or_insert(0) += 1;
            }
        }
        
        let mut queue = VecDeque::new();
        for (vertex, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(vertex.clone());
            }
        }
        
        let mut result = Vec::new();
        
        while let Some(vertex) = queue.pop_front() {
            result.push(vertex.clone());
            
            if let Some(neighbors) = self.neighbors(&vertex) {
                for neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }
        
        if result.len() == self.vertices.len() {
            Some(result)
        } else {
            None // 存在环
        }
    }
    
    // 强连通分量 (Kosaraju算法)
    fn strongly_connected_components(&self) -> Vec<Vec<T>> {
        if !self.directed {
            return vec![self.vertices.iter().cloned().collect()];
        }
        
        // 第一次DFS
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        
        for vertex in &self.vertices {
            if !visited.contains(vertex) {
                self.dfs_fill_order(vertex, &mut visited, &mut stack);
            }
        }
        
        // 构建转置图
        let transposed = self.transpose();
        
        // 第二次DFS
        visited.clear();
        let mut components = Vec::new();
        
        while let Some(vertex) = stack.pop() {
            if !visited.contains(&vertex) {
                let mut component = Vec::new();
                transposed.dfs_util(&vertex, &mut visited, &mut component);
                components.push(component);
            }
        }
        
        components
    }
    
    fn dfs_fill_order(&self, vertex: &T, visited: &mut HashSet<T>, stack: &mut Vec<T>) {
        visited.insert(vertex.clone());
        
        if let Some(neighbors) = self.neighbors(vertex) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    self.dfs_fill_order(neighbor, visited, stack);
                }
            }
        }
        
        stack.push(vertex.clone());
    }
    
    fn dfs_util(&self, vertex: &T, visited: &mut HashSet<T>, component: &mut Vec<T>) {
        visited.insert(vertex.clone());
        component.push(vertex.clone());
        
        if let Some(neighbors) = self.neighbors(vertex) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    self.dfs_util(neighbor, visited, component);
                }
            }
        }
    }
    
    fn transpose(&self) -> Graph<T> {
        let mut transposed = Graph::new(self.directed);
        
        for vertex in &self.vertices {
            transposed.add_vertex(vertex.clone());
        }
        
        for (from, neighbors) in &self.edges {
            for to in neighbors {
                transposed.add_edge(to.clone(), from.clone());
            }
        }
        
        transposed
    }
}
```

## 6. 复杂度理论

### 6.1 时间复杂度分析

```rust
// 复杂度分析
use std::time::Instant;

// 复杂度类别
#[derive(Debug, Clone, PartialEq)]
enum TimeComplexity {
    Constant,      // O(1)
    Logarithmic,   // O(log n)
    Linear,        // O(n)
    Linearithmic,  // O(n log n)
    Quadratic,     // O(n²)
    Cubic,         // O(n³)
    Exponential,   // O(2^n)
    Factorial,     // O(n!)
}

impl TimeComplexity {
    fn analyze_algorithm<F>(&self, algorithm: F, input_sizes: &[usize]) -> Vec<f64>
    where
        F: Fn(usize) -> (),
    {
        let mut times = Vec::new();
        
        for &size in input_sizes {
            let start = Instant::now();
            algorithm(size);
            let duration = start.elapsed();
            times.push(duration.as_secs_f64());
        }
        
        times
    }
    
    fn theoretical_complexity(&self, n: usize) -> f64 {
        match self {
            TimeComplexity::Constant => 1.0,
            TimeComplexity::Logarithmic => (n as f64).ln(),
            TimeComplexity::Linear => n as f64,
            TimeComplexity::Linearithmic => (n as f64) * (n as f64).ln(),
            TimeComplexity::Quadratic => (n as f64).powi(2),
            TimeComplexity::Cubic => (n as f64).powi(3),
            TimeComplexity::Exponential => 2.0_f64.powi(n as i32),
            TimeComplexity::Factorial => Self::factorial(n as f64),
        }
    }
    
    fn factorial(n: f64) -> f64 {
        if n <= 1.0 {
            1.0
        } else {
            n * Self::factorial(n - 1.0)
        }
    }
}

// 算法复杂度分析器
struct ComplexityAnalyzer;

impl ComplexityAnalyzer {
    // 线性搜索
    fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
        for (i, item) in arr.iter().enumerate() {
            if item == target {
                return Some(i);
            }
        }
        None
    }
    
    // 二分搜索
    fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
        let mut left = 0;
        let mut right = arr.len();
        
        while left < right {
            let mid = left + (right - left) / 2;
            match arr[mid].cmp(target) {
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => right = mid,
            }
        }
        None
    }
    
    // 冒泡排序
    fn bubble_sort<T: Ord>(arr: &mut [T]) {
        let n = arr.len();
        for i in 0..n {
            for j in 0..n - i - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }
    
    // 归并排序
    fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        
        let mid = arr.len() / 2;
        let mut left = arr[..mid].to_vec();
        let mut right = arr[mid..].to_vec();
        
        Self::merge_sort(&mut left);
        Self::merge_sort(&mut right);
        
        Self::merge(arr, &left, &right);
    }
    
    fn merge<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        
        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                arr[k] = left[i].clone();
                i += 1;
            } else {
                arr[k] = right[j].clone();
                j += 1;
            }
            k += 1;
        }
        
        while i < left.len() {
            arr[k] = left[i].clone();
            i += 1;
            k += 1;
        }
        
        while j < right.len() {
            arr[k] = right[j].clone();
            j += 1;
            k += 1;
        }
    }
}
```

## 7. 密码学数学基础

### 7.1 椭圆曲线

```rust
// 椭圆曲线实现
use num_bigint::BigInt;

// 椭圆曲线点
#[derive(Debug, Clone, PartialEq)]
struct EllipticCurvePoint {
    x: BigInt,
    y: BigInt,
    is_infinity: bool,
}

impl EllipticCurvePoint {
    fn new(x: BigInt, y: BigInt) -> Self {
        Self {
            x,
            y,
            is_infinity: false,
        }
    }
    
    fn infinity() -> Self {
        Self {
            x: BigInt::zero(),
            y: BigInt::zero(),
            is_infinity: true,
        }
    }
}

// 椭圆曲线
#[derive(Debug, Clone)]
struct EllipticCurve {
    a: BigInt,
    b: BigInt,
    p: BigInt,  // 素数模
}

impl EllipticCurve {
    fn new(a: BigInt, b: BigInt, p: BigInt) -> Self {
        Self { a, b, p }
    }
    
    // 检查点是否在曲线上
    fn is_on_curve(&self, point: &EllipticCurvePoint) -> bool {
        if point.is_infinity {
            return true;
        }
        
        let y_squared = (&point.y * &point.y) % &self.p;
        let x_cubed = (&point.x * &point.x * &point.x) % &self.p;
        let ax = (&self.a * &point.x) % &self.p;
        
        let left = (y_squared) % &self.p;
        let right = (x_cubed + ax + &self.b) % &self.p;
        
        left == right
    }
    
    // 点加法
    fn add(&self, p1: &EllipticCurvePoint, p2: &EllipticCurvePoint) -> EllipticCurvePoint {
        if p1.is_infinity {
            return p2.clone();
        }
        if p2.is_infinity {
            return p1.clone();
        }
        
        if p1.x == p2.x {
            if p1.y == p2.y {
                return self.double(p1);
            } else {
                return EllipticCurvePoint::infinity();
            }
        }
        
        let dx = (&p2.x - &p1.x) % &self.p;
        let dy = (&p2.y - &p1.y) % &self.p;
        
        let dx_inv = mod_inverse(&dx, &self.p).unwrap();
        let slope = (dy * dx_inv) % &self.p;
        
        let x3 = (&slope * &slope - &p1.x - &p2.x) % &self.p;
        let y3 = (&slope * (&p1.x - &x3) - &p1.y) % &self.p;
        
        EllipticCurvePoint::new(x3, y3)
    }
    
    // 点倍乘
    fn double(&self, point: &EllipticCurvePoint) -> EllipticCurvePoint {
        if point.is_infinity {
            return EllipticCurvePoint::infinity();
        }
        
        let numerator = (BigInt::from(3) * &point.x * &point.x + &self.a) % &self.p;
        let denominator = (BigInt::from(2) * &point.y) % &self.p;
        
        let denominator_inv = mod_inverse(&denominator, &self.p).unwrap();
        let slope = (numerator * denominator_inv) % &self.p;
        
        let x3 = (&slope * &slope - BigInt::from(2) * &point.x) % &self.p;
        let y3 = (&slope * (&point.x - &x3) - &point.y) % &self.p;
        
        EllipticCurvePoint::new(x3, y3)
    }
    
    // 标量乘法
    fn scalar_multiply(&self, point: &EllipticCurvePoint, scalar: &BigInt) -> EllipticCurvePoint {
        if scalar.is_zero() {
            return EllipticCurvePoint::infinity();
        }
        
        let mut result = EllipticCurvePoint::infinity();
        let mut addend = point.clone();
        let mut k = scalar.clone();
        
        while k > BigInt::zero() {
            if &k % 2 == BigInt::one() {
                result = self.add(&result, &addend);
            }
            addend = self.double(&addend);
            k /= 2;
        }
        
        result
    }
}
```

### 7.2 离散对数

```rust
// 离散对数问题
struct DiscreteLogarithm;

impl DiscreteLogarithm {
    // 暴力破解
    fn brute_force(base: &BigInt, target: &BigInt, modulus: &BigInt) -> Option<BigInt> {
        let mut result = BigInt::one();
        
        for i in 1..modulus.to_string().parse().unwrap_or(1000000) {
            result = (result * base) % modulus;
            if result == *target {
                return Some(BigInt::from(i));
            }
        }
        
        None
    }
    
    // Baby-step Giant-step算法
    fn baby_step_giant_step(base: &BigInt, target: &BigInt, modulus: &BigInt) -> Option<BigInt> {
        let m = (modulus.to_f64().unwrap().sqrt().ceil() as usize).max(1);
        let mut table = HashMap::new();
        
        // Baby steps
        let mut baby = BigInt::one();
        for j in 0..m {
            table.insert(baby.clone(), j);
            baby = (baby * base) % modulus;
        }
        
        // Giant steps
        let base_inv = mod_inverse(base, modulus)?;
        let base_inv_m = Self::mod_pow(&base_inv, &BigInt::from(m), modulus);
        let mut giant = target.clone();
        
        for i in 0..m {
            if let Some(&j) = table.get(&giant) {
                return Some(BigInt::from(i * m + j));
            }
            giant = (giant * &base_inv_m) % modulus;
        }
        
        None
    }
    
    // Pollard's rho算法
    fn pollard_rho(base: &BigInt, target: &BigInt, modulus: &BigInt) -> Option<BigInt> {
        let mut x = BigInt::one();
        let mut y = BigInt::one();
        let mut d = BigInt::one();
        
        let f = |x: &BigInt| -> BigInt {
            if x % 3 == BigInt::zero() {
                (x * x) % modulus
            } else if x % 3 == BigInt::one() {
                (x * base) % modulus
            } else {
                (x * target) % modulus
            }
        };
        
        while d == BigInt::one() {
            x = f(&x);
            y = f(&f(&y));
            d = gcd(&((&x - &y).abs()), modulus);
        }
        
        if d == *modulus {
            None
        } else {
            Some(d)
        }
    }
    
    fn mod_pow(base: &BigInt, exp: &BigInt, modulus: &BigInt) -> BigInt {
        let mut result = BigInt::one();
        let mut base = base % modulus;
        let mut exp = exp.clone();
        
        while exp > BigInt::zero() {
            if &exp % 2 == BigInt::one() {
                result = (result * &base) % modulus;
            }
            exp /= 2;
            base = (&base * &base) % modulus;
        }
        
        result
    }
}
```

## 8. 区块链数学原理

### 8.1 哈希函数数学性质

```rust
// 哈希函数数学分析
struct HashFunctionAnalysis;

impl HashFunctionAnalysis {
    // 雪崩效应分析
    fn avalanche_effect<F>(hash_function: F, input: &[u8], bit_changes: usize) -> f64
    where
        F: Fn(&[u8]) -> Vec<u8>,
    {
        let original_hash = hash_function(input);
        let mut changed_bits = 0;
        let total_bits = original_hash.len() * 8;
        
        for _ in 0..bit_changes {
            let mut modified_input = input.to_vec();
            let byte_index = rand::random::<usize>() % modified_input.len();
            let bit_index = rand::random::<usize>() % 8;
            modified_input[byte_index] ^= 1 << bit_index;
            
            let modified_hash = hash_function(&modified_input);
            changed_bits += Self::count_different_bits(&original_hash, &modified_hash);
        }
        
        changed_bits as f64 / (bit_changes * total_bits) as f64
    }
    
    fn count_different_bits(a: &[u8], b: &[u8]) -> usize {
        a.iter().zip(b.iter())
            .map(|(x, y)| (x ^ y).count_ones() as usize)
            .sum()
    }
    
    // 碰撞概率分析
    fn collision_probability(hash_size_bits: usize, num_hashes: usize) -> f64 {
        let hash_space = 2.0_f64.powi(hash_size_bits as i32);
        let n = num_hashes as f64;
        
        // 生日悖论近似
        1.0 - (-n * (n - 1.0) / (2.0 * hash_space)).exp()
    }
    
    // 抗碰撞强度
    fn collision_resistance_strength(hash_size_bits: usize) -> f64 {
        // 2^(n/2) 次操作找到碰撞
        2.0_f64.powi((hash_size_bits / 2) as i32)
    }
}
```

### 8.2 共识算法数学分析

```rust
// 共识算法数学分析
struct ConsensusAnalysis;

impl ConsensusAnalysis {
    // PoW难度调整
    fn pow_difficulty_adjustment(
        current_difficulty: u64,
        target_time: u64,
        actual_time: u64,
    ) -> u64 {
        let adjustment_factor = target_time as f64 / actual_time as f64;
        let new_difficulty = (current_difficulty as f64 * adjustment_factor) as u64;
        
        // 限制调整幅度
        let max_adjustment = current_difficulty / 4;
        if new_difficulty > current_difficulty + max_adjustment {
            current_difficulty + max_adjustment
        } else if new_difficulty < current_difficulty.saturating_sub(max_adjustment) {
            current_difficulty.saturating_sub(max_adjustment)
        } else {
            new_difficulty
        }
    }
    
    // PoS权益计算
    fn pos_stake_weight(stake_amount: u64, time_locked: u64, max_time: u64) -> f64 {
        let stake_factor = (stake_amount as f64).ln();
        let time_factor = (time_locked as f64 / max_time as f64).powf(2.0);
        stake_factor * (1.0 + time_factor)
    }
    
    // 拜占庭容错阈值
    fn byzantine_fault_tolerance_threshold(total_nodes: usize) -> usize {
        // 需要超过 2/3 的诚实节点
        (total_nodes * 2) / 3 + 1
    }
    
    // 网络分区概率
    fn network_partition_probability(
        total_nodes: usize,
        partition_size: usize,
        failure_probability: f64,
    ) -> f64 {
        // 二项分布计算分区概率
        let n = total_nodes;
        let k = partition_size;
        let p = failure_probability;
        
        Self::binomial_coefficient(n, k) as f64 * p.powi(k as i32) * (1.0 - p).powi((n - k) as i32)
    }
    
    fn binomial_coefficient(n: usize, k: usize) -> usize {
        if k > n {
            return 0;
        }
        if k == 0 || k == n {
            return 1;
        }
        
        let mut result = 1;
        for i in 0..k.min(n - k) {
            result = result * (n - i) / (i + 1);
        }
        result
    }
}
```

## 9. 总结

数学基础与算法分析为区块链技术提供了坚实的理论基础：

1. **数学基础** - 集合论、关系与函数
2. **数论基础** - 整数运算、素性测试
3. **抽象代数** - 群论、环与域
4. **概率论与统计学** - 概率分布、统计检验
5. **图论** - 图的基本概念和算法
6. **复杂度理论** - 时间复杂度分析
7. **密码学数学基础** - 椭圆曲线、离散对数
8. **区块链数学原理** - 哈希函数、共识算法分析

这些数学工具为理解和分析区块链系统的安全性、效率和正确性提供了重要的理论基础。

---

**文档版本**: v1.0.0  
**最后更新**: 2025年10月15日  
**作者**: 数学与算法专家  
**审核**: 区块链理论专家
