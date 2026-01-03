# **Yield Optimizer Bot**

A Rust-based DeFi yield optimization bot that monitors APY rates across multiple protocols and networks, stores historical data, and provides Telegram notifications.

---

## **Architecture Overview**

### **1. RPC Manager** (`src/rpc/`)

**Goal:** Reliable, fault-tolerant connection layer to blockchain nodes with automatic failover.

**Components:**
- **`loader.rs`** - Loads RPC URLs from environment variables
- **`health.rs`** - Performs health checks and latency measurements
- **`manager.rs`** - Implements `NetworkProviderPool` with automatic provider rotation
- **`utils.rs`** - Helper utilities for RPC operations

**Features:**
- Multi-provider support per network
- Automatic health checks with latency tracking
- Provider scoring system (lower = better)
- Concurrent health checks using `buffer_unordered`
- Per-network provider pools

---

### **2. Contract Bindings** (`src/contracts/`)

**Goal:** Type-safe interface to interact with DeFi protocols using Alloy.

**Structure:**
- **`addresses.rs`** - Protocol, Network, and Asset enums with support matrices
- **`bindings/`** - Auto-generated or manual contract bindings
  - `aave.rs` - Aave Pool contract
  - `compound.rs` - Compound cToken contracts  
  - `erc20.rs` - ERC20 token interface
- **`protocols/`** - Protocol-specific logic
  - `aave.rs` - APY calculation for Aave
  - `compound.rs` - APY calculation for Compound

**Supported:**
- **Protocols:** Aave, Compound
- **Networks:** Ethereum, Arbitrum, Optimism, Base, Polygon
- **Assets:** USDC, WETH

---

### **3. Database Module** (`src/db/`)

**Goal:** PostgreSQL storage for APY snapshots, positions, and historical data.

**Components:**
- **`pool.rs`** - Database connection pool management
- **`models.rs`** - Data structures (`Position`, `ApySnapshot`)
- **`queries.rs`** - SQL operations (insert positions, snapshots)
- **`migrations/001_init.sql`** - Database schema

**Schema:**
- `positions` - User positions across protocols
- `apy_snapshots` - Historical APY data with timestamps

---

### **4. Strategy Module** (`src/strategy/`)

**Goal:** Orchestrate APY data collection across all protocol/network/asset combinations.

**Components:**
- **`orchestrator.rs`** - Concurrent APY snapshot coordination
  - Dynamic provider pool with usage penalties
  - Automatic retry logic (3 attempts)
  - Failure tracking and provider scoring
  - Concurrent execution via `buffer_unordered(10)`
- **`fetcher.rs`** - Protocol-specific APY fetching and database storage

**Flow:**
1. Generate all tasks (protocol × network × asset)
2. Assign provider pools per network
3. Execute tasks concurrently with intelligent provider selection
4. Store results in database

---

### **5. Telegram Module** (`src/telegram/`)

**Goal:** Simple notification system for yield alerts.

**Components:**
- **`client.rs`** - Telegram bot client wrapper
- **`handlers.rs`** - Command handlers (placeholder)
- **`mod.rs`** - Module exports

**Features:**
- Basic message sending
- Chat ID configuration via env vars

---

## Run the bot
```bash
docker compose up --build
```

## Protocols

### Core Lending
- Aave
- Compound
- Morpho

---

### Liquid Staking
- Lido
- Rocket Pool
- Frax

---

### Low-IL Liquidity Pools
- Uniswap
- Curve
- Balancer

