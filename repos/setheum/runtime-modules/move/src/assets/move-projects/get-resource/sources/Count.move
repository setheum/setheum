script {
    use DeveloperBob::Counter;

    fun count(account: signer) {
        Counter::count(&account);
    }
}
