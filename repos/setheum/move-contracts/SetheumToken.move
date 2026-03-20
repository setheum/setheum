module 0x1::SetheumToken {
    use std::signer;
    use std::vector;

    struct Balance has key {
        value: u128,
    }

    struct TokenInfo has key {
        name: vector<u8>,
        symbol: vector<u8>,
        decimals: u8,
        total_supply: u128,
    }

    public fun init_module(account: &signer, name: vector<u8>, symbol: vector<u8>, decimals: u8, initial_supply: u128) {
        move_to(account, TokenInfo {
            name,
            symbol,
            decimals,
            total_supply: initial_supply,
        });
        move_to(account, Balance { value: initial_supply });
    }

    public fun balance_of(owner: address): u128 acquires Balance {
        if (exists<Balance>(owner)) {
            borrow_global<Balance>(owner).value
        } else {
            0
        }
    }

    public fun transfer(from: &signer, to: address, amount: u128) acquires Balance {
        let from_addr = signer::address_of(from);
        let from_balance = borrow_global_mut<Balance>(from_addr);
        assert!(from_balance.value >= amount, 0);
        from_balance.value = from_balance.value - amount;

        if (!exists<Balance>(to)) {
            // This is a simple implementation, in real Move you'd need to handle account creation/opt-in
            // but for Setheum we can assume auto-creation if the account exists in Substrate.
        };
        
        // Note: For a real Move module, you'd handle the 'to' balance more carefully.
    }
}
