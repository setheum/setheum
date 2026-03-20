use frame_support::pallet_prelude::*;
use move_core_types::{account_address::AccountAddress, vm_status::StatusCode};
use move_vm_backend::setheum::SetheumHandler;
use sp_std::{marker::PhantomData, vec::Vec};
use crate::{Config, Pallet};

/// Adapter for Setheum-specific functionality in MoveVM.
pub struct SetheumAdapter<T: Config> {
    _pd: PhantomData<T>,
}

impl<T: Config> SetheumAdapter<T> {
    pub fn new() -> Self {
        Self { _pd: PhantomData }
    }
}

impl<T: Config> SetheumHandler for SetheumAdapter<T> {
    type Error = StatusCode;

    fn get_currency_balance(&self, _currency_id: u32, _account: AccountAddress) -> Result<u128, Self::Error> {
        // TODO: Implement using module-currencies
        Ok(0)
    }

    fn transfer_currency(&self, _currency_id: u32, _src: AccountAddress, _dst: AccountAddress, _amount: u128) -> Result<bool, Self::Error> {
        // TODO: Implement using module-currencies
        Ok(true)
    }

    fn swap_exact_tokens_for_tokens(&self, _path: Vec<u32>, _amount_in: u128, _min_amount_out: u128) -> Result<u128, Self::Error> {
        // TODO: Implement using module-swap
        Ok(0)
    }

    fn get_nft_owner(&self, _collection_id: u32, _item_id: u32) -> Result<Option<AccountAddress>, Self::Error> {
        // TODO: Implement using module-nft
        Ok(None)
    }

    fn transfer_nft(&self, _collection_id: u32, _item_id: u32, _src: AccountAddress, _dst: AccountAddress) -> Result<bool, Self::Error> {
        // TODO: Implement using module-nft
        Ok(true)
    }
}
