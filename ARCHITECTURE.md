# Atupa Suit: System Architecture

The Atupa Suite is designed as a modular, high-performance infrastructure stack that provides transparency for the "Multi-VM" future of Ethereum.

## 🏗 System Components

### 1. Network Layer (The Sources)
Atupa connects to diverse execution environments:
- **Ethereum (L1)**: Standard EVM via `structLogs`.
- **Arbitrum (L2)**: Dual-VM (EVM + Stylus) via the Nitro `stylusTracer`.
- **Unichain (L2)**: Real-time "Flashblocks" (200ms pending state).

### 2. Intelligence Layer (The Engine)
This is where raw hex data becomes human insight:
- **MixedTraceStitcher**: Correlates different trace formats into a single timeline.
- **Symbol Resolver**: Uses DWARF symbols and Sourcify to map opcodes to Rust/Solidity source lines.
- **Protocol Adapters**: Domain-specific logic for Aave, Lido, and Uniswap.

### 3. Interface Layer (The UX)
How developers and auditors interact with the data:
- **`cargo-atupa`**: Local-first CLI for CI/CD and rapid debugging.
- **Atupa Studio**: High-performance web visualizer (Flamegraphs, DAGs).
- **Atupa Report**: Automated, professional audit PDFs for DAO governance.

## 🏮 Data Formats
We use a unified **Atupa Profile JSON** that includes:
- `execution_steps`: Contiguous list of all VM instructions.
- `memory_deltas`: Mapping of memory growth and spikes.
- `protocol_context`: Labels and risk flags (e.g., "High Slippage Detected").

---
🏮 *One Block: The Transparency Layer for the Hybrid Future.*
