use move_core_types::account_address::AccountAddress;
use move_core_types::vm_status::StatusCode;
use move_vm_backend::balance::BalanceHandler;
use move_vm_backend::storage::Storage;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Mock storage implementation for testing.
#[derive(Clone, Debug)]
pub struct StorageMock {
    pub data: Rc<RefCell<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl StorageMock {
    pub fn new() -> StorageMock {
        StorageMock {
            data: Rc::new(RefCell::new(Default::default())),
        }
    }
}

impl Default for StorageMock {
    fn default() -> Self {
        StorageMock::new()
    }
}

impl Storage for StorageMock {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let data = self.data.borrow();
        data.get(key).map(|blob| blob.to_owned())
    }

    fn set(&self, key: &[u8], value: &[u8]) {
        let mut data = self.data.borrow_mut();
        data.insert(key.to_owned(), value.to_owned());
    }

    fn remove(&self, key: &[u8]) {
        let mut data = self.data.borrow_mut();
        data.remove(key);
    }
}

// Mock balance handler implementation for testing.
#[derive(Clone, Debug)]
pub struct BalanceMock {
    cheques: Rc<RefCell<HashMap<AccountAddress, u128>>>,
}

impl BalanceMock {
    pub fn new() -> Self {
        Self {
            cheques: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn write_cheque(&mut self, account: AccountAddress, amount: u128) {
        let mut cheques = self.cheques.borrow_mut();

        if let Some(current_amount) = cheques.get_mut(&account) {
            *current_amount += amount;
        } else {
            cheques.insert(account, amount);
        }
    }
}

impl BalanceHandler for BalanceMock {
    type Error = StatusCode;

    fn transfer(
        &self,
        src: AccountAddress,
        dst: AccountAddress,
        cheque_amount: u128,
    ) -> Result<bool, Self::Error> {
        let mut cheques = self.cheques.borrow_mut();

        let src_balance = cheques.entry(src).or_insert(0);
        if *src_balance < cheque_amount {
            return Err(StatusCode::INSUFFICIENT_BALANCE);
        }
        *src_balance -= cheque_amount;

        if let Some(dst_balance) = cheques.get_mut(&dst) {
            *dst_balance += cheque_amount;
        } else {
            cheques.insert(dst, cheque_amount);
        }

        Ok(true)
    }

    fn cheque_amount(&self, account: AccountAddress) -> Result<u128, Self::Error> {
        Ok(*self.cheques.borrow().get(&account).unwrap_or(&0))
    }

    fn total_amount(&self, account: AccountAddress) -> Result<u128, Self::Error> {
        // We won't need it here.
        self.cheque_amount(account)
    }
}
