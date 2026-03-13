script {
    use substrate::balance;
    use std::string;

    fun balance_simple_api_test(src: signer, dst: address, amount: u128) {
        // Test that the move-stdlib works fine.
        // Since both these stdlib bundles are published under the same address.
        //
        // Create a random string and do some random string operations.
        let s = string::utf8(b"abcd");
        let sub = string::sub_string(&s, 4, 4);
        assert!(string::is_empty(&sub), 22);

        // Test cheque_amount function.
        let cheque_amount = balance::cheque_amount(@0xCAFE);
        assert!(cheque_amount == 0, 0);

        // Test total_amount function.
        let total_amount = balance::total_amount(@0xCAFE);
        assert!(total_amount == 0, 0);

        // Test transfer function.
        let ret = balance::transfer(&src, dst, amount);
        assert!(ret, 0);
    }
}

script {
    use substrate::balance;
    use std::signer;

    fun execute_transfer(src: signer, dst: address, amount: u128) {
        let src_addr = signer::address_of(&src);

        let src_cheque_amount = balance::cheque_amount(src_addr);
        let dst_cheque_amount = balance::cheque_amount(dst);
        assert!(src_cheque_amount == amount, 0);
        assert!(dst_cheque_amount == 0, 0);

        let ret = balance::transfer(&src, dst, amount);
        assert!(ret, 0);

        let src_cheque_amount = balance::cheque_amount(src_addr);
        let dst_cheque_amount = balance::cheque_amount(dst);
        assert!(src_cheque_amount == 0, 0);
        assert!(dst_cheque_amount == amount, 0);
    }
}
