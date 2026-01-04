# **Yield Optimizer Bot**

A Rust-based DeFi yield optimization bot that monitors APY rates across multiple protocols and networks, stores historical data, and provides Telegram notifications.

---

## **What is This?**

This is an **advisory bot for long-term ETH accumulation**, designed for passive DeFi investors who want to maximize returns without constant monitoring.

**Key Features:**
- **Advisory, Not Auto-Trading** - The bot sends recommendations; you execute transactions manually
- **ETH Growth Focus** - Strategies optimized for growing your ETH token balance over time
- **Passive Investing** - Weekly/daily reports, no need to check constantly
- **Smart Thresholds** - Only suggests moves when gains justify gas costs and effort
- **Safe Yield Focus** - Prioritizes established lending protocols (Aave, Compound) with optional LP strategies

**Target User:**
- Has capital in CEX, ready to deploy to DeFi
- Adds capital monthly (DCA strategy)
- Long-term holder (okay with temporary unrealized losses)
- Wants higher yields than CEX, but won't move funds for 1% temporary gains
- Prefers growing ETH holdings over USD value

**What It Does:**
- Monitors lending APYs (Aave, Compound) across multiple networks
- Tracks liquidity pool opportunities (Uniswap, Curve - coming soon)
- Sends weekly portfolio reports with current yields
- Alerts on high-yield opportunities (>5% APY)
- Suggests monthly capital allocation strategies
- Recommends rebalancing only when worth the gas + effort

**What It Doesn't Do:**
- Execute transactions automatically
- Trade or speculate on price movements
- Chase high-risk yield farms
- Require constant user attention

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
docker compose up postgres -d
cd bot/
export DATABASE_URL=postgres://bot:botpassword@localhost:5433/botdb
cargo install sqlx-cli
cargo sqlx migrate run
cargo sqlx prepare
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

