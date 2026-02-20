//! Module for handling balance functionality related to the MoveVM balance module.
//!
//! Using the MoveVM it's possible to execute balance changes, but only if the execution script
//! succeeds.
//! The possible balance changes are capped with each user's `cheque_limit` value.

use core::{cmp::Ordering, marker::PhantomData};

use codec::{Decode, Encode};
use frame_support::{
    pallet_prelude::{DispatchError, DispatchResult},
    sp_runtime::SaturatedConversion,
    traits::{Currency, ExistenceRequirement, Imbalance, WithdrawReasons},
};
use frame_system::Config as SysConfig;
use hashbrown::HashMap;
use move_core_types::{account_address::AccountAddress, vm_status::StatusCode};
use move_vm_backend::balance::BalanceHandler;
use sp_runtime::traits::Zero;
use sp_std::{
    cell::{Ref, RefCell},
    rc::Rc,
    vec::Vec,
};

use crate::{Config, Error, Pallet};

// Shortcut type definitions for accessing more easily.
pub type AccountIdOf<T> = <T as SysConfig>::AccountId;
pub type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
pub type CurrencyOf<T> = <T as Config>::Currency;
pub type NegativeImbalanceOf<T> =
    <<T as Config>::Currency as Currency<AccountIdOf<T>>>::NegativeImbalance;

/// Meaningful alias for encoded AccountIds.
pub type EncodedAccount = Vec<u8>;

/// Converts Error to StatusCode.
///
/// This is a rare direction for the error conversion, but it's needed for the balance handler
/// module which is injected into the MoveVM. If anything goes wrong in the [`BalanceHandler`]
/// implementation, we should be able to map the internal Substrate error into [`StatusCode`].
impl<T: Config> From<Error<T>> for StatusCode {
    fn from(err: Error<T>) -> Self {
        match err {
            Error::InsufficientBalance => Self::INSUFFICIENT_BALANCE,
            Error::InvalidAccountSize => Self::UNABLE_TO_DESERIALIZE_ACCOUNT,
            // If we ever see this one, we should update the function here.
            //_ => Self::INTERNAL_TYPE_ERROR,
            // TODO(eiger): Use the above commented code before the final release.
            _ => unreachable!("WARNING: update the conversion function"),
        }
    }
}

/// Balance adapter for providing basic access to balance cheques within the MoveVM.
#[derive(Clone)]
pub struct BalanceAdapter<T: Config + SysConfig> {
    _pd_config: PhantomData<T>,
    /// Virtual cheques record of involved users.
    cheques: Rc<RefCell<HashMap<EncodedAccount, BalanceOf<T>>>>,
    /// Copy of initial state, without tracking it.
    initial_state: HashMap<EncodedAccount, BalanceOf<T>>,
}

impl<T: Config + SysConfig> BalanceAdapter<T> {
    /// Create a new [`BalanceAdapter`].
    pub fn new() -> Self {
        BalanceAdapter {
            _pd_config: PhantomData,
            cheques: Rc::new(RefCell::new(HashMap::new())),
            initial_state: HashMap::new(),
        }
    }

    /// Creates a new [`BalanceAdapter`] with unlimited cheque-limits for every signer for the
    /// usage within a dry-run. Therefor, the extracted arguments and the signer count of a given
    /// script transaction has to be provided.
    pub fn for_dry_run(args: &[&[u8]], signer_count: usize) -> Result<BalanceAdapter<T>, Error<T>> {
        let mut handler = BalanceAdapter::new();

        let accounts = Pallet::<T>::extract_account_ids_from_args(args, signer_count)?;

        for acc in &accounts {
            handler.write_cheque_internal(acc, &BalanceOf::<T>::from(u128::MAX));
        }

        Ok(handler)
    }

    /// Writes a cheque for the account.
    pub fn write_cheque(
        &mut self,
        account: &AccountIdOf<T>,
        balance: &BalanceOf<T>,
    ) -> DispatchResult {
        self.ensure_can_withdraw(account, balance)?;
        self.write_cheque_internal(account, balance);
        Ok(())
    }

    // Internal method for writing the cheque.
    fn write_cheque_internal(&mut self, account: &AccountIdOf<T>, balance: &BalanceOf<T>) {
        let account: EncodedAccount = account.encode();

        let mut cheques = self.cheques.borrow_mut();
        if let Some(src_cheque) = cheques.get_mut(&account) {
            *src_cheque += *balance;
        } else {
            cheques.insert(account.clone(), *balance);
        }

        if let Some(src_cheque) = self.initial_state.get_mut(&account) {
            *src_cheque += *balance;
        } else {
            self.initial_state.insert(account.clone(), *balance);
        }
    }

