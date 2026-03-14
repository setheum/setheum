script {
    use CafeAccount::BasicCoin;

    fun publish_basic_balance(s: signer) {
        BasicCoin::publish_balance(&s);
    }
}

script {
    use CafeAccount::BasicCoin;

    fun mint(module_owner: signer, rx_addr: address, amount: u64) {
        //BasicCoin::mint(&module_owner, rx_addr, amount);

        let i = 1;
        while (i <= amount) {
            BasicCoin::mint(&module_owner, rx_addr, 1);
            i = i + 1
        };
    }
}
