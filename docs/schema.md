# Ethos: JSON Schema Reference

Ethos uses a standardized JSON format for transaction profiles and diff reports. This document defines the schema for version `1.0.0`.

## Transaction Profile Schema

The transaction profile is the primary output format from the `ethos profile` command.

```json
{
  "version": "1.0.0",
  "transaction_hash": "0x...",
  "total_gas_used": 123456,
  "generated_at": "2024-03-24T12:00:00Z",
  "hot_paths": [
    {
      "name": "Contract::Function",
      "gas_cost": 50000,
      "percentage": 40.5,
      "source_hint": {
        "file": "contracts/MyContract.sol",
        "line": 123,
        "function": "withdraw"
      }
    }
  ],
  "evm_hotspots": {
    "sstore_count": 5,
    "sload_count": 12,
    "precompile_calls": 2,
    "external_calls": 3
  },
  "all_stacks": [
    {
      "stack": ["Main", "SubCall", "InternalFunction"],
      "value": 25000
    }
  ]
}
```

## Diff Report Schema

The diff report is generated when comparing two transaction profiles using the `ethos diff` command.

```json
{
  "base_hash": "0xABC...",
  "head_hash": "0xDEF...",
  "gas_delta": 1500,
  "gas_percentage_change": 1.2,
  "hotpath_deltas": [
    {
      "name": "Contract::Function",
      "base_gas": 50000,
      "head_gas": 51500,
      "delta": 1500
    }
  ]
}
```

## Threshold Configuration (`thresholds.toml`)

Users define budgets in a `thresholds.toml` file for CI gating.

```toml
[policies.core_transfer]
max_gas = 100000
max_sstore = 2
max_sload = 5
max_gas_delta_percent = 5.0

[policies.complex_interaction]
max_gas = 500000
max_external_calls = 10
```
