# Ethos: CI Integration Guide

This guide explains how to set up **Ethos** in your GitHub Actions pipeline to enforce gas budgets and detect performance regressions automatically.

## Prerequisites

- A GitHub repository with your smart contract code.
- A scripted transaction or test suite that can be run against a local node.

## GitHub Action Configuration

Add a new workflow file (e.g., `.github/workflows/ethos.yml`):

```yaml
name: Ethos Performance Benchmarks

on: [pull_request]

jobs:
  profile:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Foundry
        uses: foundry-rs/foundry-toolchain@v1

      - name: Start Anvil
        run: anvil &
        # & starts it in the background

      - name: Deploy & Run Scripted Transaction
        run: forge script scripts/Benchmark.s.sol --rpc-url http://localhost:8545 --broadcast

      - name: Run Ethos
        uses: ethos/action@v1
        with:
          rpc_url: "http://localhost:8545"
          tx_hash: "0x..." # Obtain this from the previous step output
          thresholds: "thresholds.toml"
          github_token: ${{ secrets.GITHUB_TOKEN }}

      - name: Upload Profile Artifacts
        uses: actions/upload-artifact@v4
        with:
          name: ethos-profile
          path: output/
```

## How It Works

1.  **Start Local Node**: The action initiates a local `anvil` (Foundry) or `hardhat-node`.
2.  **Execute Transaction**: Your benchmark script performs the transaction you want to profile.
3.  **Capture Trace**: `eth-trace` fetches the trace from the local node.
4.  **Compare & Gate**: If this is a PR, the tool compares the new trace against the `main` branch baseline.
    - If gas usage or opcode counts exceed `thresholds.toml` limits, the job fails.
5.  **Report**: The tool posts a summary table and a link to the visual flamegraph directly in the PR comment.

## Baseline Caching

For faster diffs, `eth-trace` can cache the baseline profiles for the default branch:

```yaml
- name: Cache Baseline Trace
  uses: actions/cache@v4
  with:
    path: ~/.eth-trace/cache
    key: eth-trace-baseline-${{ github.sha }}
    restore-keys: |
      eth-trace-baseline-
```
