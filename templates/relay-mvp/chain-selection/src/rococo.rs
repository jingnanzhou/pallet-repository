
//! Polkadot chain configurations.

pub use rococo_chain_runtime as relay_mvp_runtime;
pub use rococo_chain_runtime_constants as relay_mvp_runtime_constants;

pub use rococo_chain_spec::RococoChainSpec as RelayChainSpec;

pub fn prod_chain_spec()  -> Result<RelayChainSpec, String> {
    rococo_chain_spec::rococo_config()
}

pub fn dev_chain_spec()  -> Result<RelayChainSpec, String> {
    rococo_chain_spec::rococo_development_config()
}

pub fn local_chain_spec()  -> Result<RelayChainSpec, String> {
    rococo_chain_spec::rococo_local_testnet_config()
}

pub fn staging_chain_spec()  -> Result<RelayChainSpec, String> {
    rococo_chain_spec::rococo_staging_testnet_config()
}

