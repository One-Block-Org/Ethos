# ethos-cli

The user-facing command-line interface for the [Ethos](https://github.com/One-Block-Org/Ethos) Ethereum tracing suite.

## Usage

```bash
# Install
cargo install ethos-cli

# Run a profile
ethos profile --tx <TX_HASH> --rpc <RPC_URL>

# Visualization
ethos profile --tx <TX_HASH> --out profile.svg
```

## Configuration

`ethos-cli` leverages a layered configuration system. You can define your default RPC URL and Etherscan API Key in `ethos.toml` at the root of your project or through environment variables like `ETHOS_RPC_URL`.
