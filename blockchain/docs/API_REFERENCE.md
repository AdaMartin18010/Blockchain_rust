# 区块链 API 参考文档

## 目录

- [区块链 API 参考文档](#区块链-api-参考文档)
  - [目录](#目录)
  - [概述](#概述)
  - [基础信息](#基础信息)
  - [通用响应格式](#通用响应格式)
    - [响应字段说明](#响应字段说明)
  - [区块链信息 API](#区块链信息-api)
    - [获取区块链基本信息](#获取区块链基本信息)
    - [获取区块信息](#获取区块信息)
    - [获取最新区块](#获取最新区块)
    - [验证区块链](#验证区块链)
  - [交易 API](#交易-api)
    - [创建交易](#创建交易)
    - [获取待处理交易](#获取待处理交易)
    - [获取交易历史](#获取交易历史)
    - [获取交易统计](#获取交易统计)
  - [账户 API](#账户-api)
    - [获取账户余额](#获取账户余额)
    - [获取所有账户余额](#获取所有账户余额)
  - [挖矿 API](#挖矿-api)
    - [开始挖矿](#开始挖矿)
  - [监控 API](#监控-api)
    - [获取监控指标](#获取监控指标)
    - [获取性能统计](#获取性能统计)
    - [获取健康状态](#获取健康状态)
    - [生成监控报告](#生成监控报告)
  - [节点信息 API](#节点信息-api)
    - [获取节点信息](#获取节点信息)
    - [获取网络统计](#获取网络统计)
  - [搜索 API](#搜索-api)
    - [搜索功能](#搜索功能)
  - [数据导出 API](#数据导出-api)
    - [导出区块链数据](#导出区块链数据)
  - [错误处理](#错误处理)
    - [错误响应格式](#错误响应格式)
    - [常见错误码](#常见错误码)
    - [错误示例](#错误示例)
  - [使用示例](#使用示例)
    - [JavaScript 示例](#javascript-示例)
    - [Python 示例](#python-示例)
  - [命令行工具](#命令行工具)
    - [基本用法](#基本用法)
    - [高级用法](#高级用法)
  - [性能优化建议](#性能优化建议)
  - [安全注意事项](#安全注意事项)
  - [版本历史](#版本历史)
  - [支持和反馈](#支持和反馈)

## 概述

本文档详细介绍了区块链项目的 REST API 接口，包括所有可用的端点、请求参数、响应格式和使用示例。

## 基础信息

- **API 版本**: v1.0.0
- **基础 URL**: `http://localhost:8080/api/v1`
- **认证**: 目前无需认证（开发版本）
- **响应格式**: JSON

## 通用响应格式

所有 API 响应都遵循以下格式：

```json
{
  "success": true,
  "data": { ... },
  "error": null
}
```

### 响应字段说明

- `success`: 布尔值，表示请求是否成功
- `data`: 成功时返回的数据对象
- `error`: 失败时的错误信息

## 区块链信息 API

### 获取区块链基本信息

**端点**: `GET /blockchain/info`

**描述**: 获取区块链的基本信息，包括链高度、难度等。

**响应示例**:

```json
{
  "success": true,
  "data": {
    "chain_length": 10,
    "difficulty": 2,
    "pending_transactions": 3,
    "latest_block_hash": "0x1234567890abcdef..."
  }
}
```

### 获取区块信息

**端点**: `GET /blockchain/block/{index}`

**参数**:

- `index`: 区块索引（路径参数）

**响应示例**:

```json
{
  "success": true,
  "data": {
    "index": 0,
    "hash": "0x1234567890abcdef...",
    "prev_hash": "0x0000000000000000...",
    "timestamp": 1640995200,
    "transactions": [
      {
        "sender": "genesis",
        "receiver": "genesis",
        "amount": 0,
        "timestamp": 1640995200
      }
    ],
    "nonce": 12345
  }
}
```

### 获取最新区块

**端点**: `GET /blockchain/block/latest`

**响应**: 与获取区块信息相同，返回最新区块的数据。

### 验证区块链

**端点**: `GET /blockchain/validate`

**响应示例**:

```json
{
  "success": true,
  "data": true
}
```

## 交易 API

### 创建交易

**端点**: `POST /transactions`

**请求体**:

```json
{
  "sender": "alice",
  "receiver": "bob",
  "amount": 100
}
```

**响应示例**:

```json
{
  "success": true,
  "data": {
    "transaction_id": "0xabcdef1234567890...",
    "status": "pending"
  }
}
```

### 获取待处理交易

**端点**: `GET /transactions/pending`

**响应示例**:

```json
{
  "success": true,
  "data": [
    {
      "sender": "alice",
      "receiver": "bob",
      "amount": 100,
      "timestamp": 1640995200
    }
  ]
}
```

### 获取交易历史

**端点**: `GET /transactions/history`

**查询参数**:

- `address`: 地址过滤器（可选）
- `limit`: 限制返回数量（默认10）

**响应示例**:

```json
{
  "success": true,
  "data": [
    {
      "sender": "alice",
      "receiver": "bob",
      "amount": 100,
      "timestamp": 1640995200,
      "block_index": 1
    }
  ]
}
```

### 获取交易统计

**端点**: `GET /transactions/stats`

**响应示例**:

```json
{
  "success": true,
  "data": {
    "total_transactions": 150,
    "total_amount": 50000,
    "unique_addresses": 25,
    "pending_transactions": 5
  }
}
```

## 账户 API

### 获取账户余额

**端点**: `GET /accounts/{address}/balance`

**参数**:

- `address`: 账户地址（路径参数）

**响应示例**:

```json
{
  "success": true,
  "data": {
    "address": "alice",
    "balance": 1000
  }
}
```

### 获取所有账户余额

**端点**: `GET /accounts/balances`

**响应示例**:

```json
{
  "success": true,
  "data": [
    {
      "address": "alice",
      "balance": 1000
    },
    {
      "address": "bob",
      "balance": 500
    }
  ]
}
```

## 挖矿 API

### 开始挖矿

**端点**: `POST /mining/start`

**请求体**:

```json
{
  "miner_address": "miner1",
  "count": 1
}
```

**响应示例**:

```json
{
  "success": true,
  "data": {
    "success": true,
    "blocks_mined": 1,
    "total_time": 2.5,
    "hash_rate": 0.4
  }
}
```

## 监控 API

### 获取监控指标

**端点**: `GET /monitoring/metrics`

**响应示例**:

```json
{
  "success": true,
  "data": {
    "timestamp": 1640995200,
    "uptime": 3600,
    "performance": {
      "transactions_per_second": 10.5,
      "blocks_per_minute": 2.0,
      "average_block_time": 30.0,
      "average_transaction_size": 256.0,
      "memory_usage": 1048576,
      "cpu_usage": 25.5,
      "network_throughput": 100.0
    },
    "blockchain": {
      "chain_height": 10,
      "total_transactions": 150,
      "pending_transactions": 5,
      "average_difficulty": 2.0,
      "total_hash_rate": 1000000.0,
      "active_miners": 5,
      "network_size": 10
    },
    "health": {
      "status": "healthy",
      "last_block_time": 1640995200,
      "sync_status": "synced",
      "error_count": 0,
      "warnings": []
    }
  }
}
```

### 获取性能统计

**端点**: `GET /monitoring/performance`

**响应示例**:

```json
{
  "success": true,
  "data": {
    "transactions_per_second": 10.5,
    "blocks_per_minute": 2.0,
    "average_block_time": 30.0,
    "average_transaction_size": 256.0,
    "memory_usage": 1048576,
    "cpu_usage": 25.5,
    "network_throughput": 100.0
  }
}
```

### 获取健康状态

**端点**: `GET /monitoring/health`

**响应示例**:

```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "last_block_time": 1640995200,
    "sync_status": "synced",
    "error_count": 0,
    "warnings": []
  }
}
```

### 生成监控报告

**端点**: `POST /monitoring/report`

**请求体**:

```json
{
  "output_file": "monitoring_report.json"
}
```

**响应示例**:

```json
{
  "success": true,
  "data": {
    "report_file": "monitoring_report.json",
    "generated_at": 1640995200
  }
}
```

## 节点信息 API

### 获取节点信息

**端点**: `GET /node/info`

**响应示例**:

```json
{
  "success": true,
  "data": {
    "node_id": "550e8400-e29b-41d4-a716-446655440000",
    "version": "1.0.0",
    "uptime": 3600,
    "connected_peers": 5,
    "sync_status": "synced"
  }
}
```

### 获取网络统计

**端点**: `GET /node/network`

**响应示例**:

```json
{
  "success": true,
  "data": {
    "total_connections": 10,
    "active_connections": 5,
    "bytes_sent": 1048576,
    "bytes_received": 2097152,
    "messages_sent": 1000,
    "messages_received": 1500
  }
}
```

## 搜索 API

### 搜索功能

**端点**: `POST /search`

**请求体**:

```json
{
  "query": "alice",
  "search_type": "Transaction",
  "limit": 10
}
```

**搜索类型**:

- `Transaction`: 搜索交易
- `Block`: 搜索区块
- `Address`: 搜索地址
- `All`: 搜索所有类型

**响应示例**:

```json
{
  "success": true,
  "data": {
    "results": [
      {
        "result_type": "Transaction",
        "data": {
          "sender": "alice",
          "receiver": "bob",
          "amount": 100,
          "timestamp": 1640995200
        },
        "relevance_score": 1.0
      }
    ],
    "total_count": 5
  }
}
```

## 数据导出 API

### 导出区块链数据

**端点**: `POST /export`

**请求体**:

```json
{
  "format": "json",
  "output_file": "blockchain_export.json"
}
```

**支持格式**:

- `json`: JSON 格式
- `csv`: CSV 格式（开发中）
- `xml`: XML 格式（开发中）
- `binary`: 二进制格式（开发中）

**响应示例**:

```json
{
  "success": true,
  "data": {
    "export_file": "blockchain_export.json",
    "exported_at": 1640995200,
    "format": "json"
  }
}
```

## 错误处理

### 错误响应格式

```json
{
  "success": false,
  "data": null,
  "error": "错误描述信息"
}
```

### 常见错误码

- `400 Bad Request`: 请求参数错误
- `404 Not Found`: 资源不存在
- `500 Internal Server Error`: 服务器内部错误

### 错误示例

```json
{
  "success": false,
  "data": null,
  "error": "Blockchain error: Insufficient balance"
}
```

## 使用示例

### JavaScript 示例

```javascript
// 获取区块链信息
async function getBlockchainInfo() {
  try {
    const response = await fetch('http://localhost:8080/api/v1/blockchain/info');
    const data = await response.json();
    
    if (data.success) {
      console.log('区块链信息:', data.data);
    } else {
      console.error('错误:', data.error);
    }
  } catch (error) {
    console.error('请求失败:', error);
  }
}

// 创建交易
async function createTransaction(sender, receiver, amount) {
  try {
    const response = await fetch('http://localhost:8080/api/v1/transactions', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        sender: sender,
        receiver: receiver,
        amount: amount
      })
    });
    
    const data = await response.json();
    return data;
  } catch (error) {
    console.error('创建交易失败:', error);
    return null;
  }
}

// 搜索交易
async function searchTransactions(query) {
  try {
    const response = await fetch('http://localhost:8080/api/v1/search', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        query: query,
        search_type: 'Transaction',
        limit: 20
      })
    });
    
    const data = await response.json();
    return data;
  } catch (error) {
    console.error('搜索失败:', error);
    return null;
  }
}
```

### Python 示例

```python
import requests
import json

# 获取区块链信息
def get_blockchain_info():
    try:
        response = requests.get('http://localhost:8080/api/v1/blockchain/info')
        data = response.json()
        
        if data['success']:
            print('区块链信息:', data['data'])
        else:
            print('错误:', data['error'])
    except Exception as e:
        print('请求失败:', e)

# 创建交易
def create_transaction(sender, receiver, amount):
    try:
        response = requests.post(
            'http://localhost:8080/api/v1/transactions',
            headers={'Content-Type': 'application/json'},
            data=json.dumps({
                'sender': sender,
                'receiver': receiver,
                'amount': amount
            })
        )
        
        return response.json()
    except Exception as e:
        print('创建交易失败:', e)
        return None

# 获取监控指标
def get_monitoring_metrics():
    try:
        response = requests.get('http://localhost:8080/api/v1/monitoring/metrics')
        data = response.json()
        
        if data['success']:
            metrics = data['data']
            print(f"TPS: {metrics['performance']['transactions_per_second']}")
            print(f"链高度: {metrics['blockchain']['chain_height']}")
            print(f"健康状态: {metrics['health']['status']}")
    except Exception as e:
        print('获取监控指标失败:', e)
```

## 命令行工具

除了 REST API，项目还提供了命令行工具用于区块链管理：

### 基本用法

```bash
# 初始化区块链
blockchain-cli init --difficulty 2 --genesis-balance 1000000

# 查看区块链信息
blockchain-cli info --detailed

# 创建交易
blockchain-cli transaction create --from alice --to bob --amount 100

# 挖矿
blockchain-cli mine --address miner --count 5

# 查看监控指标
blockchain-cli monitor metrics

# 生成监控报告
blockchain-cli monitor report --output report.json

# 验证区块链
blockchain-cli validate

# 导出数据
blockchain-cli export --format json --output blockchain.json
```

### 高级用法

```bash
# 启用详细输出
blockchain-cli --verbose info

# 指定配置和数据目录
blockchain-cli --config config.json --data-dir ./data info

# 搜索交易历史
blockchain-cli transaction history --address alice --limit 20

# 查看待处理交易
blockchain-cli transaction pending
```

## 性能优化建议

1. **批量操作**: 使用批量 API 进行大量数据操作
2. **缓存**: 客户端应该缓存频繁访问的数据
3. **分页**: 对于大量数据，使用 limit 参数进行分页
4. **异步**: 使用异步请求避免阻塞

## 安全注意事项

1. **输入验证**: 所有输入参数都应该进行验证
2. **速率限制**: 实施适当的速率限制防止滥用
3. **错误信息**: 避免在错误信息中泄露敏感信息
4. **HTTPS**: 生产环境应该使用 HTTPS

## 版本历史

- **v1.0.0** (2025-01): 初始版本，包含基础 API 功能
- **v1.1.0** (计划): 添加更多监控和分析功能
- **v1.2.0** (计划): 添加用户认证和权限管理

## 支持和反馈

如有问题或建议，请通过以下方式联系：

- 项目仓库: [GitHub Repository]
- 问题反馈: [GitHub Issues]
- 文档更新: [GitHub Discussions]

---

*本文档最后更新于 2025年1月*-
