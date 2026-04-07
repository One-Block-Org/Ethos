# Ethos: Technical Specification

Ethos is a high-performance Rust-based CLI for Ethereum transaction profiling. This document outlines the technical architecture and design principles of the system.

## Core Architecture

Ethos is organized into four primary subsystems:

### 1. RPC & Data Acquisition (`ethos-rpc`)
- **JSON-RPC Client**: A lightweight HTTP client optimized for high-throughput trace fetching.
- **Tracer Selection**: Automatically detects node capabilities and requests the most efficient tracer (e.g., `callTracer` for high-level trees, `structLogs` for opcode-level depth).
- **Batching**: Support for concurrent transaction profiling across multiple blocks.

### 2. Standardized Trace Parser (`ethos-parser`)
- **Normalization Engine**: Converts disparate trace formats from different clients (Geth, Anvil, Hardhat) into a unified internal representation.
- **Opcode Mapper**: Maps each byte of execution to a semantic context (e.g., Identifying the start of an internal call or a specific precompile execution).
- **Source Map Integrator**: Correlates Program Counters (PC) with Solidity source data via compiler-generated source maps.

### 3. Execution Aggregator (`ethos-aggregator`)
- **Stack Folding**: Collapses recursive call stacks and aggregates gas costs at each depth level.
- **Hotspot Detection**: Automatically identifies "Hot" opcodes and contracts, including:
    - High-frequency `SSTORE`/`SLOAD` operations.
    - Expensive external call boundaries.
    - Deeply nested or high-cost looping structures.

### 4. Output & Visualization (`ethos-output`)
- **SVG Renderer**: Generates interactive flamegraphs with zoom and search capabilities.
- **Markdown Generator**: Produces summarized tables for CI integration.
- **JSON Schema Export**: Emits machine-readable profile summaries for external dashboard integration.

## Data Flow

1.  **Ingest**: CLI receives a Tx Hash and RPC URL.
2.  **Fetch**: RPC Module calls `debug_traceTransaction`.
3.  **Parse**: Parser Module normalizes the JSON trace and extracts the execution hierarchy.
4.  **Aggregate**: Aggregator calculates weighted stacks based on gas consumption of each call frame.
5.  **Render**: Output Module generates the visual flamegraph and tabular report.

## Design Goals

- **Zero-Dependency UI**: The web viewer remains a single, portable HTML artifact.
- **Privacy by Default**: No telemetry or third-party cloud connections.
- **Local-First Performance**: Capable of processing 10MB+ traces in under 2 seconds on a typical laptop.
