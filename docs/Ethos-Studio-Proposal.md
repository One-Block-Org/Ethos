# Project Proposal: Ethos Studio

## Details
Ethereum's execution layer has become increasingly complex. With the rise of intricate DeFi protocols, account abstraction (ERC-4337), and sophisticated modular designs, developers often face a "trace gap." When a transaction fails or gas costs spike unexpectedly, engineers are left digging through verbose JSON `structLogs` or opaque block explorer traces. This forensic work is slow, error-prone, and rarely scales to the needs of modern CI/CD pipelines.

**Ethos Studio** turns those overwhelming traces into actionable insights. The CLI ingests standard `debug_traceTransaction` output and produces high-fidelity flamegraphs, concise function-level reports, and side-by-side diffs. It bridges the gap between low-level debugging and high-level performance observability, empowering teams to catch regressions in CI before they reach mainnet.

## Project Idea
Ethos Studio is a local-first, open-source profiling suite for the Ethereum Virtual Machine (EVM). It is designed as a **strategic combination** of the best functionalities from industry-standard tools:

- **Foundry / Anvil**: Native integration for local node simulation and high-speed tracing.
- **Hardhat Gas Reporter**: Comprehensive summary tables and CI-driven gas regression gating.
- **Tenderly**: High-fidelity visual execution tracing and "what-if" simulations, offered as a local-first, privacy-conscious alternative.

### Unique Protocol-Level Adaptations:
- **Uniswap v4 Hook Profiling**: Visual isolation of the 8+ callback points in v4 hooks, identifying gas hotspots in Flash Accounting deltas.
- **Aave v4 Performance Guard**: Benchmarking cross-spoke liquidity calls and GHO minting overhead.
- **Base L2 Optimization**: One-click CI profiling tailored for L2 performance budgets.

## Innovation and Value
- **Structural Visualization**: Unlike flat gas reports, Ethos shows the *hierarchy* of execution, making it obvious where gas is being spent in recursive or nested calls.
- **Local-First, Privacy-First**: No keys, no paid RPCs, and no cloud-dependency by default. It runs against your local `Anvil` or `Hardhat` node.
- **CI-Native Governance**: It moves performance profiling from a manual "one-off" task to a continuous, automated check.

## Target Audience
- **Smart Contract Developers**: Building complex DeFi, NFT platforms, or protocols who need transparency into their gas usage.
- **Protocol Security & Audit Teams**: Who need readable execution evidence to support their findings.
- **CI/CD & DevOps Engineers**: Enforcing performance budgets and reliability in production pipelines.

## Comparable Projects
| Tool | Relationship | Gap Addressed by Ethos |
| :--- | :--- | :--- |
| **Foundry / Hardhat**|的基础 Infra | Ethos adds visual profiling and trend analysis that these CLIs lack by default. |
| **Tenderly** | Cloud-based Powerhouse | Ethos provides a local-first, open-source, and free alternative with no data leakage. |
| **Explorer Tracers** | Reactive Debugging | Ethos is *proactive*, enabling developers to see the trace before they even deploy. |

---

## Detailed Milestones

### Milestone 1: Trace Ingest & Visual Foundation
**Timeline**: 3 Weeks
- **Deliverables**:
    - Core CLI supporting `debug_traceTransaction` (structLogs & callTracer).
    - SVG Flamegraph generation engine for EVM execution.
    - JSON profile schema for standardized trace summaries.
    - Support for Anvil and Hardhat-node local endpoints.
- **KPIs**:
    - Correctly generate flamegraphs for 5 sample DeFi transactions (Swap, Mint, Stake).
    - SVG processing time < 2s for a 10MB trace.
    - Cross-platform builds (macOS, Linux, Windows).

### Milestone 2: Comparison Engine & CI Gating
**Timeline**: 2 Weeks
- **Deliverables**:
    - Profile Diff Generator (deterministic "Before vs After" delta highlighting).
    - Policy Engine supporting `thresholds.toml` for gas and opcode budgeting.
    - Native GitHub Action that starts a local node, replays a script, and captures reports.
- **KPIs**:
    - Diff engine flags regressions of >1% gas accurately.
    - GitHub Action successfully fails a PR when an `SSTORE` threshold is exceeded.
    - PR comment integration (Markdown table summaries).

### Milestone 3: Static Viewer & Source Mapping
**Timeline**: 2 Weeks
- **Deliverables**:
    - Portable Single-Page-App viewer for local profiles.
    - Source-map integration (mapping EVM PC to Solidity file/line/function).
    - Side-by-side diff view in the browser.
- **KPIs**:
    - Viewer loads a 10MB profile in < 1s in Chrome/Firefox.
    - Source hints correctly identify function names for local compiled artifacts.

### Milestone 4: Onboarding, Templates & Docs
**Timeline**: 1 Week
- **Deliverables**:
    - Starter repository with Solidity benchmarks and a green CI pipeline.
    - Comprehensive Quickstart docs and "CI Recipes" for Foundry/Hardhat.
    - Video tutorial demonstrating a full "Debug-to-Fix" cycle using the tool.
- **KPIs**:
    - "Time to first profile" < 10 minutes from a fresh installation.
    - Documentation covers 100% of CLI flags and config options.

---

## Success Metrics
- **Adoption**: 100+ GitHub Stars and usage by at least 15 active open-source repositories within 3 months.
- **Reliability**: 0 critical bugs in the parser across Anvil, Hardhat, and Geth environments.
- **Community Impact**: At least 5 documented cases of Ethos catching a gas regression before it hit production.
