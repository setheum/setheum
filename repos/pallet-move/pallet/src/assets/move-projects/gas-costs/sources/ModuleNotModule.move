script {
    fun uses_no_module() {
        let a: u64 = 0;
        let sum: u64 = 0;
        let cnt = 0;

        while (cnt < 5000) {
            a = a + 1;
            sum = sum + a;
            cnt = cnt + 1;
        };

        assert!(sum > 5000, 0);
    }
}

script {
    use DeveloperBob::TheModule;

    fun uses_module() {
        let a: u64 = 0;
        let sum: u64 = 0;
        let cnt = 0;

        while (cnt < 5000) {
            (sum, a) = TheModule::stupid_algorithm(sum, a);
            cnt = cnt + 1;
        };

        assert!(sum > 5000, 0);
    }
}
