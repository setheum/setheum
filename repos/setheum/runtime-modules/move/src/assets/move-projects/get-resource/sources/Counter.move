module DeveloperBob::Counter {
    use std::signer;

    // Error codes
    const ACC_DOES_NOT_EXIST: u64 = 4003;
    const ACC_ALREADY_EXISTS: u64 = 4004;

    // Storage and data implementations/definitions
    struct Count has store {
        value: u64
    }

    struct Counter has key {
        counter: Count
    }

    /// Creates a new counter-account for a user. Without account it is not possible to count anything.
    public fun create_counter(account: &signer) {
        let addr = signer::address_of(account);
        assert!(!exists<Counter>(addr), ACC_ALREADY_EXISTS);
        move_to(account, Counter { counter: Count { value: 0 }});
    }

    /// Counts for the registered user by increasing the count by 1. If the given user doesn't exists, the methods throws an error.
    public fun count(account: &signer): u64 acquires Counter {
        let addr = signer::address_of(account);
        assert!(exists<Counter>(addr), ACC_DOES_NOT_EXIST);
        increase_count(&addr);
        count_of(&addr)
    }

    fun count_of(addr: &address): u64 acquires Counter {
        borrow_global<Counter>(*addr).counter.value
    }

    fun increase_count(addr: &address) acquires Counter {
        let count = count_of(addr);
        let counter_ref = &mut borrow_global_mut<Counter>(*addr).counter.value;
        *counter_ref = count + 1;
    }
}
