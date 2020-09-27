//! Toph runtime

#![allow(clippy::large_enum_variant)]
#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

mod currency;
pub mod opaque;

use currency::{deposit, MILLICENTS};
use frame_support::{
  construct_runtime, parameter_types,
  traits::{KeyOwnerProofSystem, Randomness},
  weights::{
    constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
    IdentityFee, Weight,
  },
};
use pallet_contracts_rpc_runtime_api::ContractExecResult;
use pallet_grandpa::{
  fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
  create_runtime_str, generic,
  traits::{
    BlakeTwo256, Block as BlockT, IdentifyAccount, IdentityLookup, NumberFor, Saturating, Verify,
  },
  transaction_validity::{TransactionSource, TransactionValidity},
  ApplyExtrinsicResult, MultiSignature, Perbill,
};
use sp_std::prelude::{Box, Vec};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

pub const DAYS: BlockNumber = HOURS * 24;
pub const HOURS: BlockNumber = MINUTES * 60;
pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
pub const VERSION: RuntimeVersion = RuntimeVersion {
  spec_name: create_runtime_str!("toph-node"),
  impl_name: create_runtime_str!("toph-node"),
  authoring_version: 1,
  spec_version: 1,
  impl_version: 1,
  apis: RUNTIME_API_VERSIONS,
  transaction_version: 1,
};

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type AccountIndex = u32;
pub type Address = AccountId;
pub type Balance = u128;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type BlockId = generic::BlockId<Block>;
pub type BlockNumber = u32;
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
pub type DigestItem = generic::DigestItem<Hash>;
pub type Executive = frame_executive::Executive<
  Runtime,
  Block,
  frame_system::ChainContext<Runtime>,
  Runtime,
  AllModules,
