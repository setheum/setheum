use move_core_types::{account_address::AccountAddress, vm_status::StatusCode};
use crate::types::VmResult;

/// Trait for Setheum-specific handlers.
///
/// This provides access to Setheum pallets (Swap, NFT, MultiCurrency) from MoveVM.
pub trait SetheumHandler {
    type Error: Into<StatusCode>;

    // --- MultiCurrency ---
    fn get_currency_balance(&self, currency_id: u32, account: AccountAddress) -> Result<u128, Self::Error>;
    fn transfer_currency(&self, currency_id: u32, src: AccountAddress, dst: AccountAddress, amount: u128) -> Result<bool, Self::Error>;

    // --- Swap / DEX ---
    fn swap_exact_tokens_for_tokens(&self, path: Vec<u32>, amount_in: u128, min_amount_out: u128) -> Result<u128, Self::Error>;

    // --- NFT ---
    fn get_nft_owner(&self, collection_id: u32, item_id: u32) -> Result<Option<AccountAddress>, Self::Error>;
    fn transfer_nft(&self, collection_id: u32, item_id: u32, src: AccountAddress, dst: AccountAddress) -> Result<bool, Self::Error>;
}

/// Dummy implementation for genesis/tests.
pub(crate) struct DummySetheumHandler;

impl SetheumHandler for DummySetheumHandler {
    type Error = StatusCode;

    fn get_currency_balance(&self, _currency_id: u32, _account: AccountAddress) -> Result<u128, Self::Error> {
        unreachable!()
    }
    fn transfer_currency(&self, _currency_id: u32, _src: AccountAddress, _dst: AccountAddress, _amount: u128) -> Result<bool, Self::Error> {
        unreachable!()
    }
    fn swap_exact_tokens_for_tokens(&self, _path: Vec<u32>, _amount_in: u128, _min_amount_out: u128) -> Result<u128, Self::Error> {
        unreachable!()
    }
    fn get_nft_owner(&self, _collection_id: u32, _item_id: u32) -> Result<Option<AccountAddress>, Self::Error> {
        unreachable!()
    }
    fn transfer_nft(&self, _collection_id: u32, _item_id: u32, _src: AccountAddress, _dst: AccountAddress) -> Result<bool, Self::Error> {
        unreachable!()
    }
}
