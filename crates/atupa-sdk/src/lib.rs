//! # Atupa
//!
//! **Unified Ethereum Execution Profiler** — the top-level façade crate for the
//! Atupa SDK. This crate re-exports every layer of the suite so that external
//! integrators only need to depend on a single crate:
//!
//! ```toml
//! [dependencies]
//! atupa = "0.1"
//! ```
//!
//! ## Crate Architecture
//!
//! ```text
//! atupa (this façade)
//! ├── atupa-core      → Types: TraceStep, CollapsedStack, GasCategory
//! ├── atupa-rpc       → JSON-RPC client (EthClient, EtherscanResolver)
//! ├── atupa-parser    → StructLog → TraceStep normalization
//! ├── atupa-adapters  → ProtocolAdapter trait
//! ├── atupa-output    → SvgGenerator flamegraphs
//! ├── atupa-aave      → AaveDeepTracer, GHO metrics
//! └── atupa-lido      → LidoDeepTracer, stETH / wstETH tracing
//! ```

// ─── Public re-exports ────────────────────────────────────────────────────────

/// Core types shared across the entire Atupa suite.
pub use atupa_core as core;

/// JSON-RPC transport layer: EthClient, EtherscanResolver, RawStructLog.
pub use atupa_rpc as rpc;

/// Trace normalization and aggregation.
pub use atupa_parser as parser;

/// ProtocolAdapter trait for pluggable DeFi recognizers.
pub use atupa_adapters as adapters;

/// SVG flamegraph renderer.
pub use atupa_output as output;

/// Aave v3 + GHO protocol adapter.
pub use atupa_aave as aave;

/// Lido stETH protocol adapter.
pub use atupa_lido as lido;

// ─── High-level API ───────────────────────────────────────────────────────────

pub use profile::execute_profile;

/// High-level profile execution logic, usable independently from the CLI.
pub mod profile {
    use anyhow::Result;
    use atupa_core::TraceStep;
    use atupa_output::SvgGenerator;
    use atupa_parser::{Parser as AtupaParser, aggregator::Aggregator};
    use atupa_rpc::{EthClient, etherscan::EtherscanResolver};
    use indicatif::{ProgressBar, ProgressStyle};
    use std::{fs, time::Duration};

    /// Fetch (or generate a demo), aggregate, and render an SVG flamegraph for
    /// the given transaction hash.
    ///
    /// This is the same logic that `atupa profile` runs — exposed here so it can
    /// be called programmatically by other tools or tests.
    pub async fn execute_profile(
        tx: &str,
        rpc: &str,
        is_demo: bool,
        out: Option<String>,
        etherscan_key: Option<String>,
    ) -> Result<()> {
        let pb = make_spinner();

        // 1. Fetch ─────────────────────────────────────────────────────────────
        let steps: Vec<TraceStep> = if is_demo {
            demo_trace(&pb)
        } else {
            fetch_live(&pb, tx, rpc).await?
        };

        // 2. Aggregate ─────────────────────────────────────────────────────────
        pb.set_message("Aggregating execution metrics…");
        let mut stacks = Aggregator::build_collapsed_stacks(&steps);

        // 3. Etherscan resolution ───────────────────────────────────────────────
        pb.set_message("Resolving contract names via Etherscan…");
        let resolver = EtherscanResolver::new(etherscan_key);
        for stack in &mut stacks {
            if let Some(addr) = &stack.target_address
                && let Some(name) = resolver.resolve_contract_name(addr).await
            {
                stack.target_address = Some(name);
            }
        }

        // 4. Render + save ─────────────────────────────────────────────────────
        pb.set_message("Generating SVG flamegraph…");
        let svg = SvgGenerator::generate_flamegraph(&stacks)?;
        let out_path =
            out.unwrap_or_else(|| format!("profile_{}.svg", if is_demo { "demo" } else { tx }));
        fs::write(&out_path, svg)?;

        pb.finish_with_message(format!("✔ Profile saved → {out_path}"));
        Ok(())
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn make_spinner() -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::with_template("{spinner:.cyan} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        pb.enable_steady_tick(Duration::from_millis(80));
        pb
    }

    async fn fetch_live(pb: &ProgressBar, tx: &str, rpc: &str) -> Result<Vec<TraceStep>> {
        pb.set_message("Connecting to EVM node via JSON-RPC…");
        let client = EthClient::new(rpc.to_string());

        let raw = tokio::time::timeout(Duration::from_secs(30), client.get_transaction_trace(tx))
            .await
            .map_err(|_| {
                anyhow::anyhow!("RPC timed out after 30 s — is the node reachable at {rpc}?")
            })?
            .map_err(|e| anyhow::anyhow!("RPC error: {e}\nHint: is your node running at {rpc}?"))?;

        pb.set_message(format!("Normalizing {} structLogs…", raw.struct_logs.len()));
        Ok(AtupaParser::normalize(raw.struct_logs))
    }

    fn demo_trace(pb: &ProgressBar) -> Vec<TraceStep> {
        pb.set_message("Generating offline demo trace…");
        vec![
            TraceStep {
                pc: 0,
                op: "PUSH1".into(),
                gas: 1_000_000,
                gas_cost: 3,
                depth: 1,
                stack: None,
                memory: None,
                error: None,
                reverted: false,
            },
            TraceStep {
                pc: 1,
                op: "CALL".into(),
                gas: 999_997,
                gas_cost: 2_600,
                depth: 1,
                stack: Some(vec![
                    "0x0000000000000000000000000000000000000000".into(),
                    "0x0000000000000000000000000000000000000000".into(),
                    "0x0000000000000000000000000000000000000000".into(),
                    "0x0000000000000000000000000000000000000100".into(),
                    "0x0000000000000000000000000000000000000000".into(),
                    "0x000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".into(), // USDC
                    "0x10000".into(),
                ]),
                memory: None,
                error: None,
                reverted: false,
            },
            TraceStep {
                pc: 0,
                op: "SLOAD".into(),
                gas: 500,
                gas_cost: 2_100,
                depth: 2,
                stack: None,
                memory: None,
                error: None,
                reverted: false,
            },
            TraceStep {
                pc: 1,
                op: "SSTORE".into(),
                gas: 480,
                gas_cost: 20_000,
                depth: 2,
                stack: None,
                memory: None,
                error: None,
                reverted: false,
            },
            TraceStep {
                pc: 2,
                op: "REVERT".into(),
                gas: 400,
                gas_cost: 5_000,
                depth: 2,
                stack: None,
                memory: None,
                error: None,
                reverted: true,
            },
            TraceStep {
                pc: 2,
                op: "STOP".into(),
                gas: 300,
                gas_cost: 0,
                depth: 1,
                stack: None,
                memory: None,
                error: None,
                reverted: false,
            },
        ]
    }
}
