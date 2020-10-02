use crate::chain_spec::{
  account_id_from_ss58, properties, public_key_from_ss58, ChainSpec, GenesisConfigBuilder,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::sr25519;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use toph_runtime::WASM_BINARY;

const AURA_SS58: &str = "5Do78B3k99QvEjpeyf8WrkJCqr2EwJk66C1sz5v3oT1RyLeD";
const GRANDPA_SS58: &str = "5FiDC12iX9tT9enK5kDBt4nKjpfQ7jJqqXG2GjjNogaoyNji";

pub fn testnet_joana() -> Result<ChainSpec, String> {
  let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

  let endowed_accounts = vec![account_id_from_ss58::<sr25519::Public>(AURA_SS58)?];
  let initial_authorities = vec![(
    public_key_from_ss58::<AuraId>(AURA_SS58)?,
    public_key_from_ss58::<GrandpaId>(GRANDPA_SS58)?,
  )];
  let sudo_key = account_id_from_ss58::<sr25519::Public>(GRANDPA_SS58)?;

  Ok(ChainSpec::from_genesis(
    "Joana Testnet",
    "joana_testnet",
    ChainType::Live,
    move || {
      GenesisConfigBuilder {
        endowed_accounts: &endowed_accounts,
        initial_authorities: &initial_authorities,
        sudo_key: sudo_key.clone(),
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
