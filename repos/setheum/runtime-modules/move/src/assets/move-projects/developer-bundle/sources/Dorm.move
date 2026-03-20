/// Bob's dorm. He offers several hireable flats / apartments for student groups.
module DeveloperBob::Dorm {
    use std::signer;
    use std::vector;
    use substrate::balance;

    /// Address of the owner of this module.
    const MODULE_OWNER: address = @DeveloperBob;

    /// Simple solution, fixed price for a flat per month. 
    const FLAT_PRICE_PM: u128 = 90000000000000; // equals 90 UNIT

    /// Error codes.
    const NOT_MODULE_OWNER: u64 = 0;
    const MODULE_NOT_INITIALIZED: u64 = 1;
    const MODULE_ALREADY_INITIALIZED: u64 = 2;
    const NOT_ENOUGH_MONEY: u64 = 3;

    /// Group of students, which rent a flat together.
    struct Group has store {
        list: vector<address>
    }

    /// All tenant groups will be registered here.
    struct Dorm has key {
        groups: vector<Group>
    }

    /// Initialization method for module owner.
    public fun init_module(module_owner: &signer) {
        assert!(signer::address_of(module_owner) == MODULE_OWNER, NOT_MODULE_OWNER);
        assert!(!exists<Dorm>(signer::address_of(module_owner)), MODULE_ALREADY_INITIALIZED);
        move_to(module_owner, Dorm { groups: vector::empty<Group>() });
    }

    /// Creates a new tenant group and signs a rental agreement.
    public fun rent_apartment(
        student1: &signer,
        student2: &signer,
        student3: &signer,
        months: u8) acquires Dorm
    {
        assert!(exists<Dorm>(MODULE_OWNER), MODULE_NOT_INITIALIZED);

        // Check that all of the students have enough money.
        let per_person = FLAT_PRICE_PM * (months as u128) / 3;

        let address1 = signer::address_of(student1);
        let address2 = signer::address_of(student2);
        let address3 = signer::address_of(student3);

        assert!(balance::cheque_amount(address1) >= per_person, NOT_ENOUGH_MONEY);
        assert!(balance::cheque_amount(address2) >= per_person, NOT_ENOUGH_MONEY);
        assert!(balance::cheque_amount(address3) >= per_person, NOT_ENOUGH_MONEY);

        // Transfer the money and register the new tenant group.
        balance::transfer(student1, MODULE_OWNER, per_person);
        balance::transfer(student2, MODULE_OWNER, per_person);
        balance::transfer(student3, MODULE_OWNER, per_person);

        let list = vector::empty<address>();
        vector::push_back(&mut list, address1);
        vector::push_back(&mut list, address2);
        vector::push_back(&mut list, address3);
        let group = Group { list: list };

        let groups_ref = &mut borrow_global_mut<Dorm>(MODULE_OWNER).groups;
        vector::push_back(groups_ref, group);
    }
}
