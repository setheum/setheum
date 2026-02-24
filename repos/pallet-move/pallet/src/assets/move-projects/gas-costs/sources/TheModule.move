module DeveloperBob::TheModule {
    // Useless algorithm to test difference between function calls and direct implementation.
    public fun stupid_algorithm(sum: u64, a: u64): (u64, u64) {
        a = a + 1;
        sum = sum + a;
        return (sum, a)
    }
}