>;
pub type Hash = sp_core::H256;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Index = u32;
pub type Signature = MultiSignature;
pub type SignedBlock = generic::SignedBlock<Block>;
pub type SignedExtra = (
  frame_system::CheckSpecVersion<Runtime>,
  frame_system::CheckTxVersion<Runtime>,
  frame_system::CheckGenesis<Runtime>,
  frame_system::CheckEra<Runtime>,
  frame_system::CheckNonce<Runtime>,
  frame_system::CheckWeight<Runtime>,
  pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;

construct_runtime!(
  pub enum Runtime where
    Block = Block,
    NodeBlock = crate::opaque::Block,
    UncheckedExtrinsic = UncheckedExtrinsic
  {
    Aura: pallet_aura::{Module, Config<T>, Inherent},
    Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
    Contracts: pallet_contracts::{Module, Call, Config, Storage, Event<T>},
    Grandpa: pallet_grandpa::{Module, Call, Storage, Config, Event},
    Multisig: pallet_multisig::{Module, Call, Storage, Event<T>},
    RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
    Sudo: pallet_sudo::{Module, Call, Config<T>, Storage, Event<T>},
    System: frame_system::{Module, Call, Config, Storage, Event<T>},
    Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
    TransactionPayment: pallet_transaction_payment::{Module, Storage},
  }
);

parameter_types! {
  pub const BalancesExistentialDeposit: u128 = 500;
  pub const BalancesMaxLocks: u32 = 50;
  pub const ContractsRentByteFee: Balance = 4 * MILLICENTS;
  pub const ContractsRentDepositOffset: Balance = 1000 * MILLICENTS;
  pub const ContractsSurchargeReward: Balance = 150 * MILLICENTS;
  pub const ContractsTombstoneDeposit: Balance = 16 * MILLICENTS;
  pub const MultisignDepositBase: Balance = deposit(1, 88);
  pub const MultisignDepositFactor: Balance = deposit(0, 32);
  pub const MultisignMaxSignatories: u16 = 100;
  pub const SystemAvailableBlockRatio: Perbill = Perbill::from_percent(75);
  pub const SystemBlockHashCount: BlockNumber = 2400;
  pub const SystemMaximumBlockLength: u32 = 5 * 1024 * 1024;
  pub const SystemMaximumBlockWeight: Weight = 2 * WEIGHT_PER_SECOND;
  pub const SystemVersion: RuntimeVersion = VERSION;
  pub const TimestampMinimumPeriod: u64 = SLOT_DURATION / 2;
  pub const TransactionPaymentTransactionByteFee: Balance = 1;

  pub MaximumExtrinsicWeight: Weight = {
    let percent = Perbill::from_percent(10);
    SystemAvailableBlockRatio::get().saturating_sub(percent) * SystemMaximumBlockWeight::get()
  };
}

impl pallet_aura::Trait for Runtime {
  type AuthorityId = AuraId;
}

impl pallet_balances::Trait for Runtime {
  type MaxLocks = BalancesMaxLocks;
  /// The type for recording an account's balance.
  type Balance = Balance;
  /// The ubiquitous event type.
  type Event = Event;
  type DustRemoval = ();
  type ExistentialDeposit = BalancesExistentialDeposit;
  type AccountStore = System;
  type WeightInfo = ();
}

impl pallet_grandpa::Trait for Runtime {
  type Call = Call;

  type Event = Event;

  type HandleEquivocation = ();

  type KeyOwnerProofSystem = ();

  type KeyOwnerProof =
    <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

  type KeyOwnerIdentification =
    <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::IdentificationTuple;

  type WeightInfo = ();
}

impl pallet_sudo::Trait for Runtime {
  type Call = Call;
  type Event = Event;
}

impl frame_system::Trait for Runtime {
  type AccountData = pallet_balances::AccountData<Balance>;
  type AccountId = AccountId;
  type AvailableBlockRatio = SystemAvailableBlockRatio;
  type BaseCallFilter = ();
  type BlockExecutionWeight = BlockExecutionWeight;
  type BlockHashCount = SystemBlockHashCount;
  type BlockNumber = BlockNumber;
  type Call = Call;
  type DbWeight = RocksDbWeight;
  type Event = Event;
  type ExtrinsicBaseWeight = ExtrinsicBaseWeight;
  type Hash = Hash;
  type Hashing = BlakeTwo256;
  type Header = generic::Header<BlockNumber, BlakeTwo256>;
  type Index = Index;
  type Lookup = IdentityLookup<AccountId>;
  type MaximumBlockLength = SystemMaximumBlockLength;
  type MaximumBlockWeight = SystemMaximumBlockWeight;
  type MaximumExtrinsicWeight = MaximumExtrinsicWeight;
  type OnKilledAccount = ();
  type OnNewAccount = ();
  type Origin = Origin;
  type PalletInfo = PalletInfo;
  type SystemWeightInfo = ();
  type Version = SystemVersion;
}

impl pallet_timestamp::Trait for Runtime {
  type MinimumPeriod = TimestampMinimumPeriod;
  type Moment = u64;
  type OnTimestampSet = Aura;
  type WeightInfo = ();
}

impl pallet_transaction_payment::Trait for Runtime {
  type Currency = Balances;
  type FeeMultiplierUpdate = ();
  type OnTransactionPayment = ();
  type TransactionByteFee = TransactionPaymentTransactionByteFee;
  type WeightToFee = IdentityFee<Balance>;
}

impl_runtime_apis! {
  impl fg_primitives::GrandpaApi<Block> for Runtime {
    fn generate_key_ownership_proof(
      _set_id: fg_primitives::SetId,
      _authority_id: GrandpaId,
    ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
      None
    }

    fn grandpa_authorities() -> GrandpaAuthorityList {
      Grandpa::grandpa_authorities()
    }

    fn submit_report_equivocation_unsigned_extrinsic(
      _equivocation_proof: fg_primitives::EquivocationProof<
        <Block as BlockT>::Hash,
        NumberFor<Block>,
      >,
      _key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
    ) -> Option<()> {
      None
    }
  }

  #[cfg(feature = "runtime-benchmarks")]
  impl frame_benchmarking::Benchmark<Block> for Runtime {
    fn dispatch_benchmark(
      config: frame_benchmarking::BenchmarkConfig
    ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
      use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

      use frame_system_benchmarking::Module as SystemBench;
      impl frame_system_benchmarking::Trait for Runtime {}

      let whitelist: Vec<TrackedStorageKey> = sp_std::vec![
        // Block Number
        hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
        // Total Issuance
        hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
        // Execution Phase
        hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
        // Event Count
        hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
        // System Events
        hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
      ];

      let mut batches = Vec::<BenchmarkBatch>::new();
      let params = (&config, &whitelist);

      add_benchmark!(params, batches, pallet_contracts, Contracts);
      add_benchmark!(params, batches, pallet_grandpa, Grandpa);
      add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
      add_benchmark!(params, batches, pallet_balances, Balances);
      add_benchmark!(params, batches, pallet_multisig, Multisig);
      add_benchmark!(params, batches, pallet_timestamp, Timestamp);

      if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
      Ok(batches)
    }
  }

  impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
    fn account_nonce(account: AccountId) -> Index {
      System::account_nonce(account)
    }
  }

  impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance, BlockNumber>
    for Runtime
  {
    fn call(
      origin: AccountId,
      dest: AccountId,
      value: Balance,
      gas_limit: u64,
      input_data: Vec<u8>,
    ) -> ContractExecResult {
      let (exec_result, gas_consumed) = Contracts::bare_call(origin, dest.into(), value, gas_limit, input_data);
      match exec_result {
        Ok(v) => ContractExecResult::Success {
          flags: v.flags.bits(),
          data: v.data,
          gas_consumed: gas_consumed,
        },
        Err(_) => ContractExecResult::Error,
      }
    }

    fn get_storage(
      address: AccountId,
      key: [u8; 32],
    ) -> pallet_contracts_primitives::GetStorageResult {
      Contracts::get_storage(address, key)
    }

    fn rent_projection(
      address: AccountId,
    ) -> pallet_contracts_primitives::RentProjectionResult<BlockNumber> {
      Contracts::rent_projection(address)
    }
  }

  impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
    fn query_info(
      uxt: <Block as BlockT>::Extrinsic,
      len: u32,
    ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
      TransactionPayment::query_info(uxt, len)
    }
  }

  impl sp_api::Core<Block> for Runtime {
    fn execute_block(block: Block) {
      Executive::execute_block(block)
    }

    fn initialize_block(header: &<Block as BlockT>::Header) {
      Executive::initialize_block(header)
    }

    fn version() -> RuntimeVersion {
      VERSION
    }
  }

  impl sp_api::Metadata<Block> for Runtime {
    fn metadata() -> OpaqueMetadata {
      Runtime::metadata().into()
    }
  }

  impl sp_block_builder::BlockBuilder<Block> for Runtime {
    fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
      Executive::apply_extrinsic(extrinsic)
    }

    fn check_inherents(
      block: Block,
      data: sp_inherents::InherentData,
    ) -> sp_inherents::CheckInherentsResult {
      data.check_extrinsics(&block)
    }

    fn finalize_block() -> <Block as BlockT>::Header {
      Executive::finalize_block()
    }

    fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
      data.create_extrinsics()
    }

    fn random_seed() -> <Block as BlockT>::Hash {
      RandomnessCollectiveFlip::random_seed()
    }
  }

  impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
    fn authorities() -> Vec<AuraId> {
      Aura::authorities()
    }

    fn slot_duration() -> u64 {
      Aura::slot_duration()
    }
  }

  impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
    fn offchain_worker(header: &<Block as BlockT>::Header) {
      Executive::offchain_worker(header)
    }
  }

  impl sp_session::SessionKeys<Block> for Runtime {
    fn decode_session_keys(
      encoded: Vec<u8>,
    ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
      crate::opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
    }

    fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
      crate::opaque::SessionKeys::generate(seed)
    }
  }

  impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
    fn validate_transaction(
      source: TransactionSource,
      tx: <Block as BlockT>::Extrinsic,
    ) -> TransactionValidity {
      Executive::validate_transaction(source, tx)
    }
  }
}

