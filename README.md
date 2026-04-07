<p align="center">
  <img src="assets/logo.png" width="350" alt="Ethos Logo">
</p>

<h1 align="center">Ethos</h1>

<p align="center">
  <strong>High-Fidelity Ethereum Tracing & Visual Profiling Suite</strong>
</p>

<p align="center">
  <a href="https://github.com/One-Block-Org/Ethos/actions"><img src="https://github.com/One-Block-Org/Ethos/actions/workflows/rust.yml/badge.svg" alt="CI Status"></a>
  <a href="https://crates.io/crates/ethos-core"><img src="https://img.shields.io/crates/v/ethos-core.svg" alt="Crates.io"></a>
  <a href="https://docs.rs/ethos-core"><img src="https://docs.rs/ethos-core/badge.svg" alt="Documentation"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT"></a>
</p>

---

Ethos is a professional-grade EVM execution profiler designed to turn raw JSON-RPC `debug_traceTransaction` logs into actionable visual insights. Whether you are debugging a complex DeFi reentrancy or optimizing a gas-intensive Uniswap v4 hook, Ethos provides the high-fidelity visibility you need.

## ✨ Key Features

- **🔥 Visual Flamegraphs**: Collapses thousands of EVM opcodes into hierarchical gas-weighted visualizations.
- **🚨 Crimson Revert Identification**: Instantly identifies failing sub-calls with high-contrast crimson gradients.
- **🔍 Smart Contract Resolution**: Automatically resolves hex addresses to verified contract names via Etherscan V2.
- **⚙️ Layered Configuration**: Robust configuration management via `ethos.toml`, environment variables, and CLI overrides.
- **🛠 Modular Library Architecture**: Pure Rust implementation with separate crates for RPC, Parsing, and Visualization.

## 🚀 Quick Start

### Installation

```bash
cargo install ethos-cli
```

### Profiling a Transaction

```bash
# Profile a mainnet transaction
ethos profile --tx 0x... --rpc https://mainnet.infura.io/v3/YOUR_KEY

# Run the offline demo
ethos profile --demo
```

## 📦 Project Structure

Ethos is built as a highly modular monorepo:

| Crate | Description |
|-------|-------------|
| [`ethos-cli`](crates/ethos-cli) | The primary command-line interface. |
| [`ethos-core`](crates/ethos-core) | Shared types and core configuration logic. |
| [`ethos-parser`](crates/ethos-parser) | The aggregation engine that collapses EVM traces. |
| [`ethos-rpc`](crates/ethos-rpc) | Async Ethereum JSON-RPC client & Etherscan resolver. |
| [`ethos-output`](crates/ethos-output) | SVG generation engine using Askama templates. |

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for more details.

## 📄 License

Ethos is dual-licensed under the [MIT License](LICENSE-MIT) and the [Apache License 2.0](LICENSE-APACHE).
