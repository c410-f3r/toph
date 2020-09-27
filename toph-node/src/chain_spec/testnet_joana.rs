use crate::chain_spec::{
  get_account_id_from_seed, properties, public_key_from_ss58, ChainSpec, GenesisConfigBuilder,
  account_id_from_ss58, 
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::sr25519;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use toph_runtime::WASM_BINARY;

pub fn testnet_joana() -> Result<ChainSpec, String> {
  let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

  let endowed_accounts = vec![
    account_id_from_ss58::<sr25519::Public>("5HZ1L1cNLmiapEZrB2zXMF6QwDE3hThQ1QeoKQMDkoRpiopD")?,
    account_id_from_ss58::<sr25519::Public>("5DDVKn4iAAwCd4KcKEfpGBhtWo8SjLyFR64CGHd53azmMNct")?,
  ];
  let initial_authorities = vec![
    (
      public_key_from_ss58::<AuraId>("5HZ1L1cNLmiapEZrB2zXMF6QwDE3hThQ1QeoKQMDkoRpiopD")?,
      public_key_from_ss58::<GrandpaId>("5CG4vDbTqTM7Ky76DfF1P4xkbeScDy1WA2Q8mLt93E9ik2y1")?,
    ),
    (
      public_key_from_ss58::<AuraId>("5DDVKn4iAAwCd4KcKEfpGBhtWo8SjLyFR64CGHd53azmMNct")?,
      public_key_from_ss58::<GrandpaId>("5C5HKC8SWhLKnQxnpEcvCPR7GUPzTQS4FWt36rptxFwHjuRB")?,
    ),
  ];

  Ok(ChainSpec::from_genesis(
    "Joana Testnet",
    "joana_testnet",
    ChainType::Live,
    move || {
      GenesisConfigBuilder {
        endowed_accounts: &endowed_accounts,
        initial_authorities: &initial_authorities,
        sudo_key: get_account_id_from_seed::<sr25519::Public>("Alice"),
        wasm_binary,
      }
      .build()
    },
    vec![],
    None,
    Some("joana"),
    Some(properties()),
    None,
  ))
}
