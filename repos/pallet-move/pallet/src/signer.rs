use core::marker::PhantomData;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
    pallet_prelude::RuntimeDebug,
    traits::{
        tokens::{
            currency::{LockIdentifier, LockableCurrency},
            WithdrawReasons,
        },
        Get,
    },
    BoundedBTreeMap, BoundedBTreeSet, Parameter,
};
use frame_system::{pallet_prelude::BlockNumberFor, Config as SysConfig};
use scale_info::TypeInfo;
use sp_runtime::traits::MaybeSerializeDeserialize;
use sp_std::vec::Vec;

use crate::{
    balance::{AccountIdOf, BalanceAdapter, BalanceOf},
    Config, Error,
};

// Some alias definition to make life easier.
pub type MaxSignersOf<T> = <T as Config>::MaxScriptSigners;
pub type SigDataOf<T> = SigData<AccountIdOf<T>, BalanceOf<T>, BlockNumberFor<T>, MaxSignersOf<T>>;

/// This definition stores the hash value of a script transaction.
pub type CallHash = [u8; 32];

/// A simple signature.
#[derive(Clone, Eq, PartialEq, Encode, Decode, Default, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Signature {
    /// Signer has approved the script with the signature.
    Approved,
    /// Signer's approval still missing.
    #[default]
    Missing,
}

#[derive(Clone, Eq, PartialEq)]
pub enum SigDataError {
    MaxSignersExceeded,
    ScriptSignatureFailure,
    UnableToDeserializeAccount,
}

impl<T> From<SigDataError> for Error<T> {
    fn from(err: SigDataError) -> Self {
        match err {
            SigDataError::MaxSignersExceeded => Error::<T>::MaxSignersExceeded,
            SigDataError::ScriptSignatureFailure => Error::<T>::ScriptSignatureFailure,
            SigDataError::UnableToDeserializeAccount => Error::<T>::UnableToDeserializeAccount,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct SignerData<Balance> {
    /// State of the user's signature.
    pub signature: Signature,
    /// Individual cheque-limit.
    pub cheque_limit: Balance,
    /// Lock ID for locked currency.
    pub lock_id: LockIdentifier,
}

/// Storage struct definition for a multi-signer request.
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(Size))]
pub struct SigData<AccountId, Balance, BlockNumber, Size>
where
    AccountId: Parameter + Ord + MaybeSerializeDeserialize,
    Balance: From<u128> + Into<u128> + Default,
    BlockNumber: Parameter + Ord + MaybeSerializeDeserialize + Default,
    Size: Get<u32>,
{
    /// The signers of a script transaction.
    signers: BoundedBTreeMap<AccountId, SignerData<Balance>, Size>,

    /// The block height at which this `SigData` was initally stored.
    ///
    /// Used only for multisig purposes. Is set to `None` otherwise (non-multisig scenarios).
    stored_block_height: Option<BlockNumber>,
}

impl<AccountId, Balance, BlockNumber, Size> SigData<AccountId, Balance, BlockNumber, Size>
where
    AccountId: Parameter + Ord + MaybeSerializeDeserialize,
    Balance: From<u128> + Into<u128> + Default,
    BlockNumber: Parameter + Ord + MaybeSerializeDeserialize + Default,
    Size: Get<u32>,
{
    /// Creates a new [`SigData`] with all blank signatures for the provided script.
    pub fn new(signers: BoundedBTreeSet<AccountId, Size>) -> Result<Self, SigDataError> {
        if signers.len() > (Size::get() as usize) {
            return Err(SigDataError::MaxSignersExceeded);
        }

        let mut sig_info = SigData::<AccountId, Balance, BlockNumber, Size> {
            stored_block_height: None,
            ..Default::default()
        };
        for account in signers.iter() {
            sig_info
                .try_insert(account.clone(), SignerData::default())
                .map_err(|_| SigDataError::UnableToDeserializeAccount)?;
        }

        Ok(sig_info)
    }

    pub fn stored_block_height(&self) -> Option<&BlockNumber> {
        self.stored_block_height.as_ref()
    }

    pub fn set_block_height(&mut self, block_height: BlockNumber) {
        self.stored_block_height = Some(block_height);
    }
}

impl<AccountId, Balance, BlockNumber, Size> core::ops::Deref
    for SigData<AccountId, Balance, BlockNumber, Size>
where
    AccountId: Parameter + Ord + MaybeSerializeDeserialize,
    Balance: From<u128> + Into<u128> + Default,
    BlockNumber: Parameter + Ord + MaybeSerializeDeserialize + Default,
    Size: Get<u32>,
{
    type Target = BoundedBTreeMap<AccountId, SignerData<Balance>, Size>;

    fn deref(&self) -> &Self::Target {
        &self.signers
    }
}

impl<AccountId, Balance, BlockNumber, Size> core::ops::DerefMut
    for SigData<AccountId, Balance, BlockNumber, Size>
where
    AccountId: Parameter + Ord + MaybeSerializeDeserialize,
    Balance: From<u128> + Into<u128> + Default,
    BlockNumber: Parameter + Ord + MaybeSerializeDeserialize + Default,
    Size: Get<u32>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.signers
    }
}

