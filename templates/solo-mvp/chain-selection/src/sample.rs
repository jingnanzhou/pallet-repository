
//! Polkadot chain configurations.

pub use solo_mvp_sample_runtime as solo_mvp_runtime;

pub use solo_mvp_sample_chain_spec::ChainSpec as ChainSpec;

pub fn dev_chain_spec()  ->  solo_mvp_sample_chain_spec::ChainSpec  {
    solo_mvp_sample_chain_spec::development_config()
}

pub fn test_chain_spec()  ->  solo_mvp_sample_chain_spec::ChainSpec {
    solo_mvp_sample_chain_spec::local_testnet_config()
}

pub fn fir_chain_spec()  ->  Result<solo_mvp_sample_chain_spec::ChainSpec, String> {
    solo_mvp_sample_chain_spec::flaming_fir_config()
}

pub fn staging_chain_spec()  -> solo_mvp_sample_chain_spec::ChainSpec {
    solo_mvp_sample_chain_spec::staging_testnet_config()
}

