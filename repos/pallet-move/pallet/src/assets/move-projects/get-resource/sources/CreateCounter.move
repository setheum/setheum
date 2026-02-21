script {
    use DeveloperBob::Counter;

    fun create_counter(account: signer) {
        Counter::create_counter(&account);
    }
}
