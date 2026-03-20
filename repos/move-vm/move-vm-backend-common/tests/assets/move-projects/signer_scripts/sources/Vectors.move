script {
    // Should work.
    fun signer_before_integer_vector(_s: &signer, _v: vector<u32>) {}
}

script {
    // Should fail!
    fun signer_after_integer_vector(_v: vector<u32>, _s: &signer) {}
}

script {
    // Should work.
    fun signer_before_all_possible_vectors(_s: signer, _a: vector<u8>, _b: vector<u16>, _c: vector<u32>, _e: vector<u64>, _f: vector<u128>, _g: vector<u256>, _h: vector<address>, _i: vector<bool>) {}
}

script {
    // Should fail!
    fun signer_after_all_possible_vectors(_a: vector<u8>, _b: vector<u16>, _c: vector<u32>, _e: vector<u64>, _f: vector<u128>, _g: vector<u256>, _h: vector<address>, _i: vector<bool>, _s: &signer) {}
}

script {
    // Should fail!
    fun signer_before_ref_vector(_s: &signer, _v: &vector<u32>) {}
}

script {
    // Should fail!
    fun signer_before_mut_ref_vector(_s: signer, _v: &mut vector<bool>) {}
}

script {
    // Should fail!
    fun trying_vector_containing_signer(_v: vector<signer>) {}
}

script {
    // Should fail!
    fun trying_vector_containing_struct(_v: vector<HelperModule::ModuleWithStructs::SimpleStruct>) {}
}

script {
    // Should fail!
    fun trying_vector_containing_struct_with_struct(_v: vector<HelperModule::ModuleWithStructs::StructWithStructMembers>) {}
}

script {
    // Should fail!
    fun trying_vector_containing_struct_with_generic(_v: vector<HelperModule::ModuleWithStructs::StructWithGenerics<u128>>) {}
}

script {
    // Should work.
    fun trying_vector_containing_address_vector(_v: vector<vector<address>>) {}
}

script {
    // Should work.
    fun trying_vector_containing_vector_containing_u8_vector(_v: vector<vector<vector<u8>>>) {}
}

script {
    // Should fail!
    fun trying_vector_containing_signer_vector(_v: vector<vector<signer>>) {}
}
