//! A collection of node-specific RPC methods.

use std::sync::Arc;

pub use sc_rpc_api::DenyUnsafe;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_transaction_pool::TransactionPool;
use toph_runtime::{opaque::Block, AccountId, Balance, Index};

/// Full client dependencies.
#[derive(Debug)]
pub struct FullDeps<C, P> {
  /// The client instance to use.
  pub client: Arc<C>,
  /// Whether to deny unsafe calls
  pub deny_unsafe: DenyUnsafe,
  /// Transaction pool instance.
  pub pool: Arc<P>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(deps: FullDeps<C, P>) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
where
  C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
  C: ProvideRuntimeApi<Block>,
  C: Send + Sync + 'static,
  C::Api: BlockBuilder<Block>,
  C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
  C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
  P: TransactionPool + 'static,
{
  use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
  use substrate_frame_rpc_system::{FullSystem, SystemApi};

  let mut io = jsonrpc_core::IoHandler::default();
  let FullDeps { client, pool, deny_unsafe } = deps;

  io.extend_with(SystemApi::to_delegate(FullSystem::new(client.clone(), pool, deny_unsafe)));

  io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(client)));

  io
}
