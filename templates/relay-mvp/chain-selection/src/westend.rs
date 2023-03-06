
//! Polkadot chain configurations.

pub use westend_chain_runtime as relay_mvp_runtime;
pub use westend_chain_runtime_constants as relay_mvp_runtime_constants;

pub use westend_chain_spec::WestendChainSpec as RelayChainSpec;

pub fn prod_chain_spec()  -> Result<westend_chain_spec::WestendChainSpec, String> {
    westend_chain_spec::westend_config()
}

pub fn dev_chain_spec()  -> Result<westend_chain_spec::WestendChainSpec, String> {
    westend_chain_spec::westend_development_config()
}

pub fn local_chain_spec()  -> Result<westend_chain_spec::WestendChainSpec, String> {
    westend_chain_spec::westend_local_testnet_config()
}

pub fn staging_chain_spec()  -> Result<westend_chain_spec::WestendChainSpec, String> {
    westend_chain_spec::westend_staging_testnet_config()
}

