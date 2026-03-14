script {
    use substrate::balance;

    fun verify_preconfigured_balance(account: address, preconfigured_amount: u128) {
        let total_amount = balance::total_amount(account);

        assert!(total_amount == preconfigured_amount, 0);
    }
}

script {
    use substrate::balance;

    fun single_transfer(src: signer, dst: address, amount: u128) {
        balance::transfer(&src, dst, amount);
    }
}

script {
    use substrate::balance;

    fun double_transfer(src: signer, dst1: address, amnt1: u128, dst2: address, amnt2: u128) {
        balance::transfer(&src, dst1, amnt1);
        balance::transfer(&src, dst2, amnt2);
    }
}

script {
    use substrate::balance;

    fun fail_at_the_end(src: signer, dst: address, amount: u128) {
        balance::transfer(&src, dst, amount);
        assert!(false, 0);
    }
}
