use crate::{
  chain_spec,
  cli::{Cli, Subcommand},
  service,
};
use sc_cli::{Role, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;

impl SubstrateCli for Cli {
  fn author() -> String {
    env!("CARGO_PKG_AUTHORS").into()
  }

  fn copyright_start_year() -> i32 {
    2017
  }

  fn description() -> String {
    env!("CARGO_PKG_DESCRIPTION").into()
  }

  fn impl_name() -> String {
    "Toph Node".into()
  }

  fn impl_version() -> String {
    env!("SUBSTRATE_CLI_IMPL_VERSION").into()
  }

  fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
    Ok(match id {
      "" | "local_testnet" => Box::new(chain_spec::local_testnet::local_testnet()?),
      "dev" => Box::new(chain_spec::dev::dev()?),
      "testnet_joana" => Box::new(chain_spec::testnet_joana::testnet_joana()?),
      path => Box::new(chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?),
    })
  }

  fn native_runtime_version(_: &Box<dyn sc_cli::ChainSpec>) -> &'static RuntimeVersion {
    &toph_runtime::VERSION
  }

  fn support_url() -> String {
    "support.anonymous.an".into()
  }
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
  let cli = Cli::from_args();

  match &cli.subcommand {
    None => {
      let runner = cli.create_runner(&cli.run)?;
      runner.run_node_until_exit(|config| match config.role {
        Role::Light => service::new_light(config),
        _ => service::new_full(config),
      })
    }
    #[cfg(feature = "runtime-benchmarks")]
    Some(Subcommand::Benchmark(cmd)) => {
      let runner = cli.create_runner(cmd)?;
      runner.sync_run(|config| cmd.run::<toph_runtime::Block, service::Executor>(config))
    }
    Some(Subcommand::BuildSpec(cmd)) => {
      let runner = cli.create_runner(cmd)?;
      runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
    }
    Some(Subcommand::CheckBlock(cmd)) => {
      let runner = cli.create_runner(cmd)?;
      runner.async_run(|config| {
        let PartialComponents { client, task_manager, import_queue, .. } =
          service::new_partial(&config)?;
        Ok((cmd.run(client, import_queue), task_manager))
      })
    }
    Some(Subcommand::ExportBlocks(cmd)) => {
      let runner = cli.create_runner(cmd)?;
      runner.async_run(|config| {
        let PartialComponents { client, task_manager, .. } = service::new_partial(&config)?;
        Ok((cmd.run(client, config.database), task_manager))
      })
    }
    Some(Subcommand::ExportState(cmd)) => {
      let runner = cli.create_runner(cmd)?;
      runner.async_run(|config| {
        let PartialComponents { client, task_manager, .. } = service::new_partial(&config)?;
        Ok((cmd.run(client, config.chain_spec), task_manager))
      })
    }
    Some(Subcommand::ImportBlocks(cmd)) => {
      let runner = cli.create_runner(cmd)?;
      runner.async_run(|config| {
        let PartialComponents { client, task_manager, import_queue, .. } =
          service::new_partial(&config)?;
        Ok((cmd.run(client, import_queue), task_manager))
      })
    }
    Some(Subcommand::PurgeChain(cmd)) => {
      let runner = cli.create_runner(cmd)?;
      runner.sync_run(|config| cmd.run(config.database))
    }
    Some(Subcommand::Revert(cmd)) => {
      let runner = cli.create_runner(cmd)?;
      runner.async_run(|config| {
        let PartialComponents { client, task_manager, backend, .. } =
          service::new_partial(&config)?;
        Ok((cmd.run(client, backend), task_manager))
      })
    }
  }
}
