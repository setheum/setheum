use move_core_types::{account_address::AccountAddress, vm_status::StatusCode};

/// Trait for a balance handler.
///
/// This is used to provide an access to external balance handling functionality from within the
/// MoveVM.
pub trait BalanceHandler {
    type Error: Into<StatusCode>;

    fn transfer(
        &self,
        src: AccountAddress,
        dst: AccountAddress,
        cheque_amount: u128,
    ) -> Result<bool, Self::Error>;

    fn cheque_amount(&self, account: AccountAddress) -> Result<u128, Self::Error>;

    fn total_amount(&self, account: AccountAddress) -> Result<u128, Self::Error>;
}

/// An unused [`BalanceHandler`] implementation that is needed for special cases (genesis configuration).
pub(crate) struct DummyBalanceHandler;

impl BalanceHandler for DummyBalanceHandler {
    type Error = StatusCode;

    fn transfer(
        &self,
        _src: AccountAddress,
        _dst: AccountAddress,
        _cheque_amount: u128,
    ) -> Result<bool, Self::Error> {
        unreachable!()
    }

    fn cheque_amount(&self, _account: AccountAddress) -> Result<u128, Self::Error> {
        unreachable!()
    }

    fn total_amount(&self, _account: AccountAddress) -> Result<u128, Self::Error> {
        unreachable!()
    }
}
