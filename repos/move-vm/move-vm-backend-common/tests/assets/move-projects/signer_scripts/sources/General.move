script {
    // Should work.
    fun no_param_at_all() {
        // A random script:
        let iterations: u64 = 10;
        while (iterations > 0) {
            iterations = iterations - 1;
        }
    }
}

script {
    // Should work.
    fun no_signers_param_at_all(iterations: u64, _a: u32, _b: u8, _c: u256, _d: address, _e: vector<u32>, _f: bool) {
        // A random script:
        while (iterations > 0) {
            iterations = iterations - 1;
        }
    }
}

script {
    // Should work.
    fun eight_normal_signers(_s1: signer, _s2: signer, _s3: &signer, _s4: signer, _s5: &signer, _s6: signer, _s7: &signer, _s8: &signer, _extra: u32) {}
}

script {
    // Should fail!
    fun extra_signer_in_the_middle_of_the_list(_s: &signer, _a: u32, _b: u8, _eheh: signer, _d: address, _e: vector<u32>, _f: bool) {}
}

script {
    // Should fail!
    fun trying_with_integer_reference(_ref: &u64) {}
}

script {
    // Should fail!
    fun trying_with_mut_integer_reference(_ref: &mut u32) {}
}

script {
    // Should fail!
    fun trying_with_addr_reference(_ref: &address) {}
}

script {
    // Should fail!
    fun trying_with_mut_addr_reference(_ref: &mut address) {}
}

script {
    // Should work.
    fun trying_with_signer_reference(_ref: &signer) {}
}

script {
    // Should fail!
    fun trying_with_mut_signer_reference(_ref: &mut signer) {}
}

script {
    // Should fail!
    fun trying_with_simple_struct(_struct: HelperModule::ModuleWithStructs::SimpleStruct) {}
}

script {
    // Should fail!
    fun trying_with_struct_with_struct_members(_struct: HelperModule::ModuleWithStructs::StructWithStructMembers) {}
}

script {
    // Should fail!
    fun trying_with_struct_with_generics(_struct: HelperModule::ModuleWithStructs::StructWithGenerics<u32>) {}
}
