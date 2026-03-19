module HelperModule::ModuleWithStructs {
    struct SimpleStruct has drop {
        a: u64,
        b: bool,
        c: u32,
    }

    struct StructWithStructMembers has drop {
        a: SimpleStruct,
        b: u256,
        c: address,
    }

    struct StructWithGenerics<T: store> has drop, key {
        a: T,
        b: u8,
        c: bool,
    }

    public fun does_exists_at_addr<T: key + store>(addr: address): bool {
        exists<StructWithGenerics<T>>(addr)
    }
}
