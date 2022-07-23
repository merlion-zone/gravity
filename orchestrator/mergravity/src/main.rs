extern crate core;

use clap::Parser;
use env_logger::Env;
use gbt::{config, args as gargs, keys};
use config::{init_config, get_home_dir, load_config};
use gargs::KeysSubcommand;
use keys::register_orchestrator_address::register_orchestrator_address;
use keys::show_keys;
use keys::set_eth_key;
use keys::set_orchestrator_key;
use gbt::{orchestrator::orchestrator, relayer::relayer};

mod args;

use args::{Opts, SubCommand};

#[actix_rt::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // On Linux static builds we need to probe ssl certs path to be able to
    // do TLS stuff.
    openssl_probe::init_ssl_cert_env_vars();
    // parse the arguments
    let opts: Opts = Opts::parse();

    // handle global config here
    let address_prefix = opts.address_prefix;
    let home_dir = get_home_dir(opts.home);
    let config = load_config(&home_dir);

    // control flow for the command structure
    match opts.subcmd {
        SubCommand::Init(init_opts) => init_config(init_opts, home_dir),
        SubCommand::Keys(key_opts) => match key_opts.subcmd {
            KeysSubcommand::RegisterOrchestratorAddress(set_orchestrator_address_opts) => {
                register_orchestrator_address(
                    set_orchestrator_address_opts,
                    address_prefix,
                    home_dir,
                )
                    .await
            }
            KeysSubcommand::Show => show_keys(&home_dir, &address_prefix),
            KeysSubcommand::SetEthereumKey(set_eth_key_opts) => {
                set_eth_key(&home_dir, set_eth_key_opts)
            }
            KeysSubcommand::SetOrchestratorKey(set_orch_key_opts) => {
                set_orchestrator_key(&home_dir, set_orch_key_opts)
            }
        },
        SubCommand::Orchestrator(orchestrator_opts) => {
            check_env().unwrap();
            orchestrator(orchestrator_opts, address_prefix, &home_dir, config).await
        }
        SubCommand::Relayer(relayer_opts) => {
            check_env().unwrap();
            relayer(relayer_opts, address_prefix, &home_dir, config.relayer).await
        }
    }
}

fn check_env() -> Result<(), &'static str> {
    use cosmos_gravity::CHAIN_IDENTIFIER;
    if CHAIN_IDENTIFIER.is_empty() {
        Err("Environment variable CHAIN_IDENTIFIER must be specified")
    } else {
        Ok(())
    }
}
