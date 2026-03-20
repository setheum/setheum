/// Not a perfect but simple example, for usage in tutorial only!
module DeveloperBob::CarWash {
    use std::signer;
    use substrate::balance;

    /// Address of the owner of this module.
    const MODULE_OWNER: address = @DeveloperBob;

    /// Simple solution, fixed price for one washing coin.
    const COIN_PRICE: u128 = 1000000000000; // equals 1 UNIT

    /// Error codes
    const NOT_MODULE_OWNER: u64 = 0;
    const MODULE_NOT_INITIALIZED: u64 = 1;
    const MODULE_ALREADY_INITIALIZED: u64 = 2;
    const USER_ALREADY_EXISTS: u64 = 3;
    const USER_DOES_NOT_EXIST: u64 = 4;
    const NO_COINS_AVAILABLE: u64 = 5;
    const COIN_LIMIT_REACHED: u64 = 6;
    const NOT_ENOUGH_COINS_AVAILABLE: u64 = 7;

    /// Struct stores number of coins for each user.
    struct Balance has key, store {
        coins: u8
    }

    /// Method executes the ICO without money. The module owner (also car wash owner) gets deposited the minted washing coins.
    public fun initial_coin_minting(module_owner: &signer) {
        // Only the owner of the module can initialize this module
        assert!(signer::address_of(module_owner) == MODULE_OWNER, NOT_MODULE_OWNER);

        // Do not initialize the module twice!
        assert!(!exists<Balance>(signer::address_of(module_owner)), MODULE_ALREADY_INITIALIZED);

        // Deposit maximum number of coins to module owner's account. 
        move_to(module_owner, Balance { coins: 255 });
    }

    /// Registers a new user. The account address will be added to the storage Balance with zero initial washing coins.
    public fun register_new_user(account: &signer) {
        // Verify user does not already exist.
        assert!(!exists<Balance>(signer::address_of(account)), USER_ALREADY_EXISTS);

        // Store new account with zero coins to storage.
        move_to(account, Balance { coins: 0 });
    }

    /// Buys `count` washing coin(s) for the car wash. Therfore, `COIN_PRICE` * `count` will be withdrawn from the user's account.
    public fun buy_coin(user: &signer, count: u8) acquires Balance {
        // Verify, module has been initialized.
        assert!(exists<Balance>(MODULE_OWNER), MODULE_NOT_INITIALIZED);

        // Verify, that this user does exist / is registered.
        assert!(exists<Balance>(signer::address_of(user)), USER_DOES_NOT_EXIST);

        // Verify, that enough coins are available.
        let coins = borrow_global<Balance>(MODULE_OWNER).coins;
        assert!(coins >= count, NOT_ENOUGH_COINS_AVAILABLE);

        // Transfer coin price * count from user to car-wash-and-module-owner
        balance::transfer(user, MODULE_OWNER, (count as u128)*COIN_PRICE);

        // After success, we deposit `count` more washing coin(s) at the user's account.
        transfer_coin(MODULE_OWNER, signer::address_of(user), count);
    }

    /// Initiates the washing process by paying one washing coin.
    public fun wash_car(user: &signer) acquires Balance {
        let user_addr = signer::address_of(user);

        // Verify, that user is registerd / exists.
        assert!(exists<Balance>(user_addr), USER_DOES_NOT_EXIST);

        // Successful transfer of one coin will automatically start the washing process.
        transfer_coin(user_addr, MODULE_OWNER, 1);
    }

    /// Generic coin transfer function. Transfers `count` washing coin(s).
    /// Requires both accounts to exist! For module internal usage only!
    fun transfer_coin(src: address, dst: address, count: u8) acquires Balance {
        // Check that source account has at least `count` washing coin.
        let coins_src = borrow_global<Balance>(src).coins;
        assert!(coins_src >= count, NO_COINS_AVAILABLE);

        // Check that the destination will have less than the maximum number of coins.
        let coins_dst = borrow_global<Balance>(dst).coins;
        assert!(coins_dst + count <= 255, COIN_LIMIT_REACHED);

        // Withdraw `count` washing coin(s).
        let coins_ref = &mut borrow_global_mut<Balance>(src).coins;
        *coins_ref = coins_src - count;

        // Deposit `count` washing coin(s).
        let coins_ref = &mut borrow_global_mut<Balance>(dst).coins;
        *coins_ref = coins_dst + count;
    }
}
