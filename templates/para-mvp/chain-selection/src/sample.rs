
//! Polkadot chain configurations.

pub use para_sample_runtime as para_mvp_runtime;

pub use para_sample_chain_spec::ChainSpec as ChainSpec;
pub use para_sample_chain_spec::Extensions as Extensions;


pub fn dev_chain_spec()  -> para_sample_chain_spec::ChainSpec {
    para_sample_chain_spec::development_config()
}

pub fn test_chain_spec()  -> para_sample_chain_spec::ChainSpec {
    para_sample_chain_spec::local_testnet_config()
}

