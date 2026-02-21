script {
    fun short_cheap_script() {
        assert!(0 == 0, 0);
    }
}

script {
    use DeveloperBob::CarWash;

    fun short_expensive_script(account: signer) {
        let s = 0; 
        while (s < 5000) {
            CarWash::buy_coin(&account, 1);
            CarWash::wash_car(&account);
            s = s + 1;
        }
    }
}
