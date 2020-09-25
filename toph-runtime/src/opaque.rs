//! Opaque types.

use sp_runtime::{
  generic, impl_opaque_keys, traits::BlakeTwo256, OpaqueExtrinsic as UncheckedExtrinsic,
};
use sp_std::prelude::Vec;

/// Opaque block header type.
pub type Header = generic::Header<crate::BlockNumber, BlakeTwo256>;
/// Opaque block type.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// Opaque block identifier type.
pub type BlockId = generic::BlockId<Block>;

impl_opaque_keys! {
  pub struct SessionKeys {
    pub aura: crate::Aura,
    pub grandpa: crate::Grandpa,
  }
}
