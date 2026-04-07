use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Profile a transaction from a JSON-RPC endpoint
    Profile {
        /// The transaction hash
        #[arg(short, long)]
        tx: String,

        /// RPC endpoint URL
        #[arg(short, long, default_value = "http://localhost:8545")]
        rpc: String,
    },
    /// Compare two transaction traces
    Diff {
        /// Base transaction hash
        #[arg(short, long)]
        base: String,

        /// Target transaction hash
        #[arg(short, long)]
        target: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    println!("{}", "Ethos: High-Fidelity Ethereum Tracing Suite".bold().cyan());

    match &cli.command {
        Commands::Profile { tx, rpc } => {
            println!("Profiling transaction: {} on {}", tx.green(), rpc.yellow());
        }
        Commands::Diff { base, target } => {
            println!("Comparing traces: {} and {}", base.green(), target.yellow());
        }
    }

    Ok(())
}
