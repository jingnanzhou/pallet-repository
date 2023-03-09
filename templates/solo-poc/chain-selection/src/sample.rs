
//! Polkadot chain configurations.

pub use solo_poc_sample_runtime as solo_poc_runtime;

pub use solo_poc_sample_chain_spec::ChainSpec as ChainSpec;


pub fn dev_chain_spec()  ->  Result<solo_poc_sample_chain_spec::ChainSpec, String>  {
    solo_poc_sample_chain_spec::development_config()
}

pub fn test_chain_spec()  ->  Result<solo_poc_sample_chain_spec::ChainSpec, String> {
    solo_poc_sample_chain_spec::local_testnet_config()
}

