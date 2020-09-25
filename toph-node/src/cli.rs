use sc_cli::RunCmd;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
  #[structopt(flatten)]
  pub run: RunCmd,

  #[structopt(subcommand)]
  pub subcommand: Option<Subcommand>,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
  /// The custom benchmark subcommmand benchmarking runtime pallets.
  #[cfg(feature = "runtime-benchmarks")]
  #[structopt(name = "benchmark", about = "Benchmark runtime pallets.")]
  Benchmark(frame_benchmarking_cli::BenchmarkCmd),

  /// Build a chain specification.
  BuildSpec(sc_cli::BuildSpecCmd),

  /// Validate blocks.
  CheckBlock(sc_cli::CheckBlockCmd),

  /// Export blocks.
  ExportBlocks(sc_cli::ExportBlocksCmd),

  /// Export the state of a given block into a chain spec.
  ExportState(sc_cli::ExportStateCmd),

  /// Import blocks.
  ImportBlocks(sc_cli::ImportBlocksCmd),

  /// Remove the whole chain.
  PurgeChain(sc_cli::PurgeChainCmd),

  /// Revert the chain to a previous state.
  Revert(sc_cli::RevertCmd),
}