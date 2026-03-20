script {
    use std::signer;

    // Should fail!
    fun signer_before_generic<T: copy + drop>(s: signer, x: T) {
        let _y = x;

        // A way to make some use of this signer.
        assert!(signer::address_of(&s) == @0xCAFE, 0);
    }
}

script {
    use std::signer;

    // Should fail!
    fun two_signers_before_generic<T: copy + drop>(s1: signer, s2: signer, x: T) {
        let _y = x;

        // A way to make some use of these signers.
        assert!(signer::address_of(&s1) == @0xCAFE, 0);
        assert!(signer::address_of(&s2) != @0xCAFE, 1);
    }
}

script {
    use std::signer;

    // Should fail!
    fun signer_after_generic<T: copy + drop>(x: T, s: signer) {
        let _y = x;

        // A way to make some use of this signer.
        assert!(signer::address_of(&s) == @0xCAFE, 0);
    }
}

script {
    use std::signer;

    // Should fail!
    fun signer_before_ref_generic<T: copy + drop>(s: &signer, x: &T, _extra_param: u32) {
        let _y = x;

        // A way to make some use of this signer.
        assert!(signer::address_of(s) == @0xCAFE, 0);
    }
}

script {
    use std::signer;

    // Should work.
    fun simple_function_with_generic_inside_with_signer_param<T: key + store>(s: &signer) {
        let exists: bool = HelperModule::ModuleWithStructs::does_exists_at_addr<T>(signer::address_of(s));
        assert!(exists == false, 0);
    }
}

script {
    // Should work.
    fun simple_function_with_generic_inside_without_signer_param<T: key + store>(addr: address) {
        let exists: bool = HelperModule::ModuleWithStructs::does_exists_at_addr<T>(addr);
        assert!(exists == false, 0);
    }
}
