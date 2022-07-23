use std::path::PathBuf;
use clap::Parser;
use gbt::args as gargs;
use gargs::{InitOpts, KeyOpts, OrchestratorOpts, RelayerOpts};

/// mergravity provides oracle/eth-singer/relayer routines for the Merlion blockching.
#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
pub struct Opts {
    /// Increase the logging verbosity
    #[clap(short, long)]
    pub verbose: bool,
    /// Decrease the logging verbosity
    #[clap(short, long)]
    pub quiet: bool,
    /// The home directory for mergravity, by default
    /// $HOME/.mergravity/
    #[clap(short, long, parse(from_str))]
    pub home: Option<PathBuf>,
    /// Set the address prefix for the Merlion chain
    #[clap(short, long, default_value = "mer")]
    pub address_prefix: String,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Parser)]
pub enum SubCommand {
    Init(InitOpts),
    Keys(KeyOpts),
    Orchestrator(OrchestratorOpts),
    Relayer(RelayerOpts),
}
