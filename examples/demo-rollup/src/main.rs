use core::fmt::Debug as DebugTrait;
use std::env;
use std::path::Path;
use std::str::FromStr;

use anyhow::{anyhow, Context as _};
use bitcoin_da::service::{BitcoinService, DaServiceConfig};
use bitcoin_da::spec::RollupParams;
use chainway_sequencer::ChainwaySequencer;
use clap::Parser;
use const_rollup_config::{ROLLUP_NAME, TEST_PRIVATE_KEY};
use demo_stf::genesis_config::GenesisPaths;
use reth_primitives::hex;
use reth_primitives::serde_helper::num::from_int_or_hex;
use sequencer_client::SequencerClient;
use serde::de::DeserializeOwned;
use sov_celestia_adapter::{CelestiaConfig, CelestiaService};
use sov_demo_rollup::{initialize_logging, BitcoinRollup, CelestiaDemoRollup, MockDemoRollup};
use sov_mock_da::{MockDaConfig, MockDaService};
use sov_modules_api::default_context::DefaultContext;
use sov_modules_api::default_signature::private_key::DefaultPrivateKey;
use sov_modules_api::{DaSpec, PrivateKey, Spec};
use sov_modules_rollup_blueprint::{Rollup, RollupAndStorage, RollupBlueprint};
use sov_modules_stf_blueprint::kernels::basic::{
    BasicKernelGenesisConfig, BasicKernelGenesisPaths,
};
use sov_rollup_interface::services::da::DaService;
use sov_rollup_interface::stf::ProverConfig;
use sov_state::storage::NativeStorage;
use sov_state::{DefaultStorageSpec, ProverStorage, Storage};
use sov_stf_runner::{from_toml_path, RollupConfig, RollupProverConfig};
use tracing::log::debug;

#[cfg(test)]
mod test_rpc;

/// Main demo runner. Initializes a DA chain, and starts a demo-rollup using the provided.
/// If you're trying to sign or submit transactions to the rollup, the `sov-cli` binary
/// is the one you want. You can run it `cargo run --bin sov-cli`.

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The data layer type.
    #[arg(long, default_value = "mock")]
    da_layer: SupportedDaLayer,

    /// The path to the rollup config.
    #[arg(long, default_value = "mock_rollup_config.toml")]
    rollup_config_path: String,

    /// If set, runs the node in sequencer mode, otherwise in full node mode.
    #[arg(long)]
    sequence: bool,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum SupportedDaLayer {
    Celestia,
    Mock,
    Bitcoin,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum SupportedDaLayer {
    Celestia,
    Mock,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    initialize_logging();

    let args = Args::parse();
    let rollup_config_path = args.rollup_config_path.as_str();

    match args.da_layer {
        SupportedDaLayer::Mock => {
            start_rollup::<MockDemoRollup, MockDaConfig>(
                &GenesisPaths::from_dir("../test-data/genesis/demo-tests/mock"),
                &BasicKernelGenesisPaths {
                    chain_state: "../test-data/genesis/demo-tests/mock/chain_state.json".into(),
                },
                rollup_config_path,
                RollupProverConfig::Execute,
                args.sequence,
            )
            .await?;
        }
        SupportedDaLayer::Bitcoin => {
            start_rollup::<BitcoinRollup, DaServiceConfig>(
                &GenesisPaths::from_dir("../test-data/genesis/demo-tests/bitcoin"),
                &BasicKernelGenesisPaths {
                    chain_state: "../test-data/genesis/demo-tests/mock/chain_state.json".into(),
                },
                rollup_config_path,
                RollupProverConfig::Execute,
                args.sequence,
            )
            .await?;
        }
        SupportedDaLayer::Celestia => {
            start_rollup::<CelestiaDemoRollup, CelestiaConfig>(
                &GenesisPaths::from_dir("../test-data/genesis/demo-tests/celestia"),
                &BasicKernelGenesisPaths {
                    chain_state: "../test-data/genesis/demo-tests/mock/chain_state.json".into(),
                },
                rollup_config_path,
                RollupProverConfig::Execute,
                args.sequence,
            )
            .await?;
        }
    }

    Ok(())
}

async fn start_rollup<S, DaC>(
    rt_genesis_paths: &GenesisPaths,
    kernel_genesis_paths: &BasicKernelGenesisPaths,
    rollup_config_path: &str,
    prover_config: RollupProverConfig,
    // genesis_paths: &<<S as RollupBlueprint>::NativeRuntime as sov_modules_stf_blueprint::Runtime<
    //     <S as RollupBlueprint>::NativeContext,
    //     <S as RollupBlueprint>::DaSpec,
    // >>::GenesisPaths,
    is_sequencer: bool,
) -> Result<(), anyhow::Error>
where
    DaC: serde::de::DeserializeOwned + DebugTrait + Clone,
    S: RollupBlueprint<DaConfig = DaC>,
    <<S as RollupBlueprint>::NativeContext as Spec>::Storage: NativeStorage,
{
    let mut rollup_config: RollupConfig<DaC> = from_toml_path(rollup_config_path)
        .context("Failed to read rollup configuration")
        .unwrap();
    let rollup_blueprint = S::new();
    let da_service = rollup_blueprint.create_da_service(&rollup_config).await;

    if is_sequencer {
        rollup_config.sequencer_client = None;
    }

    let kernel_genesis = BasicKernelGenesisConfig {
        chain_state: serde_json::from_str(
            &std::fs::read_to_string(&kernel_genesis_paths.chain_state)
                .context("Failed to read chain state")?,
        )?,
    };
    let RollupAndStorage { rollup, storage } = rollup_blueprint
        .create_new_rollup(
            rt_genesis_paths,
            kernel_genesis,
            rollup_config,
            prover_config,
        )
        .await
        .unwrap();

    if is_sequencer {
        let mut seq: ChainwaySequencer<
            <S as RollupBlueprint>::NativeContext,
            <S as RollupBlueprint>::DaService,
            S,
        > = ChainwaySequencer::new(
            rollup,
            da_service,
            <<<S as RollupBlueprint>::NativeContext as Spec>::PrivateKey as TryFrom<&[u8]>>::try_from(
                hex::decode(TEST_PRIVATE_KEY).unwrap().as_slice(),
            )
            .unwrap(),
            storage,
        );
        seq.start_rpc_server(None).await?;
        seq.run().await?;
    } else {
        if rollup_config.sequencer_client.is_none() {
            return Err(anyhow!("Must have sequencer client for full nodes!"));
        }
        rollup.run().await?;
    }

    Ok(())
}