// Because in substrate_move::AccountAddress Default impl is missing.
impl<AccountId, Balance, BlockNumber, Size> Default
    for SigData<AccountId, Balance, BlockNumber, Size>
where
    AccountId: Parameter + Ord + MaybeSerializeDeserialize,
    Balance: From<u128> + Into<u128> + Default,
    BlockNumber: Parameter + Ord + MaybeSerializeDeserialize + Default,
    Size: Get<u32>,
{
    fn default() -> Self {
        SigData {
            signers: BoundedBTreeMap::<AccountId, SignerData<Balance>, Size>::new(),
            stored_block_height: None,
        }
    }
}

/// Script signature handler tracks required signatures for the provided script.
pub(crate) struct ScriptSignatureHandler<T: Config + SysConfig> {
    _pd_config: PhantomData<T>,
    /// All required script signature info.
    sig_info: SigDataOf<T>,
}

impl<T: Config + SysConfig> ScriptSignatureHandler<T> {
    /// Creates a new [`ScriptSignatureHandler`] with all blank signatures for the provided script.
    pub(crate) fn new(
        accounts: BoundedBTreeSet<T::AccountId, T::MaxScriptSigners>,
    ) -> Result<Self, Error<T>> {
        Ok(Self {
            _pd_config: PhantomData,
            sig_info: SigDataOf::<T>::new(accounts).map_err(Into::<Error<T>>::into)?,
        })
    }

    /// Provide a signature to this script.
    ///
    /// In case the script doesn't require the signature, or it doesn't require a signature from
    /// the current signer, the signature shall be ignored, and no error will be thrown for simplicity reasons.
    ///
    /// In case the signer is not the one who should sign the script, the signature shall be stored
    /// until all necessary signatures are collected.
    pub(crate) fn sign_script(
        &mut self,
        account: &T::AccountId,
        cheque_limit: &BalanceOf<T>,
        lock_id: LockIdentifier,
    ) -> Result<(), Error<T>> {
        // Only users that are on the signer list can sign this script.
        let Some(ms_data) = self.sig_info.get_mut(account) else {
            return Err(Error::<T>::UnexpectedUserSignature);
        };

        // Signing again will update the setup parameters.
        if matches!(ms_data.signature, Signature::Approved) {
            T::Currency::remove_lock(ms_data.lock_id, account);
        }

        ms_data.signature = Signature::Approved;
        ms_data.cheque_limit = *cheque_limit;
        ms_data.lock_id = lock_id;
        T::Currency::set_lock(lock_id, account, *cheque_limit, WithdrawReasons::all());
        Ok(())
    }

    /// Check whether the script has been approved by all required signers.
    pub(crate) fn all_signers_approved(&self) -> bool {
        self.sig_info
            .values()
            .all(|signer| signer.signature == Signature::Approved)
    }

    /// Creates a [`BalanceAdapter`] from the internal stored cheque-limits.
    /// Function returns an error if not all signers have signed.
    pub(crate) fn write_cheques(&self) -> Result<BalanceAdapter<T>, Error<T>> {
        let mut balances = BalanceAdapter::<T>::new();
        for (account, ms_data) in self.sig_info.iter() {
            T::Currency::remove_lock(ms_data.lock_id, account);
            balances
                .write_cheque(account, &ms_data.cheque_limit)
                .map_err(|_| Error::<T>::InsufficientBalance)?;
        }

        Ok(balances)
    }

    /// Consumes [`ScriptSignatureHandler`] and returns inner `SigData`.
    pub(crate) fn into_inner(self) -> SigDataOf<T> {
        self.sig_info
    }

    /// Consumes [`ScriptSignatureHandler`] and returns accounts of all signers as vector.
    pub(crate) fn into_signer_accounts(self) -> Result<Vec<T::AccountId>, Error<T>> {
        let accounts: Vec<T::AccountId> = self.sig_info.keys().cloned().collect();
        Ok(accounts)
    }
}

impl<T: Config + SysConfig> From<SigDataOf<T>> for ScriptSignatureHandler<T> {
    fn from(sig_info: SigDataOf<T>) -> Self {
        Self {
            _pd_config: PhantomData,
            sig_info,
        }
    }
}
