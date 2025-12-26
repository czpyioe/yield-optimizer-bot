# **Yield Optimizer Bot**


## **1. RPC Manager**

### **Goal:** A reliable connection layer to a blockchain node.

**Steps:**

1. Create a module dedicated to RPC operations.
2. Add a function that loads RPC URLs from env/config.
3. Implement a rotating/fallback system between multiple RPC providers.
4. Add automatic reconnection logic for WebSocket streams.
5. Add rate limiting to avoid RPC bans.
6. Add periodic health checks for each provider.
---

## **2. Contract Bindings (Alloy)**

### **Goal:** A stable interface to interact with DeFi protocols.

**Steps:**

1. Import abis.
2. Create a folder for protocol contract bindings.
3. Generate or write bindings for each protocol (Aave, Curve, Pendle, Uniswap, etc.).
4. Create helper functions for reading data (e.g., balances, reserves, APYs).
5. Create helper functions for building transaction requests.
6. Add basic sanity checks for invalid addresses, missing ABIs, or RPC errors.

---

## **3. Database Module**

### **Goal:** Implement postgre database to store protocols datas, transactions, ...

**Steps:**

1. Add migrations folder and rust db folder.
2. Setup connection and apply migrations.
3. 

---


## Run the bot
```bash
docker compose up --build
```