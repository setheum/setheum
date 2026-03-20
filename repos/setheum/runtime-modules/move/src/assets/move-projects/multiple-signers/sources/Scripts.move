script {
    use DeveloperBob::Dorm;

    fun init_module(account: signer) {
        Dorm::init_module(&account);
    }
}

script {
    use DeveloperBob::Dorm;

    fun rent_apartment(acc1: signer, acc2: signer, acc3: signer, months: u8) {
        Dorm::rent_apartment(&acc1, &acc2, &acc3, months);
    }
}