impl pallet_contracts::Trait for Runtime {
  type Currency = Balances;
  type DetermineContractAddress = pallet_contracts::SimpleAddressDeterminer<Runtime>;
  type Event = Event;
  type MaxDepth = pallet_contracts::DefaultMaxDepth;
  type MaxValueSize = pallet_contracts::DefaultMaxValueSize;
  type Randomness = RandomnessCollectiveFlip;
  type RentByteFee = ContractsRentByteFee;
  type RentDepositOffset = ContractsRentDepositOffset;
  type RentPayment = ();
  type SignedClaimHandicap = pallet_contracts::DefaultSignedClaimHandicap;
  type StorageSizeOffset = pallet_contracts::DefaultStorageSizeOffset;
  type SurchargeReward = ContractsSurchargeReward;
  type Time = Timestamp;
  type TombstoneDeposit = ContractsTombstoneDeposit;
  type TrieIdGenerator = pallet_contracts::TrieIdFromParentCounter<Runtime>;
  type WeightPrice = pallet_transaction_payment::Module<Self>;
}

impl pallet_multisig::Trait for Runtime {
  type Event = Event;
  type Call = Call;
  type Currency = Balances;
  type DepositBase = MultisignDepositBase;
  type DepositFactor = MultisignDepositFactor;
  type MaxSignatories = MultisignMaxSignatories;
  type WeightInfo = ();
}

#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
  NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}