    /// Executes the true transactions on the blockchain/substrate side after execution of
    /// Move-script.
    ///
    /// Important note: This can only be called from within the pallet.
    pub(super) fn apply_transactions(&self) -> DispatchResult {
        let zero = BalanceOf::<T>::zero();

        self.cmp_with_initial_state()?;

        let cheques = self.cheques.borrow();
        let mut purse = NegativeImbalanceOf::<T>::zero();
        let mut depts = Vec::<(AccountIdOf<T>, BalanceOf<T>)>::new();

        // Calculate balance differences and withdraw negative ones from user's accounts.
        for (account, balance) in cheques.iter() {
            let true_balance = self.initial_state.get(account).unwrap_or(&zero);
            let account_id = vec_to_account_id::<T>(account)?;
            match (*true_balance).cmp(balance) {
                Ordering::Greater => {
                    let dept = *true_balance - *balance;
                    let imbalance = T::Currency::withdraw(
                        &account_id,
                        dept,
                        WithdrawReasons::TRANSFER,
                        ExistenceRequirement::AllowDeath,
                    )?;
                    purse = purse.merge(imbalance);
                }
                Ordering::Less => {
                    let dept = *balance - *true_balance;
                    depts.push((account_id, dept));
                }
                Ordering::Equal => {}
            }
        }

        // Now deposit depts from purse to new owners.
        for (account, balance) in depts.into_iter() {
            let imbalance = purse.extract(balance);
            T::Currency::resolve_creating(&account, imbalance);
        }

        Ok(())
    }

    /// Ensures that user can withdraw that given amount of money, which eventually will be used
    /// within the Move-script execution.
    fn ensure_can_withdraw(
        &self,
        account: &AccountIdOf<T>,
        amount: &BalanceOf<T>,
    ) -> DispatchResult {
        if *amount <= T::Currency::free_balance(account) {
            Ok(())
        } else {
            Err(Error::<T>::InsufficientBalance.into())
        }
    }

    /// Does a state checking on initial state of cheques with current state.
    fn cmp_with_initial_state(&self) -> DispatchResult {
        let cheques: Ref<HashMap<EncodedAccount, BalanceOf<T>>> = self.cheques.borrow();

        let sum_initial = self
            .initial_state
            .values()
            .fold(BalanceOf::<T>::zero(), |acc, x| acc + *x);
        let sum_cheques = cheques
            .values()
            .fold(BalanceOf::<T>::zero(), |acc, x| acc + *x);

        if sum_initial == sum_cheques {
            Ok(())
        } else {
            Err(DispatchError::Corruption)
        }
    }
}

impl<T: Config> Default for BalanceAdapter<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Config> BalanceHandler for BalanceAdapter<T> {
    type Error = StatusCode;

    fn transfer(
        &self,
        src: AccountAddress,
        dst: AccountAddress,
        cheque_amount: u128,
    ) -> Result<bool, Self::Error> {
        let from: EncodedAccount = Pallet::<T>::to_native_account(&src)?.encode();
        let to: EncodedAccount = Pallet::<T>::to_native_account(&dst)?.encode();
        let amount = BalanceOf::<T>::from(cheque_amount);

        let mut cheques = self.cheques.borrow_mut();

        let src_balance = cheques.entry(from).or_insert(BalanceOf::<T>::zero());
        if *src_balance < amount {
            return Err(StatusCode::INSUFFICIENT_BALANCE);
        }
        *src_balance -= amount;

        if let Some(dst_balance) = cheques.get_mut(&to) {
            *dst_balance += amount;
        } else {
            cheques.insert(to, amount);
        }

        Ok(true)
    }

    fn cheque_amount(&self, account: AccountAddress) -> Result<u128, Self::Error> {
        let zero = BalanceOf::<T>::zero();
        let acc: EncodedAccount = Pallet::<T>::to_native_account(&account)?.encode();
        let cheques = self.cheques.borrow();
        let balance = cheques.get(&acc).unwrap_or(&zero);
        Ok((*balance).into())
    }

    fn total_amount(&self, account: AccountAddress) -> Result<u128, Self::Error> {
        let native_account: T::AccountId =
            Pallet::<T>::to_native_account(&account).map_err(Into::<Self::Error>::into)?;

        let amount = T::Currency::free_balance(&native_account).saturated_into::<u128>();
        Ok(amount)
    }
}

fn vec_to_account_id<T: SysConfig>(vec: &EncodedAccount) -> Result<AccountIdOf<T>, DispatchError> {
    let mut ref_acc: &[u8] = vec;
    AccountIdOf::<T>::decode(&mut ref_acc)
        .map_err(|_| DispatchError::Other("Decode::decode error for T::AccountId"))
}
